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
//! ```rust
//! use metrics_lib::{init, metrics};
//!
//! // Initialize metrics (do this once at startup)
//! init();
//!
//! // Counters (sub-nanosecond)
//! metrics().counter("requests").inc();
//! metrics().counter("errors").add(5);
//!
//! // Gauges (atomic)  
//! metrics().gauge("cpu_usage").set(87.3);
//! metrics().gauge("memory_mb").set(1024.5);
//!
//! // Timers (high precision)
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
//!
//! // System health
//! let cpu_pct = metrics().system().cpu_used();
//! let mem_mb = metrics().system().mem_used_mb();
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
#[cfg(feature = "meter")]
mod rate_meter;
#[cfg(feature = "timer")]
mod timer;

// Always-compiled infrastructure modules.
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
#[cfg(feature = "meter")]
pub use rate_meter::{RateMeter, RateStats};
#[cfg(feature = "timer")]
pub use timer::*;

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
    #[cfg(feature = "count")]
    #[inline(always)]
    pub fn counter(&self, name: &'static str) -> std::sync::Arc<Counter> {
        self.registry.get_or_create_counter(name)
    }

    /// Get or create a gauge by name. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    #[inline(always)]
    pub fn gauge(&self, name: &'static str) -> std::sync::Arc<Gauge> {
        self.registry.get_or_create_gauge(name)
    }

    /// Get or create a timer by name. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    #[inline(always)]
    pub fn timer(&self, name: &'static str) -> std::sync::Arc<Timer> {
        self.registry.get_or_create_timer(name)
    }

    /// Get or create a rate meter by name. Requires the `meter` feature.
    #[cfg(feature = "meter")]
    #[inline(always)]
    pub fn rate(&self, name: &'static str) -> std::sync::Arc<RateMeter> {
        self.registry.get_or_create_rate_meter(name)
    }

    /// Time a synchronous closure and record the elapsed duration.
    /// Requires the `timer` feature.
    #[cfg(feature = "timer")]
    #[inline]
    pub fn time<T>(&self, name: &'static str, f: impl FnOnce() -> T) -> T {
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

        let s1 = format!("{e1}");
        let s2 = format!("{e2}");
        let s3 = format!("{e3}");
        let s4 = format!("{e4}");

        assert!(s1.contains("Circuit breaker is open"));
        assert!(s2.contains("System is overloaded"));
        assert!(s3.contains("Invalid metric name"));
        assert!(s4.contains("Configuration error"));
        assert!(s4.contains("bad cfg"));
    }
}
