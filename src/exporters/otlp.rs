//! OpenTelemetry (OTLP) metrics exporter (feature `otlp`).
//!
//! Builds an `ExportMetricsServiceRequest` from the registry and encodes it
//! as OTLP/HTTP **JSON** ([`OtlpExporter::render_json`]) or **protobuf**
//! ([`OtlpExporter::render_protobuf`], hand-encoded, no extra dependencies).
//! Transport is up to you: POST the body to the collector's `/v1/metrics`
//! with [`JSON_CONTENT_TYPE`] or [`PROTOBUF_CONTENT_TYPE`].
//!
//! # Mapping
//!
//! | Registry kind | OTLP metric |
//! |---|---|
//! | counter, sharded counter | `Sum`, monotonic, integer points |
//! | up/down counter | `Sum`, non-monotonic, integer points |
//! | gauge | `Gauge`, double points (NaN/±∞ points are skipped) |
//! | timer | `Summary` in seconds: count, sum, quantile 0 = min, quantile 1 = max |
//! | rate meter | `Sum` of events (monotonic) plus a `Gauge` `<name>.per_second` |
//! | histogram | `Histogram` with explicit bounds |
//! | summary | `Summary` with the configured quantiles |
//!
//! All series of one name are data points of one `Metric`.
//!
//! # Temporality and start times
//!
//! With [`Temporality::Cumulative`] (the default) every point's
//! `start_time_unix_nano` is the wall-clock time its series was created, so
//! it is stable across exports as the specification requires (1.x reset it
//! on every export). When a counter, histogram, timer or summary is reset
//! between two exports (its count goes down), the exporter starts a new
//! series: the start time becomes the previous export time, so consumers
//! see a reset instead of a negative rate. With [`Temporality::Delta`] the exporter remembers the
//! previous export: sums and histograms report the change since then, with
//! the previous export time as the start. Gauges and summaries have no
//! temporality and always report the current value.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{LabelSet, MetricsCore};
//! use metrics_lib::exporters::otlp::{OtlpExporter, Temporality};
//!
//! let core = MetricsCore::new();
//! core.counter_with("requests", &LabelSet::from([("status", "200")])).add(3);
//!
//! let exporter = OtlpExporter::new("checkout")
//!     .with_resource_attribute("deployment.environment", "prod")
//!     .with_temporality(Temporality::Delta);
//! let json = exporter.render_json(core.registry()).unwrap();
//! assert!(json.contains(r#""asInt":"3""#));
//! let protobuf: Vec<u8> = exporter.render_protobuf(core.registry());
//! assert!(!protobuf.is_empty());
//! # }
//! ```

mod proto;

use crate::registry::{FamilySnapshot, Handle};
use crate::{LabelSet, MetricKind, Registry};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

/// `Content-Type` for OTLP/HTTP JSON bodies.
pub const JSON_CONTENT_TYPE: &str = "application/json";
/// `Content-Type` for OTLP/HTTP protobuf bodies.
pub const PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";

/// Aggregation temporality of sums and histograms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Temporality {
    /// Totals since each series was created.
    #[default]
    Cumulative,
    /// Changes since the previous export by this exporter.
    Delta,
}

impl Temporality {
    const fn wire(self) -> u32 {
        match self {
            Temporality::Delta => 1,
            Temporality::Cumulative => 2,
        }
    }
}

/// Stateful OTLP exporter.
///
/// Keep one per destination: in [`Temporality::Delta`] mode it remembers
/// what it sent last time. Series that disappear from the registry are
/// forgotten on the next export.
pub struct OtlpExporter {
    service_name: String,
    resource: LabelSet,
    temporality: Temporality,
    previous: Mutex<HashMap<(String, LabelSet), Previous>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(not(feature = "histogram"), allow(dead_code))]
enum PreviousValue {
    Int(i64),
    UInt(u64),
    /// Cumulative mode: the last exported total and the start time in use.
    Cumulative {
        total: u64,
        start: u64,
    },
    Histogram {
        count: u64,
        sum: f64,
        buckets: Vec<u64>,
    },
}

#[derive(Debug, Clone)]
struct Previous {
    time: u64,
    value: PreviousValue,
}

impl OtlpExporter {
    /// Exporter whose resource carries `service.name = service_name`.
    #[must_use]
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            resource: LabelSet::new(),
            temporality: Temporality::Cumulative,
            previous: Mutex::new(HashMap::new()),
        }
    }

    /// Choose the temporality of sums and histograms.
    #[must_use]
    pub fn with_temporality(mut self, temporality: Temporality) -> Self {
        self.temporality = temporality;
        self
    }

    /// Add a resource attribute (for example `deployment.environment`).
    #[must_use]
    pub fn with_resource_attribute(
        mut self,
        key: impl Into<std::borrow::Cow<'static, str>>,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        let _ = self.resource.add(key, value);
        self
    }

    /// The configured temporality.
    #[must_use]
    pub fn temporality(&self) -> Temporality {
        self.temporality
    }

    /// Build the request in the serde model used for JSON.
    ///
    /// In delta mode this advances the exporter's baselines.
    #[must_use]
    pub fn build(&self, registry: &Registry) -> ExportMetricsServiceRequest {
        let model = self.model(registry);
        to_json_model(&model, &self.service_name, &self.resource)
    }

    /// Encode the request as OTLP/HTTP JSON.
    ///
    /// # Errors
    ///
    /// Returns the `serde_json` error if serialisation fails; the request
    /// holds only strings, integers and finite floats, so this does not
    /// happen in practice.
    pub fn render_json(&self, registry: &Registry) -> serde_json::Result<String> {
        serde_json::to_string(&self.build(registry))
    }

    /// Encode the request as OTLP protobuf (`ExportMetricsServiceRequest`).
    ///
    /// In delta mode this advances the exporter's baselines.
    #[must_use]
    pub fn render_protobuf(&self, registry: &Registry) -> Vec<u8> {
        let model = self.model(registry);
        proto::encode_request(&model, &self.service_name, &self.resource)
    }

    fn model(&self, registry: &Registry) -> Vec<MetricModel> {
        let now = unix_nanos_now();
        let mut previous = self.previous.lock().unwrap_or_else(PoisonError::into_inner);
        let mut next: HashMap<(String, LabelSet), Previous> = HashMap::new();
        let mut ctx = ModelCtx {
            now,
            temporality: self.temporality,
            previous: &previous,
            next: &mut next,
        };
        let mut out = Vec::new();
        // A derived name (`<rate>.per_second`) can equal another family's
        // name; the first family in name order keeps it, as in the text
        // exporters, and the clash is counted.
        let mut claimed: HashSet<String> = HashSet::new();
        let mut family_metrics = Vec::new();
        for family in registry.collect() {
            family_metrics.clear();
            ctx.family(&family, &mut family_metrics);
            if family_metrics.iter().any(|m| claimed.contains(&m.name)) {
                registry.note_exposition_conflict();
                continue;
            }
            claimed.extend(family_metrics.iter().map(|m| m.name.clone()));
            out.append(&mut family_metrics);
        }
        *previous = next;
        out
    }
}

impl std::fmt::Debug for OtlpExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OtlpExporter")
            .field("service_name", &self.service_name)
            .field("temporality", &self.temporality)
            .finish_non_exhaustive()
    }
}

/// One-shot cumulative OTLP/HTTP JSON payload.
///
/// # Errors
///
/// See [`OtlpExporter::render_json`].
pub fn render(registry: &Registry, service_name: impl Into<String>) -> serde_json::Result<String> {
    OtlpExporter::new(service_name).render_json(registry)
}

/// One-shot cumulative OTLP protobuf payload.
#[must_use]
pub fn render_protobuf(registry: &Registry, service_name: impl Into<String>) -> Vec<u8> {
    OtlpExporter::new(service_name).render_protobuf(registry)
}

fn unix_nanos_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
}

// ---------------------------------------------------------------------------
// Internal model shared by the JSON and protobuf encoders.
// ---------------------------------------------------------------------------

pub(crate) struct MetricModel {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) unit: String,
    pub(crate) data: DataModel,
}

pub(crate) enum DataModel {
    Gauge(Vec<NumberPoint>),
    Sum {
        points: Vec<NumberPoint>,
        monotonic: bool,
        temporality: Temporality,
    },
    Histogram {
        points: Vec<HistogramPoint>,
        temporality: Temporality,
    },
    Summary(Vec<SummaryPoint>),
}

pub(crate) struct NumberPoint {
    pub(crate) attributes: LabelSet,
    pub(crate) start: u64,
    pub(crate) time: u64,
    pub(crate) value: Number,
}

#[derive(Clone, Copy)]
pub(crate) enum Number {
    Int(i64),
    Double(f64),
}

pub(crate) struct HistogramPoint {
    pub(crate) attributes: LabelSet,
    pub(crate) start: u64,
    pub(crate) time: u64,
    pub(crate) count: u64,
    pub(crate) sum: Option<f64>,
    pub(crate) bucket_counts: Vec<u64>,
    pub(crate) explicit_bounds: Vec<f64>,
}

pub(crate) struct SummaryPoint {
    pub(crate) attributes: LabelSet,
    pub(crate) start: u64,
    pub(crate) time: u64,
    pub(crate) count: u64,
    pub(crate) sum: f64,
    pub(crate) quantiles: Vec<(f64, f64)>,
}

struct ModelCtx<'a> {
    now: u64,
    temporality: Temporality,
    previous: &'a HashMap<(String, LabelSet), Previous>,
    next: &'a mut HashMap<(String, LabelSet), Previous>,
}

impl ModelCtx<'_> {
    /// Start time and baseline for a cumulative value under the configured
    /// temporality; records the new baseline.
    fn uint_point(&mut self, key: (String, LabelSet), created: u64, total: u64) -> (u64, u64) {
        if self.temporality == Temporality::Cumulative {
            return (self.cumulative_start(key, created, total), total);
        }
        let (start, value) = match self.previous.get(&key) {
            Some(Previous {
                time,
                value: PreviousValue::UInt(prev),
            }) if *prev <= total => (*time, total - prev),
            // Reset since the last export: everything counted since then.
            Some(Previous {
                time,
                value: PreviousValue::UInt(_),
            }) => (*time, total),
            _ => (created, total),
        };
        let _ = self.next.insert(
            key,
            Previous {
                time: self.now,
                value: PreviousValue::UInt(total),
            },
        );
        (start, value)
    }

    /// Start time of a cumulative series. A total lower than at the last
    /// export means the metric was reset in between; OTLP signals that with
    /// a new start time, so the last export time becomes the start.
    fn cumulative_start(&mut self, key: (String, LabelSet), created: u64, total: u64) -> u64 {
        let start = match self.previous.get(&key) {
            Some(Previous {
                time,
                value: PreviousValue::Cumulative { total: prev, start },
            }) => {
                if total < *prev {
                    *time
                } else {
                    *start
                }
            }
            _ => created,
        };
        let _ = self.next.insert(
            key,
            Previous {
                time: self.now,
                value: PreviousValue::Cumulative { total, start },
            },
        );
        start
    }

    fn int_point(&mut self, key: (String, LabelSet), created: u64, value: i64) -> (u64, i64) {
        if self.temporality == Temporality::Cumulative {
            return (created, value);
        }
        let (start, delta) = match self.previous.get(&key) {
            Some(Previous {
                time,
                value: PreviousValue::Int(prev),
            }) => (*time, value.wrapping_sub(*prev)),
            _ => (created, value),
        };
        let _ = self.next.insert(
            key,
            Previous {
                time: self.now,
                value: PreviousValue::Int(value),
            },
        );
        (start, delta)
    }

    #[allow(clippy::too_many_lines)] // one arm per metric kind
    #[allow(unused_mut, irrefutable_let_patterns)] // arms are feature-gated
    fn family(&mut self, family: &FamilySnapshot, out: &mut Vec<MetricModel>) {
        let description = family.help.as_deref().unwrap_or_default().to_owned();
        let unit = family.unit.as_ucum().into_owned();
        let name = family.name.clone();
        let now = self.now;
        match family.kind {
            MetricKind::Counter | MetricKind::UpDownCounter => {
                let monotonic = family.kind == MetricKind::Counter;
                let mut points = Vec::with_capacity(family.series.len());
                for s in &family.series {
                    let key = (name.clone(), LabelSet::clone(&s.labels));
                    let point = match &s.handle {
                        #[cfg(feature = "count")]
                        Handle::Counter(c) => {
                            let (start, v) = self.uint_point(key, s.created_unix_nanos, c.get());
                            Some((start, Number::Int(saturating_i64(v))))
                        }
                        #[cfg(feature = "count")]
                        Handle::ShardedCounter(c) => {
                            let (start, v) = self.uint_point(key, s.created_unix_nanos, c.get());
                            Some((start, Number::Int(saturating_i64(v))))
                        }
                        #[cfg(feature = "count")]
                        Handle::UpDownCounter(c) => {
                            let (start, v) = self.int_point(key, s.created_unix_nanos, c.get());
                            Some((start, Number::Int(v)))
                        }
                        #[allow(unreachable_patterns)]
                        _ => None,
                    };
                    if let Some((start, value)) = point {
                        points.push(NumberPoint {
                            attributes: LabelSet::clone(&s.labels),
                            start,
                            time: now,
                            value,
                        });
                    }
                }
                out.push(MetricModel {
                    name,
                    description,
                    unit,
                    data: DataModel::Sum {
                        points,
                        monotonic,
                        temporality: self.temporality,
                    },
                });
            }
            MetricKind::Gauge => {
                let mut points = Vec::new();
                #[cfg(feature = "gauge")]
                for s in &family.series {
                    if let Handle::Gauge(g) = &s.handle {
                        let v = g.get();
                        if v.is_finite() {
                            points.push(NumberPoint {
                                attributes: LabelSet::clone(&s.labels),
                                start: s.created_unix_nanos,
                                time: now,
                                value: Number::Double(v),
                            });
                        }
                    }
                }
                out.push(MetricModel {
                    name,
                    description,
                    unit,
                    data: DataModel::Gauge(points),
                });
            }
            MetricKind::Timer => {
                let mut points = Vec::new();
                #[cfg(feature = "timer")]
                for s in &family.series {
                    if let Handle::Timer(t) = &s.handle {
                        let count = t.count();
                        #[allow(clippy::cast_precision_loss)] // seconds as f64 is the contract
                        let sum = t.total_nanos() as f64 / 1e9;
                        let quantiles = if count > 0 {
                            vec![(0.0, t.min().as_secs_f64()), (1.0, t.max().as_secs_f64())]
                        } else {
                            Vec::new()
                        };
                        let key = (name.clone(), LabelSet::clone(&s.labels));
                        let start = self.cumulative_start(key, s.created_unix_nanos, count);
                        points.push(SummaryPoint {
                            attributes: LabelSet::clone(&s.labels),
                            start,
                            time: now,
                            count,
                            sum,
                            quantiles,
                        });
                    }
                }
                out.push(MetricModel {
                    name,
                    description,
                    unit: "s".to_owned(),
                    data: DataModel::Summary(points),
                });
            }
            MetricKind::Rate => {
                let mut totals = Vec::new();
                let mut rates = Vec::new();
                #[cfg(feature = "meter")]
                for s in &family.series {
                    if let Handle::Rate(r) = &s.handle {
                        let key = (name.clone(), LabelSet::clone(&s.labels));
                        let (start, v) = self.uint_point(key, s.created_unix_nanos, r.total());
                        totals.push(NumberPoint {
                            attributes: LabelSet::clone(&s.labels),
                            start,
                            time: now,
                            value: Number::Int(saturating_i64(v)),
                        });
                        rates.push(NumberPoint {
                            attributes: LabelSet::clone(&s.labels),
                            start: s.created_unix_nanos,
                            time: now,
                            value: Number::Double(r.rate()),
                        });
                    }
                }
                out.push(MetricModel {
                    name: name.clone(),
                    description: description.clone(),
                    unit: unit.clone(),
                    data: DataModel::Sum {
                        points: totals,
                        monotonic: true,
                        temporality: self.temporality,
                    },
                });
                out.push(MetricModel {
                    name: format!("{name}.per_second"),
                    description,
                    unit: "1/s".to_owned(),
                    data: DataModel::Gauge(rates),
                });
            }
            MetricKind::Histogram => {
                let mut points = Vec::new();
                #[cfg(feature = "histogram")]
                for s in &family.series {
                    if let Handle::Histogram(h) = &s.handle {
                        points.push(self.histogram_point(
                            &name,
                            &s.labels,
                            s.created_unix_nanos,
                            h,
                        ));
                    }
                }
                out.push(MetricModel {
                    name,
                    description,
                    unit,
                    data: DataModel::Histogram {
                        points,
                        temporality: self.temporality,
                    },
                });
            }
            MetricKind::Summary => {
                let mut points = Vec::new();
                #[cfg(feature = "histogram")]
                for s in &family.series {
                    if let Handle::Summary(q) = &s.handle {
                        let snap = q.snapshot();
                        let key = (name.clone(), LabelSet::clone(&s.labels));
                        let start = self.cumulative_start(key, s.created_unix_nanos, snap.count);
                        points.push(SummaryPoint {
                            attributes: LabelSet::clone(&s.labels),
                            start,
                            time: now,
                            count: snap.count,
                            sum: snap.sum,
                            quantiles: snap
                                .quantiles
                                .into_iter()
                                .filter(|(_, v)| v.is_finite())
                                .collect(),
                        });
                    }
                }
                out.push(MetricModel {
                    name,
                    description,
                    unit,
                    data: DataModel::Summary(points),
                });
            }
        }
    }

    #[cfg(feature = "histogram")]
    fn histogram_point(
        &mut self,
        name: &str,
        labels: &LabelSet,
        created: u64,
        h: &crate::Histogram,
    ) -> HistogramPoint {
        let snap = h.snapshot_counts();
        // OTLP bucket counts are per bucket, not cumulative.
        let mut buckets = Vec::with_capacity(snap.buckets.len());
        let mut prev = 0u64;
        for b in &snap.buckets {
            buckets.push(b.count.saturating_sub(prev));
            prev = b.count;
        }
        let bounds: Vec<f64> = h.bounds().to_vec();
        let (start, count, sum, buckets) = if self.temporality == Temporality::Cumulative {
            let key = (name.to_owned(), labels.clone());
            let start = self.cumulative_start(key, created, snap.count);
            (start, snap.count, snap.sum, buckets)
        } else {
            let key = (name.to_owned(), labels.clone());
            let delta = match self.previous.get(&key) {
                Some(Previous {
                    time,
                    value:
                        PreviousValue::Histogram {
                            count,
                            sum,
                            buckets: prev_buckets,
                        },
                }) if *count <= snap.count && prev_buckets.len() == buckets.len() => Some((
                    *time,
                    snap.count - count,
                    snap.sum - sum,
                    buckets
                        .iter()
                        .zip(prev_buckets)
                        .map(|(now, before)| now.saturating_sub(*before))
                        .collect(),
                )),
                // Reset (or reconfigured) since the last export.
                Some(Previous { time, .. }) => Some((*time, snap.count, snap.sum, buckets.clone())),
                _ => None,
            };
            let _ = self.next.insert(
                key,
                Previous {
                    time: self.now,
                    value: PreviousValue::Histogram {
                        count: snap.count,
                        sum: snap.sum,
                        buckets: buckets.clone(),
                    },
                },
            );
            delta.unwrap_or((created, snap.count, snap.sum, buckets))
        };
        HistogramPoint {
            attributes: labels.clone(),
            start,
            time: self.now,
            count,
            sum: sum.is_finite().then_some(sum),
            bucket_counts: buckets,
            explicit_bounds: bounds,
        }
    }
}

fn saturating_i64(v: u64) -> i64 {
    i64::try_from(v).unwrap_or(i64::MAX)
}

// ---------------------------------------------------------------------------
// JSON model (OTLP/HTTP JSON encoding: camelCase names, 64-bit integers as
// strings, enums as integers).
// ---------------------------------------------------------------------------

/// `ExportMetricsServiceRequest` in the OTLP/HTTP JSON encoding.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ExportMetricsServiceRequest {
    /// Resource-scoped metrics (one entry).
    pub resource_metrics: Vec<ResourceMetrics>,
}

/// `ResourceMetrics`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResourceMetrics {
    /// The resource (`service.name` plus configured attributes).
    pub resource: Resource,
    /// Scope-scoped metrics (one entry: `metrics-lib`).
    pub scope_metrics: Vec<ScopeMetrics>,
}

/// `Resource`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Resource {
    /// Resource attributes.
    pub attributes: Vec<KeyValue>,
}

/// `ScopeMetrics`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ScopeMetrics {
    /// Instrumentation scope.
    pub scope: InstrumentationScope,
    /// One metric per family (rate meters contribute two).
    pub metrics: Vec<Metric>,
}

/// `InstrumentationScope`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InstrumentationScope {
    /// `"metrics-lib"`.
    pub name: String,
    /// The crate version.
    pub version: String,
}

/// `Metric`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Metric {
    /// Metric name.
    pub name: String,
    /// Help text.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// UCUM unit.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unit: String,
    /// The data, keyed by its type (`gauge`, `sum`, `histogram`, `summary`).
    #[serde(flatten)]
    pub data: MetricData,
}

/// The `data` oneof of `Metric`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum MetricData {
    /// `Gauge`.
    Gauge {
        /// Data points.
        #[serde(rename = "dataPoints")]
        data_points: Vec<NumberDataPoint>,
    },
    /// `Sum`.
    Sum {
        /// Data points.
        #[serde(rename = "dataPoints")]
        data_points: Vec<NumberDataPoint>,
        /// 1 = delta, 2 = cumulative.
        #[serde(rename = "aggregationTemporality")]
        aggregation_temporality: u32,
        /// Whether the sum only increases.
        #[serde(rename = "isMonotonic")]
        is_monotonic: bool,
    },
    /// `Histogram`.
    Histogram {
        /// Data points.
        #[serde(rename = "dataPoints")]
        data_points: Vec<HistogramDataPoint>,
        /// 1 = delta, 2 = cumulative.
        #[serde(rename = "aggregationTemporality")]
        aggregation_temporality: u32,
    },
    /// `Summary`.
    Summary {
        /// Data points.
        #[serde(rename = "dataPoints")]
        data_points: Vec<SummaryDataPoint>,
    },
}

/// `NumberDataPoint`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NumberDataPoint {
    /// Point attributes (the series labels).
    pub attributes: Vec<KeyValue>,
    /// Start of the aggregation interval (Unix nanoseconds, as a string).
    pub start_time_unix_nano: String,
    /// Time of the point (Unix nanoseconds, as a string).
    pub time_unix_nano: String,
    /// `asInt` or `asDouble`.
    #[serde(flatten)]
    pub value: NumberValue,
}

/// The `value` oneof of `NumberDataPoint`.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum NumberValue {
    /// Integer value (as a string).
    #[serde(rename = "asInt")]
    AsInt(String),
    /// Double value.
    #[serde(rename = "asDouble")]
    AsDouble(f64),
}

/// `HistogramDataPoint`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct HistogramDataPoint {
    /// Point attributes.
    pub attributes: Vec<KeyValue>,
    /// Start of the aggregation interval.
    pub start_time_unix_nano: String,
    /// Time of the point.
    pub time_unix_nano: String,
    /// Number of observations (as a string).
    pub count: String,
    /// Sum of observations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    /// Per-bucket counts (as strings), one more than `explicit_bounds`.
    pub bucket_counts: Vec<String>,
    /// Bucket upper bounds.
    pub explicit_bounds: Vec<f64>,
}

/// `SummaryDataPoint`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SummaryDataPoint {
    /// Point attributes.
    pub attributes: Vec<KeyValue>,
    /// Start of the aggregation interval.
    pub start_time_unix_nano: String,
    /// Time of the point.
    pub time_unix_nano: String,
    /// Number of observations (as a string).
    pub count: String,
    /// Sum of observations.
    pub sum: f64,
    /// Quantile estimates.
    pub quantile_values: Vec<ValueAtQuantile>,
}

/// `SummaryDataPoint.ValueAtQuantile`.
#[derive(Debug, Clone, Copy, Serialize)]
#[non_exhaustive]
pub struct ValueAtQuantile {
    /// Quantile in `[0, 1]`.
    pub quantile: f64,
    /// Estimate.
    pub value: f64,
}

/// `KeyValue` with a string value.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct KeyValue {
    /// Attribute key.
    pub key: String,
    /// Attribute value.
    pub value: AnyValue,
}

/// `AnyValue` (only `stringValue` is produced).
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum AnyValue {
    /// A string.
    #[serde(rename = "stringValue")]
    StringValue(String),
}

fn attributes(labels: &LabelSet) -> Vec<KeyValue> {
    labels
        .iter()
        .map(|(k, v)| KeyValue {
            key: k.to_owned(),
            value: AnyValue::StringValue(v.to_owned()),
        })
        .collect()
}

fn to_json_model(
    model: &[MetricModel],
    service_name: &str,
    resource: &LabelSet,
) -> ExportMetricsServiceRequest {
    let mut resource_attrs = vec![KeyValue {
        key: "service.name".to_owned(),
        value: AnyValue::StringValue(service_name.to_owned()),
    }];
    resource_attrs.extend(
        attributes(resource)
            .into_iter()
            .filter(|kv| kv.key != "service.name"),
    );
    let metrics = model
        .iter()
        .map(|m| Metric {
            name: m.name.clone(),
            description: m.description.clone(),
            unit: m.unit.clone(),
            data: match &m.data {
                DataModel::Gauge(points) => MetricData::Gauge {
                    data_points: points.iter().map(number_point).collect(),
                },
                DataModel::Sum {
                    points,
                    monotonic,
                    temporality,
                } => MetricData::Sum {
                    data_points: points.iter().map(number_point).collect(),
                    aggregation_temporality: temporality.wire(),
                    is_monotonic: *monotonic,
                },
                DataModel::Histogram {
                    points,
                    temporality,
                } => MetricData::Histogram {
                    data_points: points
                        .iter()
                        .map(|p| HistogramDataPoint {
                            attributes: attributes(&p.attributes),
                            start_time_unix_nano: p.start.to_string(),
                            time_unix_nano: p.time.to_string(),
                            count: p.count.to_string(),
                            sum: p.sum,
                            bucket_counts: p.bucket_counts.iter().map(u64::to_string).collect(),
                            explicit_bounds: p.explicit_bounds.clone(),
                        })
                        .collect(),
                    aggregation_temporality: temporality.wire(),
                },
                DataModel::Summary(points) => MetricData::Summary {
                    data_points: points
                        .iter()
                        .map(|p| SummaryDataPoint {
                            attributes: attributes(&p.attributes),
                            start_time_unix_nano: p.start.to_string(),
                            time_unix_nano: p.time.to_string(),
                            count: p.count.to_string(),
                            sum: p.sum,
                            quantile_values: p
                                .quantiles
                                .iter()
                                .map(|(q, v)| ValueAtQuantile {
                                    quantile: *q,
                                    value: *v,
                                })
                                .collect(),
                        })
                        .collect(),
                },
            },
        })
        .collect();
    ExportMetricsServiceRequest {
        resource_metrics: vec![ResourceMetrics {
            resource: Resource {
                attributes: resource_attrs,
            },
            scope_metrics: vec![ScopeMetrics {
                scope: InstrumentationScope {
                    name: "metrics-lib".to_owned(),
                    version: env!("CARGO_PKG_VERSION").to_owned(),
                },
                metrics,
            }],
        }],
    }
}

fn number_point(p: &NumberPoint) -> NumberDataPoint {
    NumberDataPoint {
        attributes: attributes(&p.attributes),
        start_time_unix_nano: p.start.to_string(),
        time_unix_nano: p.time.to_string(),
        value: match p.value {
            Number::Int(v) => NumberValue::AsInt(v.to_string()),
            Number::Double(v) => NumberValue::AsDouble(v),
        },
    }
}

#[cfg(test)]
#[cfg(all(feature = "count", feature = "gauge"))]
mod tests {
    use super::proto::decode::{Value, bytes, fields, one, string};
    use super::*;

    fn metric_by_name<'a>(req: &'a ExportMetricsServiceRequest, name: &str) -> &'a Metric {
        req.resource_metrics[0].scope_metrics[0]
            .metrics
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("metric {name} missing"))
    }

    fn sum_value(req: &ExportMetricsServiceRequest, name: &str) -> (u32, String) {
        match &metric_by_name(req, name).data {
            MetricData::Sum {
                data_points,
                aggregation_temporality,
                ..
            } => {
                let v = match &data_points[0].value {
                    NumberValue::AsInt(v) => v.clone(),
                    NumberValue::AsDouble(v) => v.to_string(),
                };
                (*aggregation_temporality, v)
            }
            _ => panic!("expected a sum"),
        }
    }

    #[cfg(all(feature = "meter", feature = "gauge"))]
    #[test]
    fn test_derived_name_collisions_are_skipped_regression_ml2_007() {
        let r = Registry::new();
        // The rate meter `q` emits `q.per_second`; the gauge clashes.
        r.rate_meter("q").tick();
        r.gauge("q.per_second").set(5.0);
        let req = OtlpExporter::new("svc").build(&r);
        let names: Vec<&str> = req.resource_metrics[0].scope_metrics[0]
            .metrics
            .iter()
            .map(|m| m.name.as_str())
            .collect();
        assert_eq!(names, vec!["q", "q.per_second"]);
        assert!(matches!(
            metric_by_name(&req, "q.per_second").data,
            MetricData::Gauge { .. }
        ));
        assert_eq!(r.exposition_conflicts(), 1);
    }

    #[test]
    fn test_reset_starts_a_new_cumulative_series_regression_ml2_010() {
        let r = Registry::new();
        let c = r.counter("c");
        c.add(5);
        let exporter = OtlpExporter::new("svc");
        let start = |req: &ExportMetricsServiceRequest| match &metric_by_name(req, "c").data {
            MetricData::Sum { data_points, .. } => data_points[0].start_time_unix_nano.clone(),
            _ => panic!("expected a sum"),
        };
        let first = exporter.build(&r);
        std::thread::sleep(std::time::Duration::from_millis(2));
        c.add(1);
        let second = exporter.build(&r);
        assert_eq!(start(&first), start(&second));
        c.reset();
        c.add(1);
        std::thread::sleep(std::time::Duration::from_millis(2));
        let third = exporter.build(&r);
        let parse = |s: String| s.parse::<u64>().unwrap();
        assert!(parse(start(&third)) > parse(start(&second)));
        // The new start sticks while the counter grows again.
        c.add(1);
        assert_eq!(start(&third), start(&exporter.build(&r)));
    }

    #[test]
    fn test_series_of_one_name_share_one_metric() {
        let r = Registry::new();
        r.counter_with("hits", &LabelSet::from([("k", "a")])).inc();
        r.counter_with("hits", &LabelSet::from([("k", "b")])).add(2);
        let req = OtlpExporter::new("svc").build(&r);
        let metrics = &req.resource_metrics[0].scope_metrics[0].metrics;
        assert_eq!(metrics.len(), 1);
        let MetricData::Sum {
            data_points,
            aggregation_temporality,
            is_monotonic,
        } = &metrics[0].data
        else {
            panic!("expected a sum");
        };
        assert_eq!(data_points.len(), 2);
        assert_eq!(*aggregation_temporality, 2);
        assert!(*is_monotonic);
    }

    #[test]
    fn test_cumulative_start_time_is_stable_regression_ml012() {
        // ML-012: the start time was reset to "now" on every export.
        let r = Registry::new();
        r.counter("c").inc();
        let exporter = OtlpExporter::new("svc");
        let start = |req: &ExportMetricsServiceRequest| match &metric_by_name(req, "c").data {
            MetricData::Sum { data_points, .. } => data_points[0].start_time_unix_nano.clone(),
            _ => panic!("expected a sum"),
        };
        let first = exporter.build(&r);
        std::thread::sleep(std::time::Duration::from_millis(5));
        let second = exporter.build(&r);
        assert_eq!(start(&first), start(&second));
        let time = |req: &ExportMetricsServiceRequest| match &metric_by_name(req, "c").data {
            MetricData::Sum { data_points, .. } => data_points[0].time_unix_nano.clone(),
            _ => panic!("expected a sum"),
        };
        assert_ne!(time(&first), time(&second));
    }

    #[test]
    fn test_delta_temporality_reports_changes() {
        let r = Registry::new();
        let c = r.counter("c");
        let u = r.up_down_counter("u");
        let exporter = OtlpExporter::new("svc").with_temporality(Temporality::Delta);
        assert_eq!(exporter.temporality(), Temporality::Delta);
        c.add(5);
        u.add(3);
        let first = exporter.build(&r);
        assert_eq!(sum_value(&first, "c"), (1, "5".to_owned()));
        assert_eq!(sum_value(&first, "u"), (1, "3".to_owned()));
        c.add(2);
        u.add(-4);
        let second = exporter.build(&r);
        assert_eq!(sum_value(&second, "c"), (1, "2".to_owned()));
        assert_eq!(sum_value(&second, "u"), (1, "-4".to_owned()));
        c.reset();
        c.add(1);
        assert_eq!(sum_value(&exporter.build(&r), "c"), (1, "1".to_owned()));
    }

    #[test]
    fn test_non_finite_gauges_are_skipped_regression_ml024() {
        let r = Registry::new();
        r.gauge("nan").set(f64::NAN);
        r.gauge("ok").set(2.0);
        let json = render(&r, "svc").unwrap();
        assert!(!json.contains("null"), "{json}");
        let req = OtlpExporter::new("svc").build(&r);
        match &metric_by_name(&req, "nan").data {
            MetricData::Gauge { data_points } => assert!(data_points.is_empty()),
            _ => panic!("expected a gauge"),
        }
    }

    #[test]
    fn test_json_uses_otlp_field_conventions() {
        let r = Registry::new();
        r.describe_counter("c", "Count of things", crate::Unit::None)
            .unwrap();
        r.counter("c").add(7);
        let exporter = OtlpExporter::new("svc").with_resource_attribute("env", "test");
        let json = exporter.render_json(&r).unwrap();
        for needle in [
            r#""resourceMetrics""#,
            r#""scopeMetrics""#,
            r#""service.name""#,
            r#""stringValue":"svc""#,
            r#""key":"env""#,
            r#""dataPoints""#,
            r#""startTimeUnixNano":""#,
            r#""asInt":"7""#,
            r#""aggregationTemporality":2"#,
            r#""isMonotonic":true"#,
            r#""description":"Count of things""#,
        ] {
            assert!(json.contains(needle), "{needle} missing in {json}");
        }
        assert!(format!("{exporter:?}").contains("OtlpExporter"));
    }

    #[test]
    fn test_protobuf_structure() {
        let r = Registry::new();
        r.counter_with("c", &LabelSet::from([("k", "v")])).add(3);
        r.gauge("g").set(1.5);
        let body = OtlpExporter::new("svc")
            .with_resource_attribute("env", "prod")
            .render_protobuf(&r);
        let req = fields(&body);
        let rm = fields(&bytes(&req, 1)[0]);
        let resource = fields(&bytes(&rm, 1)[0]);
        let attrs = bytes(&resource, 1);
        assert_eq!(attrs.len(), 2);
        assert_eq!(string(&fields(&attrs[0]), 1), "service.name");
        let sm = fields(&bytes(&rm, 2)[0]);
        let scope = fields(&bytes(&sm, 1)[0]);
        assert_eq!(string(&scope, 1), "metrics-lib");
        let metrics = bytes(&sm, 2);
        assert_eq!(metrics.len(), 2);
        let counter = fields(&metrics[0]);
        assert_eq!(string(&counter, 1), "c");
        let sum = fields(&bytes(&counter, 7)[0]);
        assert_eq!(one(&sum, 2), Value::Varint(2));
        assert_eq!(one(&sum, 3), Value::Varint(1));
        let point = fields(&bytes(&sum, 1)[0]);
        assert_eq!(one(&point, 6), Value::Fixed64(3));
        let gauge = fields(&metrics[1]);
        let g = fields(&bytes(&gauge, 5)[0]);
        let gp = fields(&bytes(&g, 1)[0]);
        assert_eq!(one(&gp, 4), Value::Fixed64(1.5f64.to_bits()));
        assert_ne!(render_protobuf(&r, "svc").len(), 0);
    }

    #[cfg(all(feature = "timer", feature = "meter", feature = "histogram"))]
    #[test]
    fn test_every_kind_maps_and_encodes() {
        let r = Registry::new();
        r.timer("t").record(std::time::Duration::from_millis(250));
        r.rate_meter("rate").tick_n(4);
        r.configure_histogram("h", [0.1, 1.0]).unwrap();
        let h = r.histogram("h");
        h.observe(0.05);
        h.observe(5.0);
        r.summary("s").observe(0.5);
        let _ = r.summary("s_empty");
        r.sharded_counter("sc").inc();
        let exporter = OtlpExporter::new("svc").with_temporality(Temporality::Delta);
        let req = exporter.build(&r);
        match &metric_by_name(&req, "t").data {
            MetricData::Summary { data_points } => {
                assert_eq!(data_points[0].count, "1");
                assert_eq!(data_points[0].quantile_values.len(), 2);
                assert!((data_points[0].sum - 0.25).abs() < 1e-9);
            }
            _ => panic!("timer should be a summary"),
        }
        assert_eq!(metric_by_name(&req, "t").unit, "s");
        assert!(matches!(
            metric_by_name(&req, "rate").data,
            MetricData::Sum { .. }
        ));
        assert!(matches!(
            metric_by_name(&req, "rate.per_second").data,
            MetricData::Gauge { .. }
        ));
        match &metric_by_name(&req, "h").data {
            MetricData::Histogram { data_points, .. } => {
                assert_eq!(data_points[0].bucket_counts, vec!["1", "0", "1"]);
                assert_eq!(data_points[0].explicit_bounds, vec![0.1, 1.0]);
            }
            _ => panic!("expected a histogram"),
        }
        match &metric_by_name(&req, "s_empty").data {
            MetricData::Summary { data_points } => {
                assert!(data_points[0].quantile_values.is_empty());
            }
            _ => panic!("expected a summary"),
        }
        // A second delta export only carries the new observation.
        h.observe(0.5);
        let req2 = exporter.build(&r);
        match &metric_by_name(&req2, "h").data {
            MetricData::Histogram { data_points, .. } => {
                assert_eq!(data_points[0].count, "1");
                assert_eq!(data_points[0].bucket_counts, vec!["0", "1", "0"]);
            }
            _ => panic!("expected a histogram"),
        }
        let body = exporter.render_protobuf(&r);
        let rm = fields(&bytes(&fields(&body), 1)[0]);
        let sm = fields(&bytes(&rm, 2)[0]);
        assert!(bytes(&sm, 2).len() >= 7);
        assert!(
            serde_json::to_string(&req2)
                .unwrap()
                .contains("explicitBounds")
        );
    }
}
