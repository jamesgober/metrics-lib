//! # Ultimate Metrics Library
//!
//! The most powerful, lightweight, and efficient metrics library ever built.
//!
//! ## Features
//!
//! - **Sub-nanosecond operations** - Counter increments in ~2-3ns
//! - **Lock-free everything** - No locks anywhere in hot paths
//! - **System health monitoring** - Built-in CPU/memory tracking
//! - **Dynamic configuration** - Runtime tuning without restarts
//! - **Circuit breakers** - Fault tolerance with auto-recovery
//! - **Dead simple API** - `METRICS.counter("requests").inc()`
//!
//! ## Quick Start
//!
//! ```no_run
//! use metrics_lib::{init, metrics};
//!
//! // Initialize metrics (do this once at startup)
//! init();
//!
//! // Counters (sub-nanosecond)
//! #[cfg(feature = "count")]
//! {
//! metrics().counter("requests").inc();
//! metrics().counter("errors").add(5);
//! }
//!
//! // Gauges (atomic)  
//! #[cfg(feature = "gauge")]
//! {
//! metrics().gauge("cpu_usage").set(87.3);
//! metrics().gauge("memory_mb").set(1024.5);
//! }
//!
//! // Timers (high precision)
//! #[cfg(feature = "timer")]
//! {
//! let timer_metric = metrics().timer("api_call");
//! let timer = timer_metric.start();
//! // ... do work ...
//! timer.stop(); // Auto-records
//!
//! // Or even simpler
//! let result = metrics().time("db_query", || {
//!     // Simulated database query
//!     "user data"
//! });
//! let _ = result;
//! }
//!
//! // System health
//! let cpu_pct = metrics().system().cpu_used();
//! let mem_mb = metrics().system().mem_used_mb();
//! let _ = (cpu_pct, mem_mb);
//!
//! // Rate limiting
//! #[cfg(feature = "meter")]
//! {
//! metrics().rate("api_calls").tick();
//! let rate_per_sec = metrics().rate("api_calls").rate();
//! let _ = rate_per_sec;
//! }
//! ```

#![warn(missing_docs)]
#![allow(unsafe_code)] // For pin-projection in async support

use std::sync::OnceLock;

// Core metric-type modules — each gated on its own Cargo feature.
#[cfg(feature = "sample")]
mod adaptive;
#[cfg(feature = "async")]
mod async_support;
#[cfg(feature = "count")]
mod counter;
#[cfg(feature = "gauge")]
mod gauge;
#[cfg(feature = "histogram")]
mod histogram;
#[cfg(feature = "meter")]
mod rate_meter;
#[cfg(feature = "timer")]
mod timer;

// Always-compiled infrastructure modules.
pub mod exporters;
mod labels;
mod metadata;
mod registry;
mod system_health;

// Public re-exports — gated to match their feature.
#[cfg(feature = "sample")]
pub use adaptive::{
    AdaptiveSampler, BackpressureController, MetricCircuitBreaker, SamplingStrategy,
};
#[cfg(feature = "async")]
pub use async_support::{AsyncMetricBatch, AsyncMetricsBatcher, AsyncTimerExt, AsyncTimerGuard};
#[cfg(feature = "count")]
pub use counter::*;
#[cfg(feature = "gauge")]
pub use gauge::{Gauge, GaugeStats};
#[cfg(feature = "histogram")]
pub use histogram::{Histogram, HistogramBucket, HistogramSnapshot, DEFAULT_SECONDS_BUCKETS};
#[cfg(feature = "meter")]
pub use rate_meter::{RateMeter, RateStats};
#[cfg(feature = "timer")]
pub use timer::*;

pub use labels::{Label, LabelSet};
pub use metadata::{MetricKind, MetricMetadata, Unit};
pub use registry::*;
pub use system_health::*;

// Specialised sub-module re-exports.
#[cfg(feature = "gauge")]
pub use gauge::specialized as gauge_specialized;
#[cfg(feature = "meter")]
pub use rate_meter::specialized as rate_meter_specialized;

/// Global metrics instance - initialize once, use everywhere
pub static METRICS: OnceLock<MetricsCore> = OnceLock::new();

/// Initialize the global metrics instance
///
/// Call this once at the start of your application
pub fn init() -> &'static MetricsCore {
    METRICS.get_or_init(MetricsCore::new)
}

/// Get the global metrics instance
///
/// Panics if not initialized - call `init()` first.
///
/// Panic conditions:
/// - If [`init()`] has not been called yet, this function will panic with a clear message.
///   Prefer passing `&MetricsCore` explicitly in library code to avoid relying on globals.
pub fn metrics() -> &'static MetricsCore {
    METRICS
        .get()
        .expect("Metrics not initialized - call metrics_lib::init() first")
}

/// Main metrics interface - the core of everything
#[repr(align(64))] // Cache line aligned
pub struct MetricsCore {
    registry: Registry,
    system: SystemHealth,
}

impl MetricsCore {
    /// Create new metrics core
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            system: SystemHealth::new(),
        }
    }

    /// Get or create a counter by name. Requires the `count` feature.
    ///
    /// `name` is accepted as `&str` — string literals (`"counter"`) and
    /// owned/borrowed runtime names both work. The first lookup for a given
    /// name allocates a `String` key inside the registry; subsequent lookups
    /// of the same name reuse the cached `Arc` and perform no allocations.
    #[cfg(feature = "count")]
    #[inline(always)]
    pub fn counter(&self, name: &str) -> std::sync::Arc<Counter> {
        self.registry.get_or_create_counter(name)
    }

    /// Get or create a labeled counter. Requires the `count` feature.
    ///
    /// Routes to the cardinality overflow sink when the cap is full; use
    /// [`Self::try_counter_with`] to receive an explicit error instead.
    #[cfg(feature = "count")]
    #[inline]
    pub fn counter_with(&self, name: &str, labels: &LabelSet) -> std::sync::Arc<Counter> {
        self.registry.get_or_create_counter_with(name, labels)
    }

    /// Labeled counter returning `Err(CardinalityExceeded)` when the cap is
    /// full. Requires the `count` feature.
    #[cfg(feature = "count")]
    #[inline]
    pub fn try_counter_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<std::sync::Arc<Counter>> {
        self.registry.try_get_or_create_counter_with(name, labels)
    }

    /// Get or create a gauge by name. Requires the `gauge` feature.
    ///
    /// `name` is accepted as `&str` — see [`Self::counter`] for allocation
    /// semantics.
    #[cfg(feature = "gauge")]
    #[inline(always)]
    pub fn gauge(&self, name: &str) -> std::sync::Arc<Gauge> {
        self.registry.get_or_create_gauge(name)
    }

    /// Get or create a labeled gauge. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    #[inline]
    pub fn gauge_with(&self, name: &str, labels: &LabelSet) -> std::sync::Arc<Gauge> {
        self.registry.get_or_create_gauge_with(name, labels)
    }

    /// Labeled gauge returning `Err(CardinalityExceeded)` when the cap is
    /// full. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    #[inline]
    pub fn try_gauge_with(&self, name: &str, labels: &LabelSet) -> Result<std::sync::Arc<Gauge>> {
        self.registry.try_get_or_create_gauge_with(name, labels)
    }

    /// Get or create a timer by name. Requires the `timer` feature.
    ///
    /// `name` is accepted as `&str` — see [`Self::counter`] for allocation
    /// semantics.
    #[cfg(feature = "timer")]
    #[inline(always)]
    pub fn timer(&self, name: &str) -> std::sync::Arc<Timer> {
        self.registry.get_or_create_timer(name)
    }

    /// Get or create a labeled timer. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    #[inline]
    pub fn timer_with(&self, name: &str, labels: &LabelSet) -> std::sync::Arc<Timer> {
        self.registry.get_or_create_timer_with(name, labels)
    }

    /// Labeled timer returning `Err(CardinalityExceeded)` when the cap is
    /// full. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    #[inline]
    pub fn try_timer_with(&self, name: &str, labels: &LabelSet) -> Result<std::sync::Arc<Timer>> {
        self.registry.try_get_or_create_timer_with(name, labels)
    }

    /// Get or create a rate meter by name. Requires the `meter` feature.
    ///
    /// `name` is accepted as `&str` — see [`Self::counter`] for allocation
    /// semantics.
    #[cfg(feature = "meter")]
    #[inline(always)]
    pub fn rate(&self, name: &str) -> std::sync::Arc<RateMeter> {
        self.registry.get_or_create_rate_meter(name)
    }

    /// Get or create a labeled rate meter. Requires the `meter` feature.
    #[cfg(feature = "meter")]
    #[inline]
    pub fn rate_with(&self, name: &str, labels: &LabelSet) -> std::sync::Arc<RateMeter> {
        self.registry.get_or_create_rate_meter_with(name, labels)
    }

    /// Labeled rate meter returning `Err(CardinalityExceeded)` when the cap
    /// is full. Requires the `meter` feature.
    #[cfg(feature = "meter")]
    #[inline]
    pub fn try_rate_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<std::sync::Arc<RateMeter>> {
        self.registry
            .try_get_or_create_rate_meter_with(name, labels)
    }

    /// Get or create an unlabeled histogram. Requires the `histogram`
    /// feature.
    ///
    /// Uses buckets pre-configured via [`Registry::configure_histogram`] for
    /// the same name, or the standard Prometheus latency-seconds buckets
    /// ([`crate::DEFAULT_SECONDS_BUCKETS`]) when none configured.
    #[cfg(feature = "histogram")]
    #[inline]
    pub fn histogram(&self, name: &str) -> std::sync::Arc<Histogram> {
        self.registry.get_or_create_histogram(name)
    }

    /// Get or create a labeled histogram. Requires the `histogram` feature.
    #[cfg(feature = "histogram")]
    #[inline]
    pub fn histogram_with(&self, name: &str, labels: &LabelSet) -> std::sync::Arc<Histogram> {
        self.registry.get_or_create_histogram_with(name, labels)
    }

    /// Labeled histogram returning `Err(CardinalityExceeded)` when the cap
    /// is full. Requires the `histogram` feature.
    #[cfg(feature = "histogram")]
    #[inline]
    pub fn try_histogram_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<std::sync::Arc<Histogram>> {
        self.registry.try_get_or_create_histogram_with(name, labels)
    }

    /// Time a synchronous closure and record the elapsed duration.
    /// Requires the `timer` feature.
    #[cfg(feature = "timer")]
    #[inline]
    pub fn time<T>(&self, name: &str, f: impl FnOnce() -> T) -> T {
        let binding = self.timer(name);
        let timer = binding.start();
        let result = f();
        timer.stop();
        result
    }

    /// Get system health interface
    #[inline(always)]
    pub fn system(&self) -> &SystemHealth {
        &self.system
    }

    /// Get registry for advanced operations
    #[inline(always)]
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}

impl Default for MetricsCore {
    fn default() -> Self {
        Self::new()
    }
}

/// Common result type for metrics operations
pub type Result<T> = std::result::Result<T, MetricsError>;

/// Metrics errors
#[derive(Debug, Clone, PartialEq)]
pub enum MetricsError {
    /// Circuit breaker is open and the operation is not allowed.
    CircuitOpen,
    /// System is overloaded (e.g., adaptive sampler reduced acceptance) and rejected the operation.
    Overloaded,
    /// Invalid metric name (empty, overly long, or otherwise rejected by a policy).
    InvalidName,
    /// Invalid value supplied (NaN, non-finite, out-of-range, or otherwise invalid).
    InvalidValue {
        /// Short, static explanation of why the value was invalid (e.g., "value is not finite").
        reason: &'static str,
    },
    /// Arithmetic would overflow the counter or index (checked variants only).
    Overflow,
    /// Arithmetic would underflow (checked variants only).
    Underflow,
    /// Operation would exceed a configured limit (rate limiting, quotas, etc.).
    OverLimit,
    /// Operation would block and a non-blocking/try path was requested.
    WouldBlock,
    /// Global metrics were not initialized and the operation requires initialization.
    NotInitialized,
    /// Registering this `(name, labels)` combination would exceed the
    /// configured cardinality cap. The `try_*_with` lookup variants return
    /// this error; the non-`try` variants route to a per-type sink instead
    /// (see [`Registry::set_cardinality_cap`]).
    CardinalityExceeded,
    /// Configuration error with details.
    Config(String),
}

impl std::fmt::Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricsError::CircuitOpen => write!(f, "Circuit breaker is open"),
            MetricsError::Overloaded => write!(f, "System is overloaded"),
            MetricsError::InvalidName => write!(f, "Invalid metric name"),
            MetricsError::InvalidValue { reason } => write!(f, "Invalid value: {reason}"),
            MetricsError::Overflow => write!(f, "Operation would overflow"),
            MetricsError::Underflow => write!(f, "Operation would underflow"),
            MetricsError::OverLimit => write!(f, "Operation would exceed limit"),
            MetricsError::WouldBlock => write!(f, "Operation would block"),
            MetricsError::NotInitialized => write!(f, "Global metrics not initialized"),
            MetricsError::CardinalityExceeded => {
                write!(f, "Cardinality cap exceeded for labeled metric")
            }
            MetricsError::Config(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for MetricsError {}

/// Prelude for convenient glob imports.
///
/// Items that require a Cargo feature are only re-exported when that feature is
/// enabled — they will be absent from the prelude on minimal builds.
pub mod prelude {
    #[cfg(feature = "count")]
    pub use crate::Counter;
    #[cfg(feature = "gauge")]
    pub use crate::Gauge;
    #[cfg(feature = "meter")]
    pub use crate::RateMeter;
    #[cfg(feature = "timer")]
    pub use crate::Timer;
    pub use crate::{init, metrics, MetricsCore, MetricsError, Result, METRICS};
    pub use crate::{Registry, SystemHealth};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        let metrics = MetricsCore::new();
        // SystemHealth is always available regardless of active features.
        let _cpu = metrics.system().cpu_used();
        let _mem = metrics.system().mem_used_mb();
        #[cfg(feature = "count")]
        {
            metrics.counter("test").inc();
            assert_eq!(metrics.counter("test").get(), 1);
        }
        #[cfg(feature = "gauge")]
        {
            metrics.gauge("test").set(42.5);
            assert_eq!(metrics.gauge("test").get(), 42.5);
        }
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_global_metrics() {
        let _metrics = init();
        metrics().counter("global_test").inc();
        assert_eq!(metrics().counter("global_test").get(), 1);
    }

    #[cfg(feature = "timer")]
    #[test]
    fn test_time_function_records_and_returns() {
        let metrics = MetricsCore::new();
        let result = metrics.time("timed_op", || 123usize);
        assert_eq!(result, 123);
        assert_eq!(metrics.timer("timed_op").count(), 1);
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_accessors_system_and_registry() {
        let metrics = MetricsCore::new();
        let _ = metrics.system().cpu_used();
        let reg = metrics.registry();
        let c = reg.get_or_create_counter("from_registry");
        c.add(2);
        assert_eq!(metrics.counter("from_registry").get(), 2);
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_default_impl() {
        let metrics: MetricsCore = Default::default();
        metrics.counter("default_impl").inc();
        assert_eq!(metrics.counter("default_impl").get(), 1);
    }

    #[test]
    fn test_metrics_error_display() {
        let e1 = MetricsError::CircuitOpen;
        let e2 = MetricsError::Overloaded;
        let e3 = MetricsError::InvalidName;
        let e4 = MetricsError::Config("bad cfg".to_string());
        let e5 = MetricsError::CardinalityExceeded;
        let e6 = MetricsError::Overflow;
        let e7 = MetricsError::Underflow;
        let e8 = MetricsError::OverLimit;
        let e9 = MetricsError::WouldBlock;
        let e10 = MetricsError::NotInitialized;
        let e11 = MetricsError::InvalidValue { reason: "x" };

        for (err, needle) in [
            (e1, "Circuit breaker is open"),
            (e2, "System is overloaded"),
            (e3, "Invalid metric name"),
            (e5, "Cardinality"),
            (e6, "overflow"),
            (e7, "underflow"),
            (e8, "exceed"),
            (e9, "block"),
            (e10, "not initialized"),
            (e11, "Invalid value"),
        ] {
            assert!(
                format!("{err}")
                    .to_lowercase()
                    .contains(&needle.to_lowercase()),
                "err {err:?} should contain {needle}"
            );
        }
        let s4 = format!("{e4}");
        assert!(s4.contains("Configuration error"));
        assert!(s4.contains("bad cfg"));
    }

    // ---------- v0.9.3 MetricsCore labeled-method coverage ----------

    #[test]
    #[cfg(all(feature = "count", feature = "gauge", feature = "timer"))]
    fn metricscore_labeled_methods_exercise_all_paths() {
        let m = MetricsCore::new();
        let labels = LabelSet::from([("k", "v")]);

        // counter_with + try_counter_with happy paths
        m.counter_with("c", &labels).inc();
        assert!(m.try_counter_with("c", &labels).is_ok());
        // gauge_with + try_gauge_with
        m.gauge_with("g", &labels).set(2.5);
        assert!(m.try_gauge_with("g", &labels).is_ok());
        // timer_with + try_timer_with
        m.timer_with("t", &labels)
            .record(std::time::Duration::from_micros(1));
        assert!(m.try_timer_with("t", &labels).is_ok());

        assert_eq!(m.registry().cardinality_count(), 3);
    }

    #[test]
    #[cfg(feature = "meter")]
    fn metricscore_rate_with_paths() {
        let m = MetricsCore::new();
        let labels = LabelSet::from([("tier", "1")]);
        m.rate_with("qps", &labels).tick();
        assert!(m.try_rate_with("qps", &labels).is_ok());
        assert_eq!(m.registry().cardinality_count(), 1);
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn metricscore_histogram_paths() {
        let m = MetricsCore::new();
        m.histogram("default_buckets").observe(0.5);
        let labels = LabelSet::from([("op", "x")]);
        m.histogram_with("custom", &labels).observe(0.1);
        assert!(m.try_histogram_with("custom", &labels).is_ok());
    }
}
