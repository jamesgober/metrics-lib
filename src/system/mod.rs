//! Process and host metrics (feature `system`).
//!
//! Two entry points:
//!
//! - [`ProcessCollector`] samples the standard Prometheus process metrics
//!   (`process_cpu_seconds_total`, `process_resident_memory_bytes`, …) on
//!   demand. The Prometheus/OpenMetrics exporters use it to add `process_*`
//!   series to a scrape.
//! - [`SystemHealth`] keeps a cached view of host and process usage
//!   (CPU %, memory, load, threads, file descriptors) refreshed by a
//!   background thread, plus a configurable 0–100 health score. Reads are
//!   single atomic loads.
//!
//! # Platform support
//!
//! | Data | Linux | macOS / BSD | Windows |
//! |---|---|---|---|
//! | host CPU % (delta between samples) | `/proc/stat` | sysinfo | sysinfo |
//! | host memory | `/proc/meminfo` | sysinfo | sysinfo |
//! | load average | `/proc/loadavg` | sysinfo | not available (`None`) |
//! | process CPU time, RSS | `/proc/self/stat` | sysinfo | sysinfo |
//! | virtual memory | `/proc/self/stat` | sysinfo | sysinfo |
//! | open file descriptors | `/proc/self/fd` | sysinfo | handle count |
//! | max file descriptors | `/proc/self/limits` | sysinfo | sysinfo |
//! | thread count | `/proc/self/stat` | not available (`None`) | not available (`None`) |
//! | process start time | `/proc/self/stat` + `btime` | sysinfo | sysinfo |
//!
//! Values that a platform cannot provide are `None`, never a made-up
//! constant, and the exporters omit the corresponding series.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(not(target_os = "linux"))]
mod other;
#[cfg(any(target_os = "linux", test))]
pub(crate) mod procfs;

#[cfg(target_os = "linux")]
use linux::Platform;
#[cfg(not(target_os = "linux"))]
use other::Platform;

use crate::{MetricsError, Result};
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Default interval between background samples.
pub const DEFAULT_SAMPLE_INTERVAL: Duration = Duration::from_secs(1);
/// Shortest accepted background interval (shorter values are raised to it).
pub const MIN_SAMPLE_INTERVAL: Duration = Duration::from_millis(50);
/// Longest single sleep of the sampler before it re-checks for shutdown.
const MAX_SLEEP_CHUNK: Duration = Duration::from_secs(1);

/// The standard Prometheus process metrics.
///
/// Field names follow the series they export as
/// (`process_cpu_seconds_total`, …). `None` means the platform does not
/// provide the value; see the [module documentation](self).
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct ProcessMetrics {
    /// User plus system CPU time consumed, in seconds.
    pub cpu_seconds_total: f64,
    /// Resident set size, in bytes.
    pub resident_memory_bytes: u64,
    /// Virtual memory size, in bytes.
    pub virtual_memory_bytes: Option<u64>,
    /// Open file descriptors (open handles on Windows).
    pub open_fds: Option<u64>,
    /// Soft limit on open file descriptors.
    pub max_fds: Option<u64>,
    /// Number of OS threads.
    pub threads: Option<u64>,
    /// Process start time, in seconds since the Unix epoch.
    pub start_time_seconds: Option<f64>,
}

/// Host-wide sample produced by the platform backend.
#[derive(Debug, Clone, Default)]
pub(crate) struct HostSample {
    /// Busy CPU share since the previous sample; `None` on the first one.
    pub(crate) cpu_percent: Option<f64>,
    pub(crate) memory_total_bytes: u64,
    pub(crate) memory_used_bytes: u64,
    pub(crate) load_average: Option<f64>,
}

/// On-demand sampler for [`ProcessMetrics`].
///
/// Cheap to keep around: on Linux a collection reads three small `/proc`
/// files; elsewhere it refreshes one process through `sysinfo`.
///
/// # Examples
///
/// ```
/// use metrics_lib::ProcessCollector;
///
/// let collector = ProcessCollector::new();
/// let metrics = collector.collect().unwrap();
/// assert!(metrics.resident_memory_bytes > 0);
/// assert!(metrics.cpu_seconds_total >= 0.0);
/// ```
pub struct ProcessCollector {
    platform: Mutex<Platform>,
}

impl ProcessCollector {
    /// Create a collector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            platform: Mutex::new(Platform::new()),
        }
    }

    /// Sample the process now.
    ///
    /// # Errors
    ///
    /// Returns the I/O error when the platform source cannot be read (for
    /// example `/proc` is not mounted) or the process cannot be found.
    pub fn collect(&self) -> io::Result<ProcessMetrics> {
        self.platform
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .process()
    }
}

impl Default for ProcessCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ProcessCollector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessCollector").finish_non_exhaustive()
    }
}

/// One threshold/penalty pair of a [`HealthConfig`] rule.
///
/// When a value exceeds `threshold`, `penalty` is subtracted from the health
/// score. Within one rule, list steps in **descending** threshold order: the
/// first match wins.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Step {
    /// The value must exceed this for the penalty to apply.
    pub threshold: f64,
    /// Points subtracted from the score (which starts at 100).
    pub penalty: f64,
}

impl Step {
    /// Construct a step.
    #[must_use]
    pub const fn new(threshold: f64, penalty: f64) -> Self {
        Self { threshold, penalty }
    }
}

/// Thresholds for [`SystemHealth::health_score`].
///
/// # Examples
///
/// ```
/// use metrics_lib::{HealthConfig, Step};
///
/// let config = HealthConfig::default().with_system_cpu(vec![
///     Step::new(70.0, 30.0),
///     Step::new(50.0, 15.0),
/// ]);
/// assert_eq!(config.system_cpu.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct HealthConfig {
    /// Steps on host CPU usage, percent (0–100).
    pub system_cpu: Vec<Step>,
    /// Steps on the one-minute load average divided by the CPU count
    /// (`2.0` trips when load exceeds twice the cores). Skipped when the
    /// platform has no load average.
    pub load_avg: Vec<Step>,
    /// Steps on process CPU usage, percent of all cores (0–100).
    pub process_cpu: Vec<Step>,
    /// Steps on host memory in use, percent of total (0–100).
    pub memory_percent: Vec<Step>,
    /// Steps on the process thread count. Skipped when unavailable.
    pub threads: Vec<Step>,
    /// Steps on the process open file descriptors. Skipped when unavailable.
    pub fds: Vec<Step>,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            system_cpu: vec![
                Step::new(80.0, 30.0),
                Step::new(60.0, 15.0),
                Step::new(40.0, 5.0),
            ],
            load_avg: vec![
                Step::new(2.0, 25.0),
                Step::new(1.5, 10.0),
                Step::new(1.0, 5.0),
            ],
            process_cpu: vec![Step::new(50.0, 15.0), Step::new(25.0, 8.0)],
            memory_percent: vec![Step::new(90.0, 10.0), Step::new(80.0, 5.0)],
            threads: vec![
                Step::new(1000.0, 20.0),
                Step::new(500.0, 10.0),
                Step::new(200.0, 5.0),
            ],
            fds: vec![
                Step::new(10_000.0, 15.0),
                Step::new(5_000.0, 8.0),
                Step::new(1_000.0, 3.0),
            ],
        }
    }
}

macro_rules! health_setter {
    ($($field:ident => $setter:ident),* $(,)?) => {
        impl HealthConfig {
            $(
                #[doc = concat!("Replace the `", stringify!($field), "` steps.")]
                #[must_use]
                pub fn $setter(mut self, steps: Vec<Step>) -> Self {
                    self.$field = steps;
                    self
                }
            )*
        }
    };
}

health_setter!(
    system_cpu => with_system_cpu,
    load_avg => with_load_avg,
    process_cpu => with_process_cpu,
    memory_percent => with_memory_percent,
    threads => with_threads,
    fds => with_fds,
);

fn penalty(value: Option<f64>, steps: &[Step]) -> f64 {
    value
        .and_then(|v| steps.iter().find(|s| v > s.threshold))
        .map_or(0.0, |s| s.penalty)
}

/// Health classification derived from the score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum HealthStatus {
    /// Score of 80 or more.
    Healthy,
    /// Score from 60 to 80.
    Warning,
    /// Score from 40 to 60.
    Degraded,
    /// Score below 40.
    Critical,
}

impl HealthStatus {
    /// `true` for [`HealthStatus::Degraded`] and [`HealthStatus::Critical`].
    #[must_use]
    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded | Self::Critical)
    }

    /// `true` for [`HealthStatus::Healthy`].
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// `true` for anything but [`HealthStatus::Healthy`].
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.is_healthy()
    }

    fn from_score(score: f64) -> Self {
        if score >= 80.0 {
            Self::Healthy
        } else if score >= 60.0 {
            Self::Warning
        } else if score >= 40.0 {
            Self::Degraded
        } else {
            Self::Critical
        }
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Healthy => "Healthy",
            Self::Warning => "Warning",
            Self::Degraded => "Degraded",
            Self::Critical => "Critical",
        })
    }
}

/// Snapshot of everything [`SystemHealth`] tracks.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct SystemSnapshot {
    /// Host CPU usage, percent (0 until a second sample exists).
    pub system_cpu_percent: f64,
    /// Process CPU usage, percent of all cores (0 until a second sample).
    pub process_cpu_percent: f64,
    /// Host memory, total bytes.
    pub memory_total_bytes: u64,
    /// Host memory in use, bytes.
    pub memory_used_bytes: u64,
    /// Process resident memory, bytes.
    pub process_memory_bytes: u64,
    /// One-minute load average, when the platform has one.
    pub load_average: Option<f64>,
    /// Process thread count, when available.
    pub thread_count: Option<u32>,
    /// Process open file descriptors, when available.
    pub fd_count: Option<u32>,
    /// Health score, 0–100.
    pub health_score: f64,
    /// Time since the cached values were last refreshed.
    pub last_update: Duration,
}

/// Process-focused statistics from [`SystemHealth::process`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct ProcessStats {
    /// CPU usage, percent of all cores.
    pub cpu_percent: f64,
    /// Resident memory, MiB.
    pub memory_mb: f64,
    /// Thread count, when available.
    pub threads: Option<u32>,
    /// Open file descriptors (handles on Windows), when available.
    pub file_handles: Option<u32>,
    /// Time since the process started (since the monitor was created when
    /// the platform does not report a start time).
    pub uptime: Duration,
}

/// Sentinel for "not available" in the `u64` atomics.
const NONE_U64: u64 = u64::MAX;

#[derive(Default)]
struct Cached {
    system_cpu: AtomicU64,
    process_cpu: AtomicU64,
    memory_total: AtomicU64,
    memory_used: AtomicU64,
    process_rss: AtomicU64,
    load: AtomicU64,
    threads: AtomicU64,
    fds: AtomicU64,
    health: AtomicU64,
    last_update_ns: AtomicU64,
}

struct SamplerState {
    platform: Platform,
    /// `(cpu_seconds_total, when)` of the previous process sample.
    previous_process: Option<(f64, Instant)>,
    latest_process: ProcessMetrics,
}

struct HealthInner {
    state: Mutex<SamplerState>,
    cached: Cached,
    config: HealthConfig,
    cores: f64,
    created_at: Instant,
}

fn store_f64(cell: &AtomicU64, v: f64) {
    cell.store(v.to_bits(), Ordering::Relaxed);
}

fn load_f64(cell: &AtomicU64) -> f64 {
    f64::from_bits(cell.load(Ordering::Relaxed))
}

fn store_opt(cell: &AtomicU64, v: Option<u64>) {
    cell.store(v.unwrap_or(NONE_U64), Ordering::Relaxed);
}

fn load_opt_u32(cell: &AtomicU64) -> Option<u32> {
    match cell.load(Ordering::Relaxed) {
        NONE_U64 => None,
        v => Some(u32::try_from(v).unwrap_or(u32::MAX)),
    }
}

impl HealthInner {
    #[allow(clippy::cast_precision_loss)] // core and byte counts are far below 2^53
    fn new(config: HealthConfig) -> Self {
        let cached = Cached::default();
        store_f64(&cached.health, 100.0);
        store_f64(&cached.load, f64::NAN);
        cached.threads.store(NONE_U64, Ordering::Relaxed);
        cached.fds.store(NONE_U64, Ordering::Relaxed);
        Self {
            state: Mutex::new(SamplerState {
                platform: Platform::new(),
                previous_process: None,
                latest_process: ProcessMetrics::default(),
            }),
            cached,
            config,
            cores: std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
                as f64,
            created_at: Instant::now(),
        }
    }

    #[allow(clippy::cast_precision_loss)] // byte counts are far below 2^53
    fn update(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let now = Instant::now();
        let c = &self.cached;
        if let Ok(host) = state.platform.host() {
            if let Some(cpu) = host.cpu_percent {
                store_f64(&c.system_cpu, cpu);
            }
            c.memory_total
                .store(host.memory_total_bytes, Ordering::Relaxed);
            c.memory_used
                .store(host.memory_used_bytes, Ordering::Relaxed);
            store_f64(&c.load, host.load_average.unwrap_or(f64::NAN));
        }
        if let Ok(process) = state.platform.process() {
            if let Some((prev_cpu, prev_at)) = state.previous_process {
                let wall = now.duration_since(prev_at).as_secs_f64();
                if wall > 0.0 {
                    let busy = (process.cpu_seconds_total - prev_cpu).max(0.0);
                    store_f64(
                        &c.process_cpu,
                        (busy / wall / self.cores * 100.0).clamp(0.0, 100.0),
                    );
                }
            }
            state.previous_process = Some((process.cpu_seconds_total, now));
            c.process_rss
                .store(process.resident_memory_bytes, Ordering::Relaxed);
            store_opt(&c.threads, process.threads);
            store_opt(&c.fds, process.open_fds);
            state.latest_process = process;
        }
        store_f64(&c.health, self.score());
        c.last_update_ns.store(
            crate::util::duration_to_nanos(self.created_at.elapsed()),
            Ordering::Relaxed,
        );
    }

    #[allow(clippy::cast_precision_loss)] // byte counts are far below 2^53
    fn score(&self) -> f64 {
        let c = &self.cached;
        let cfg = &self.config;
        let total = c.memory_total.load(Ordering::Relaxed);
        let memory_percent = (total > 0)
            .then(|| c.memory_used.load(Ordering::Relaxed) as f64 / total as f64 * 100.0);
        let load = load_f64(&c.load);
        let as_f64 = |v: Option<u32>| v.map(f64::from);
        let score = 100.0
            - penalty(Some(load_f64(&c.system_cpu)), &cfg.system_cpu)
            - penalty(
                (!load.is_nan()).then(|| load / self.cores.max(1.0)),
                &cfg.load_avg,
            )
            - penalty(Some(load_f64(&c.process_cpu)), &cfg.process_cpu)
            - penalty(memory_percent, &cfg.memory_percent)
            - penalty(as_f64(load_opt_u32(&c.threads)), &cfg.threads)
            - penalty(as_f64(load_opt_u32(&c.fds)), &cfg.fds);
        score.clamp(0.0, 100.0)
    }
}

struct SamplerThread {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for SamplerThread {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(t) = self.thread.take() {
            t.thread().unpark();
            // A panicking sampler has nothing left to clean up; the cached
            // values simply stop changing.
            let _ = t.join();
        }
    }
}

fn spawn_sampler(inner: Arc<HealthInner>, interval: Duration) -> io::Result<SamplerThread> {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    let thread = thread::Builder::new()
        .name("metrics-lib-system-sampler".into())
        .spawn(move || {
            while !flag.load(Ordering::Acquire) {
                let deadline = Instant::now() + interval;
                while let Some(left) = deadline.checked_duration_since(Instant::now()) {
                    if flag.load(Ordering::Acquire) || left.is_zero() {
                        break;
                    }
                    thread::park_timeout(left.min(MAX_SLEEP_CHUNK));
                }
                if flag.load(Ordering::Acquire) {
                    return;
                }
                inner.update();
            }
        })?;
    Ok(SamplerThread {
        stop,
        thread: Some(thread),
    })
}

/// Cached host and process usage with a background sampler.
///
/// Every reader method is a single relaxed atomic load: no locking and no
/// system call on the read path. The values are refreshed by a background
/// thread every interval (default [`DEFAULT_SAMPLE_INTERVAL`]), or only on
/// [`SystemHealth::update`] in manual mode.
///
/// If the sampler thread cannot be spawned (for example on targets without
/// threads), [`SystemHealth::with_config`] falls back to manual mode;
/// [`SystemHealth::is_sampling`] reports which mode is active, and with the
/// `tracing` feature a warning is logged. Use
/// [`SystemHealth::try_with_config`] to treat that as an error instead.
///
/// CPU percentages are deltas between two samples and read `0` until the
/// second sample.
///
/// # Examples
///
/// ```
/// use metrics_lib::SystemHealth;
///
/// let health = SystemHealth::manual();
/// health.update();
/// assert!(health.mem_used_mb() <= health.mem_total_mb());
/// assert!((0.0..=100.0).contains(&health.health_score()));
/// ```
pub struct SystemHealth {
    inner: Arc<HealthInner>,
    sampler: Option<SamplerThread>,
    interval: Duration,
}

impl SystemHealth {
    /// Monitor with a background sampler every [`DEFAULT_SAMPLE_INTERVAL`].
    #[must_use]
    pub fn new() -> Self {
        Self::with_interval(DEFAULT_SAMPLE_INTERVAL)
    }

    /// Monitor with a background sampler every `interval` (raised to at
    /// least [`MIN_SAMPLE_INTERVAL`]); `Duration::ZERO` means manual mode.
    #[must_use]
    pub fn with_interval(interval: Duration) -> Self {
        Self::with_config(interval, HealthConfig::default())
    }

    /// Monitor without a background thread; call [`SystemHealth::update`]
    /// to refresh.
    #[must_use]
    pub fn manual() -> Self {
        Self::with_interval(Duration::ZERO)
    }

    /// Monitor with a custom interval and score thresholds. Falls back to
    /// manual mode when the sampler thread cannot be spawned.
    #[must_use]
    pub fn with_config(interval: Duration, config: HealthConfig) -> Self {
        match Self::build(interval, config) {
            Ok(health) => health,
            Err((health, error)) => {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    error = %error,
                    "metrics-lib: could not spawn the system sampler thread; SystemHealth is in manual mode"
                );
                #[cfg(not(feature = "tracing"))]
                drop(error);
                health
            }
        }
    }

    /// Like [`SystemHealth::with_config`] but reports a sampler spawn
    /// failure instead of falling back to manual mode.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Config`] when the background thread cannot be
    /// spawned.
    pub fn try_with_config(interval: Duration, config: HealthConfig) -> Result<Self> {
        Self::build(interval, config).map_err(|(_, e)| {
            MetricsError::Config(format!("could not spawn the system sampler thread: {e}").into())
        })
    }

    fn build(
        interval: Duration,
        config: HealthConfig,
    ) -> std::result::Result<Self, (Self, io::Error)> {
        let inner = Arc::new(HealthInner::new(config));
        inner.update();
        if interval.is_zero() {
            return Ok(Self {
                inner,
                sampler: None,
                interval: Duration::ZERO,
            });
        }
        let interval = interval.max(MIN_SAMPLE_INTERVAL);
        match spawn_sampler(Arc::clone(&inner), interval) {
            Ok(sampler) => Ok(Self {
                inner,
                sampler: Some(sampler),
                interval,
            }),
            Err(e) => Err((
                Self {
                    inner,
                    sampler: None,
                    interval: Duration::ZERO,
                },
                e,
            )),
        }
    }

    /// The background interval, or `Duration::ZERO` in manual mode.
    #[must_use]
    pub fn update_interval(&self) -> Duration {
        self.interval
    }

    /// `true` when a background sampler thread is running.
    #[must_use]
    pub fn is_sampling(&self) -> bool {
        self.sampler.is_some()
    }

    /// Refresh every cached value now, on the calling thread.
    pub fn update(&self) {
        self.inner.update();
    }

    /// Host CPU usage, percent.
    #[must_use]
    pub fn cpu_used(&self) -> f64 {
        load_f64(&self.inner.cached.system_cpu)
    }

    /// Host CPU idle share, percent.
    #[must_use]
    pub fn cpu_free(&self) -> f64 {
        100.0 - self.cpu_used()
    }

    /// Host memory in use, MiB.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // byte counts are far below 2^53
    pub fn mem_used_mb(&self) -> f64 {
        self.inner.cached.memory_used.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0)
    }

    /// Host memory in use, GiB.
    #[must_use]
    pub fn mem_used_gb(&self) -> f64 {
        self.mem_used_mb() / 1024.0
    }

    /// Host memory, total MiB.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // byte counts are far below 2^53
    pub fn mem_total_mb(&self) -> f64 {
        self.inner.cached.memory_total.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0)
    }

    /// Host memory in use, percent of total (`0` when unknown).
    #[must_use]
    pub fn mem_used_percent(&self) -> f64 {
        let total = self.mem_total_mb();
        if total > 0.0 {
            self.mem_used_mb() / total * 100.0
        } else {
            0.0
        }
    }

    /// Process CPU usage, percent of all cores.
    #[must_use]
    pub fn process_cpu_used(&self) -> f64 {
        load_f64(&self.inner.cached.process_cpu)
    }

    /// Process resident memory, MiB.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // byte counts are far below 2^53
    pub fn process_mem_used_mb(&self) -> f64 {
        self.inner.cached.process_rss.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0)
    }

    /// One-minute load average, when the platform has one.
    #[must_use]
    pub fn load_avg(&self) -> Option<f64> {
        let v = load_f64(&self.inner.cached.load);
        (!v.is_nan()).then_some(v)
    }

    /// Process thread count, when the platform reports it.
    #[must_use]
    pub fn thread_count(&self) -> Option<u32> {
        load_opt_u32(&self.inner.cached.threads)
    }

    /// Process open file descriptors (handles on Windows), when available.
    #[must_use]
    pub fn fd_count(&self) -> Option<u32> {
        load_opt_u32(&self.inner.cached.fds)
    }

    /// Health score, 0–100 (100 = no threshold exceeded).
    #[must_use]
    pub fn health_score(&self) -> f64 {
        load_f64(&self.inner.cached.health)
    }

    /// Classification of [`SystemHealth::health_score`].
    #[must_use]
    pub fn quick_check(&self) -> HealthStatus {
        HealthStatus::from_score(self.health_score())
    }

    /// The thresholds in use.
    #[must_use]
    pub fn config(&self) -> &HealthConfig {
        &self.inner.config
    }

    /// The latest process sample (as taken by the last refresh).
    #[must_use]
    pub fn process_metrics(&self) -> ProcessMetrics {
        self.inner
            .state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .latest_process
            .clone()
    }

    /// Every cached value at once.
    #[must_use]
    pub fn snapshot(&self) -> SystemSnapshot {
        let c = &self.inner.cached;
        let now = crate::util::duration_to_nanos(self.inner.created_at.elapsed());
        let last = c.last_update_ns.load(Ordering::Relaxed);
        SystemSnapshot {
            system_cpu_percent: self.cpu_used(),
            process_cpu_percent: self.process_cpu_used(),
            memory_total_bytes: c.memory_total.load(Ordering::Relaxed),
            memory_used_bytes: c.memory_used.load(Ordering::Relaxed),
            process_memory_bytes: c.process_rss.load(Ordering::Relaxed),
            load_average: self.load_avg(),
            thread_count: self.thread_count(),
            fd_count: self.fd_count(),
            health_score: self.health_score(),
            last_update: Duration::from_nanos(now.saturating_sub(last)),
        }
    }

    /// Process-focused statistics.
    #[must_use]
    pub fn process(&self) -> ProcessStats {
        let start = self.process_metrics().start_time_seconds;
        let uptime = start
            .and_then(|start| {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
                let secs = now.as_secs_f64() - start;
                (secs.is_finite() && secs >= 0.0).then(|| Duration::from_secs_f64(secs))
            })
            .unwrap_or_else(|| self.inner.created_at.elapsed());
        ProcessStats {
            cpu_percent: self.process_cpu_used(),
            memory_mb: self.process_mem_used_mb(),
            threads: self.thread_count(),
            file_handles: self.fd_count(),
            uptime,
        }
    }
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SystemHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SystemHealth(CPU: {:.1}%, Mem: {:.0}/{:.0} MB, Health: {:.1})",
            self.cpu_used(),
            self.mem_used_mb(),
            self.mem_total_mb(),
            self.health_score()
        )
    }
}

impl std::fmt::Debug for SystemHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemHealth")
            .field("snapshot", &self.snapshot())
            .field("interval", &self.interval)
            .field("sampling", &self.is_sampling())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact comparisons of exactly representable values
mod tests {
    use super::*;

    #[test]
    fn test_manual_health_reads_sane_values() {
        let health = SystemHealth::manual();
        assert!(!health.is_sampling());
        assert_eq!(health.update_interval(), Duration::ZERO);
        health.update();
        let snap = health.snapshot();
        assert!((0.0..=100.0).contains(&snap.system_cpu_percent));
        assert!((0.0..=100.0).contains(&snap.health_score));
        assert!(health.cpu_free() <= 100.0);
        assert!(health.process_cpu_used() >= 0.0);
        assert_eq!(
            health.quick_check(),
            HealthStatus::from_score(health.health_score())
        );
        assert!(format!("{health}").contains("SystemHealth"));
        assert!(format!("{health:?}").contains("SystemHealth"));
        assert_eq!(health.config(), &HealthConfig::default());
    }

    #[test]
    fn test_memory_units_are_bytes_not_kib_regression_ml002() {
        // ML-002: on Windows/macOS memory read 1024x too high (KiB as MiB).
        let health = SystemHealth::manual();
        let total = health.mem_total_mb();
        let used = health.mem_used_mb();
        let rss = health.process_mem_used_mb();
        assert!(total > 0.0, "total memory must be known");
        // No host this test runs on has more than 64 TiB.
        assert!(total < 64.0 * 1024.0 * 1024.0, "total {total} MiB");
        assert!(used <= total, "used {used} > total {total}");
        assert!(
            rss > 0.0 && rss < used.max(1.0) * 1.01 + 1.0,
            "rss {rss} MiB, used {used}"
        );
        assert!((health.mem_used_gb() * 1024.0 - used).abs() < 1e-6);
        assert!((0.0..=100.0).contains(&health.mem_used_percent()));
    }

    #[test]
    fn test_process_collector_reports_real_values() {
        let collector = ProcessCollector::default();
        let m = collector.collect().unwrap();
        assert!(m.resident_memory_bytes > 1024 * 1024);
        assert!(m.cpu_seconds_total >= 0.0);
        if let Some(start) = m.start_time_seconds {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            assert!(start <= now + 1.0 && start > now - 86_400.0 * 365.0);
        }
        #[cfg(target_os = "linux")]
        {
            assert!(m.threads.unwrap() >= 1);
            assert!(m.open_fds.unwrap() >= 3);
        }
        assert!(format!("{collector:?}").contains("ProcessCollector"));
    }

    #[test]
    fn test_cpu_time_grows_with_work() {
        let collector = ProcessCollector::new();
        let before = collector.collect().unwrap().cpu_seconds_total;
        let start = Instant::now();
        let mut x = 0u64;
        while start.elapsed() < Duration::from_millis(300) {
            x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        }
        let _ = std::hint::black_box(x);
        let after = collector.collect().unwrap().cpu_seconds_total;
        assert!(after > before, "{before} -> {after}");
    }

    #[test]
    fn test_unavailable_values_are_none_not_constants_regression_ml025() {
        let health = SystemHealth::manual();
        #[cfg(not(target_os = "linux"))]
        assert_eq!(health.thread_count(), None);
        #[cfg(target_os = "linux")]
        assert!(health.thread_count().unwrap() >= 1);
        #[cfg(windows)]
        assert_eq!(health.load_avg(), None);
        let _ = health.fd_count();
    }

    #[test]
    fn test_background_sampler_starts_and_stops_promptly() {
        let health = SystemHealth::with_interval(Duration::from_millis(1));
        assert!(health.is_sampling());
        assert_eq!(health.update_interval(), MIN_SAMPLE_INTERVAL);
        let started = Instant::now();
        while health.snapshot().last_update > Duration::from_millis(40)
            && started.elapsed() < Duration::from_secs(5)
        {
            thread::sleep(Duration::from_millis(10));
        }
        let before_drop = Instant::now();
        drop(health);
        assert!(before_drop.elapsed() < Duration::from_secs(2));
        let strict =
            SystemHealth::try_with_config(Duration::from_secs(60), HealthConfig::default())
                .unwrap();
        assert!(strict.is_sampling());
        assert!(SystemHealth::default().is_sampling());
    }

    #[test]
    fn test_score_uses_configured_steps() {
        let inner = HealthInner::new(
            HealthConfig::default()
                .with_system_cpu(vec![Step::new(-1.0, 40.0)])
                .with_load_avg(vec![])
                .with_process_cpu(vec![])
                .with_memory_percent(vec![Step::new(-1.0, 25.0)])
                .with_threads(vec![])
                .with_fds(vec![]),
        );
        store_f64(&inner.cached.system_cpu, 10.0);
        inner.cached.memory_total.store(100, Ordering::Relaxed);
        inner.cached.memory_used.store(50, Ordering::Relaxed);
        assert_eq!(inner.score(), 35.0);
        // Unknown memory total skips the memory rule.
        inner.cached.memory_total.store(0, Ordering::Relaxed);
        assert_eq!(inner.score(), 60.0);
    }

    #[test]
    fn test_penalty_first_matching_step_wins() {
        let steps = [Step::new(80.0, 30.0), Step::new(60.0, 15.0)];
        assert_eq!(penalty(Some(90.0), &steps), 30.0);
        assert_eq!(penalty(Some(70.0), &steps), 15.0);
        assert_eq!(penalty(Some(10.0), &steps), 0.0);
        assert_eq!(penalty(None, &steps), 0.0);
    }

    #[test]
    fn test_health_status_classification() {
        assert_eq!(HealthStatus::from_score(90.0), HealthStatus::Healthy);
        assert_eq!(HealthStatus::from_score(70.0), HealthStatus::Warning);
        assert_eq!(HealthStatus::from_score(50.0), HealthStatus::Degraded);
        assert_eq!(HealthStatus::from_score(10.0), HealthStatus::Critical);
        assert!(HealthStatus::Critical.is_degraded());
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(HealthStatus::Warning.has_issues());
        assert_eq!(HealthStatus::Degraded.to_string(), "Degraded");
        assert_eq!(HealthStatus::Healthy.to_string(), "Healthy");
        assert_eq!(HealthStatus::Warning.to_string(), "Warning");
        assert_eq!(HealthStatus::Critical.to_string(), "Critical");
    }

    #[test]
    fn test_process_stats_uptime_and_metrics() {
        let health = SystemHealth::manual();
        let stats = health.process();
        assert!(stats.memory_mb > 0.0);
        assert!(stats.uptime < Duration::from_secs(86_400 * 365));
        assert!(health.process_metrics().resident_memory_bytes > 0);
    }

    #[test]
    fn test_option_sentinel_helpers() {
        let cell = AtomicU64::new(0);
        store_opt(&cell, None);
        assert_eq!(load_opt_u32(&cell), None);
        store_opt(&cell, Some(5));
        assert_eq!(load_opt_u32(&cell), Some(5));
        store_opt(&cell, Some(u64::from(u32::MAX) + 5));
        assert_eq!(load_opt_u32(&cell), Some(u32::MAX));
    }
}
