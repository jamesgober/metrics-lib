//! The OTLP/HTTP JSON request written through jsonx-lib (feature
//! `otlp-json`).
//!
//! Writes straight from the internal model the protobuf encoder also uses,
//! and writes exactly what the serde request model (`ExportMetricsServiceRequest`
//! and friends, feature `otlp`) writes through `serde_json`: camelCase keys
//! in the same order, 64-bit integers as strings, enums as integers, empty
//! `description`/`unit` and a missing histogram `sum` left out, floats in
//! `serde_json`'s text (the request is written in `json_float::COMPACT`).
//! The parity test at the end compares the two byte for byte.

use super::{DataModel, HistogramPoint, MetricModel, Number, NumberPoint, SummaryPoint};
use crate::LabelSet;
use crate::exporters::json_float;
use jsonx_lib::{ObjectWriter, ToJson, ValueWriter, Written};

/// The whole `ExportMetricsServiceRequest`, compact.
pub(super) fn write_request(
    model: &[MetricModel],
    service_name: &str,
    resource: &LabelSet,
) -> String {
    jsonx_lib::to_string_with(
        &Request {
            model,
            service_name,
            resource,
        },
        json_float::COMPACT,
    )
}

struct Request<'a> {
    model: &'a [MetricModel],
    service_name: &'a str,
    resource: &'a LabelSet,
}

impl ToJson for Request<'_> {
    fn to_json(&self, out: ValueWriter<'_>) -> Written {
        out.object(|o| {
            o.member_with("resourceMetrics", |w| {
                w.array(|a| a.item_with(|w| self.resource_metrics(w)))
            });
        })
    }
}

impl Request<'_> {
    fn resource_metrics(&self, out: ValueWriter<'_>) -> Written {
        out.object(|o| {
            o.member_with("resource", |w| {
                w.object(|o| {
                    o.member_with("attributes", |w| {
                        w.array(|a| {
                            // `service.name` first; a resource attribute of
                            // the same key never replaces it.
                            a.item_with(|w| key_value(w, "service.name", self.service_name));
                            for (key, value) in self.resource.iter() {
                                if key != "service.name" {
                                    a.item_with(|w| key_value(w, key, value));
                                }
                            }
                        })
                    });
                })
            });
            o.member_with("scopeMetrics", |w| {
                w.array(|a| {
                    a.item_with(|w| {
                        w.object(|o| {
                            o.member_with("scope", |w| {
                                w.object(|o| {
                                    o.member("name", "metrics-lib");
                                    o.member("version", env!("CARGO_PKG_VERSION"));
                                })
                            });
                            o.member_with("metrics", |w| {
                                w.array(|a| {
                                    for metric in self.model {
                                        a.item_with(|w| write_metric(w, metric));
                                    }
                                })
                            });
                        })
                    });
                })
            });
        })
    }
}

/// `{"key": key, "value": {"stringValue": value}}`.
fn key_value(out: ValueWriter<'_>, key: &str, value: &str) -> Written {
    out.object(|o| {
        o.member("key", key);
        o.member_with("value", |w| w.object(|o| o.member("stringValue", value)));
    })
}

fn attributes(o: &mut ObjectWriter<'_>, labels: &LabelSet) {
    o.member_with("attributes", |w| {
        w.array(|a| {
            for (key, value) in labels.iter() {
                a.item_with(|w| key_value(w, key, value));
            }
        })
    });
}

/// A `u64` as the decimal string OTLP/JSON uses for 64-bit integers.
fn u64_string(o: &mut ObjectWriter<'_>, key: &str, value: u64) {
    let mut buf = [0u8; 20];
    o.member(key, decimal(&mut buf, value));
}

/// `value` in decimal, written into `buf` (20 bytes hold `u64::MAX`).
fn decimal(buf: &mut [u8; 20], mut value: u64) -> &str {
    let mut at = buf.len();
    loop {
        at -= 1;
        // `value % 10` is below 10, so the cast cannot truncate.
        #[allow(clippy::cast_possible_truncation)]
        let digit = (value % 10) as u8;
        if let Some(slot) = buf.get_mut(at) {
            *slot = b'0' + digit;
        }
        value /= 10;
        if value == 0 {
            break;
        }
    }
    buf.get(at..)
        .and_then(|digits| std::str::from_utf8(digits).ok())
        .unwrap_or("0")
}

/// The start and point times every data point carries.
fn times(o: &mut ObjectWriter<'_>, start: u64, time: u64) {
    u64_string(o, "startTimeUnixNano", start);
    u64_string(o, "timeUnixNano", time);
}

fn write_metric(out: ValueWriter<'_>, metric: &MetricModel) -> Written {
    out.object(|o| {
        o.member("name", &metric.name);
        if !metric.description.is_empty() {
            o.member("description", &metric.description);
        }
        if !metric.unit.is_empty() {
            o.member("unit", &metric.unit);
        }
        match &metric.data {
            DataModel::Gauge(points) => {
                o.member_with("gauge", |w| w.object(|o| number_points(o, points)));
            }
            DataModel::Sum {
                points,
                monotonic,
                temporality,
            } => o.member_with("sum", |w| {
                w.object(|o| {
                    number_points(o, points);
                    o.member("aggregationTemporality", &temporality.wire());
                    o.member("isMonotonic", monotonic);
                })
            }),
            DataModel::Histogram {
                points,
                temporality,
            } => o.member_with("histogram", |w| {
                w.object(|o| {
                    o.member_with("dataPoints", |w| {
                        w.array(|a| {
                            for p in points {
                                a.item_with(|w| histogram_point(w, p));
                            }
                        })
                    });
                    o.member("aggregationTemporality", &temporality.wire());
                })
            }),
            DataModel::Summary(points) => o.member_with("summary", |w| {
                w.object(|o| {
                    o.member_with("dataPoints", |w| {
                        w.array(|a| {
                            for p in points {
                                a.item_with(|w| summary_point(w, p));
                            }
                        })
                    });
                })
            }),
        }
    })
}

fn number_points(o: &mut ObjectWriter<'_>, points: &[NumberPoint]) {
    o.member_with("dataPoints", |w| {
        w.array(|a| {
            for p in points {
                a.item_with(|w| {
                    w.object(|o| {
                        attributes(o, &p.attributes);
                        times(o, p.start, p.time);
                        match p.value {
                            Number::Int(v) => {
                                // An `i64` as a string: at most 20 bytes.
                                let text = v.to_string();
                                o.member("asInt", &text);
                            }
                            Number::Double(v) => o.member("asDouble", &v),
                        }
                    })
                });
            }
        })
    });
}

fn histogram_point(out: ValueWriter<'_>, p: &HistogramPoint) -> Written {
    out.object(|o| {
        attributes(o, &p.attributes);
        times(o, p.start, p.time);
        u64_string(o, "count", p.count);
        if let Some(sum) = p.sum {
            o.member("sum", &sum);
        }
        o.member_with("bucketCounts", |w| {
            w.array(|a| {
                let mut buf = [0u8; 20];
                for count in &p.bucket_counts {
                    a.item(decimal(&mut buf, *count));
                }
            })
        });
        o.member_with("explicitBounds", |w| {
            w.array(|a| {
                for bound in &p.explicit_bounds {
                    a.item(bound);
                }
            })
        });
    })
}

fn summary_point(out: ValueWriter<'_>, p: &SummaryPoint) -> Written {
    out.object(|o| {
        attributes(o, &p.attributes);
        times(o, p.start, p.time);
        u64_string(o, "count", p.count);
        o.member("sum", &p.sum);
        o.member_with("quantileValues", |w| {
            w.array(|a| {
                for (quantile, value) in &p.quantiles {
                    a.item_with(|w| {
                        w.object(|o| {
                            o.member("quantile", quantile);
                            o.member("value", value);
                        })
                    });
                }
            })
        });
    })
}

#[cfg(test)]
mod tests {
    use super::decimal;

    #[test]
    fn test_decimal_matches_to_string() {
        let mut buf = [0u8; 20];
        for v in [
            0,
            1,
            9,
            10,
            99,
            100,
            12_345,
            u64::from(u32::MAX),
            u64::MAX - 1,
            u64::MAX,
        ] {
            assert_eq!(decimal(&mut buf, v), v.to_string());
        }
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_to_json_reads_back_with_jsonx() {
        let r = crate::Registry::new();
        r.counter_with("orders", &crate::LabelSet::from([("region", "eu\"west")]))
            .add(7);
        let exporter = super::super::OtlpExporter::new("shop")
            .with_resource_attribute("service.name", "ignored")
            .with_resource_attribute("host", "a1");
        let text = exporter.to_json(&r);
        let doc = jsonx_lib::parse(&text).unwrap();
        let rm = &doc["resourceMetrics"][0];
        assert_eq!(rm["resource"]["attributes"][0]["key"], "service.name");
        assert_eq!(
            rm["resource"]["attributes"][0]["value"]["stringValue"],
            "shop"
        );
        assert_eq!(rm["resource"]["attributes"][1]["key"], "host");
        assert!(rm["resource"]["attributes"][2].is_null());
        let metric = &rm["scopeMetrics"][0]["metrics"][0];
        assert_eq!(metric["name"], "orders");
        assert!(metric["description"].is_null());
        let point = &metric["sum"]["dataPoints"][0];
        assert_eq!(point["asInt"], "7");
        assert_eq!(point["attributes"][0]["value"]["stringValue"], "eu\"west");
        assert_eq!(metric["sum"]["aggregationTemporality"].as_u64(), Some(2));
        assert_eq!(metric["sum"]["isMonotonic"].as_bool(), Some(true));
    }

    #[cfg(feature = "otlp")]
    /// Strings that need every kind of escape JSON has, and some that need
    /// none (`/`, `<`, DEL, non-ASCII, a character outside the BMP).
    const AWKWARD: [&str; 6] = [
        "quote \" and backslash \\",
        "line\nfeed\rreturn\ttab",
        "nul \u{0} unit sep \u{1f} bell \u{7}",
        "del \u{7f} slash / html <b>&amp;</b>",
        "caf\u{e9} \u{4e2d}\u{6587} \u{1F600}",
        "",
    ];

    /// The serde model's text and this module's for the same model.
    #[cfg(feature = "otlp")]
    fn both(
        model: &[super::MetricModel],
        service_name: &str,
        resource: &crate::LabelSet,
    ) -> (String, String) {
        let serde_text =
            serde_json::to_string(&super::super::to_json_model(model, service_name, resource))
                .unwrap();
        (
            serde_text,
            super::write_request(model, service_name, resource),
        )
    }

    /// Byte-identical to `render_json` for registries with every kind, over
    /// both temporalities and several exports (resets and deltas included).
    #[cfg(all(
        feature = "otlp",
        feature = "count",
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram"
    ))]
    #[test]
    fn test_parity_with_serde_model_for_a_registry() {
        use crate::{LabelSet, Registry, Unit};
        use std::time::Duration;

        use super::super::{OtlpExporter, Temporality};

        let r = Registry::new();
        r.describe_counter("c", AWKWARD[0], Unit::Bytes).unwrap();
        r.counter("c").add(u64::MAX);
        for (i, value) in AWKWARD.iter().enumerate() {
            r.counter_with("c_labels", &LabelSet::from([("awkward", *value)]))
                .add(i as u64);
        }
        r.sharded_counter("sc").add(3);
        r.up_down_counter("ud").add(i64::MIN);
        r.describe_gauge("g", AWKWARD[1], Unit::Custom("req/\"s\""))
            .unwrap();
        for (i, v) in [
            0.0,
            -0.0,
            -1.5,
            1e16,
            1e-7,
            f64::MAX,
            f64::NAN,
            f64::INFINITY,
        ]
        .iter()
        .enumerate()
        {
            r.gauge_with("g", &LabelSet::from([("i", i.to_string())]))
                .set(*v);
        }
        let t = r.timer("t");
        t.record_ns(1);
        t.record(Duration::from_secs(2));
        let _empty = r.timer("t_empty");
        r.rate_meter("rm").tick_n(10);
        r.configure_histogram("h", [0.5, 1.0, 1e300]).unwrap();
        let h = r.histogram("h");
        h.observe(0.25);
        h.observe(f64::MAX);
        r.histogram("h_neg").observe(-1.0);
        let _empty = r.histogram("h_empty");
        let s = r.summary_with("s", &LabelSet::from([("awkward", AWKWARD[4])]));
        s.observe(0.5);
        s.observe(1e6);
        let _empty = r.summary("s_empty");

        for temporality in [Temporality::Cumulative, Temporality::Delta] {
            let exporter = OtlpExporter::new(AWKWARD[2])
                .with_resource_attribute("service.name", "replaced?")
                .with_resource_attribute("deployment.environment", AWKWARD[3])
                .with_temporality(temporality);
            for round in 0..3 {
                let model = exporter.model(&r);
                let (serde_text, jsonx_text) =
                    both(&model, &exporter.service_name, &exporter.resource);
                assert_eq!(jsonx_text, serde_text, "{temporality:?}, round {round}");
                // Between exports: growth, then a reset.
                r.counter("c_labels").inc();
                h.observe(0.75);
                if round == 1 {
                    r.counter("c").reset();
                    t.reset();
                }
            }
        }
    }

    /// Byte-identical for hand-built models: every data type, empty and
    /// non-empty description and unit, missing and non-finite sums, extreme
    /// integers and floats.
    #[cfg(feature = "otlp")]
    #[test]
    fn test_parity_with_serde_model_for_edge_values() {
        use super::super::{DataModel, HistogramPoint, MetricModel, Number, NumberPoint};
        use super::super::{SummaryPoint, Temporality};
        use crate::LabelSet;

        let labels = |i: usize| LabelSet::from([("k", AWKWARD[i % AWKWARD.len()])]);
        let number = |i: usize, value| NumberPoint {
            attributes: labels(i),
            start: 0,
            time: u64::MAX,
            value,
        };
        let model = vec![
            MetricModel {
                name: "gauge".to_owned(),
                description: String::new(),
                unit: String::new(),
                data: DataModel::Gauge(vec![
                    number(0, Number::Double(0.1 + 0.2)),
                    number(1, Number::Double(f64::NAN)),
                    number(2, Number::Double(-0.0)),
                    number(3, Number::Double(5e-324)),
                    number(4, Number::Int(i64::MIN)),
                ]),
            },
            MetricModel {
                name: "sum".to_owned(),
                description: AWKWARD[1].to_owned(),
                unit: "{req/\"s\"}".to_owned(),
                data: DataModel::Sum {
                    points: vec![number(0, Number::Int(i64::MAX)), number(1, Number::Int(0))],
                    monotonic: false,
                    temporality: Temporality::Delta,
                },
            },
            MetricModel {
                name: "empty_sum".to_owned(),
                description: String::new(),
                unit: "By".to_owned(),
                data: DataModel::Sum {
                    points: Vec::new(),
                    monotonic: true,
                    temporality: Temporality::Cumulative,
                },
            },
            MetricModel {
                name: "histogram".to_owned(),
                description: AWKWARD[0].to_owned(),
                unit: String::new(),
                data: DataModel::Histogram {
                    points: vec![
                        HistogramPoint {
                            attributes: labels(2),
                            start: 1,
                            time: 2,
                            count: u64::MAX,
                            sum: None,
                            bucket_counts: vec![0, u64::MAX, 7],
                            explicit_bounds: vec![-1.0, 1e300],
                        },
                        HistogramPoint {
                            attributes: LabelSet::new(),
                            start: 3,
                            time: 4,
                            count: 0,
                            sum: Some(f64::INFINITY),
                            bucket_counts: vec![0],
                            explicit_bounds: Vec::new(),
                        },
                    ],
                    temporality: Temporality::Cumulative,
                },
            },
            MetricModel {
                name: "summary".to_owned(),
                description: String::new(),
                unit: "s".to_owned(),
                data: DataModel::Summary(vec![
                    SummaryPoint {
                        attributes: labels(4),
                        start: 5,
                        time: 6,
                        count: 3,
                        sum: f64::NAN,
                        quantiles: vec![(0.0, f64::NAN), (0.5, 1e-6), (1.0, f64::NEG_INFINITY)],
                    },
                    SummaryPoint {
                        attributes: LabelSet::new(),
                        start: 0,
                        time: 0,
                        count: 0,
                        sum: 0.0,
                        quantiles: Vec::new(),
                    },
                ]),
            },
        ];
        let resource = LabelSet::from([("a", AWKWARD[0]), ("service.name", "x"), ("z", "")]);
        let (serde_text, jsonx_text) = both(&model, AWKWARD[1], &resource);
        assert_eq!(jsonx_text, serde_text);
        let (serde_text, jsonx_text) = both(&[], "", &LabelSet::new());
        assert_eq!(jsonx_text, serde_text);
    }
}
