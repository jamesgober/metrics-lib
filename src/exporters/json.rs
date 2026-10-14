//! JSON snapshot exporter (feature `serde`).
//!
//! [`snapshot`] captures every series as a [`RegistrySnapshot`], which
//! implements `serde::Serialize`, so it can go through any serde format;
//! [`render`] and [`render_pretty`] produce JSON text directly. Useful for
//! `/debug/metrics` endpoints, periodic dumps and test fixtures.
//!
//! JSON has no NaN or infinity, so non-finite gauge values and quantile
//! estimates are written as the strings `"NaN"`, `"+Inf"` and `"-Inf"`, and
//! the last histogram bucket bound as `"+Inf"`.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{LabelSet, MetricsCore};
//! use metrics_lib::exporters::json;
//!
//! let core = MetricsCore::new();
//! core.counter("hits").inc();
//! core.counter_with("hits", &LabelSet::from([("status", "200")])).inc();
//!
//! let snap = json::snapshot(core.registry());
//! assert_eq!(snap.series.len(), 2);
//! let body = json::render(core.registry()).unwrap();
//! assert!(body.contains(r#""type":"counter""#));
//! # }
//! ```

use crate::registry::Handle;
use crate::{LabelSet, MetricKind, Registry, Unit};
use serde::{Serialize, Serializer};

/// Version of the JSON shape. Bumped on incompatible changes (1.x wrote
/// version 1, with one list per metric type).
pub const SCHEMA_VERSION: u32 = 2;

/// Every series in a registry.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct RegistrySnapshot {
    /// [`SCHEMA_VERSION`] of this snapshot.
    pub schema_version: u32,
    /// Series sorted by name, then labels.
    pub series: Vec<JsonSeries>,
    /// Registry limits and counters.
    pub cardinality: CardinalitySnapshot,
}

/// One series.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct JsonSeries {
    /// Registered name.
    pub name: String,
    /// Family kind.
    pub kind: MetricKind,
    /// Labels, serialised as a JSON object.
    pub labels: LabelSet,
    /// Help text, when the family was described.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    /// Unit, when the family was described with one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// The value, tagged by `"type"`.
    pub value: JsonValue,
}

/// Value of one series, tagged by metric type.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum JsonValue {
    /// Counter or sharded counter.
    Counter {
        /// Current value.
        value: u64,
    },
    /// Up/down counter.
    UpDownCounter {
        /// Current value.
        value: i64,
    },
    /// Gauge.
    Gauge {
        /// Current value (`"NaN"`/`"+Inf"`/`"-Inf"` when non-finite).
        #[serde(serialize_with = "finite_or_string")]
        value: f64,
    },
    /// Timer.
    #[cfg(feature = "timer")]
    Timer(crate::TimerStats),
    /// Rate meter.
    #[cfg(feature = "meter")]
    Rate(crate::RateStats),
    /// Histogram.
    #[cfg(feature = "histogram")]
    Histogram(crate::HistogramSnapshot),
    /// Summary.
    #[cfg(feature = "histogram")]
    Summary {
        /// Number of observations.
        count: u64,
        /// Sum of observations.
        sum: f64,
        /// `[quantile, estimate]` pairs (`"NaN"` estimates when empty).
        quantiles: Vec<QuantileValue>,
    },
}

/// One quantile estimate of a summary.
#[derive(Debug, Clone, Copy, Serialize)]
#[non_exhaustive]
pub struct QuantileValue {
    /// Quantile in `[0, 1]`.
    pub quantile: f64,
    /// Estimate (`"NaN"` when the summary is empty).
    #[serde(serialize_with = "finite_or_string")]
    pub value: f64,
}

/// Registry limits and counters at snapshot time.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct CardinalitySnapshot {
    /// Cap slots in use.
    pub current: usize,
    /// Configured cap.
    pub cap: usize,
    /// Lifetime overflow events.
    pub overflows: u64,
    /// Lifetime repaired names/labels.
    pub invalid_inputs: u64,
    /// Lifetime kind conflicts.
    pub kind_conflicts: u64,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // signature required by serde
fn finite_or_string<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.is_finite() {
        s.serialize_f64(*v)
    } else if v.is_nan() {
        s.serialize_str("NaN")
    } else if *v > 0.0 {
        s.serialize_str("+Inf")
    } else {
        s.serialize_str("-Inf")
    }
}

fn value_of(handle: &Handle) -> JsonValue {
    match handle {
        #[cfg(feature = "count")]
        Handle::Counter(c) => JsonValue::Counter { value: c.get() },
        #[cfg(feature = "count")]
        Handle::ShardedCounter(c) => JsonValue::Counter { value: c.get() },
        #[cfg(feature = "count")]
        Handle::UpDownCounter(c) => JsonValue::UpDownCounter { value: c.get() },
        #[cfg(feature = "gauge")]
        Handle::Gauge(g) => JsonValue::Gauge { value: g.get() },
        #[cfg(feature = "timer")]
        Handle::Timer(t) => JsonValue::Timer(t.stats()),
        #[cfg(feature = "meter")]
        Handle::Rate(r) => JsonValue::Rate(r.stats()),
        #[cfg(feature = "histogram")]
        Handle::Histogram(h) => JsonValue::Histogram(h.snapshot()),
        #[cfg(feature = "histogram")]
        Handle::Summary(s) => {
            let snap = s.snapshot();
            JsonValue::Summary {
                count: snap.count,
                sum: snap.sum,
                quantiles: snap
                    .quantiles
                    .iter()
                    .map(|(q, v)| QuantileValue {
                        quantile: *q,
                        value: *v,
                    })
                    .collect(),
            }
        }
        #[cfg(not(any(
            feature = "count",
            feature = "gauge",
            feature = "timer",
            feature = "meter",
            feature = "histogram"
        )))]
        _ => match *handle {},
    }
}

/// Capture every series of `registry`.
#[must_use]
pub fn snapshot(registry: &Registry) -> RegistrySnapshot {
    let mut series = Vec::new();
    for family in registry.collect() {
        let help = family.help.as_deref().map(str::to_owned);
        let unit = (family.unit != Unit::None).then_some(family.unit);
        for s in family.series {
            series.push(JsonSeries {
                name: family.name.clone(),
                kind: family.kind,
                labels: s.labels,
                help: help.clone(),
                unit,
                value: value_of(&s.handle),
            });
        }
    }
    RegistrySnapshot {
        schema_version: SCHEMA_VERSION,
        series,
        cardinality: CardinalitySnapshot {
            current: registry.cardinality_count(),
            cap: registry.cardinality_cap(),
            overflows: registry.cardinality_overflows(),
            invalid_inputs: registry.invalid_inputs(),
            kind_conflicts: registry.kind_conflicts(),
        },
    }
}

/// Snapshot `registry` and serialise it as compact JSON.
///
/// # Errors
///
/// Returns the `serde_json` error if serialisation fails. The snapshot only
/// holds strings, integers and finite floats (non-finite values are written
/// as strings), so this does not happen in practice.
pub fn render(registry: &Registry) -> serde_json::Result<String> {
    serde_json::to_string(&snapshot(registry))
}

/// Like [`render`] but indented.
///
/// # Errors
///
/// See [`render`].
pub fn render_pretty(registry: &Registry) -> serde_json::Result<String> {
    serde_json::to_string_pretty(&snapshot(registry))
}

#[cfg(test)]
#[cfg(all(feature = "count", feature = "gauge"))]
mod tests {
    use super::*;

    #[test]
    fn test_json_snapshot_shape_and_order() {
        let r = Registry::new();
        r.describe_counter("b_hits", "Hits", Unit::None).unwrap();
        r.counter("b_hits").add(3);
        r.counter_with("a_hits", &LabelSet::from([("k", "v")]))
            .inc();
        r.up_down_counter("c_level").add(-4);
        r.gauge("d_temp").set(21.5);
        let snap = snapshot(&r);
        assert_eq!(snap.schema_version, SCHEMA_VERSION);
        let names: Vec<_> = snap.series.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["a_hits", "b_hits", "c_level", "d_temp"]);
        let json = render(&r).unwrap();
        assert!(json.contains(r#""labels":{"k":"v"}"#), "{json}");
        assert!(json.contains(r#""help":"Hits""#));
        assert!(json.contains(r#""type":"up_down_counter","value":-4"#));
        assert!(json.contains(r#""type":"gauge","value":21.5"#));
        assert!(render_pretty(&r).unwrap().contains('\n'));
        assert_eq!(snap.cardinality.current, 4);
    }

    #[test]
    fn test_json_non_finite_gauges_are_strings_regression_ml024() {
        let r = Registry::new();
        r.gauge("nan").set(f64::NAN);
        r.gauge("pinf").set(f64::INFINITY);
        r.gauge("ninf").set(f64::NEG_INFINITY);
        let json = render(&r).unwrap();
        assert!(json.contains(r#""value":"NaN""#));
        assert!(json.contains(r#""value":"+Inf""#));
        assert!(json.contains(r#""value":"-Inf""#));
        assert!(!json.contains("null"));
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["series"].as_array().unwrap().len(), 3);
    }

    #[cfg(all(feature = "timer", feature = "meter", feature = "histogram"))]
    #[test]
    fn test_json_covers_every_type() {
        let r = Registry::new();
        r.timer("t").record_ns(1_000);
        r.rate_meter("r").tick();
        r.histogram("h").observe(0.3);
        r.summary("s").observe(0.3);
        let _ = r.summary("s_empty");
        r.sharded_counter("sc").inc();
        let json = render(&r).unwrap();
        for needle in [
            r#""type":"timer""#,
            r#""type":"rate""#,
            r#""type":"histogram""#,
            r#""type":"summary""#,
            r#""upper_bound":"+Inf""#,
            r#""value":"NaN""#,
        ] {
            assert!(json.contains(needle), "{needle} missing from {json}");
        }
    }
}
