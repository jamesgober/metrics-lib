//! The snapshot document written through jsonx-lib (feature `json`).
//!
//! Every function here writes exactly what the `serde` derives on the
//! snapshot types write through `serde_json`: the same keys in the same
//! order, the same omitted fields, the same tags, the same float text (the
//! document is written in `json_float`'s formats) and the same text for
//! non-finite floats. The parity tests compare the two byte for byte.
//! jsonx-lib's types stay private to this module: [`Doc`] is the only
//! `ToJson` type, and it is not public.

use super::{CardinalitySnapshot, JsonSeries, JsonValue, QuantileValue, RegistrySnapshot};
use crate::{LabelSet, MetricKind, Unit};
use jsonx_lib::{ObjectWriter, ToJson, ValueWriter, Written};
use std::time::Duration;

/// A [`RegistrySnapshot`] as a jsonx-lib document.
pub(super) struct Doc<'a>(pub(super) &'a RegistrySnapshot);

impl ToJson for Doc<'_> {
    fn to_json(&self, out: ValueWriter<'_>) -> Written {
        let snap = self.0;
        out.object(|o| {
            o.member("schema_version", &snap.schema_version);
            o.member_with("series", |w| {
                w.array(|a| {
                    for series in &snap.series {
                        a.item_with(|w| write_series(w, series));
                    }
                })
            });
            o.member_with("cardinality", |w| write_cardinality(w, &snap.cardinality));
        })
    }
}

/// About how long the compact text of `snap` is, on the generous side, so
/// the output is allocated once instead of growing by doubling (about 13
/// reallocations and copies for a 30 KiB snapshot). Cheap: no label is
/// read, only counted. An underestimate costs one growth, an overestimate
/// spare capacity; neither changes the text.
pub(super) fn estimated_len(snap: &RegistrySnapshot) -> usize {
    snap.series.iter().fold(256, |len, series| {
        let fixed = 80 + series.name.len() + 40 * series.labels.len();
        let help = series.help.as_ref().map_or(0, |help| help.len() + 10);
        let value = match &series.value {
            #[cfg(feature = "histogram")]
            JsonValue::Histogram(h) => 112 + 40 * h.buckets.len() + 160 * h.exemplars.len(),
            #[cfg(feature = "histogram")]
            JsonValue::Summary { quantiles, .. } => 64 + 48 * quantiles.len(),
            #[cfg(feature = "timer")]
            JsonValue::Timer(_) => 320,
            #[cfg(feature = "meter")]
            JsonValue::Rate(_) => 320,
            _ => 48,
        };
        len.saturating_add(fixed + help + value)
    })
}

fn write_series(out: ValueWriter<'_>, series: &JsonSeries) -> Written {
    out.object(|o| {
        o.member("name", &series.name);
        o.member("kind", kind_name(series.kind));
        o.member_with("labels", |w| write_labels(w, &series.labels));
        if let Some(help) = &series.help {
            o.member("help", help);
        }
        if let Some(unit) = series.unit {
            o.member_with("unit", |w| write_unit(w, unit));
        }
        o.member_with("value", |w| write_value(w, &series.value));
    })
}

/// The `serde` derive's name for each kind: the variant name.
fn kind_name(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::Counter => "Counter",
        MetricKind::UpDownCounter => "UpDownCounter",
        MetricKind::Gauge => "Gauge",
        MetricKind::Timer => "Timer",
        MetricKind::Rate => "Rate",
        MetricKind::Histogram => "Histogram",
        MetricKind::Summary => "Summary",
    }
}

/// The `serde` derive's form of a unit: the variant name, and
/// `{"Custom": "..."}` for a custom unit.
fn write_unit(out: ValueWriter<'_>, unit: Unit) -> Written {
    let name = match unit {
        Unit::None => "None",
        Unit::Seconds => "Seconds",
        Unit::Milliseconds => "Milliseconds",
        Unit::Microseconds => "Microseconds",
        Unit::Nanoseconds => "Nanoseconds",
        Unit::Bytes => "Bytes",
        Unit::Kilobytes => "Kilobytes",
        Unit::Megabytes => "Megabytes",
        Unit::Gigabytes => "Gigabytes",
        Unit::Percent => "Percent",
        Unit::Ratio => "Ratio",
        Unit::Custom(custom) => return out.object(|o| o.member("Custom", custom)),
    };
    out.str(name)
}

/// Labels as one object, in the set's order.
fn write_labels(out: ValueWriter<'_>, labels: &LabelSet) -> Written {
    out.object(|o| {
        for (key, value) in labels.iter() {
            o.member(key, value);
        }
    })
}

/// A `Duration` as `serde` writes it: `{"secs": u64, "nanos": u32}`.
#[cfg_attr(
    not(any(feature = "timer", feature = "meter", feature = "histogram")),
    allow(dead_code)
)]
fn write_duration(out: ValueWriter<'_>, d: Duration) -> Written {
    out.object(|o| duration_members(o, d))
}

#[cfg_attr(
    not(any(feature = "timer", feature = "meter", feature = "histogram")),
    allow(dead_code)
)]
fn duration_members(o: &mut ObjectWriter<'_>, d: Duration) {
    o.member("secs", &d.as_secs());
    o.member("nanos", &d.subsec_nanos());
}

/// A float that JSON can hold as a number, or `"NaN"`, `"+Inf"`, `"-Inf"`.
fn finite_or_string(out: ValueWriter<'_>, v: f64) -> Written {
    if v.is_finite() {
        out.f64(v)
    } else if v.is_nan() {
        out.str("NaN")
    } else if v > 0.0 {
        out.str("+Inf")
    } else {
        out.str("-Inf")
    }
}

/// The value, internally tagged by `"type"` (the tag comes first).
fn write_value(out: ValueWriter<'_>, value: &JsonValue) -> Written {
    out.object(|o| match value {
        JsonValue::Counter { value } => {
            o.member("type", "counter");
            o.member("value", value);
        }
        JsonValue::UpDownCounter { value } => {
            o.member("type", "up_down_counter");
            o.member("value", value);
        }
        JsonValue::Gauge { value } => {
            o.member("type", "gauge");
            o.member_with("value", |w| finite_or_string(w, *value));
        }
        #[cfg(feature = "timer")]
        JsonValue::Timer(t) => {
            o.member("type", "timer");
            o.member("count", &t.count);
            o.member_with("total", |w| write_duration(w, t.total));
            o.member_with("average", |w| write_duration(w, t.average));
            o.member_with("min", |w| write_duration(w, t.min));
            o.member_with("max", |w| write_duration(w, t.max));
            o.member_with("age", |w| write_duration(w, t.age));
            o.member("rate_per_second", &t.rate_per_second);
        }
        #[cfg(feature = "meter")]
        JsonValue::Rate(r) => {
            o.member("type", "rate");
            o.member("total_events", &r.total_events);
            o.member("rate", &r.rate);
            o.member("per_second", &r.per_second);
            o.member("per_minute", &r.per_minute);
            o.member("per_hour", &r.per_hour);
            o.member("average_rate", &r.average_rate);
            o.member_with("age", |w| write_duration(w, r.age));
            o.member_with("window", |w| write_duration(w, r.window));
        }
        #[cfg(feature = "histogram")]
        JsonValue::Histogram(h) => {
            o.member("type", "histogram");
            o.member_with("buckets", |w| {
                w.array(|a| {
                    for bucket in &h.buckets {
                        a.item_with(|w| {
                            w.object(|o| {
                                // `HistogramBucket`'s own `Serialize`: any
                                // bound that is not finite is the `+Inf`
                                // bucket.
                                if bucket.upper_bound.is_finite() {
                                    o.member("upper_bound", &bucket.upper_bound);
                                } else {
                                    o.member("upper_bound", "+Inf");
                                }
                                o.member("count", &bucket.count);
                            })
                        });
                    }
                })
            });
            o.member("sum", &h.sum);
            o.member("count", &h.count);
            o.member_with("age", |w| write_duration(w, h.age));
            if !h.exemplars.is_empty() {
                o.member_with("exemplars", |w| {
                    w.array(|a| {
                        for exemplar in &h.exemplars {
                            match exemplar {
                                Some(e) => a.item_with(|w| {
                                    w.object(|o| {
                                        o.member_with("labels", |w| write_labels(w, &e.labels));
                                        o.member("value", &e.value);
                                        o.member("timestamp", &e.timestamp);
                                    })
                                }),
                                // `()` writes `null`.
                                None => a.item(&()),
                            }
                        }
                    })
                });
            }
        }
        #[cfg(feature = "histogram")]
        JsonValue::Summary {
            count,
            sum,
            quantiles,
        } => {
            o.member("type", "summary");
            o.member("count", count);
            o.member("sum", sum);
            o.member_with("quantiles", |w| {
                w.array(|a| {
                    for q in quantiles {
                        a.item_with(|w| write_quantile(w, q));
                    }
                })
            });
        }
    })
}

#[cfg_attr(not(feature = "histogram"), allow(dead_code))]
fn write_quantile(out: ValueWriter<'_>, q: &QuantileValue) -> Written {
    out.object(|o| {
        o.member("quantile", &q.quantile);
        o.member_with("value", |w| finite_or_string(w, q.value));
    })
}

fn write_cardinality(out: ValueWriter<'_>, c: &CardinalitySnapshot) -> Written {
    out.object(|o| {
        o.member("current", &c.current);
        o.member("cap", &c.cap);
        o.member("overflows", &c.overflows);
        o.member("invalid_inputs", &c.invalid_inputs);
        o.member("kind_conflicts", &c.kind_conflicts);
    })
}

#[cfg(test)]
mod tests {
    use crate::Registry;
    use crate::exporters::json::snapshot;

    #[cfg(feature = "count")]
    #[test]
    fn test_to_json_reads_back_with_jsonx() {
        let r = Registry::new();
        r.counter_with("hits", &crate::LabelSet::from([("path", "/a\"b\\c\n")]))
            .add(3);
        let text = snapshot(&r).to_json();
        let doc = jsonx_lib::parse(&text).unwrap();
        assert_eq!(doc["schema_version"].as_u64(), Some(2));
        assert_eq!(doc["series"][0]["labels"]["path"], "/a\"b\\c\n");
        assert_eq!(doc["series"][0]["value"]["type"], "counter");
        assert_eq!(doc["series"][0]["value"]["value"].as_u64(), Some(3));
        assert_eq!(doc["series"][0]["kind"], "Counter");
        assert!(doc["series"][0]["help"].is_null());
    }

    #[cfg(feature = "gauge")]
    #[test]
    fn test_to_json_non_finite_gauges_are_strings() {
        let r = Registry::new();
        r.gauge("nan").set(f64::NAN);
        r.gauge("ninf").set(f64::NEG_INFINITY);
        r.gauge("pinf").set(f64::INFINITY);
        let text = crate::exporters::json::to_json(&r);
        assert!(text.contains(r#""value":"NaN""#), "{text}");
        assert!(text.contains(r#""value":"-Inf""#), "{text}");
        assert!(text.contains(r#""value":"+Inf""#), "{text}");
        assert!(!text.contains("null"), "{text}");
        let pretty = crate::exporters::json::to_json_pretty(&r);
        assert!(pretty.contains("\n  \"series\": [\n"), "{pretty}");
    }

    /// The estimate covers ordinary snapshots, so they are allocated once:
    /// short labels on every kind, compact and indented.
    #[cfg(all(
        feature = "count",
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram"
    ))]
    #[test]
    fn test_estimated_len_covers_ordinary_snapshots() {
        let r = Registry::new();
        r.describe_counter("http_requests", "Requests served", crate::Unit::None)
            .unwrap();
        for status in ["200", "404", "500"] {
            let labels = crate::LabelSet::from([
                ("method", "GET"),
                ("route", "/api/users/:id"),
                ("status", status),
            ]);
            r.counter_with("http_requests", &labels).add(17);
        }
        r.gauge("pool_in_use").set(0.1 + 0.2);
        r.timer("db_query").record_ns(37_000);
        r.rate_meter("events").tick_n(12_345);
        let h = r.histogram_with("latency", &crate::LabelSet::from([("route", "/")]));
        h.observe(0.37);
        let s = r.summary("payload_size");
        s.observe(3.5);
        let snap = snapshot(&r);
        let estimate = super::estimated_len(&snap);
        assert!(estimate >= snap.to_json().len(), "{estimate}");
        assert!(estimate * 2 >= snap.to_json_pretty().len(), "{estimate}");
        assert!(estimate <= 3 * snap.to_json().len(), "{estimate}");
    }

    #[test]
    fn test_to_json_of_an_empty_registry() {
        let text = snapshot(&Registry::new()).to_json();
        assert!(
            text.starts_with(
                r#"{"schema_version":2,"series":[],"cardinality":{"current":0,"cap":"#
            )
        );
        assert!(text.ends_with(r#","overflows":0,"invalid_inputs":0,"kind_conflicts":0}}"#));
    }
}
