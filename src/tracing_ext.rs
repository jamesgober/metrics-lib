//! Optional [`tracing`](https://docs.rs/tracing) integration helpers
//! (v0.9.5, behind the `tracing` Cargo feature).
//!
//! This module is intentionally minimal: it adds opt-in adapters that wrap
//! existing `Timer`/`MetricsCore` operations with a `tracing` span, so the
//! observation shows up in both the metric histogram **and** the user's
//! tracing subscriber. Hot paths in the metric types themselves are
//! unchanged — enabling the `tracing` feature does not slow down
//! `Counter::inc`, `Gauge::set`, `Timer::record`, etc.
//!
//! # When to use
//!
//! - You already use `tracing` for structured logging and want per-operation
//!   timings to appear under a named span in addition to being recorded as
//!   a metric.
//! - You're profiling a code path and want one call site to populate both
//!   the metrics export and the tracing flamegraph.
//!
//! # Example
//!
//! ```
//! # #[cfg(all(feature = "timer", feature = "tracing"))]
//! # {
//! use metrics_lib::{init, metrics};
//! use metrics_lib::tracing_ext::time_in_span;
//!
//! init();
//! let timer = metrics().timer("db.query");
//! let result = time_in_span("db.query", &timer, || {
//!     // ... work happens inside both the Timer and a tracing info_span!
//!     42
//! });
//! assert_eq!(result, 42);
//! # }
//! ```

#[cfg(feature = "timer")]
use crate::Timer;
use std::time::Instant;

/// Time `f` and record the elapsed duration into `timer`, simultaneously
/// emitting a `tracing::info_span!("metric.time", name = …)` for the
/// duration of the call.
///
/// Requires the `tracing` and `timer` features.
#[cfg(feature = "timer")]
pub fn time_in_span<T>(name: &'static str, timer: &Timer, f: impl FnOnce() -> T) -> T {
    let span = tracing::info_span!("metric.time", name = name);
    let _enter = span.enter();
    let start = Instant::now();
    let result = f();
    timer.record(start.elapsed());
    result
}

/// Time `f` and record the elapsed duration into the global registry's
/// timer named `name`. Wraps the call in a `tracing::info_span!`.
///
/// Equivalent to `time_in_span(name, &metrics().timer(name), f)` but avoids
/// looking up the timer manually. Requires the `tracing` and `timer`
/// features.
#[cfg(feature = "timer")]
pub fn time_global<T>(name: &'static str, f: impl FnOnce() -> T) -> T {
    let timer = crate::metrics().timer(name);
    time_in_span(name, &timer, f)
}

#[cfg(all(test, feature = "timer"))]
mod tests {
    use super::*;

    #[test]
    fn time_in_span_records_into_timer() {
        let t = Timer::new();
        let v = time_in_span("test.span", &t, || 7);
        assert_eq!(v, 7);
        assert_eq!(t.count(), 1);
    }

    #[test]
    fn time_global_records_under_named_timer() {
        crate::init();
        // Use a distinctive name to avoid collisions with other tests.
        let v = time_global("tracing_ext.global", || 11);
        assert_eq!(v, 11);
        assert!(crate::metrics().timer("tracing_ext.global").count() >= 1);
    }
}
