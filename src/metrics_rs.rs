//! Bridge for the [`metrics`](https://docs.rs/metrics) facade (feature
//! `metrics-rs`).
//!
//! Many crates (HTTP servers, database pools, client libraries) instrument
//! themselves with the `metrics` facade macros (`metrics::counter!`, …)
//! and report into whatever *recorder* the application installs.
//! [`MetricsLibRecorder`] is such a recorder: everything recorded through the
//! facade lands in a [`MetricsCore`] next to your own metrics and leaves
//! through any metrics-lib exporter.
//!
//! | Facade | metrics-lib |
//! |---|---|
//! | `counter!` | [`Counter`] (`increment` adds, `absolute` raises) |
//! | `gauge!` | [`Gauge`] |
//! | `histogram!` | [`Histogram`] with the family's configured buckets (default: [`DEFAULT_SECONDS_BUCKETS`](crate::DEFAULT_SECONDS_BUCKETS)) |
//! | `describe_*!` | [`Registry::describe`](crate::Registry::describe) |
//!
//! Facade names and label keys are free-form; they are repaired with
//! [`sanitize_metric_name`](crate::sanitize_metric_name) /
//! [`sanitize_label_key`](crate::sanitize_label_key) (so `http.server.requests`
//! stays, `my-metric` becomes `my_metric`), and every registry limit (series
//! cap, kind conflicts) applies as usual.
//!
//! # Example
//!
//! ```
//! use metrics_lib::metrics_rs::MetricsLibRecorder;
//! use metrics_lib::MetricsCore;
//! use std::sync::Arc;
//!
//! let core = Arc::new(MetricsCore::new());
//! let recorder = MetricsLibRecorder::new(Arc::clone(&core));
//! metrics::with_local_recorder(&recorder, || {
//!     metrics::counter!("requests", "method" => "GET").increment(2);
//! });
//! let labels = metrics_lib::labels!("method" => "GET");
//! assert_eq!(core.counter_with("requests", &labels).get(), 2);
//! ```
//!
//! Install it process-wide once at start-up with [`install`].

use crate::{Counter, Gauge, Histogram, LabelSet, MetricKind, MetricMetadata, MetricsCore, Unit};
use crate::{sanitize_label_key, sanitize_metric_name};
use metrics::{Key, KeyName, Metadata, SharedString};
use std::sync::Arc;

impl metrics::CounterFn for Counter {
    fn increment(&self, value: u64) {
        self.add(value);
    }

    fn absolute(&self, value: u64) {
        self.set_max(value);
    }
}

impl metrics::GaugeFn for Gauge {
    fn increment(&self, value: f64) {
        self.add(value);
    }

    fn decrement(&self, value: f64) {
        self.sub(value);
    }

    fn set(&self, value: f64) {
        Gauge::set(self, value);
    }
}

impl metrics::HistogramFn for Histogram {
    fn record(&self, value: f64) {
        self.observe(value);
    }
}

/// A `metrics::Recorder` that records into a [`MetricsCore`].
#[derive(Debug, Clone)]
pub struct MetricsLibRecorder {
    core: Arc<MetricsCore>,
}

impl MetricsLibRecorder {
    /// Record into `core`.
    #[must_use]
    pub fn new(core: Arc<MetricsCore>) -> Self {
        Self { core }
    }

    /// The core this recorder writes to.
    #[must_use]
    pub fn core(&self) -> &Arc<MetricsCore> {
        &self.core
    }

    fn describe(
        &self,
        key: &KeyName,
        kind: MetricKind,
        unit: Option<metrics::Unit>,
        help: SharedString,
    ) {
        let name = sanitize_metric_name(key.as_str());
        let metadata =
            MetricMetadata::new(kind, help.into_owned(), unit.map_or(Unit::None, map_unit));
        // The `Recorder` trait cannot return errors. A description that does
        // not apply (kind conflict, full cap) is dropped; the metric itself
        // still records, and conflicts surface through
        // `Registry::kind_conflicts` when it is registered.
        let _ = self.core.registry().describe(&name, metadata);
    }
}

fn labels_of(key: &Key) -> LabelSet {
    key.labels()
        .map(|l| {
            (
                sanitize_label_key(l.key()).into_owned(),
                l.value().to_owned(),
            )
        })
        .collect()
}

fn map_unit(unit: metrics::Unit) -> Unit {
    match unit {
        metrics::Unit::Seconds => Unit::Seconds,
        metrics::Unit::Milliseconds => Unit::Milliseconds,
        metrics::Unit::Microseconds => Unit::Microseconds,
        metrics::Unit::Nanoseconds => Unit::Nanoseconds,
        metrics::Unit::Bytes => Unit::Bytes,
        metrics::Unit::Kibibytes => Unit::Kilobytes,
        metrics::Unit::Mebibytes => Unit::Megabytes,
        metrics::Unit::Gibibytes => Unit::Gigabytes,
        metrics::Unit::Percent => Unit::Percent,
        metrics::Unit::Count => Unit::None,
        other => Unit::Custom(other.as_str()),
    }
}

impl metrics::Recorder for MetricsLibRecorder {
    fn describe_counter(
        &self,
        key: KeyName,
        unit: Option<metrics::Unit>,
        description: SharedString,
    ) {
        self.describe(&key, MetricKind::Counter, unit, description);
    }

    fn describe_gauge(&self, key: KeyName, unit: Option<metrics::Unit>, description: SharedString) {
        self.describe(&key, MetricKind::Gauge, unit, description);
    }

    fn describe_histogram(
        &self,
        key: KeyName,
        unit: Option<metrics::Unit>,
        description: SharedString,
    ) {
        self.describe(&key, MetricKind::Histogram, unit, description);
    }

    fn register_counter(&self, key: &Key, _: &Metadata<'_>) -> metrics::Counter {
        let name = sanitize_metric_name(key.name());
        metrics::Counter::from_arc(self.core.counter_with(&name, &labels_of(key)))
    }

    fn register_gauge(&self, key: &Key, _: &Metadata<'_>) -> metrics::Gauge {
        let name = sanitize_metric_name(key.name());
        metrics::Gauge::from_arc(self.core.gauge_with(&name, &labels_of(key)))
    }

    fn register_histogram(&self, key: &Key, _: &Metadata<'_>) -> metrics::Histogram {
        let name = sanitize_metric_name(key.name());
        metrics::Histogram::from_arc(self.core.histogram_with(&name, &labels_of(key)))
    }
}

/// Install a [`MetricsLibRecorder`] for `core` as the process-wide `metrics`
/// recorder.
///
/// # Errors
///
/// Returns `metrics::SetRecorderError` when a global recorder is already
/// installed.
pub fn install(
    core: Arc<MetricsCore>,
) -> Result<(), metrics::SetRecorderError<MetricsLibRecorder>> {
    metrics::set_global_recorder(MetricsLibRecorder::new(core))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facade_counters_gauges_histograms_land_in_the_core() {
        let core = Arc::new(MetricsCore::new());
        let recorder = MetricsLibRecorder::new(Arc::clone(&core));
        assert!(Arc::ptr_eq(recorder.core(), &core));
        metrics::with_local_recorder(&recorder, || {
            metrics::describe_counter!("jobs", metrics::Unit::Count, "Jobs done");
            metrics::describe_gauge!("depth", metrics::Unit::Bytes, "Queue depth");
            metrics::describe_histogram!("lat", metrics::Unit::Seconds, "Latency");
            metrics::counter!("jobs", "queue" => "a").increment(3);
            metrics::counter!("jobs", "queue" => "a").absolute(10);
            metrics::counter!("jobs", "queue" => "a").absolute(4);
            metrics::gauge!("depth").set(5.0);
            metrics::gauge!("depth").increment(2.0);
            metrics::gauge!("depth").decrement(1.0);
            metrics::histogram!("lat").record(0.25);
            metrics::counter!("http.requests-total", "bad-key" => "v").increment(1);
        });
        let labels = LabelSet::from([("queue", "a")]);
        assert_eq!(core.counter_with("jobs", &labels).get(), 10);
        assert!((core.gauge("depth").get() - 6.0).abs() < f64::EPSILON);
        assert_eq!(core.histogram("lat").count(), 1);
        assert_eq!(core.registry().metadata("jobs").unwrap().help, "Jobs done");
        assert_eq!(core.registry().metadata("depth").unwrap().unit, Unit::Bytes);
        assert_eq!(core.registry().metadata("lat").unwrap().unit, Unit::Seconds);
        let repaired = LabelSet::from([("bad_key", "v")]);
        assert_eq!(core.counter_with("http.requests_total", &repaired).get(), 1);
    }

    #[test]
    fn test_unit_mapping() {
        assert_eq!(map_unit(metrics::Unit::Milliseconds), Unit::Milliseconds);
        assert_eq!(map_unit(metrics::Unit::Microseconds), Unit::Microseconds);
        assert_eq!(map_unit(metrics::Unit::Nanoseconds), Unit::Nanoseconds);
        assert_eq!(map_unit(metrics::Unit::Kibibytes), Unit::Kilobytes);
        assert_eq!(map_unit(metrics::Unit::Mebibytes), Unit::Megabytes);
        assert_eq!(map_unit(metrics::Unit::Gibibytes), Unit::Gigabytes);
        assert_eq!(map_unit(metrics::Unit::Percent), Unit::Percent);
        assert!(matches!(
            map_unit(metrics::Unit::BitsPerSecond),
            Unit::Custom(_)
        ));
    }

    #[test]
    fn test_install_sets_the_global_recorder_once() {
        let core = Arc::new(MetricsCore::new());
        assert!(install(Arc::clone(&core)).is_ok());
        metrics::counter!("installed_total").increment(1);
        assert_eq!(core.counter("installed_total").get(), 1);
        assert!(install(core).is_err());
    }

    #[test]
    fn test_conflicting_description_is_dropped_but_metric_records() {
        let core = Arc::new(MetricsCore::new());
        core.gauge("jobs").set(1.0);
        let recorder = MetricsLibRecorder::new(Arc::clone(&core));
        metrics::with_local_recorder(&recorder, || {
            metrics::describe_counter!("jobs", "would conflict");
        });
        assert_eq!(core.registry().kind_of("jobs"), Some(MetricKind::Gauge));
    }
}
