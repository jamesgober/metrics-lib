//! JSON snapshot exporter (features `serde` and `json`).
//!
//! [`snapshot`] captures every series as a [`RegistrySnapshot`]. Useful for
//! `/debug/metrics` endpoints, periodic dumps and test fixtures. Two ways
//! write it as JSON text, and they write the same text:
//!
//! - feature `serde`: the snapshot implements `serde::Serialize`, so it can
//!   go through any serde format, and `render` and `render_pretty` produce
//!   JSON with `serde_json`;
//! - feature `json`, which does not need serde: `to_json` and
//!   `to_json_pretty` (and [`RegistrySnapshot`]'s methods of the same names)
//!   write the document through jsonx-lib's typed writer, with no
//!   intermediate tree, and cannot fail.
//!
//! For the same snapshot both produce the same bytes (key order, float and
//! integer text, escaping, omitted fields, indentation), with one
//! exception: U+2028 and U+2029 in a string (possible in label values and
//! help text) are escaped as `\u2028` and `\u2029` by `to_json` and written
//! raw by `render`. Both forms are valid JSON and decode to the same string;
//! the escaped form is also safe to embed in JavaScript before ES2019.
//!
//! The float text is that of `serde_json` 1.0.147 and later (checked
//! against 1.0.154). Versions up to 1.0.146 wrote a positive exponent
//! without its sign (`1e16` where later versions and `to_json` write
//! `1e+16`), so with those `render` differs there too.
//!
//! JSON has no NaN or infinity, so non-finite gauge values and quantile
//! estimates are written as the strings `"NaN"`, `"+Inf"` and `"-Inf"`, and
//! the last histogram bucket bound as `"+Inf"`. Other non-finite floats
//! (a sum that overflowed, for example) are written as `null`.
//!
//! # Example
//!
//! ```
//! # #[cfg(all(feature = "count", feature = "serde"))]
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
//!
//! Without serde (feature `json`):
//!
//! ```
//! # #[cfg(all(feature = "count", feature = "json"))]
//! # {
//! use metrics_lib::MetricsCore;
//! use metrics_lib::exporters::json;
//!
//! let core = MetricsCore::new();
//! core.counter("hits").add(2);
//! let body: String = json::to_json(core.registry());
//! assert!(body.contains(r#""type":"counter","value":2"#));
//! # }
//! ```

use crate::registry::Handle;
use crate::{LabelSet, MetricKind, Registry, Unit};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

#[cfg(feature = "json")]
mod write;

/// Version of the JSON shape. Bumped on incompatible changes (1.x wrote
/// version 1, with one list per metric type).
pub const SCHEMA_VERSION: u32 = 2;

/// Every series in a registry.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[non_exhaustive]
pub struct JsonSeries {
    /// Registered name.
    pub name: String,
    /// Family kind.
    pub kind: MetricKind,
    /// Labels, serialised as a JSON object.
    pub labels: LabelSet,
    /// Help text, when the family was described.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub help: Option<String>,
    /// Unit, when the family was described with one.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unit: Option<Unit>,
    /// The value, tagged by `"type"`.
    pub value: JsonValue,
}

/// Value of one series, tagged by metric type.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
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
        #[cfg_attr(feature = "serde", serde(serialize_with = "finite_or_string"))]
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
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[non_exhaustive]
pub struct QuantileValue {
    /// Quantile in `[0, 1]`.
    pub quantile: f64,
    /// Estimate (`"NaN"` when the summary is empty).
    #[cfg_attr(feature = "serde", serde(serialize_with = "finite_or_string"))]
    pub value: f64,
}

/// Registry limits and counters at snapshot time.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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

#[cfg(feature = "serde")]
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
                labels: std::sync::Arc::unwrap_or_clone(s.labels),
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

/// Snapshot `registry` and serialise it as compact JSON (feature `serde`).
///
/// `to_json` writes the same text without serde (feature `json`).
///
/// # Errors
///
/// Returns the `serde_json` error if serialisation fails. The snapshot only
/// holds strings, integers and finite floats (non-finite values are written
/// as strings), so this does not happen in practice.
#[cfg(feature = "serde")]
pub fn render(registry: &Registry) -> serde_json::Result<String> {
    serde_json::to_string(&snapshot(registry))
}

/// Like [`render`] but indented (feature `serde`).
///
/// # Errors
///
/// See [`render`].
#[cfg(feature = "serde")]
pub fn render_pretty(registry: &Registry) -> serde_json::Result<String> {
    serde_json::to_string_pretty(&snapshot(registry))
}

#[cfg(feature = "json")]
impl RegistrySnapshot {
    /// This snapshot as compact JSON, written by jsonx-lib (feature `json`).
    ///
    /// The text is the same as `serde_json` writes for this snapshot (see the
    /// [module documentation](self) for the one difference, U+2028 and
    /// U+2029 in strings). Writing cannot fail.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "gauge")]
    /// # {
    /// use metrics_lib::Registry;
    /// use metrics_lib::exporters::json;
    ///
    /// let registry = Registry::new();
    /// registry.gauge("temp").set(f64::NAN);
    /// let snap = json::snapshot(&registry);
    /// assert!(snap.to_json().contains(r#""type":"gauge","value":"NaN""#));
    /// # }
    /// ```
    #[must_use]
    pub fn to_json(&self) -> String {
        jsonx_lib::to_string(&write::Doc(self))
    }

    /// Like [`to_json`](Self::to_json) but indented: two spaces per level,
    /// as `serde_json::to_string_pretty` writes it.
    #[must_use]
    pub fn to_json_pretty(&self) -> String {
        jsonx_lib::to_string_pretty(&write::Doc(self))
    }
}

/// Snapshot `registry` and write it as compact JSON with jsonx-lib (feature
/// `json`, which does not need serde).
///
/// The same text as `render`, and infallible. See
/// [`RegistrySnapshot::to_json`].
#[cfg(feature = "json")]
#[must_use]
pub fn to_json(registry: &Registry) -> String {
    snapshot(registry).to_json()
}

/// Like [`to_json`] but indented (feature `json`); the same text as
/// `render_pretty`.
#[cfg(feature = "json")]
#[must_use]
pub fn to_json_pretty(registry: &Registry) -> String {
    snapshot(registry).to_json_pretty()
}

#[cfg(test)]
#[cfg(all(feature = "count", feature = "gauge", feature = "serde"))]
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
