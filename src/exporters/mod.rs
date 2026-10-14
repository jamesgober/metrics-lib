//! Exporters that turn a [`Registry`](crate::Registry) into a wire format.
//!
//! | Exporter | Feature | Output |
//! |---|---|---|
//! | [`prometheus`] | always | Prometheus text format 0.0.4 |
//! | [`openmetrics`] | always | OpenMetrics 1.0 text format |
//! | `json` | `serde` | JSON snapshot of every series |
//! | `statsd` | `statsd` | StatsD / `DogStatsD` lines over UDP |
//! | `otlp` | `otlp` | OTLP metrics as HTTP+JSON or protobuf |
//! | `http` | `http` | `/metrics` server and Pushgateway client |
//!
//! All exporters read the same validated registry, so no exporter has to
//! trust names or labels: names are `[A-Za-z0-9_.]` (the text formats render
//! `.` as `_`), label keys are `[A-Za-z0-9_]`, and every label value is
//! escaped or sanitised for the target format. The text formats additionally
//! guarantee one `# TYPE` per family and no sample-name clashes.

#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "serde")]
pub mod json;
pub mod openmetrics;
#[cfg(feature = "otlp")]
pub mod otlp;
pub mod prometheus;
#[cfg(feature = "statsd")]
pub mod statsd;
mod text;
