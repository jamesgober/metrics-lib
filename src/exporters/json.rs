//! JSON snapshot exporter (behind the `serde` Cargo feature).
//!
//! Captures the entire registry as a single serializable [`RegistrySnapshot`]
//! value. The struct is `#[derive(serde::Serialize)]` so callers can route it
//! to any `serde`-compatible writer (`serde_json`, `simd-json`, `rmp-serde`,
//! …).
//!
//! For convenience, [`render`] returns a JSON `String` directly via
//! `serde_json`. The structure is stable, diff-friendly, and suitable for:
//!
//! - HTTP `/debug/metrics` endpoints.
//! - Periodic disk snapshots (config hot-reload, post-mortem diagnostics).
//! - Test-fixture comparisons.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{init, metrics, LabelSet};
//! use metrics_lib::exporters::json;
//!
//! init();
//! metrics().counter("hits").inc();
//! metrics().counter_with("hits", &LabelSet::from([("status", "200")])).inc();
//!
//! let snap = json::snapshot(metrics().registry());
//! assert_eq!(snap.counters.len(), 2);
//!
//! // Render as JSON text via serde_json.
//! let body = json::render(metrics().registry());
//! assert!(body.contains("\"counters\""));
//! # }
//! ```

use crate::{LabelSet, MetricMetadata, Registry};

/// A single counter series snapshot.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CounterSeries {
    /// Metric name.
    pub name: String,
    /// Label set; serialised as a JSON object.
    pub labels: LabelSet,
    /// Current counter value.
    pub value: u64,
    /// Optional metadata (help / unit / kind).
    pub metadata: Option<MetricMetadata>,
}

/// A single gauge series snapshot.
#[derive(Debug, Clone, serde::Serialize)]
pub struct GaugeSeries {
    /// Metric name.
    pub name: String,
    /// Label set.
    pub labels: LabelSet,
    /// Current gauge value.
    pub value: f64,
    /// Optional metadata.
    pub metadata: Option<MetricMetadata>,
}

/// A single timer series snapshot.
#[cfg(feature = "timer")]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TimerSeries {
    /// Metric name.
    pub name: String,
    /// Label set.
    pub labels: LabelSet,
    /// Timer statistics.
    pub stats: crate::TimerStats,
    /// Optional metadata.
    pub metadata: Option<MetricMetadata>,
}

/// A single rate-meter series snapshot.
#[cfg(feature = "meter")]
#[derive(Debug, Clone, serde::Serialize)]
pub struct RateSeries {
    /// Metric name.
    pub name: String,
    /// Label set.
    pub labels: LabelSet,
    /// Rate statistics.
    pub stats: crate::RateStats,
    /// Optional metadata.
    pub metadata: Option<MetricMetadata>,
}

/// A single histogram series snapshot.
#[cfg(feature = "histogram")]
#[derive(Debug, Clone, serde::Serialize)]
pub struct HistogramSeries {
    /// Metric name.
    pub name: String,
    /// Label set.
    pub labels: LabelSet,
    /// Histogram snapshot (cumulative buckets + sum + count).
    pub histogram: crate::HistogramSnapshot,
    /// Optional metadata.
    pub metadata: Option<MetricMetadata>,
}

/// Top-level snapshot of every metric in the registry.
///
/// Feature-gated sections of the struct (`timers`, `rate_meters`,
/// `histograms`) are present only when the corresponding metric-type feature
/// is enabled.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RegistrySnapshot {
    /// Schema version of this snapshot format. Bumps when the JSON shape
    /// changes in a breaking way.
    pub schema_version: u32,
    /// All counter series.
    pub counters: Vec<CounterSeries>,
    /// All gauge series.
    pub gauges: Vec<GaugeSeries>,
    /// All timer series.
    #[cfg(feature = "timer")]
    pub timers: Vec<TimerSeries>,
    /// All rate-meter series.
    #[cfg(feature = "meter")]
    pub rate_meters: Vec<RateSeries>,
    /// All histogram series.
    #[cfg(feature = "histogram")]
    pub histograms: Vec<HistogramSeries>,
    /// Cardinality counter snapshot (`current`, `cap`, `overflows`).
    pub cardinality: CardinalitySnapshot,
}

/// Cardinality state at snapshot time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CardinalitySnapshot {
    /// Current count of unique `(name, labels)` tuples.
    pub current: usize,
    /// Configured cap.
    pub cap: usize,
    /// Lifetime overflow events.
    pub overflows: u64,
}

const SCHEMA_VERSION: u32 = 1;

/// Build a [`RegistrySnapshot`] containing every metric in `registry`.
///
/// Series are sorted by `(name, labels)` for deterministic, diff-friendly
/// output. Snapshot is point-in-time; updates after the call are not
/// reflected.
#[must_use]
pub fn snapshot(registry: &Registry) -> RegistrySnapshot {
    let mut counters = Vec::new();
    #[cfg(feature = "count")]
    {
        for (name, labels, c) in registry.counter_entries() {
            let metadata = registry.metadata(&name);
            counters.push(CounterSeries {
                name,
                labels,
                value: c.get(),
                metadata,
            });
        }
        counters.sort_by(|a, b| (a.name.as_str(), &a.labels).cmp(&(b.name.as_str(), &b.labels)));
    }

    let mut gauges = Vec::new();
    #[cfg(feature = "gauge")]
    {
        for (name, labels, g) in registry.gauge_entries() {
            let metadata = registry.metadata(&name);
            gauges.push(GaugeSeries {
                name,
                labels,
                value: g.get(),
                metadata,
            });
        }
        gauges.sort_by(|a, b| (a.name.as_str(), &a.labels).cmp(&(b.name.as_str(), &b.labels)));
    }

    #[cfg(feature = "timer")]
    let mut timers = Vec::new();
    #[cfg(feature = "timer")]
    {
        for (name, labels, t) in registry.timer_entries() {
            let metadata = registry.metadata(&name);
            timers.push(TimerSeries {
                name,
                labels,
                stats: t.stats(),
                metadata,
            });
        }
        timers.sort_by(|a, b| (a.name.as_str(), &a.labels).cmp(&(b.name.as_str(), &b.labels)));
    }

    #[cfg(feature = "meter")]
    let mut rate_meters = Vec::new();
    #[cfg(feature = "meter")]
    {
        for (name, labels, r) in registry.rate_meter_entries() {
            let metadata = registry.metadata(&name);
            rate_meters.push(RateSeries {
                name,
                labels,
                stats: r.stats(),
                metadata,
            });
        }
        rate_meters.sort_by(|a, b| (a.name.as_str(), &a.labels).cmp(&(b.name.as_str(), &b.labels)));
    }

    #[cfg(feature = "histogram")]
    let mut histograms = Vec::new();
    #[cfg(feature = "histogram")]
    {
        for (name, labels, h) in registry.histogram_entries() {
            let metadata = registry.metadata(&name);
            histograms.push(HistogramSeries {
                name,
                labels,
                histogram: h.snapshot(),
                metadata,
            });
        }
        histograms.sort_by(|a, b| (a.name.as_str(), &a.labels).cmp(&(b.name.as_str(), &b.labels)));
    }

    RegistrySnapshot {
        schema_version: SCHEMA_VERSION,
        counters,
        gauges,
        #[cfg(feature = "timer")]
        timers,
        #[cfg(feature = "meter")]
        rate_meters,
        #[cfg(feature = "histogram")]
        histograms,
        cardinality: CardinalitySnapshot {
            current: registry.cardinality_count(),
            cap: registry.cardinality_cap(),
            overflows: registry.cardinality_overflows(),
        },
    }
}

/// Snapshot + serialise to a JSON `String` via `serde_json`.
///
/// Equivalent to `serde_json::to_string(&snapshot(registry)).unwrap()`. The
/// snapshot contains only finite primitives and owned strings, so
/// serialisation cannot fail under normal conditions; the `unwrap` is sound.
#[must_use]
pub fn render(registry: &Registry) -> String {
    serde_json::to_string(&snapshot(registry)).expect("RegistrySnapshot serialises to JSON")
}

/// Same as [`render`] but pretty-printed (newlines + indentation).
#[must_use]
pub fn render_pretty(registry: &Registry) -> String {
    serde_json::to_string_pretty(&snapshot(registry)).expect("RegistrySnapshot serialises to JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_snapshot() {
        let r = Registry::new();
        let snap = snapshot(&r);
        assert_eq!(snap.schema_version, SCHEMA_VERSION);
        assert!(snap.counters.is_empty());
        assert_eq!(snap.cardinality.current, 0);
        assert!(snap.cardinality.cap > 0);
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_snapshot_includes_labels_and_value() {
        let r = Registry::new();
        r.describe_counter("hits", "Total hits", crate::Unit::Custom("hits"));
        r.get_or_create_counter("hits").add(7);
        let labels = LabelSet::from([("status", "200")]);
        r.get_or_create_counter_with("hits", &labels).add(3);
        let snap = snapshot(&r);
        assert_eq!(snap.counters.len(), 2);
        assert!(snap.counters.iter().any(|c| c.value == 7));
        assert!(snap.counters.iter().any(|c| c.value == 3));
        let labelled = snap.counters.iter().find(|c| !c.labels.is_empty()).unwrap();
        assert_eq!(labelled.labels.get("status"), Some("200"));
        assert!(labelled.metadata.is_some());
    }

    #[test]
    fn render_produces_valid_json() {
        let r = Registry::new();
        let body = render(&r);
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["schema_version"], 1);
    }

    #[test]
    fn render_pretty_indents() {
        let r = Registry::new();
        let body = render_pretty(&r);
        assert!(body.contains('\n'));
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn histogram_snapshot_round_trips() {
        let r = Registry::new();
        r.configure_histogram("rtt", [0.01, 0.05]);
        let h = r.get_or_create_histogram("rtt");
        h.observe(0.005);
        h.observe(0.03);
        let body = render(&r);
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["histograms"][0]["name"], "rtt");
        assert_eq!(v["histograms"][0]["histogram"]["count"], 2);
    }
}
