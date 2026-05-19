//! # System Health Monitoring
//!
//! Ultra-fast system resource monitoring with process introspection.
//!
//! ## Architecture (v0.9.4)
//!
//! `SystemHealth` separates state from sampling:
//!
//! - All atomic state lives in `HealthInner` behind an `Arc`.
//! - Reader methods (`cpu_used` / `mem_used_mb` / `health_score` / …) do a
//!   single `Relaxed` atomic load and return — they never block, never call
//!   into the OS, never acquire a lock.
//! - A **background sampler thread**, owned by the `SystemHealth` instance,
//!   wakes on the configured interval and refreshes the atomics. The thread
//!   is the only writer; readers see a fresh snapshot every
//!   `update_interval_ms` (default: 1000 ms).
//! - `SystemHealth::manual()` constructs an instance with no sampler thread
//!   for callers who want full control via [`SystemHealth::update`].
//!
//! Before 0.9.4, readers called `maybe_update()` which contended on the
//! sysinfo mutex on non-Linux platforms and stalled async runtimes during
//! refresh. The new architecture moves that work off the read path entirely.
//!
//! ## Features
//!
//! - **Process CPU/Memory tracking** — automatic per-process sampling.
//! - **System-wide monitoring** — CPU, memory, load average.
//! - **Background refresh** — non-blocking reads regardless of platform.
//! - **Cross-platform** — `/proc` on Linux, `sysinfo` elsewhere.
//! - **Zero allocations** on the hot path.
//! - **Health scoring** — composite 0–100 health score.

use std::io;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[cfg(not(target_os = "linux"))]
use sysinfo::{get_current_pid, CpuExt, ProcessExt, System, SystemExt};

/// Default interval between background samples (milliseconds).
const DEFAULT_INTERVAL_MS: u64 = 1000;
/// Hard floor on the sampler sleep duration so a misconfigured 0 ms interval
/// does not become a CPU spin loop.
const MIN_INTERVAL_MS: u64 = 50;
/// Maximum sleep window before the sampler re-checks the stop flag — keeps
/// `Drop` latency bounded even on very long configured intervals.
const MAX_SLEEP_CHUNK_MS: u64 = 1000;

/// One threshold/penalty pair used by [`HealthConfig`].
///
/// When a metric value exceeds `threshold`, the configured `penalty` is
/// subtracted from the running health score. Thresholds inside a `&[Step]`
/// slice must be supplied in **descending order** — the first match wins.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Step {
    /// Inclusive lower bound that must be exceeded for the penalty to apply.
    pub threshold: f64,
    /// Amount subtracted from the running health score (0..=100) on match.
    pub penalty: f64,
}

impl Step {
    /// Construct a step with the given threshold and penalty.
    #[inline]
    pub const fn new(threshold: f64, penalty: f64) -> Self {
        Self { threshold, penalty }
    }
}

/// Configurable thresholds for [`SystemHealth::quick_check`] /
/// [`SystemHealth::health_score`].
///
/// Each metric (`system_cpu` / `load_avg` / `process_cpu` / `memory_gb` /
/// `threads` / `fds`) accepts a list of [`Step`]s. When the metric exceeds a
/// step's threshold, the step's `penalty` is subtracted from the running
/// score (starting at 100). The first matching step wins, so list steps in
/// **descending threshold order**.
///
/// The load-average steps are interpreted as multipliers of `num_cpus::get()`
/// (e.g. `Step::new(2.0, 25.0)` applies when load exceeds `2 × CPU count`).
///
/// The defaults in [`HealthConfig::default`] match the v0.9.x behavior
/// exactly so existing dashboards do not shift unexpectedly.
///
/// # Example
///
/// ```
/// use metrics_lib::{HealthConfig, Step, SystemHealth};
/// use std::time::Duration;
///
/// // Stricter CPU thresholds for a CPU-bound workload.
/// let cfg = HealthConfig {
///     system_cpu: vec![
///         Step::new(70.0, 30.0),
///         Step::new(50.0, 15.0),
///         Step::new(30.0, 5.0),
///     ],
///     ..HealthConfig::default()
/// };
/// let health = SystemHealth::with_config(Duration::from_millis(500), cfg);
/// let _ = health.health_score();
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HealthConfig {
    /// Penalty steps applied to the system-wide CPU percentage (0..=100).
    pub system_cpu: Vec<Step>,
    /// Penalty steps applied to the 1-minute load average, expressed as a
    /// **multiplier of `num_cpus::get()`** (e.g. threshold `2.0` ⇒ trips
    /// when load > 2× cores).
    pub load_avg: Vec<Step>,
    /// Penalty steps applied to the process CPU percentage (0..=100).
    pub process_cpu: Vec<Step>,
    /// Penalty steps applied to system memory used, in **gigabytes**.
    pub memory_gb: Vec<Step>,
    /// Penalty steps applied to the process thread count.
    pub threads: Vec<Step>,
    /// Penalty steps applied to the process file-descriptor count.
    pub fds: Vec<Step>,
}

impl Default for HealthConfig {
    /// Defaults match the v0.9.x scoring exactly. Each `Vec<Step>` is
    /// ordered descending by threshold so the first match wins.
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
            memory_gb: vec![Step::new(16.0, 10.0), Step::new(8.0, 5.0)],
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

fn apply_steps(value: f64, steps: &[Step]) -> f64 {
    for step in steps {
        if value > step.threshold {
            return step.penalty;
        }
    }
    0.0
}

/// Mutable state of a [`SystemHealth`] instance.
///
/// Shared between the sampler thread (sole writer) and any number of reader
/// threads via `Arc`. All public fields are atomic so reads never block.
#[repr(align(64))]
struct HealthInner {
    /// Last system CPU usage (percentage * 100).
    system_cpu: AtomicU32,
    /// Last process CPU usage (percentage * 100).
    process_cpu: AtomicU32,
    /// System memory usage in MB.
    system_memory_mb: AtomicU64,
    /// Process memory usage in MB.
    process_memory_mb: AtomicU64,
    /// System load average (1 min * 100).
    load_average: AtomicU32,
    /// Process thread count.
    thread_count: AtomicU32,
    /// Process file descriptor count.
    fd_count: AtomicU32,
    /// Overall health score (0-10000, where 10000 = 100%).
    health_score: AtomicU32,
    /// Milliseconds since `created_at` at the last metrics refresh.
    last_update_ms: AtomicU64,
    /// Creation timestamp (process start, effectively).
    created_at: Instant,
    /// Linux-only delta-sample state for process CPU.
    #[cfg(target_os = "linux")]
    proc_cpu_prev: AtomicU64,
    /// Linux-only delta-sample state for process CPU. `u64::MAX` sentinel =
    /// "no prior sample yet".
    #[cfg(target_os = "linux")]
    proc_cpu_prev_ms: AtomicU64,
    /// Non-Linux: shared `sysinfo::System` used by the sampler thread.
    /// Readers never touch this mutex — only the sampler does.
    #[cfg(not(target_os = "linux"))]
    sys: parking_lot::Mutex<System>,
    #[cfg(not(target_os = "linux"))]
    pid: Option<sysinfo::Pid>,
    /// Tunable health-score thresholds (v0.9.5).
    config: HealthConfig,
}

impl HealthInner {
    fn new(config: HealthConfig) -> Self {
        Self {
            system_cpu: AtomicU32::new(0),
            process_cpu: AtomicU32::new(0),
            system_memory_mb: AtomicU64::new(0),
            process_memory_mb: AtomicU64::new(0),
            load_average: AtomicU32::new(0),
            thread_count: AtomicU32::new(0),
            fd_count: AtomicU32::new(0),
            health_score: AtomicU32::new(10000),
            last_update_ms: AtomicU64::new(0),
            created_at: Instant::now(),
            #[cfg(target_os = "linux")]
            proc_cpu_prev: AtomicU64::new(0),
            #[cfg(target_os = "linux")]
            proc_cpu_prev_ms: AtomicU64::new(u64::MAX),
            #[cfg(not(target_os = "linux"))]
            sys: parking_lot::Mutex::new(System::new()),
            #[cfg(not(target_os = "linux"))]
            pid: get_current_pid().ok(),
            config,
        }
    }

    fn update_metrics(&self) {
        let now_ms = self.created_at.elapsed().as_millis() as u64;

        if let Ok(cpu) = self.get_system_cpu() {
            self.system_cpu
                .store((cpu * 100.0) as u32, Ordering::Relaxed);
        }
        if let Ok(memory_mb) = self.get_system_memory_mb() {
            self.system_memory_mb.store(memory_mb, Ordering::Relaxed);
        }
        if let Ok(load) = self.get_load_average() {
            self.load_average
                .store((load * 100.0) as u32, Ordering::Relaxed);
        }
        if let Ok(cpu) = self.get_process_cpu() {
            self.process_cpu
                .store((cpu * 100.0) as u32, Ordering::Relaxed);
        }
        if let Ok(memory_mb) = self.get_process_memory_mb() {
            self.process_memory_mb.store(memory_mb, Ordering::Relaxed);
        }
        if let Ok(threads) = self.get_thread_count() {
            self.thread_count.store(threads, Ordering::Relaxed);
        }
        if let Ok(fds) = self.get_fd_count() {
            self.fd_count.store(fds, Ordering::Relaxed);
        }

        let health = self.calculate_health_score();
        self.health_score
            .store((health * 100.0) as u32, Ordering::Relaxed);

        self.last_update_ms.store(now_ms, Ordering::Relaxed);
    }

    fn calculate_health_score(&self) -> f64 {
        let cfg = &self.config;
        let cpu_count = num_cpus::get() as f64;

        let system_cpu = self.system_cpu.load(Ordering::Relaxed) as f64 / 100.0;
        let load_norm =
            (self.load_average.load(Ordering::Relaxed) as f64 / 100.0) / cpu_count.max(1.0);
        let process_cpu = self.process_cpu.load(Ordering::Relaxed) as f64 / 100.0;
        let memory_gb = self.system_memory_mb.load(Ordering::Relaxed) as f64 / 1024.0;
        let threads = self.thread_count.load(Ordering::Relaxed) as f64;
        let fds = self.fd_count.load(Ordering::Relaxed) as f64;

        let score = 100.0
            - apply_steps(system_cpu, &cfg.system_cpu)
            - apply_steps(load_norm, &cfg.load_avg)
            - apply_steps(process_cpu, &cfg.process_cpu)
            - apply_steps(memory_gb, &cfg.memory_gb)
            - apply_steps(threads, &cfg.threads)
            - apply_steps(fds, &cfg.fds);

        score.max(0.0)
    }

    // ----- platform-specific samplers -----

    #[cfg(target_os = "linux")]
    fn get_system_cpu(&self) -> io::Result<f64> {
        let contents = std::fs::read_to_string("/proc/stat")?;
        if let Some(line) = contents.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 && parts[0] == "cpu" {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let nice: u64 = parts[2].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let total = user + nice + system + idle;
                let used = user + nice + system;
                if total > 0 {
                    return Ok(used as f64 / total as f64 * 100.0);
                }
            }
        }
        Ok(0.0)
    }

    #[cfg(not(target_os = "linux"))]
    fn get_system_cpu(&self) -> io::Result<f64> {
        let mut guard = self.sys.lock();
        guard.refresh_cpu();
        Ok(guard.global_cpu_info().cpu_usage() as f64)
    }

    #[cfg(target_os = "linux")]
    fn get_system_memory_mb(&self) -> io::Result<u64> {
        let contents = std::fs::read_to_string("/proc/meminfo")?;
        let mut total_kb = 0u64;
        let mut free_kb = 0u64;
        let mut available_kb = 0u64;
        for line in contents.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                total_kb = rest
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if let Some(rest) = line.strip_prefix("MemFree:") {
                free_kb = rest
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
                available_kb = rest
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            }
        }
        let used_kb = if available_kb > 0 {
            total_kb.saturating_sub(available_kb)
        } else {
            total_kb.saturating_sub(free_kb)
        };
        Ok(used_kb / 1024)
    }

    #[cfg(not(target_os = "linux"))]
    fn get_system_memory_mb(&self) -> io::Result<u64> {
        let mut guard = self.sys.lock();
        guard.refresh_memory();
        let used_kib = guard.used_memory();
        Ok(used_kib / 1024)
    }

    #[cfg(target_os = "linux")]
    fn get_load_average(&self) -> io::Result<f64> {
        let contents = std::fs::read_to_string("/proc/loadavg")?;
        if let Some(first) = contents.split_whitespace().next() {
            return first
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid load average"));
        }
        Ok(0.0)
    }

    #[cfg(not(target_os = "linux"))]
    fn get_load_average(&self) -> io::Result<f64> {
        let guard = self.sys.lock();
        Ok(guard.load_average().one)
    }

    #[cfg(target_os = "linux")]
    fn get_process_cpu(&self) -> io::Result<f64> {
        // Delta sample: ((utime + stime) - prev) / (CLK_TCK * elapsed_s * cores) * 100.
        let contents = std::fs::read_to_string("/proc/self/stat")?;
        let parts: Vec<&str> = contents.split_whitespace().collect();
        if parts.len() < 15 {
            return Ok(0.0);
        }
        let utime: u64 = parts[13].parse().unwrap_or(0);
        let stime: u64 = parts[14].parse().unwrap_or(0);
        let total_ticks = utime.saturating_add(stime);
        let now_ms = self.created_at.elapsed().as_millis() as u64;

        let prev_ticks = self.proc_cpu_prev.load(Ordering::Relaxed);
        let prev_ms = self.proc_cpu_prev_ms.load(Ordering::Relaxed);
        self.proc_cpu_prev.store(total_ticks, Ordering::Relaxed);
        self.proc_cpu_prev_ms.store(now_ms, Ordering::Relaxed);

        if prev_ms == u64::MAX {
            return Ok(0.0);
        }
        let elapsed_ms = now_ms.saturating_sub(prev_ms);
        if elapsed_ms == 0 {
            return Ok(0.0);
        }
        let delta_ticks = total_ticks.saturating_sub(prev_ticks) as f64;
        let clk_tck: f64 = 100.0;
        let elapsed_s = elapsed_ms as f64 / 1000.0;
        let cores = num_cpus::get().max(1) as f64;
        let pct = (delta_ticks / (clk_tck * elapsed_s * cores)) * 100.0;
        Ok(pct.clamp(0.0, 100.0))
    }

    #[cfg(not(target_os = "linux"))]
    fn get_process_cpu(&self) -> io::Result<f64> {
        let mut guard = self.sys.lock();
        if let Some(pid) = self.pid {
            guard.refresh_process(pid);
            if let Some(proc_) = guard.process(pid) {
                let raw = proc_.cpu_usage() as f64;
                let cores = num_cpus::get() as f64;
                let norm = if cores > 0.0 { raw / cores } else { raw };
                return Ok(norm.clamp(0.0, 100.0));
            }
        }
        Ok(0.0)
    }

    #[cfg(target_os = "linux")]
    fn get_process_memory_mb(&self) -> io::Result<u64> {
        let contents = std::fs::read_to_string("/proc/self/status")?;
        for line in contents.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                if let Some(kb) = rest
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<u64>().ok())
                {
                    return Ok(kb / 1024);
                }
            }
        }
        Ok(0)
    }

    #[cfg(not(target_os = "linux"))]
    fn get_process_memory_mb(&self) -> io::Result<u64> {
        let mut guard = self.sys.lock();
        if let Some(pid) = self.pid {
            guard.refresh_process(pid);
            if let Some(proc_) = guard.process(pid) {
                return Ok(proc_.memory() / 1024);
            }
        }
        Ok(0)
    }

    #[cfg(target_os = "linux")]
    fn get_thread_count(&self) -> io::Result<u32> {
        let contents = std::fs::read_to_string("/proc/self/status")?;
        for line in contents.lines() {
            if let Some(rest) = line.strip_prefix("Threads:") {
                if let Some(c) = rest.split_whitespace().next().and_then(|s| s.parse().ok()) {
                    return Ok(c);
                }
            }
        }
        Ok(1)
    }

    #[cfg(not(target_os = "linux"))]
    fn get_thread_count(&self) -> io::Result<u32> {
        Ok(1)
    }

    #[cfg(target_os = "linux")]
    fn get_fd_count(&self) -> io::Result<u32> {
        match std::fs::read_dir("/proc/self/fd") {
            Ok(entries) => Ok(entries.count() as u32),
            Err(_) => Ok(0),
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn get_fd_count(&self) -> io::Result<u32> {
        Ok(0)
    }
}

/// System health monitor with process introspection.
///
/// Owns a background sampler thread (unless constructed via
/// [`SystemHealth::manual`]) that refreshes the cached values every
/// `update_interval_ms`. All accessor methods are lock-free atomic loads.
#[repr(align(64))]
pub struct SystemHealth {
    inner: Arc<HealthInner>,
    /// Lives only for its `Drop` side-effect (stops + joins the sampler
    /// thread). Prefixed `_` so the `dead_code` lint doesn't flag the
    /// drop-only field.
    _sampler: Option<SamplerHandle>,
    /// Configured interval in milliseconds (0 = manual mode, no sampler).
    update_interval_ms: u64,
}

/// Per-instance sampler thread handle. Stops + joins the thread on `Drop`.
struct SamplerHandle {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for SamplerHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            // Wake the sleeper so `Drop` doesn't block until the next tick.
            t.thread().unpark();
            let _ = t.join();
        }
    }
}

/// System resource usage snapshot
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SystemSnapshot {
    /// System CPU usage percentage (0.0-100.0)
    pub system_cpu_percent: f64,
    /// Process CPU usage percentage (0.0-100.0)
    pub process_cpu_percent: f64,
    /// System memory usage in MB
    pub system_memory_mb: u64,
    /// Process memory usage in MB
    pub process_memory_mb: u64,
    /// System load average (1 minute)
    pub load_average: f64,
    /// Number of process threads
    pub thread_count: u32,
    /// Number of file descriptors
    pub fd_count: u32,
    /// Overall health score (0.0-100.0)
    pub health_score: f64,
    /// Time since last sampler refresh
    pub last_update: Duration,
}

/// Process-specific resource usage
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ProcessStats {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in megabytes
    pub memory_mb: f64,
    /// Number of threads
    pub threads: u32,
    /// Number of file handles
    pub file_handles: u32,
    /// Process uptime
    pub uptime: Duration,
}

impl SystemHealth {
    /// Create a new system health monitor with the default refresh interval
    /// of 1 second. A background sampler thread is spawned and joined on
    /// [`Drop`].
    #[inline]
    pub fn new() -> Self {
        Self::with_interval(Duration::from_millis(DEFAULT_INTERVAL_MS))
    }

    /// Create with a custom refresh interval. Uses [`HealthConfig::default`]
    /// for the health-score thresholds.
    ///
    /// - `interval == Duration::ZERO` ⇒ no sampler thread is spawned;
    ///   callers must use [`Self::update`] to refresh the cached values
    ///   (equivalent to [`Self::manual`]).
    /// - Intervals below `50 ms` are clamped to `50 ms` to prevent the
    ///   sampler from becoming a CPU spin loop.
    #[inline]
    pub fn with_interval(interval: Duration) -> Self {
        Self::with_config(interval, HealthConfig::default())
    }

    /// Create with both a custom refresh interval and a custom
    /// [`HealthConfig`] (v0.9.5+). See [`Self::with_interval`] for interval
    /// semantics.
    pub fn with_config(interval: Duration, config: HealthConfig) -> Self {
        let inner = Arc::new(HealthInner::new(config));
        // Always seed the initial snapshot so the first read returns
        // meaningful values even before the sampler ticks.
        inner.update_metrics();

        if interval.is_zero() {
            return Self {
                inner,
                _sampler: None,
                update_interval_ms: 0,
            };
        }
        let interval_ms = (interval.as_millis() as u64).max(MIN_INTERVAL_MS);
        let sampler = spawn_sampler(inner.clone(), interval_ms);
        Self {
            inner,
            _sampler: Some(sampler),
            update_interval_ms: interval_ms,
        }
    }

    /// Construct a manual-mode instance with no sampler thread. Callers
    /// must invoke [`Self::update`] to refresh the cached values.
    #[inline]
    pub fn manual() -> Self {
        Self::with_interval(Duration::ZERO)
    }

    /// Configured refresh interval in milliseconds. `0` indicates manual
    /// mode (no sampler thread).
    #[must_use]
    #[inline]
    pub fn update_interval_ms(&self) -> u64 {
        self.update_interval_ms
    }

    /// Get system CPU usage percentage. Lock-free atomic load.
    #[inline(always)]
    pub fn cpu_used(&self) -> f64 {
        self.inner.system_cpu.load(Ordering::Relaxed) as f64 / 100.0
    }

    /// Get system CPU free percentage.
    #[inline]
    pub fn cpu_free(&self) -> f64 {
        100.0 - self.cpu_used()
    }

    /// Get system memory usage in MB. Lock-free atomic load.
    #[inline(always)]
    pub fn mem_used_mb(&self) -> f64 {
        self.inner.system_memory_mb.load(Ordering::Relaxed) as f64
    }

    /// Get system memory usage in GB.
    #[inline]
    pub fn mem_used_gb(&self) -> f64 {
        self.mem_used_mb() / 1024.0
    }

    /// Get process CPU usage percentage. Lock-free atomic load.
    #[inline(always)]
    pub fn process_cpu_used(&self) -> f64 {
        self.inner.process_cpu.load(Ordering::Relaxed) as f64 / 100.0
    }

    /// Get process memory usage in MB. Lock-free atomic load.
    #[inline(always)]
    pub fn process_mem_used_mb(&self) -> f64 {
        self.inner.process_memory_mb.load(Ordering::Relaxed) as f64
    }

    /// Get system load average. Lock-free atomic load.
    #[inline(always)]
    pub fn load_avg(&self) -> f64 {
        self.inner.load_average.load(Ordering::Relaxed) as f64 / 100.0
    }

    /// Get process thread count. Lock-free atomic load.
    #[inline(always)]
    pub fn thread_count(&self) -> u32 {
        self.inner.thread_count.load(Ordering::Relaxed)
    }

    /// Get process file descriptor count. Lock-free atomic load.
    #[inline(always)]
    pub fn fd_count(&self) -> u32 {
        self.inner.fd_count.load(Ordering::Relaxed)
    }

    /// Get overall system health score (0.0-100.0). Lock-free atomic load.
    #[inline(always)]
    pub fn health_score(&self) -> f64 {
        self.inner.health_score.load(Ordering::Relaxed) as f64 / 100.0
    }

    /// Quick health check. Lock-free atomic load.
    #[inline(always)]
    pub fn quick_check(&self) -> HealthStatus {
        let score = self.health_score();
        if score >= 80.0 {
            HealthStatus::Healthy
        } else if score >= 60.0 {
            HealthStatus::Warning
        } else if score >= 40.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Critical
        }
    }

    /// Force immediate (synchronous) refresh of every cached metric.
    ///
    /// Bypasses the sampler interval — useful for tests, on-demand
    /// snapshots, or manual-mode operation. Safe to call from any thread.
    #[inline]
    pub fn update(&self) {
        self.inner.update_metrics();
    }

    /// Get a detailed system snapshot. Lock-free atomic loads.
    pub fn snapshot(&self) -> SystemSnapshot {
        let inner = &self.inner;
        let now_ms = inner.created_at.elapsed().as_millis() as u64;
        let last_ms = inner.last_update_ms.load(Ordering::Relaxed);
        let last_update = Duration::from_millis(now_ms.saturating_sub(last_ms));

        SystemSnapshot {
            system_cpu_percent: inner.system_cpu.load(Ordering::Relaxed) as f64 / 100.0,
            process_cpu_percent: inner.process_cpu.load(Ordering::Relaxed) as f64 / 100.0,
            system_memory_mb: inner.system_memory_mb.load(Ordering::Relaxed),
            process_memory_mb: inner.process_memory_mb.load(Ordering::Relaxed),
            load_average: inner.load_average.load(Ordering::Relaxed) as f64 / 100.0,
            thread_count: inner.thread_count.load(Ordering::Relaxed),
            fd_count: inner.fd_count.load(Ordering::Relaxed),
            health_score: inner.health_score.load(Ordering::Relaxed) as f64 / 100.0,
            last_update,
        }
    }

    /// Get process-specific statistics. Lock-free atomic loads.
    pub fn process(&self) -> ProcessStats {
        let inner = &self.inner;
        ProcessStats {
            cpu_percent: inner.process_cpu.load(Ordering::Relaxed) as f64 / 100.0,
            memory_mb: inner.process_memory_mb.load(Ordering::Relaxed) as f64,
            threads: inner.thread_count.load(Ordering::Relaxed),
            file_handles: inner.fd_count.load(Ordering::Relaxed),
            uptime: inner.created_at.elapsed(),
        }
    }
}

/// Spawn the background sampler thread and return its handle.
fn spawn_sampler(inner: Arc<HealthInner>, interval_ms: u64) -> SamplerHandle {
    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = stop.clone();
    let thread = thread::Builder::new()
        .name("metrics-lib-health-sampler".into())
        .spawn(move || run_sampler(inner, stop2, interval_ms))
        .expect("spawn metrics-lib sampler thread");
    SamplerHandle {
        stop,
        thread: Some(thread),
    }
}

fn run_sampler(inner: Arc<HealthInner>, stop: Arc<AtomicBool>, interval_ms: u64) {
    while !stop.load(Ordering::Relaxed) {
        // Park in `MAX_SLEEP_CHUNK_MS` chunks so `Drop` can wake us
        // promptly via `thread.unpark()` without waiting for the full
        // configured interval to elapse. Manual ceiling-divide keeps MSRV
        // 1.70 (`u64::div_ceil` is 1.73+).
        let chunks = interval_ms.saturating_add(MAX_SLEEP_CHUNK_MS - 1) / MAX_SLEEP_CHUNK_MS;
        let chunk_ms = interval_ms.min(MAX_SLEEP_CHUNK_MS);
        for _ in 0..chunks.max(1) {
            if stop.load(Ordering::Relaxed) {
                return;
            }
            thread::park_timeout(Duration::from_millis(chunk_ms));
        }
        if stop.load(Ordering::Relaxed) {
            return;
        }
        inner.update_metrics();
    }
}

/// System health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum HealthStatus {
    /// System is healthy (80%+ score)
    Healthy,
    /// System has warnings (60-80% score)
    Warning,
    /// System is degraded (40-60% score)
    Degraded,
    /// System is in critical state (<40% score)
    Critical,
}

impl HealthStatus {
    /// `true` if the status indicates degraded or worse.
    #[inline]
    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded | Self::Critical)
    }

    /// `true` if the status is healthy.
    #[inline]
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// `true` if the status has warnings or worse.
    #[inline]
    pub fn has_issues(&self) -> bool {
        !matches!(self, Self::Healthy)
    }
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SystemHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let snapshot = self.snapshot();
        write!(
            f,
            "SystemHealth(CPU: {:.1}%, Mem: {} MB, Health: {:.1}%)",
            snapshot.system_cpu_percent, snapshot.system_memory_mb, snapshot.health_score
        )
    }
}

impl std::fmt::Debug for SystemHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let snapshot = self.snapshot();
        f.debug_struct("SystemHealth")
            .field("system_cpu", &snapshot.system_cpu_percent)
            .field("process_cpu", &snapshot.process_cpu_percent)
            .field("system_memory_mb", &snapshot.system_memory_mb)
            .field("process_memory_mb", &snapshot.process_memory_mb)
            .field("load_average", &snapshot.load_average)
            .field("threads", &snapshot.thread_count)
            .field("fds", &snapshot.fd_count)
            .field("health_score", &snapshot.health_score)
            .field("update_interval_ms", &self.update_interval_ms)
            .finish()
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "Healthy"),
            Self::Warning => write!(f, "Warning"),
            Self::Degraded => write!(f, "Degraded"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

// Thread safety:
// `SystemHealth` is composed of `Arc<HealthInner>` (Send + Sync) and an
// optional `SamplerHandle` (Send + Sync). The compiler derives Send + Sync
// automatically.

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_basic_functionality() {
        let health = SystemHealth::new();
        let _cpu = health.cpu_used();
        let _mem = health.mem_used_mb();
        let _process_cpu = health.process_cpu_used();
        let _process_mem = health.process_mem_used_mb();
        let _load = health.load_avg();
        let _threads = health.thread_count();
        let _fds = health.fd_count();
        let _score = health.health_score();

        let status = health.quick_check();
        assert!(matches!(
            status,
            HealthStatus::Healthy
                | HealthStatus::Warning
                | HealthStatus::Degraded
                | HealthStatus::Critical
        ));
    }

    #[test]
    fn test_cpu_free() {
        let health = SystemHealth::new();
        let used = health.cpu_used();
        let free = health.cpu_free();
        assert!((used + free - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_memory_units() {
        let health = SystemHealth::new();
        let mb = health.mem_used_mb();
        let gb = health.mem_used_gb();
        if mb > 0.0 {
            assert!((gb * 1024.0 - mb).abs() < 1.0);
        }
    }

    #[test]
    fn test_snapshot() {
        let health = SystemHealth::new();
        let snapshot = health.snapshot();
        assert!(snapshot.system_cpu_percent >= 0.0);
        assert!(snapshot.system_cpu_percent <= 100.0);
        assert!(snapshot.health_score >= 0.0);
        assert!(snapshot.health_score <= 100.0);
        assert!(snapshot.thread_count > 0);
    }

    #[test]
    fn test_process_stats() {
        let health = SystemHealth::new();
        let stats = health.process();
        assert!(stats.threads > 0);
        assert!(stats.uptime > Duration::ZERO);
        assert!(stats.cpu_percent >= 0.0);
        assert!(stats.memory_mb >= 0.0);
    }

    #[test]
    fn test_health_status() {
        for hs in [
            HealthStatus::Healthy,
            HealthStatus::Warning,
            HealthStatus::Degraded,
            HealthStatus::Critical,
        ] {
            let _ = format!("{hs}");
        }
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(!HealthStatus::Healthy.has_issues());
        assert!(HealthStatus::Warning.has_issues());
        assert!(HealthStatus::Degraded.is_degraded());
        assert!(HealthStatus::Critical.is_degraded());
    }

    #[test]
    fn test_custom_interval_floors_to_50ms() {
        let health = SystemHealth::with_interval(Duration::from_millis(5));
        assert!(health.update_interval_ms() >= MIN_INTERVAL_MS);
    }

    #[test]
    fn test_background_sampler_refreshes_snapshot_after_interval() {
        // v0.9.4: with a background sampler the snapshot's `last_update`
        // should bound itself within `interval + slack` even when readers
        // don't actively trigger refreshes.
        let health = SystemHealth::with_interval(Duration::from_millis(50));
        let snap_before = health.snapshot();
        assert!(snap_before.system_cpu_percent.is_finite());

        thread::sleep(Duration::from_millis(250));

        let snap_after = health.snapshot();
        assert!(
            snap_after.last_update <= Duration::from_millis(500),
            "snapshot.last_update should be 'time since last sampler refresh' \
             (≤ interval + slack); got {:?}",
            snap_after.last_update,
        );
        assert!(snap_after.system_cpu_percent.is_finite());
    }

    #[test]
    fn test_manual_mode_does_not_spawn_sampler() {
        let health = SystemHealth::manual();
        assert_eq!(health.update_interval_ms(), 0);
        // The values should be seeded by the initial in-constructor refresh.
        let snap = health.snapshot();
        assert!(snap.system_cpu_percent >= 0.0);
        // Explicit update still works.
        health.update();
    }

    #[test]
    fn test_force_update() {
        let health = SystemHealth::new();
        let score_before = health.health_score();
        health.update();
        let score_after = health.health_score();
        assert!(score_before >= 0.0);
        assert!(score_after >= 0.0);
    }

    #[test]
    fn test_concurrent_access() {
        let health = std::sync::Arc::new(SystemHealth::new());
        let mut handles = vec![];
        for _ in 0..10 {
            let health_clone = health.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _cpu = health_clone.cpu_used();
                    let _mem = health_clone.mem_used_mb();
                    let _status = health_clone.quick_check();
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let final_score = health.health_score();
        assert!((0.0..=100.0).contains(&final_score));
    }

    #[test]
    fn test_display_formatting() {
        let health = SystemHealth::new();
        let display_str = format!("{health}");
        assert!(display_str.contains("SystemHealth"));
        assert!(display_str.contains("CPU"));
        assert!(display_str.contains("Mem"));

        let debug_str = format!("{health:?}");
        assert!(debug_str.contains("SystemHealth"));

        let status = health.quick_check();
        let status_str = format!("{status}");
        assert!(!status_str.is_empty());
    }

    #[test]
    fn test_drop_joins_sampler_thread() {
        // Best-effort: dropping a SystemHealth should not leak the sampler.
        // We can't directly assert the join, but we exercise the path.
        let health = SystemHealth::with_interval(Duration::from_millis(50));
        thread::sleep(Duration::from_millis(75));
        drop(health);
        // If `Drop` deadlocked we'd hang here; reaching the next line passes.
    }
}

#[cfg(all(test, feature = "bench-tests", not(tarpaulin), not(coverage)))]
#[allow(unused_imports)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[cfg_attr(not(feature = "bench-tests"), ignore)]
    #[test]
    fn bench_quick_check() {
        let health = SystemHealth::new();
        let iterations = 1_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = health.quick_check();
        }
        let elapsed = start.elapsed();
        println!(
            "SystemHealth quick_check: {:.2} ns/op",
            elapsed.as_nanos() as f64 / iterations as f64
        );
        // Throughput-only smoke check; Criterion is the regression detector.
    }

    #[cfg_attr(not(feature = "bench-tests"), ignore)]
    #[test]
    fn bench_cached_metrics() {
        let health = SystemHealth::new();
        let iterations = 1_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = health.cpu_used();
            let _ = health.mem_used_mb();
            let _ = health.health_score();
        }
        let elapsed = start.elapsed();
        println!(
            "SystemHealth cached metrics: {:.2} ns/op",
            elapsed.as_nanos() as f64 / iterations as f64 / 3.0
        );
        // Throughput-only smoke check; Criterion is the regression detector.
    }

    #[cfg_attr(not(feature = "bench-tests"), ignore)]
    #[test]
    fn bench_force_update() {
        let health = SystemHealth::manual();
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            health.update();
        }
        let elapsed = start.elapsed();
        println!(
            "SystemHealth force update: {:.2} μs/op",
            elapsed.as_micros() as f64 / iterations as f64
        );
        // Throughput-only smoke check; Criterion is the regression detector.
    }
}
