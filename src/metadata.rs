//! Per-metric metadata: help text + unit.
//!
//! Exporters use this metadata to emit `# HELP` / `# TYPE` lines (Prometheus,
//! OpenMetrics), `attributes` (OTLP), and unit suffixes (StatsD). It is
//! optional — every metric registered through `MetricsCore::counter(name)` /
//! `gauge(name)` / `timer(name)` / `rate(name)` / `counter_with(name, …)` /
//! etc. exports successfully without metadata; help text and units are
//! purely additive.
//!
//! Register metadata via [`crate::Registry::describe`] or the convenience
//! shorthands `Registry::describe_counter` / `describe_gauge` /
//! `describe_timer` / `describe_rate` / `describe_histogram`.

use std::borrow::Cow;

/// Metric kind tag stored alongside help/unit metadata.
///
/// Used by exporters to emit the correct `# TYPE` line and to choose a
/// rendering strategy (e.g., `counter` vs. `gauge` semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MetricKind {
    /// Monotonic counter (resets only on process restart / explicit reset).
    Counter,
    /// Arbitrary-direction gauge.
    Gauge,
    /// Timing distribution.
    Timer,
    /// Rate over a tumbling window.
    Rate,
    /// Histogram of observations bucketed by value.
    Histogram,
}

impl MetricKind {
    /// Lower-case Prometheus/OpenMetrics `# TYPE` token.
    #[inline]
    pub const fn as_prometheus_type(self) -> &'static str {
        match self {
            // `Timer` and `Rate` aren't first-class Prometheus types; we
            // export them as `histogram`-shaped and `gauge`-shaped
            // respectively in the Prometheus exporter, but the metadata
            // kind preserves the original semantic.
            MetricKind::Counter => "counter",
            MetricKind::Gauge => "gauge",
            MetricKind::Timer => "histogram",
            MetricKind::Rate => "gauge",
            MetricKind::Histogram => "histogram",
        }
    }
}

/// Unit of measurement for a metric value.
///
/// Exporters use the unit for two things:
/// 1. Emit OpenMetrics `# UNIT` lines and Prometheus name suffixes
///    (`_seconds`, `_bytes`, `_total`, …).
/// 2. Normalise values where appropriate (e.g., Timer values exported in
///    seconds rather than nanoseconds).
///
/// `Unit::Custom` carries a free-form static string for units not enumerated
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Unit {
    /// No unit / dimensionless quantity.
    #[default]
    None,
    /// SI seconds.
    Seconds,
    /// SI milliseconds.
    Milliseconds,
    /// SI microseconds.
    Microseconds,
    /// SI nanoseconds.
    Nanoseconds,
    /// Bytes (binary, 1024-scale used by the `MemoryGauge` helpers).
    Bytes,
    /// Kilobytes (1024 bytes).
    Kilobytes,
    /// Megabytes (1024² bytes).
    Megabytes,
    /// Gigabytes (1024³ bytes).
    Gigabytes,
    /// Percentage 0..=100.
    Percent,
    /// Ratio 0.0..=1.0.
    Ratio,
    /// Free-form unit name (e.g., `"requests"`, `"connections"`).
    Custom(&'static str),
}

impl Unit {
    /// Lower-case Prometheus/OpenMetrics unit name. Returns `""` for
    /// [`Unit::None`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Unit::None => "",
            Unit::Seconds => "seconds",
            Unit::Milliseconds => "milliseconds",
            Unit::Microseconds => "microseconds",
            Unit::Nanoseconds => "nanoseconds",
            Unit::Bytes => "bytes",
            Unit::Kilobytes => "kilobytes",
            Unit::Megabytes => "megabytes",
            Unit::Gigabytes => "gigabytes",
            Unit::Percent => "percent",
            Unit::Ratio => "ratio",
            Unit::Custom(s) => s,
        }
    }
}

/// Per-metric metadata stored in the [`crate::Registry`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetricMetadata {
    /// Free-form help text rendered as `# HELP` in Prometheus exports and
    /// `description` in OTLP exports.
    pub help: Cow<'static, str>,
    /// Unit of the metric value.
    pub unit: Unit,
    /// Declared metric kind (informational; exporters infer the actual
    /// shape from the metric type in the registry).
    pub kind: MetricKind,
}

impl MetricMetadata {
    /// Construct metadata with the given kind, help text, and unit.
    pub fn new(kind: MetricKind, help: impl Into<Cow<'static, str>>, unit: Unit) -> Self {
        Self {
            kind,
            help: help.into(),
            unit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_strings_match_prometheus_conventions() {
        assert_eq!(Unit::Seconds.as_str(), "seconds");
        assert_eq!(Unit::Bytes.as_str(), "bytes");
        assert_eq!(Unit::None.as_str(), "");
        assert_eq!(Unit::Custom("foo").as_str(), "foo");
    }

    #[test]
    fn kind_prometheus_type_tokens_are_lowercase() {
        for k in [
            MetricKind::Counter,
            MetricKind::Gauge,
            MetricKind::Timer,
            MetricKind::Rate,
            MetricKind::Histogram,
        ] {
            let s = k.as_prometheus_type();
            assert!(s.chars().all(|c| c.is_ascii_lowercase()));
        }
    }
}
