//! [`tracing`](https://docs.rs/tracing) integration (feature `tracing`).
//!
//! - [`time_in_span`] runs a closure inside an `info_span!` and records its
//!   duration into a [`Timer`], so one call site feeds both the
//!   metric and the trace.
//! - [`span_exemplar`] turns the current span into exemplar labels, and
//!   [`observe_with_span_exemplar`] records a histogram observation with it,
//!   so a latency bucket in Grafana links back to the span that produced it.
//!
//! Enabling this feature changes nothing on the metric hot paths.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "timer")]
//! # {
//! use metrics_lib::MetricsCore;
//! use metrics_lib::tracing_ext::time_in_span;
//!
//! let core = MetricsCore::new();
//! let timer = core.timer("db_query_seconds");
//! let rows = time_in_span("db.query", &timer, || 42);
//! assert_eq!(rows, 42);
//! assert_eq!(timer.count(), 1);
//! # }
//! ```

#[cfg(feature = "timer")]
use crate::Timer;

/// Run `f` inside `tracing::info_span!("metric.time", name = name)` and
/// record its duration into `timer`.
#[cfg(feature = "timer")]
pub fn time_in_span<T>(name: &'static str, timer: &Timer, f: impl FnOnce() -> T) -> T {
    let span = tracing::info_span!("metric.time", name = name);
    let _entered = span.enter();
    let _guard = timer.start();
    f()
}

/// [`time_in_span`] with the global instance's timer named `name`.
#[cfg(feature = "timer")]
pub fn time_global<T>(name: &'static str, f: impl FnOnce() -> T) -> T {
    let timer = crate::metrics().timer(name);
    time_in_span(name, &timer, f)
}

/// Exemplar labels for the current span: `{span_id="<16 hex digits>"}`.
///
/// Returns `None` outside any span, or when the installed subscriber does
/// not track the current span. `tracing` span ids identify spans within one
/// process; to link to a distributed trace, add your propagated `trace_id`
/// to the labels as well.
#[must_use]
pub fn span_exemplar() -> Option<crate::LabelSet> {
    span_exemplar_for(&tracing::Span::current())
}

/// Exemplar labels for `span`: `{span_id="<16 hex digits>"}`, or `None` when
/// the span is disabled (has no id).
#[must_use]
pub fn span_exemplar_for(span: &tracing::Span) -> Option<crate::LabelSet> {
    let id = span.id()?;
    let mut labels = crate::LabelSet::new();
    let _ = labels.add("span_id", format!("{:016x}", id.into_u64()));
    Some(labels)
}

/// Observe `value` in `histogram`, attaching the current span as the
/// bucket's exemplar when there is one.
#[cfg(feature = "histogram")]
pub fn observe_with_span_exemplar(histogram: &crate::Histogram, value: f64) {
    match span_exemplar() {
        Some(labels) => histogram.observe_with_exemplar(value, &labels),
        None => histogram.observe(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "timer")]
    #[test]
    fn test_time_in_span_records_into_timer() {
        let t = Timer::new();
        assert_eq!(time_in_span("test.span", &t, || 7), 7);
        assert_eq!(t.count(), 1);
    }

    #[cfg(feature = "timer")]
    #[test]
    fn test_time_global_uses_the_global_registry() {
        assert_eq!(time_global("tracing_ext_global", || 11), 11);
        assert!(crate::metrics().timer("tracing_ext_global").count() >= 1);
    }

    #[test]
    fn test_span_exemplar_outside_span_is_none() {
        assert!(span_exemplar().is_none());
    }

    #[cfg(feature = "histogram")]
    #[test]
    fn test_observe_without_span_records_plain_observation() {
        let h = crate::Histogram::with_buckets([1.0]);
        observe_with_span_exemplar(&h, 0.5);
        assert_eq!(h.count(), 1);
        assert!(h.snapshot().exemplar(0).is_none());
    }

    #[test]
    fn test_span_exemplar_for_formats_span_id() {
        let dispatch = tracing::Dispatch::new(IdSubscriber::default());
        tracing::dispatcher::with_default(&dispatch, || {
            let span = tracing::info_span!("request");
            let labels = span_exemplar_for(&span).unwrap();
            assert_eq!(labels.get("span_id"), Some("0000000000000001"));
        });
        assert!(span_exemplar_for(&tracing::Span::none()).is_none());
    }

    /// A subscriber that only hands out sequential span ids.
    #[derive(Default)]
    struct IdSubscriber {
        next: std::sync::atomic::AtomicU64,
    }

    impl tracing::Subscriber for IdSubscriber {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            let n = self.next.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            tracing::span::Id::from_u64(n)
        }
        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
        fn event(&self, _: &tracing::Event<'_>) {}
        fn enter(&self, _: &tracing::span::Id) {}
        fn exit(&self, _: &tracing::span::Id) {}
    }
}
