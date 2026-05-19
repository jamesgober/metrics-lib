//! Built-in exporters for emitting metrics to popular telemetry backends.
//!
//! Each exporter renders the current state of a [`crate::Registry`] (plus the
//! associated [`crate::MetricMetadata`]) into a backend-specific format.
//! Exporters are stateless: callers control sampling cadence, transport, and
//! buffer reuse.
//!
//! | Exporter | Feature flag | Output |
//! |---|---|---|
//! | [`prometheus`] | (always on) | Prometheus text exposition format (`String`) |
//! | [`openmetrics`] | (always on) | OpenMetrics text format (`String`) |
//! | [`json`] | `serde` | `serde_json::Value` snapshot |
//! | [`statsd`] | `statsd` | UDP push, individual lines per metric |
//! | [`otlp`] | `otlp` | OTLP/HTTP+JSON payload (`String`) |
//!
//! All exporters share a render policy:
//! - Metric **names** are emitted verbatim (callers are expected to choose
//!   identifiers that satisfy the backend's naming rules — most often
//!   `[a-zA-Z_:][a-zA-Z0-9_:]*` for Prometheus / OpenMetrics).
//! - Metric **values** are escaped per the backend's rules.
//! - Labels are rendered via [`crate::LabelSet`]'s per-backend helpers.
//! - When metadata is missing for a metric, exporters emit reasonable
//!   defaults (kind inferred from the metric type; empty help; no unit).

pub mod openmetrics;
pub mod prometheus;

#[cfg(feature = "serde")]
pub mod json;

#[cfg(feature = "statsd")]
pub mod statsd;

#[cfg(feature = "otlp")]
pub mod otlp;
