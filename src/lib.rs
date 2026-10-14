//! Lock-free metrics for Rust services.
//!
//! `metrics-lib` provides atomic metric primitives — [`Counter`],
//! [`ShardedCounter`], [`UpDownCounter`], [`Gauge`], [`Timer`],
//! [`RateMeter`], [`Histogram`], [`Summary`] — a bounded, validated
//! [`Registry`] that names them, and exporters for Prometheus, OpenMetrics,
//! StatsD, OTLP and JSON. Optional modules add process/host metrics,
//! load-shedding helpers, Tokio helpers, a `tracing` integration, a bridge for
//! the `metrics` facade and Tower middleware.
//!
//! # Quick start
//!
//! ```
//! # #[cfg(all(feature = "count", feature = "gauge", feature = "timer"))]
//! # {
//! use metrics_lib::{metrics, LabelSet};
//!
//! // The global instance initialises itself on first use.
//! metrics().counter("requests").inc();
//! metrics()
//!     .counter_with("requests", &LabelSet::from([("status", "500")]))
//!     .inc();
//! metrics().gauge("queue_depth").set(42.0);
//!
//! // Time a block; the guard records when it goes out of scope.
//! {
//!     let _t = metrics().timer("db_query_seconds").start_owned();
//!     // ... query ...
//! }
//!
//! // Or time a closure.
//! let rows = metrics().time("load_rows_seconds", || 7);
//! assert_eq!(rows, 7);
//!
//! // Hot paths: keep the handle and skip the lookup.
//! let hits = metrics().counter("cache_hits");
//! for _ in 0..1000 {
//!     hits.inc();
//! }
//!
//! // Expose everything.
//! let body = metrics_lib::exporters::prometheus::render(metrics().registry());
//! assert!(body.contains("cache_hits 1000"));
//! # }
//! ```
//!
//! Prefer an explicit [`MetricsCore`] (or [`Registry`]) passed to the code
//! that records, and use the global one only at the edges of an application;
//! see [`init_with_config`] to configure it.
//!
//! # Feature flags
//!
//! | Feature | Default | Enables |
//! |---|---|---|
//! | `count` | yes | [`Counter`], [`ShardedCounter`], [`UpDownCounter`] |
//! | `gauge` | yes | [`Gauge`] and the specialised gauges |
//! | `timer` | yes | [`Timer`] and its guards |
//! | `system` | yes | [`SystemHealth`], [`ProcessCollector`], `process_*` series |
//! | `meter` | | [`RateMeter`] and the specialised meters |
//! | `histogram` | | [`Histogram`] (with exemplars) and [`Summary`] |
//! | `sample` | | [`AdaptiveSampler`], [`MetricCircuitBreaker`], [`BackpressureController`] |
//! | `async` | | Tokio helpers (`async_support`) |
//! | `serde` | | `Serialize` on stats types; JSON exporter |
//! | `statsd` | | StatsD/DogStatsD UDP exporter |
//! | `otlp` | | OTLP HTTP+JSON and protobuf payloads |
//! | `http` | | Blocking `/metrics` server and Pushgateway client |
//! | `tracing` | | Span-wrapped timing and exemplar helpers |
//! | `metrics-rs` | | `metrics` facade `Recorder` bridge |
//! | `tower` | | RED middleware for Tower services |
//!
//! Aggregates: `minimal` (`count`), `all` (every metric type plus `sample`
//! and `system`), `full` (everything), `exporters-all`.
//!
//! # Performance
//!
//! Recording into a held handle is one relaxed atomic operation for
//! counters and gauge stores, and a handful for timers and histograms. Every
//! number in the documentation comes from the Criterion benchmarks in
//! `benches/`; see `docs/PERFORMANCE.md` for the figures and the machine
//! they were measured on.

#![cfg_attr(docsrs, feature(doc_cfg))]
// `unsafe` is denied everywhere except the two FFI modules that read process
// and host statistics from the OS (`src/system/windows.rs`,
// `src/system/macos.rs`). Every block there has a `SAFETY` comment and the
// FFI struct layouts are checked at compile time.
#![deny(unsafe_code)]
// Warnings are denied in CI (`RUSTFLAGS="-D warnings"`, clippy `-D warnings`)
// rather than here, so a lint added to a future compiler cannot break the
// build for users or on docs.rs.
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(unused_results)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::unreachable)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::pedantic)]
// With no metric type compiled in (for example `--no-default-features
// --features system`), the registry's handle enum is uninhabited and the
// registry and exporters have nothing to store or render: their generic code
// still has to compile, but is unreachable and partly unused. Builds with any
// metric type enabled get the full lint set.
#![cfg_attr(
    not(any(
        feature = "count",
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram"
    )),
    allow(
        unreachable_code,
        unused_imports,
        unused_variables,
        unused_mut,
        dead_code,
        clippy::unused_self,
        clippy::never_loop
    )
)]
// Tests unwrap freely: a failed unwrap is a failed test (REPS allows this at
// module level).
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
// Tests compare exactly representable floats, cast loop indices and build
// long fixtures; these pedantic lints add noise there and nothing else.
#![cfg_attr(
    test,
    allow(
        clippy::float_cmp,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::unreadable_literal,
        clippy::too_many_lines,
        clippy::match_same_arms,
        clippy::single_match_else,
        clippy::manual_range_contains,
        clippy::items_after_statements
    )
)]

#[cfg(feature = "sample")]
mod adaptive;
#[cfg(feature = "async")]
pub mod async_support;
#[cfg(feature = "count")]
mod counter;
mod error;
pub mod exporters;
#[cfg(feature = "gauge")]
mod gauge;
#[cfg(feature = "histogram")]
mod histogram;
mod labels;
mod macros;
mod metadata;
#[cfg(feature = "metrics-rs")]
pub mod metrics_rs;
mod name;
#[cfg(feature = "meter")]
mod rate_meter;
mod registry;
#[cfg(feature = "count")]
mod sharded;
#[cfg(feature = "histogram")]
mod summary;
#[cfg(feature = "system")]
mod system;
#[cfg(feature = "timer")]
mod timer;
mod token_bucket;
#[cfg(feature = "tower")]
pub mod tower;
#[cfg(feature = "tracing")]
pub mod tracing_ext;
#[cfg(feature = "count")]
mod updown;
mod util;

#[cfg(feature = "sample")]
pub use adaptive::{
    AdaptiveSampler, BackpressureController, BackpressureGuard, CircuitBreakerConfig, CircuitState,
    MetricCircuitBreaker, SamplingStats, SamplingStrategy,
};
#[cfg(feature = "async")]
pub use async_support::{AsyncMetricBatch, AsyncMetricsBatcher};
#[cfg(all(feature = "async", feature = "timer"))]
pub use async_support::{AsyncTimerExt, AsyncTimerGuard, TimedFuture};
#[cfg(feature = "count")]
pub use counter::{Counter, CounterStats};
pub use error::{MetricsError, Result};
#[cfg(feature = "gauge")]
pub use gauge::specialized as gauge_specialized;
#[cfg(feature = "gauge")]
pub use gauge::{Gauge, GaugeStats};
#[cfg(feature = "histogram")]
pub use histogram::{
    DEFAULT_SECONDS_BUCKETS, Exemplar, Histogram, HistogramBucket, HistogramSnapshot,
    MAX_EXEMPLAR_LABEL_CHARS, MAX_HISTOGRAM_BUCKETS,
};
pub use labels::{Label, LabelSet};
pub use metadata::{MetricKind, MetricMetadata, Unit};
pub use name::{
    MAX_LABEL_KEY_LEN, MAX_LABEL_VALUE_LEN, MAX_LABELS, MAX_NAME_LEN, sanitize_label_key,
    sanitize_metric_name, validate_label_key, validate_label_value, validate_metric_name,
};
#[cfg(feature = "meter")]
pub use rate_meter::specialized as rate_meter_specialized;
#[cfg(feature = "meter")]
pub use rate_meter::{MIN_RATE_WINDOW, RateMeter, RateStats};
pub use registry::{DEFAULT_CARDINALITY_CAP, Registry, ScopedRegistry};
#[cfg(feature = "count")]
pub use sharded::{MAX_SHARDS, ShardedCounter};
#[cfg(feature = "histogram")]
pub use summary::{
    DEFAULT_QUANTILES, MAX_SUMMARY_BUCKETS, Summary, SummaryConfig, SummarySnapshot,
};
#[cfg(feature = "system")]
pub use system::{
    DEFAULT_SAMPLE_INTERVAL, HealthConfig, HealthStatus, MIN_SAMPLE_INTERVAL, ProcessCollector,
    ProcessMetrics, ProcessStats, Step, SystemHealth, SystemSnapshot,
};
#[cfg(feature = "timer")]
pub use timer::utils as timer_utils;
#[cfg(feature = "timer")]
pub use timer::{OwnedTimerGuard, RunningTimer, Timer, TimerStats};
pub use token_bucket::{MAX_REFILL_PER_SECOND, TokenBucket};
#[cfg(feature = "count")]
pub use updown::UpDownCounter;

use std::sync::{Arc, OnceLock};
#[cfg(feature = "system")]
use std::time::Duration;

/// Configuration for a [`MetricsCore`].
///
/// Build it from [`MetricsConfig::default`] with the `with_*` methods and
/// pass it to [`MetricsCore::with_config`] or [`init_with_config`].
///
/// # Examples
///
/// ```
/// use metrics_lib::{MetricsConfig, MetricsCore};
///
/// let core = MetricsCore::with_config(MetricsConfig::default().with_max_series(5_000)).unwrap();
/// assert_eq!(core.registry().cardinality_cap(), 5_000);
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MetricsConfig {
    /// Series cap of the registry ([`Registry::set_cardinality_cap`]).
    pub max_series: usize,
    /// Background refresh interval of [`MetricsCore::system`];
    /// `Duration::ZERO` means manual mode.
    #[cfg(feature = "system")]
    pub system_interval: Duration,
    /// Health-score thresholds of [`MetricsCore::system`].
    #[cfg(feature = "system")]
    pub health: HealthConfig,
    /// Start [`MetricsCore::system`] when the core is built instead of on
    /// first use.
    #[cfg(feature = "system")]
    pub eager_system: bool,
    /// Include the standard `process_*` families in
    /// [`exporters::prometheus::render_core`] and
    /// [`exporters::openmetrics::render_core`].
    #[cfg(feature = "system")]
    pub process_metrics: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            max_series: DEFAULT_CARDINALITY_CAP,
            #[cfg(feature = "system")]
            system_interval: DEFAULT_SAMPLE_INTERVAL,
            #[cfg(feature = "system")]
            health: HealthConfig::default(),
            #[cfg(feature = "system")]
            eager_system: false,
            #[cfg(feature = "system")]
            process_metrics: true,
        }
    }
}

impl MetricsConfig {
    /// Set [`MetricsConfig::max_series`].
    #[must_use]
    pub fn with_max_series(mut self, max_series: usize) -> Self {
        self.max_series = max_series;
        self
    }

    /// Set [`MetricsConfig::system_interval`].
    #[cfg(feature = "system")]
    #[must_use]
    pub fn with_system_interval(mut self, interval: Duration) -> Self {
        self.system_interval = interval;
        self
    }

    /// Set [`MetricsConfig::health`].
    #[cfg(feature = "system")]
    #[must_use]
    pub fn with_health_config(mut self, health: HealthConfig) -> Self {
        self.health = health;
        self
    }

    /// Set [`MetricsConfig::eager_system`].
    #[cfg(feature = "system")]
    #[must_use]
    pub fn with_eager_system(mut self, eager: bool) -> Self {
        self.eager_system = eager;
        self
    }

    /// Set [`MetricsConfig::process_metrics`].
    #[cfg(feature = "system")]
    #[must_use]
    pub fn with_process_metrics(mut self, enabled: bool) -> Self {
        self.process_metrics = enabled;
        self
    }

    /// Check the configuration.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Config`] when `max_series` is zero.
    pub fn validate(&self) -> Result<()> {
        if self.max_series == 0 {
            return Err(MetricsError::Config("max_series must be at least 1".into()));
        }
        Ok(())
    }
}

/// A registry plus lazily started system monitoring: the object most
/// applications pass around.
///
/// Creating one is cheap: no thread is started and no system call is made
/// until [`MetricsCore::system`] (or a `render_core` exporter) is first used,
/// unless [`MetricsConfig::eager_system`] is set.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "count")]
/// # {
/// use metrics_lib::MetricsCore;
///
/// let core = MetricsCore::new();
/// core.counter("jobs").inc();
/// assert_eq!(core.counter("jobs").get(), 1);
/// # }
/// ```
pub struct MetricsCore {
    registry: Registry,
    config: MetricsConfig,
    #[cfg(feature = "system")]
    system: OnceLock<SystemHealth>,
    #[cfg(feature = "system")]
    process: OnceLock<ProcessCollector>,
}

macro_rules! core_lookups {
    (
        $feature:literal, $ty:ty,
        $plain:ident => $reg_plain:ident,
        $with:ident => $reg_with:ident,
        $try_plain:ident => $reg_try_plain:ident,
        $try_with:ident => $reg_try_with:ident,
        $what:literal
    ) => {
        #[cfg(feature = $feature)]
        impl MetricsCore {
            #[doc = concat!("Get or create the unlabeled ", $what, " `name`. See [`Registry::", stringify!($reg_plain), "`].")]
            #[inline]
            #[must_use]
            pub fn $plain(&self, name: &str) -> Arc<$ty> {
                self.registry.$reg_plain(name)
            }

            #[doc = concat!("Get or create the ", $what, " series `(name, labels)`. See [`Registry::", stringify!($reg_with), "`].")]
            #[inline]
            #[must_use]
            pub fn $with(&self, name: &str, labels: &LabelSet) -> Arc<$ty> {
                self.registry.$reg_with(name, labels)
            }

            #[doc = concat!("Checked unlabeled ", $what, " lookup. See [`Registry::", stringify!($reg_try_plain), "`].")]
            ///
            /// # Errors
            ///
            /// As the registry method.
            #[inline]
            pub fn $try_plain(&self, name: &str) -> Result<Arc<$ty>> {
                self.registry.$reg_try_plain(name)
            }

            #[doc = concat!("Checked labeled ", $what, " lookup. See [`Registry::", stringify!($reg_try_with), "`].")]
            ///
            /// # Errors
            ///
            /// As the registry method.
            #[inline]
            pub fn $try_with(&self, name: &str, labels: &LabelSet) -> Result<Arc<$ty>> {
                self.registry.$reg_try_with(name, labels)
            }
        }
    };
}

core_lookups!("count", Counter, counter => counter, counter_with => counter_with,
    try_counter => try_counter, try_counter_with => try_counter_with, "counter");
core_lookups!("count", ShardedCounter, sharded_counter => sharded_counter,
    sharded_counter_with => sharded_counter_with, try_sharded_counter => try_sharded_counter,
    try_sharded_counter_with => try_sharded_counter_with, "sharded counter");
core_lookups!("count", UpDownCounter, up_down_counter => up_down_counter,
    up_down_counter_with => up_down_counter_with, try_up_down_counter => try_up_down_counter,
    try_up_down_counter_with => try_up_down_counter_with, "up/down counter");
core_lookups!("gauge", Gauge, gauge => gauge, gauge_with => gauge_with,
    try_gauge => try_gauge, try_gauge_with => try_gauge_with, "gauge");
core_lookups!("timer", Timer, timer => timer, timer_with => timer_with,
    try_timer => try_timer, try_timer_with => try_timer_with, "timer");
core_lookups!("meter", RateMeter, rate => rate_meter, rate_with => rate_meter_with,
    try_rate => try_rate_meter, try_rate_with => try_rate_meter_with, "rate meter");
core_lookups!("histogram", Histogram, histogram => histogram, histogram_with => histogram_with,
    try_histogram => try_histogram, try_histogram_with => try_histogram_with, "histogram");
core_lookups!("histogram", Summary, summary => summary, summary_with => summary_with,
    try_summary => try_summary, try_summary_with => try_summary_with, "summary");

impl MetricsCore {
    /// Create a core with [`MetricsConfig::default`].
    #[must_use]
    pub fn new() -> Self {
        Self::build(MetricsConfig::default())
    }

    /// Create a core with a custom configuration.
    ///
    /// # Errors
    ///
    /// Returns the error of [`MetricsConfig::validate`].
    pub fn with_config(config: MetricsConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self::build(config))
    }

    fn build(config: MetricsConfig) -> Self {
        let core = Self {
            registry: Registry::with_cardinality_cap(config.max_series),
            #[cfg(feature = "system")]
            system: OnceLock::new(),
            #[cfg(feature = "system")]
            process: OnceLock::new(),
            config,
        };
        #[cfg(feature = "system")]
        if core.config.eager_system {
            let _ = core.system();
        }
        core
    }

    /// The configuration this core was built with.
    #[must_use]
    pub fn config(&self) -> &MetricsConfig {
        &self.config
    }

    /// The underlying registry.
    #[inline]
    #[must_use]
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// A view that prefixes every name; see [`Registry::scoped`].
    #[must_use]
    pub fn scoped(&self, prefix: impl Into<String>) -> ScopedRegistry<'_> {
        self.registry.scoped(prefix)
    }

    /// Run `f`, record its duration into the timer `name`, and return its
    /// result.
    #[cfg(feature = "timer")]
    pub fn time<T>(&self, name: &str, f: impl FnOnce() -> T) -> T {
        let _guard = self.timer(name).start_owned();
        f()
    }

    /// Start a guard that records into the timer `name` when dropped.
    #[cfg(feature = "timer")]
    #[must_use = "dropping the guard immediately records a near-zero duration"]
    pub fn start_timer(&self, name: &str) -> OwnedTimerGuard {
        self.timer(name).start_owned()
    }

    /// Host and process monitor, started on first call with the configured
    /// interval and thresholds.
    #[cfg(feature = "system")]
    pub fn system(&self) -> &SystemHealth {
        self.system.get_or_init(|| {
            SystemHealth::with_config(self.config.system_interval, self.config.health.clone())
        })
    }

    /// On-demand process sampler, created on first call.
    #[cfg(feature = "system")]
    pub fn process_collector(&self) -> &ProcessCollector {
        self.process.get_or_init(ProcessCollector::new)
    }

    /// The `process_*` metrics, when [`MetricsConfig::process_metrics`] is
    /// enabled and the platform can provide them.
    ///
    /// On Linux, Windows and macOS this samples now (microseconds). On the
    /// targets served by `sysinfo` a sample scans the whole process table
    /// (milliseconds or more), so the latest sample of the background
    /// [`MetricsCore::system`] monitor is returned instead; the first call
    /// starts the monitor and waits for its first sample.
    #[cfg(feature = "system")]
    #[must_use]
    pub fn sample_process_metrics(&self) -> Option<ProcessMetrics> {
        if !self.config.process_metrics {
            return None;
        }
        if cfg!(any(target_os = "linux", target_os = "macos", windows)) {
            return self.process_collector().collect().ok();
        }
        let system = self.system();
        if system.is_sampling() {
            let sample = system.process_metrics();
            // A zero resident size means the platform could not sample it.
            return (sample.resident_memory_bytes > 0).then_some(sample);
        }
        self.process_collector().collect().ok()
    }
}

impl Default for MetricsCore {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for MetricsCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricsCore")
            .field("registry", &self.registry)
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

/// The global instance used by [`metrics()`](fn@metrics).
///
/// Prefer [`init`], [`init_with_config`], [`metrics()`](fn@metrics) and [`try_metrics`]
/// over touching this directly.
pub static METRICS: OnceLock<MetricsCore> = OnceLock::new();

/// Initialise the global instance with the default configuration (if it is
/// not initialised yet) and return it.
pub fn init() -> &'static MetricsCore {
    METRICS.get_or_init(MetricsCore::new)
}

/// Initialise the global instance with `config`.
///
/// Call it once at start-up, before anything uses [`metrics()`](fn@metrics).
///
/// # Errors
///
/// - The error of [`MetricsConfig::validate`].
/// - [`MetricsError::AlreadyInitialized`] when the global instance already
///   exists (for example because [`metrics()`](fn@metrics) was called first); the
///   configuration is not applied.
///
/// # Examples
///
/// ```
/// use metrics_lib::{init_with_config, metrics, MetricsConfig, MetricsError};
///
/// let core = init_with_config(MetricsConfig::default().with_max_series(20_000)).unwrap();
/// assert_eq!(core.registry().cardinality_cap(), 20_000);
/// assert!(std::ptr::eq(core, metrics()));
/// assert_eq!(
///     init_with_config(MetricsConfig::default()).unwrap_err(),
///     MetricsError::AlreadyInitialized
/// );
/// ```
pub fn init_with_config(config: MetricsConfig) -> Result<&'static MetricsCore> {
    config.validate()?;
    let mut fresh = false;
    let core = METRICS.get_or_init(|| {
        fresh = true;
        MetricsCore::build(config)
    });
    if fresh {
        Ok(core)
    } else {
        Err(MetricsError::AlreadyInitialized)
    }
}

/// The global instance, initialised with the default configuration on
/// first use. Never panics.
#[inline]
#[must_use]
pub fn metrics() -> &'static MetricsCore {
    init()
}

/// The global instance, if it was initialised.
#[must_use]
pub fn try_metrics() -> Option<&'static MetricsCore> {
    METRICS.get()
}

/// Glob-import of the most used items.
pub mod prelude {
    #[cfg(feature = "gauge")]
    pub use crate::Gauge;
    #[cfg(feature = "meter")]
    pub use crate::RateMeter;
    #[cfg(feature = "system")]
    pub use crate::SystemHealth;
    #[cfg(feature = "timer")]
    pub use crate::Timer;
    #[cfg(feature = "count")]
    pub use crate::{Counter, ShardedCounter, UpDownCounter};
    #[cfg(feature = "histogram")]
    pub use crate::{Histogram, Summary};
    pub use crate::{
        LabelSet, METRICS, MetricKind, MetricsConfig, MetricsCore, MetricsError, Registry, Result,
        Unit, init, init_with_config, metrics, try_metrics,
    };
}

/// Every Rust block in the README and in `docs/` is compiled and run as a
/// doctest (with all features), so the documentation cannot drift from the
/// API.
#[cfg(all(doctest, feature = "full"))]
mod doc_tests {
    #[doc = include_str!("../README.md")]
    struct Readme;
    #[doc = include_str!("../docs/API.md")]
    struct ApiReference;
    #[doc = include_str!("../docs/GUIDELINES.md")]
    struct Guidelines;
    #[doc = include_str!("../docs/MIGRATION-2.0.md")]
    struct Migration;
    #[doc = include_str!("../docs/migrating-from-metrics-rs.md")]
    struct MigratingFromMetricsRs;
    #[doc = include_str!("../docs/PERFORMANCE.md")]
    struct Performance;
    #[doc = include_str!("../docs/STABILITY.md")]
    struct Stability;
    #[doc = include_str!("../docs/zero-overhead-proof.md")]
    struct ZeroOverhead;
    #[doc = include_str!("../docs/performance-tuning.md")]
    struct PerformanceTuning;
    #[doc = include_str!("../docs/release/v2.0.0.md")]
    struct ReleaseNotes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_and_builders() {
        assert!(MetricsConfig::default().validate().is_ok());
        let zero = MetricsConfig::default().with_max_series(0);
        assert!(matches!(
            MetricsCore::with_config(zero),
            Err(MetricsError::Config(_))
        ));
        let core = MetricsCore::with_config(MetricsConfig::default().with_max_series(7)).unwrap();
        assert_eq!(core.registry().cardinality_cap(), 7);
        assert_eq!(core.config().max_series, 7);
        assert!(format!("{core:?}").contains("MetricsCore"));
    }

    #[test]
    fn test_global_instance_never_panics() {
        let a = metrics();
        let b = init();
        assert!(std::ptr::eq(a, b));
        assert!(try_metrics().is_some());
        assert_eq!(
            init_with_config(MetricsConfig::default()).unwrap_err(),
            MetricsError::AlreadyInitialized
        );
        assert!(init_with_config(MetricsConfig::default().with_max_series(0)).is_err());
    }

    #[cfg(feature = "system")]
    #[test]
    fn test_system_is_lazy_and_configurable() {
        let core = MetricsCore::with_config(
            MetricsConfig::default()
                .with_system_interval(Duration::ZERO)
                .with_health_config(HealthConfig::default())
                .with_process_metrics(false),
        )
        .unwrap();
        assert!(core.system.get().is_none(), "no monitor before first use");
        assert!(!core.system().is_sampling());
        assert!(core.sample_process_metrics().is_none());
        let eager = MetricsCore::with_config(
            MetricsConfig::default()
                .with_eager_system(true)
                .with_system_interval(Duration::ZERO),
        )
        .unwrap();
        assert!(eager.system.get().is_some());
        assert!(eager.sample_process_metrics().is_some());
        let _ = eager.process_collector();
    }

    #[cfg(all(feature = "count", feature = "gauge", feature = "timer"))]
    #[test]
    fn test_core_lookups_and_timing() {
        let core = MetricsCore::default();
        core.counter("c").inc();
        core.gauge("g").set(1.5);
        assert_eq!(core.time("t", || 3), 3);
        {
            let _g = core.start_timer("t");
        }
        assert_eq!(core.timer("t").count(), 2);
        let labels = LabelSet::from([("k", "v")]);
        core.counter_with("c", &labels).inc();
        assert!(core.try_counter("c").is_ok());
        assert!(core.try_counter_with("c", &labels).is_ok());
        assert!(core.try_gauge("c").is_err());
        core.gauge_with("g", &labels).set(2.0);
        assert!(core.try_gauge_with("g", &labels).is_ok());
        assert!(core.try_timer("t").is_ok());
        core.timer_with("t", &labels)
            .record(std::time::Duration::from_millis(1));
        assert!(core.try_timer_with("t", &labels).is_ok());
        core.sharded_counter("s").inc();
        core.sharded_counter_with("s", &labels).inc();
        assert!(core.try_sharded_counter("s").is_ok());
        assert!(core.try_sharded_counter_with("s", &labels).is_ok());
        core.up_down_counter("u").dec();
        core.up_down_counter_with("u", &labels).dec();
        assert!(core.try_up_down_counter("u").is_ok());
        assert!(core.try_up_down_counter_with("u", &labels).is_ok());
        core.scoped("app.").counter("x").inc();
        assert_eq!(core.counter("app.x").get(), 1);
    }

    #[cfg(all(feature = "meter", feature = "histogram"))]
    #[test]
    fn test_core_rate_histogram_summary_lookups() {
        let core = MetricsCore::new();
        let labels = LabelSet::from([("k", "v")]);
        core.rate("r").tick();
        core.rate_with("r", &labels).tick();
        assert!(core.try_rate("r").is_ok());
        assert!(core.try_rate_with("r", &labels).is_ok());
        core.histogram("h").observe(0.1);
        core.histogram_with("h", &labels).observe(0.1);
        assert!(core.try_histogram("h").is_ok());
        assert!(core.try_histogram_with("h", &labels).is_ok());
        core.summary("s").observe(0.1);
        core.summary_with("s", &labels).observe(0.1);
        assert!(core.try_summary("s").is_ok());
        assert!(core.try_summary_with("s", &labels).is_ok());
        assert_eq!(core.registry().metric_count(), 6);
    }
}
