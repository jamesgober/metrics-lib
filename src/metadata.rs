//! Per-metric metadata: kind, help text and unit.
//!
//! Exporters use metadata to emit `# HELP` / `# TYPE` / `# UNIT` lines
//! (Prometheus, OpenMetrics) and the `description` / `unit` fields (OTLP).
//! Metadata is optional: every registered metric exports without it.
//!
//! Register it with [`Registry::describe`](crate::Registry::describe) or the
//! kind-specific shorthands (`describe_counter`, `describe_gauge`, …). The
//! registry checks that the described kind matches the kind actually
//! registered under the name, so metadata can never mislabel a series.

use std::borrow::Cow;
use std::fmt;

/// The kind of a metric family.
///
/// One name maps to one kind for the lifetime of the registry entry (see
/// [`MetricsError::KindMismatch`](crate::MetricsError::KindMismatch)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum MetricKind {
    /// Monotonic counter ([`Counter`](crate::Counter) or
    /// [`ShardedCounter`](crate::ShardedCounter)).
    Counter,
    /// Signed counter that may go up or down
    /// ([`UpDownCounter`](crate::UpDownCounter)).
    UpDownCounter,
    /// Arbitrary-direction floating-point gauge.
    Gauge,
    /// Duration statistics (count, sum, min, max).
    Timer,
    /// Event rate over sliding windows.
    Rate,
    /// Bucketed distribution.
    Histogram,
    /// Distribution with streaming quantiles.
    Summary,
}

impl MetricKind {
    /// The Prometheus / OpenMetrics `# TYPE` token for the *primary* family
    /// this kind exports as.
    ///
    /// Timers export as `summary` (count and sum, no quantiles), up/down
    /// counters as `gauge`, and rate meters as a `counter` of total events
    /// plus a separate `gauge` family for the current rate.
    #[inline]
    #[must_use]
    pub const fn as_prometheus_type(self) -> &'static str {
        match self {
            MetricKind::Counter | MetricKind::Rate => "counter",
            MetricKind::UpDownCounter | MetricKind::Gauge => "gauge",
            MetricKind::Timer | MetricKind::Summary => "summary",
            MetricKind::Histogram => "histogram",
        }
    }

    /// Lower-case human-readable name (`"counter"`, `"up/down counter"`, …).
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            MetricKind::Counter => "counter",
            MetricKind::UpDownCounter => "up/down counter",
            MetricKind::Gauge => "gauge",
            MetricKind::Timer => "timer",
            MetricKind::Rate => "rate meter",
            MetricKind::Histogram => "histogram",
            MetricKind::Summary => "summary",
        }
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Unit of measurement for a metric value.
///
/// Exporters use the unit for the OpenMetrics `# UNIT` line (only when the
/// family name ends in `_<unit>`, as the specification requires) and for the
/// OTLP `unit` field (as a UCUM code, see [`Unit::as_ucum`]). Values are never
/// rescaled: a gauge described in `Megabytes` exports the number you set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum Unit {
    /// No unit / dimensionless.
    #[default]
    None,
    /// Seconds.
    Seconds,
    /// Milliseconds.
    Milliseconds,
    /// Microseconds.
    Microseconds,
    /// Nanoseconds.
    Nanoseconds,
    /// Bytes.
    Bytes,
    /// Kilobytes (1024 bytes).
    Kilobytes,
    /// Megabytes (1024² bytes).
    Megabytes,
    /// Gigabytes (1024³ bytes).
    Gigabytes,
    /// Percentage, 0..=100.
    Percent,
    /// Ratio, 0.0..=1.0.
    Ratio,
    /// Free-form unit (for example `"requests"`).
    ///
    /// Only `[A-Za-z0-9_]` characters survive into the OpenMetrics `# UNIT`
    /// line; anything else suppresses that line.
    Custom(&'static str),
}

impl Unit {
    /// Lower-case unit name as used in Prometheus/OpenMetrics names
    /// (`"seconds"`, `"bytes"`, …). Returns `""` for [`Unit::None`].
    #[must_use]
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

    /// The UCUM code OTLP expects in `Metric.unit` (`"s"`, `"By"`, `"%"`,
    /// `"1"`, …). Custom units are passed through unchanged, wrapped in curly
    /// braces as UCUM annotations (`"{requests}"`).
    #[must_use]
    pub fn as_ucum(self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Unit::None => "",
            Unit::Seconds => "s",
            Unit::Milliseconds => "ms",
            Unit::Microseconds => "us",
            Unit::Nanoseconds => "ns",
            Unit::Bytes => "By",
            Unit::Kilobytes => "KiBy",
            Unit::Megabytes => "MiBy",
            Unit::Gigabytes => "GiBy",
            Unit::Percent => "%",
            Unit::Ratio => "1",
            Unit::Custom(s) => return Cow::Owned(format!("{{{s}}}")),
        })
    }
}

/// Help text, unit and kind for one metric family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct MetricMetadata {
    /// Free-form description, rendered as `# HELP` (Prometheus, OpenMetrics)
    /// or `description` (OTLP).
    pub help: Cow<'static, str>,
    /// Unit of the metric value.
    pub unit: Unit,
    /// Kind of the family.
    pub kind: MetricKind,
}

impl MetricMetadata {
    /// Construct metadata with the given kind, help text and unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{MetricKind, MetricMetadata, Unit};
    ///
    /// let meta = MetricMetadata::new(MetricKind::Counter, "Requests served", Unit::None);
    /// assert_eq!(meta.kind, MetricKind::Counter);
    /// ```
    #[must_use]
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

    const ALL_KINDS: [MetricKind; 7] = [
        MetricKind::Counter,
        MetricKind::UpDownCounter,
        MetricKind::Gauge,
        MetricKind::Timer,
        MetricKind::Rate,
        MetricKind::Histogram,
        MetricKind::Summary,
    ];

    #[test]
    fn test_unit_strings_match_prometheus_conventions() {
        assert_eq!(Unit::Seconds.as_str(), "seconds");
        assert_eq!(Unit::Bytes.as_str(), "bytes");
        assert_eq!(Unit::None.as_str(), "");
        assert_eq!(Unit::Custom("foo").as_str(), "foo");
    }

    #[test]
    fn test_unit_ucum_codes() {
        assert_eq!(Unit::Seconds.as_ucum(), "s");
        assert_eq!(Unit::Bytes.as_ucum(), "By");
        assert_eq!(Unit::Ratio.as_ucum(), "1");
        assert_eq!(Unit::Custom("requests").as_ucum(), "{requests}");
        for unit in [
            Unit::None,
            Unit::Milliseconds,
            Unit::Microseconds,
            Unit::Nanoseconds,
            Unit::Kilobytes,
            Unit::Megabytes,
            Unit::Gigabytes,
            Unit::Percent,
        ] {
            let _ = unit.as_ucum();
            let _ = unit.as_str();
        }
    }

    #[test]
    fn test_kind_tokens_are_valid_prometheus_types() {
        for k in ALL_KINDS {
            assert!(matches!(
                k.as_prometheus_type(),
                "counter" | "gauge" | "summary" | "histogram"
            ));
            assert_eq!(k.to_string(), k.as_str());
        }
    }
}
