//! Exporters that turn a [`Registry`](crate::Registry) into a wire format.
//!
//! | Exporter | Feature | Output |
//! |---|---|---|
//! | [`prometheus`] | always | Prometheus text format 0.0.4 |
//! | [`openmetrics`] | always | OpenMetrics 1.0 text format |
//! | `json` | `serde` or `json` | JSON snapshot of every series (`json` writes it without serde) |
//! | `statsd` | `statsd` | StatsD / `DogStatsD` lines over UDP |
//! | `otlp` | `otlp` or `otlp-json` | OTLP metrics as HTTP+JSON or protobuf (`otlp-json` writes the JSON without serde) |
//! | `http` | `http` | `/metrics` server and Pushgateway client |
//!
//! All exporters read the same validated registry, so no exporter has to
//! trust names or labels: names are `[A-Za-z0-9_.]` (the text formats render
//! `.` as `_`), label keys are `[A-Za-z0-9_]`, and every label value is
//! escaped or sanitised for the target format. The text formats additionally
//! guarantee one `# TYPE` per family and no sample-name clashes.

#[cfg(feature = "http")]
pub mod http;
#[cfg(any(feature = "serde", feature = "json"))]
pub mod json;
#[cfg(feature = "json")]
mod json_float;
pub mod openmetrics;
#[cfg(any(feature = "otlp", feature = "otlp-json"))]
pub mod otlp;
pub mod prometheus;
#[cfg(feature = "statsd")]
pub mod statsd;
mod text;
