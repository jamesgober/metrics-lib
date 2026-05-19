//! OTLP/HTTP+JSON exporter (behind the `otlp` feature, which pulls in
//! `serde` + `serde_json`).
//!
//! Produces the JSON payload defined by the OpenTelemetry Protocol
//! ([OTLP/HTTP](https://opentelemetry.io/docs/specs/otlp/#otlphttp)) for the
//! `metrics` signal. The exporter is *transport-agnostic*: it returns the
//! payload as a `String` (or [`ExportMetricsServiceRequest`] for re-use)
//! that callers POST to their collector endpoint (`/v1/metrics`) with
//! `Content-Type: application/json`.
//!
//! Mapping summary:
//!
//! - **Counter** → OTLP `Sum` with `isMonotonic = true`,
//!   `aggregationTemporality = CUMULATIVE`.
//! - **Gauge** → OTLP `Gauge`.
//! - **Timer** → OTLP `Histogram` with `_seconds`-suffixed name, exposing
//!   `count`, `sum`, `min`, `max`, and a single `+Inf` bucket.
//! - **RateMeter** → OTLP `Gauge` with `_per_second` suffix.
//! - **Histogram** → OTLP `Histogram` with non-cumulative `bucketCounts`
//!   plus `explicitBounds`.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{init, metrics, LabelSet};
//! use metrics_lib::exporters::otlp;
//!
//! init();
//! metrics().counter_with("requests", &LabelSet::from([("status", "200")])).inc();
//!
//! let payload: String = otlp::render(metrics().registry(), "my-service");
//! assert!(payload.contains("\"resourceMetrics\""));
//! # }
//! ```

use crate::{LabelSet, MetricMetadata, Registry, Unit};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// OTLP `AggregationTemporality::CUMULATIVE`.
const AGGREGATION_TEMPORALITY_CUMULATIVE: u32 = 2;

/// Render an OTLP/HTTP+JSON metrics payload for the supplied registry.
///
/// `service_name` is attached as the `service.name` resource attribute.
#[must_use]
pub fn render(registry: &Registry, service_name: impl Into<String>) -> String {
    serde_json::to_string(&build(registry, service_name))
        .expect("ExportMetricsServiceRequest serialises to JSON")
}

/// Same as [`render`] but pretty-printed.
#[must_use]
pub fn render_pretty(registry: &Registry, service_name: impl Into<String>) -> String {
    serde_json::to_string_pretty(&build(registry, service_name))
        .expect("ExportMetricsServiceRequest serialises to JSON")
}

/// Build the OTLP payload as a serialisable
/// [`ExportMetricsServiceRequest`]. Use this when you want to route the
/// payload through a different serializer or augment it before sending.
pub fn build(registry: &Registry, service_name: impl Into<String>) -> ExportMetricsServiceRequest {
    let now_nanos = now_unix_nanos();
    let now_str = now_nanos.to_string();
    let mut metrics: Vec<Metric> = Vec::new();

    #[cfg(feature = "count")]
    for (name, labels, c) in registry.counter_entries() {
        let meta = registry.metadata(&name);
        metrics.push(Metric {
            name,
            description: meta
                .as_ref()
                .map(|m| m.help.to_string())
                .unwrap_or_default(),
            unit: meta.as_ref().map(unit_str).unwrap_or_default(),
            data: MetricData::Sum {
                sum: NumberData {
                    data_points: vec![NumberDataPoint {
                        attributes: labels_to_attributes(&labels),
                        start_time_unix_nano: now_str.clone(),
                        time_unix_nano: now_str.clone(),
                        value: NumberValue::AsInt {
                            as_int: c.get().to_string(),
                        },
                    }],
                    aggregation_temporality: Some(AGGREGATION_TEMPORALITY_CUMULATIVE),
                    is_monotonic: Some(true),
                },
            },
        });
    }

    #[cfg(feature = "gauge")]
    for (name, labels, g) in registry.gauge_entries() {
        let meta = registry.metadata(&name);
        metrics.push(Metric {
            name,
            description: meta
                .as_ref()
                .map(|m| m.help.to_string())
                .unwrap_or_default(),
            unit: meta.as_ref().map(unit_str).unwrap_or_default(),
            data: MetricData::Gauge {
                gauge: NumberData {
                    data_points: vec![NumberDataPoint {
                        attributes: labels_to_attributes(&labels),
                        start_time_unix_nano: now_str.clone(),
                        time_unix_nano: now_str.clone(),
                        value: NumberValue::AsDouble { as_double: g.get() },
                    }],
                    aggregation_temporality: None,
                    is_monotonic: None,
                },
            },
        });
    }

    #[cfg(feature = "timer")]
    for (name, labels, t) in registry.timer_entries() {
        let meta = registry.metadata(&name);
        let count = t.count();
        let sum = t.total().as_secs_f64();
        let (min, max) = if count == 0 {
            (None, None)
        } else {
            (Some(t.min().as_secs_f64()), Some(t.max().as_secs_f64()))
        };
        metrics.push(Metric {
            name: format!("{name}_seconds"),
            description: meta
                .as_ref()
                .map(|m| m.help.to_string())
                .unwrap_or_default(),
            unit: "s".to_string(),
            data: MetricData::Histogram {
                histogram: HistogramData {
                    data_points: vec![HistogramDataPoint {
                        attributes: labels_to_attributes(&labels),
                        start_time_unix_nano: now_str.clone(),
                        time_unix_nano: now_str.clone(),
                        count: count.to_string(),
                        sum,
                        min,
                        max,
                        bucket_counts: vec![count.to_string()],
                        explicit_bounds: vec![],
                    }],
                    aggregation_temporality: AGGREGATION_TEMPORALITY_CUMULATIVE,
                },
            },
        });
    }

    #[cfg(feature = "meter")]
    for (name, labels, r) in registry.rate_meter_entries() {
        let meta = registry.metadata(&name);
        metrics.push(Metric {
            name: format!("{name}_per_second"),
            description: meta
                .as_ref()
                .map(|m| m.help.to_string())
                .unwrap_or_default(),
            unit: "1".to_string(),
            data: MetricData::Gauge {
                gauge: NumberData {
                    data_points: vec![NumberDataPoint {
                        attributes: labels_to_attributes(&labels),
                        start_time_unix_nano: now_str.clone(),
                        time_unix_nano: now_str.clone(),
                        value: NumberValue::AsDouble {
                            as_double: r.rate(),
                        },
                    }],
                    aggregation_temporality: None,
                    is_monotonic: None,
                },
            },
        });
    }

    #[cfg(feature = "histogram")]
    for (name, labels, h) in registry.histogram_entries() {
        let meta = registry.metadata(&name);
        let snap = h.snapshot();
        // OTLP `bucketCounts` is non-cumulative — undo the cumulative form.
        let mut bucket_counts: Vec<String> = Vec::with_capacity(snap.buckets.len());
        let mut prev = 0u64;
        for b in &snap.buckets {
            bucket_counts.push((b.count.saturating_sub(prev)).to_string());
            prev = b.count;
        }
        let explicit_bounds: Vec<f64> = snap
            .buckets
            .iter()
            .filter(|b| b.upper_bound.is_finite())
            .map(|b| b.upper_bound)
            .collect();
        metrics.push(Metric {
            name,
            description: meta
                .as_ref()
                .map(|m| m.help.to_string())
                .unwrap_or_default(),
            unit: meta.as_ref().map(unit_str).unwrap_or_default(),
            data: MetricData::Histogram {
                histogram: HistogramData {
                    data_points: vec![HistogramDataPoint {
                        attributes: labels_to_attributes(&labels),
                        start_time_unix_nano: now_str.clone(),
                        time_unix_nano: now_str.clone(),
                        count: snap.count.to_string(),
                        sum: snap.sum,
                        min: None,
                        max: None,
                        bucket_counts,
                        explicit_bounds,
                    }],
                    aggregation_temporality: AGGREGATION_TEMPORALITY_CUMULATIVE,
                },
            },
        });
    }

    ExportMetricsServiceRequest {
        resource_metrics: vec![ResourceMetrics {
            resource: Resource {
                attributes: vec![KeyValue {
                    key: "service.name".to_string(),
                    value: AnyValue::StringValue {
                        string_value: service_name.into(),
                    },
                }],
            },
            scope_metrics: vec![ScopeMetrics {
                scope: InstrumentationScope {
                    name: "metrics-lib".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                },
                metrics,
            }],
        }],
    }
}

fn now_unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
}

fn unit_str(m: &MetricMetadata) -> String {
    match m.unit {
        Unit::None => String::new(),
        u => u.as_str().to_string(),
    }
}

fn labels_to_attributes(labels: &LabelSet) -> Vec<KeyValue> {
    labels
        .iter()
        .map(|(k, v)| KeyValue {
            key: k.to_string(),
            value: AnyValue::StringValue {
                string_value: v.to_string(),
            },
        })
        .collect()
}

// ---------------------------------------------------------------------
// OTLP wire types — minimal serde subset of
// opentelemetry.proto.collector.metrics.v1.ExportMetricsServiceRequest.
// ---------------------------------------------------------------------

/// Top-level OTLP payload.
#[derive(Debug, Clone, Serialize)]
pub struct ExportMetricsServiceRequest {
    /// One `ResourceMetrics` per logical resource (typically one).
    #[serde(rename = "resourceMetrics")]
    pub resource_metrics: Vec<ResourceMetrics>,
}

/// Resource-scoped metrics block.
#[derive(Debug, Clone, Serialize)]
pub struct ResourceMetrics {
    /// Resource attributes (e.g. `service.name`).
    pub resource: Resource,
    /// Per-instrumentation-scope metric groups.
    #[serde(rename = "scopeMetrics")]
    pub scope_metrics: Vec<ScopeMetrics>,
}

/// OTLP `Resource`.
#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    /// Resource attribute list.
    pub attributes: Vec<KeyValue>,
}

/// OTLP `ScopeMetrics`.
#[derive(Debug, Clone, Serialize)]
pub struct ScopeMetrics {
    /// Instrumentation scope descriptor.
    pub scope: InstrumentationScope,
    /// Metrics emitted from this scope.
    pub metrics: Vec<Metric>,
}

/// OTLP `InstrumentationScope`.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentationScope {
    /// Scope name (we use `"metrics-lib"`).
    pub name: String,
    /// Scope version (the crate version at compile time).
    pub version: String,
}

/// OTLP `Metric`.
#[derive(Debug, Clone, Serialize)]
pub struct Metric {
    /// Metric name.
    pub name: String,
    /// Free-form metric description.
    pub description: String,
    /// Unit string (e.g. `"s"`, `"By"`, `"1"`).
    pub unit: String,
    /// Metric data variant.
    #[serde(flatten)]
    pub data: MetricData,
}

/// OTLP `Metric.data` (the `oneof` variant).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MetricData {
    /// `Sum` variant (counters).
    Sum {
        /// Sum payload.
        sum: NumberData,
    },
    /// `Gauge` variant.
    Gauge {
        /// Gauge payload.
        gauge: NumberData,
    },
    /// `Histogram` variant.
    Histogram {
        /// Histogram payload.
        histogram: HistogramData,
    },
}

/// OTLP `Sum`/`Gauge` data block.
#[derive(Debug, Clone, Serialize)]
pub struct NumberData {
    /// Per-attribute data points.
    #[serde(rename = "dataPoints")]
    pub data_points: Vec<NumberDataPoint>,
    /// Aggregation temporality (1 = DELTA, 2 = CUMULATIVE).
    #[serde(
        rename = "aggregationTemporality",
        skip_serializing_if = "Option::is_none"
    )]
    pub aggregation_temporality: Option<u32>,
    /// Monotonic flag (set for counters; absent for gauges).
    #[serde(rename = "isMonotonic", skip_serializing_if = "Option::is_none")]
    pub is_monotonic: Option<bool>,
}

/// One OTLP `NumberDataPoint`.
#[derive(Debug, Clone, Serialize)]
pub struct NumberDataPoint {
    /// Per-point attribute set (label values).
    pub attributes: Vec<KeyValue>,
    /// Datapoint start timestamp (Unix-epoch nanoseconds, JSON-string-encoded).
    #[serde(rename = "startTimeUnixNano")]
    pub start_time_unix_nano: String,
    /// Datapoint timestamp.
    #[serde(rename = "timeUnixNano")]
    pub time_unix_nano: String,
    /// Datapoint value (`asInt` or `asDouble`).
    #[serde(flatten)]
    pub value: NumberValue,
}

/// OTLP `NumberDataPoint.value` (the `oneof` variant).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NumberValue {
    /// 64-bit signed integer encoded as a JSON string.
    AsInt {
        /// Integer value (string-encoded per OTLP/JSON convention).
        #[serde(rename = "asInt")]
        as_int: String,
    },
    /// 64-bit double encoded as a JSON number.
    AsDouble {
        /// Double value.
        #[serde(rename = "asDouble")]
        as_double: f64,
    },
}

/// OTLP `Histogram` data block.
#[derive(Debug, Clone, Serialize)]
pub struct HistogramData {
    /// Per-attribute histogram data points.
    #[serde(rename = "dataPoints")]
    pub data_points: Vec<HistogramDataPoint>,
    /// Aggregation temporality (1 = DELTA, 2 = CUMULATIVE).
    #[serde(rename = "aggregationTemporality")]
    pub aggregation_temporality: u32,
}

/// One OTLP `HistogramDataPoint`.
#[derive(Debug, Clone, Serialize)]
pub struct HistogramDataPoint {
    /// Per-point attribute set.
    pub attributes: Vec<KeyValue>,
    /// Start timestamp (Unix-epoch nanoseconds, JSON-string-encoded).
    #[serde(rename = "startTimeUnixNano")]
    pub start_time_unix_nano: String,
    /// Datapoint timestamp.
    #[serde(rename = "timeUnixNano")]
    pub time_unix_nano: String,
    /// Total observation count (string-encoded 64-bit).
    pub count: String,
    /// Sum of all observations.
    pub sum: f64,
    /// Minimum observation (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum observation (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Non-cumulative per-bucket counts, including the trailing `+Inf` bucket.
    #[serde(rename = "bucketCounts")]
    pub bucket_counts: Vec<String>,
    /// Explicit bucket upper bounds (excludes the `+Inf` bucket).
    #[serde(rename = "explicitBounds")]
    pub explicit_bounds: Vec<f64>,
}

/// OTLP `KeyValue` attribute pair.
#[derive(Debug, Clone, Serialize)]
pub struct KeyValue {
    /// Attribute key.
    pub key: String,
    /// Attribute value.
    pub value: AnyValue,
}

/// OTLP `AnyValue` — restricted here to string attributes.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AnyValue {
    /// String attribute value.
    StringValue {
        /// Inner string.
        #[serde(rename = "stringValue")]
        string_value: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_has_resource_metrics_block() {
        let r = Registry::new();
        let body = render(&r, "test-service");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(
            v["resourceMetrics"][0]["resource"]["attributes"][0]["key"],
            "service.name"
        );
        assert_eq!(
            v["resourceMetrics"][0]["resource"]["attributes"][0]["value"]["stringValue"],
            "test-service"
        );
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_renders_as_sum_monotonic() {
        let r = Registry::new();
        r.describe_counter("requests", "Total HTTP requests", Unit::Custom("1"));
        let labels = LabelSet::from([("method", "GET")]);
        r.get_or_create_counter_with("requests", &labels).add(42);
        let body = render(&r, "test-service");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        let metric = &v["resourceMetrics"][0]["scopeMetrics"][0]["metrics"][0];
        assert_eq!(metric["name"], "requests");
        assert_eq!(metric["sum"]["isMonotonic"], true);
        assert_eq!(metric["sum"]["aggregationTemporality"], 2);
        let dp = &metric["sum"]["dataPoints"][0];
        assert_eq!(dp["asInt"], "42");
        assert_eq!(dp["attributes"][0]["key"], "method");
        assert_eq!(dp["attributes"][0]["value"]["stringValue"], "GET");
    }

    #[test]
    #[cfg(feature = "gauge")]
    fn gauge_renders_as_gauge_double() {
        let r = Registry::new();
        r.get_or_create_gauge("temp_c").set(21.5);
        let body = render(&r, "svc");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        let metric = &v["resourceMetrics"][0]["scopeMetrics"][0]["metrics"][0];
        assert_eq!(metric["name"], "temp_c");
        let dp = &metric["gauge"]["dataPoints"][0];
        assert_eq!(dp["asDouble"], 21.5);
    }

    #[test]
    #[cfg(feature = "timer")]
    fn timer_renders_as_histogram_with_seconds_suffix() {
        let r = Registry::new();
        let t = r.get_or_create_timer("rpc");
        t.record(std::time::Duration::from_millis(3));
        let body = render(&r, "svc");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        let metric = &v["resourceMetrics"][0]["scopeMetrics"][0]["metrics"][0];
        assert_eq!(metric["name"], "rpc_seconds");
        assert_eq!(metric["unit"], "s");
        let dp = &metric["histogram"]["dataPoints"][0];
        assert_eq!(dp["count"], "1");
        assert!(dp["min"].is_number());
        assert!(dp["max"].is_number());
        assert_eq!(dp["explicitBounds"], serde_json::json!([]));
    }

    #[test]
    #[cfg(feature = "meter")]
    fn rate_meter_renders_as_gauge_per_second() {
        let r = Registry::new();
        r.get_or_create_rate_meter("qps").tick_n(2);
        let body = render(&r, "svc");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        let metric = &v["resourceMetrics"][0]["scopeMetrics"][0]["metrics"][0];
        assert_eq!(metric["name"], "qps_per_second");
        assert_eq!(metric["unit"], "1");
        let dp = &metric["gauge"]["dataPoints"][0];
        assert!(dp["asDouble"].is_number());
    }

    #[test]
    fn render_pretty_is_indented() {
        let r = Registry::new();
        let body = render_pretty(&r, "svc");
        assert!(body.contains('\n'));
    }

    #[test]
    fn unit_str_passes_through_known_units() {
        // Indirectly exercise unit_str via describe + render.
        let r = Registry::new();
        #[cfg(feature = "count")]
        {
            r.describe_counter("bytes_total_in", "Bytes received", Unit::Bytes);
            r.get_or_create_counter("bytes_total_in").add(1024);
        }
        let body = render(&r, "svc");
        let _v: serde_json::Value = serde_json::from_str(&body).unwrap();
        #[cfg(feature = "count")]
        assert!(body.contains("\"unit\":\"bytes\""), "{body}");
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn histogram_renders_non_cumulative_buckets() {
        let r = Registry::new();
        r.configure_histogram("rtt", [0.01, 0.1]);
        let h = r.get_or_create_histogram("rtt");
        h.observe(0.005); // → bucket[0]
        h.observe(0.05); //  → bucket[1]
        h.observe(0.5); //   → +Inf
        let body = render(&r, "svc");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        let metric = &v["resourceMetrics"][0]["scopeMetrics"][0]["metrics"][0];
        let dp = &metric["histogram"]["dataPoints"][0];
        assert_eq!(dp["count"], "3");
        assert_eq!(dp["bucketCounts"], serde_json::json!(["1", "1", "1"]));
        assert_eq!(dp["explicitBounds"], serde_json::json!([0.01, 0.1]));
    }
}
