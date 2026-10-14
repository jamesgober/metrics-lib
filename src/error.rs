//! Error type shared by every fallible operation in the crate.

use crate::MetricKind;
use std::borrow::Cow;
use std::fmt;

/// Result alias used by every fallible operation in this crate.
pub type Result<T> = std::result::Result<T, MetricsError>;

/// Errors returned by the checked (`try_*`) operations and the registry.
///
/// The enum is `#[non_exhaustive]`: new variants may be added in minor
/// releases, so `match` statements need a wildcard arm.
///
/// Every variant says what was rejected and why. None of them carries a
/// metric *value*, so errors are safe to log.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetricsError {
    /// A metric name failed validation.
    ///
    /// Names must be 1..=[`MAX_NAME_LEN`](crate::MAX_NAME_LEN) bytes of
    /// `[A-Za-z0-9_:.]`, must not start with a digit and must not start with
    /// `__` (reserved by Prometheus). The non-`try` registry methods sanitise
    /// the name instead of returning this error; see
    /// [`sanitize_metric_name`](crate::sanitize_metric_name).
    ///
    /// **What to do:** fix the name at the call site. Names built from
    /// request data are almost always a bug; use labels instead.
    InvalidName {
        /// The rejected name, truncated to 64 bytes for display.
        name: String,
        /// Which rule the name broke.
        reason: &'static str,
    },
    /// A label key or value failed validation.
    ///
    /// Keys must be 1..=[`MAX_LABEL_KEY_LEN`](crate::MAX_LABEL_KEY_LEN) bytes
    /// of `[A-Za-z0-9_]`, must not start with a digit or with `__`. Values may
    /// hold any UTF-8 up to [`MAX_LABEL_VALUE_LEN`](crate::MAX_LABEL_VALUE_LEN)
    /// bytes. A set may hold at most [`MAX_LABELS`](crate::MAX_LABELS) pairs.
    ///
    /// **What to do:** fix the label at the call site.
    InvalidLabel {
        /// The offending key, truncated to 64 bytes for display.
        key: String,
        /// Which rule the label broke.
        reason: &'static str,
    },
    /// A value was rejected (NaN, infinite, out of range, empty bucket list…).
    ///
    /// **What to do:** validate the input before recording it, or use the
    /// non-`try` method, which ignores invalid values.
    InvalidValue {
        /// Short explanation of why the value was invalid.
        reason: &'static str,
    },
    /// The operation would overflow the metric's integer range.
    ///
    /// **What to do:** reset the metric, or use the wrapping/saturating
    /// variant if wrap-around is acceptable.
    Overflow,
    /// The operation would underflow the metric's integer range.
    ///
    /// Returned by the checked [`UpDownCounter`](crate::UpDownCounter)
    /// operations.
    Underflow,
    /// Not enough capacity right now (for example, a
    /// [`TokenBucket`](crate::TokenBucket) without enough tokens).
    ///
    /// **What to do:** retry later, shed the work, or queue it.
    WouldBlock,
    /// The circuit breaker is open and is rejecting calls.
    ///
    /// Returned by `MetricCircuitBreaker::check` (feature `sample`).
    ///
    /// **What to do:** fail fast; the breaker re-tests the dependency after
    /// its configured timeout.
    CircuitOpen,
    /// Registering this series would exceed the registry's series cap.
    ///
    /// The `try_*` lookups return this error; the non-`try` lookups hand out
    /// a detached overflow metric instead and bump
    /// [`Registry::cardinality_overflows`](crate::Registry::cardinality_overflows).
    ///
    /// **What to do:** reduce label cardinality, remove stale series with
    /// [`Registry::remove`](crate::Registry::remove) /
    /// [`Registry::evict_idle`](crate::Registry::evict_idle), or raise the cap.
    CardinalityExceeded,
    /// The name is already registered as a different kind of metric.
    ///
    /// One name maps to exactly one metric family. Registering `"jobs"` as a
    /// counter and then as a gauge would produce an invalid exposition, so
    /// the second registration is refused.
    ///
    /// **What to do:** pick a distinct name for the second metric.
    KindMismatch {
        /// The contested name.
        name: String,
        /// The kind already registered under `name`.
        existing: MetricKind,
        /// The kind the caller asked for.
        requested: MetricKind,
    },
    /// The global instance was already initialised, so the supplied
    /// configuration could not be applied.
    ///
    /// **What to do:** call [`init_with_config`](crate::init_with_config)
    /// once, before any call to [`metrics`](crate::metrics).
    AlreadyInitialized,
    /// A configuration value was rejected.
    ///
    /// **What to do:** read the message; it names the offending setting.
    Config(Cow<'static, str>),
}

impl MetricsError {
    /// Build an [`MetricsError::InvalidName`] with a display-safe copy of
    /// `name` (truncated to 64 bytes on a character boundary).
    pub(crate) fn invalid_name(name: &str, reason: &'static str) -> Self {
        Self::InvalidName {
            name: truncate_for_display(name),
            reason,
        }
    }

    /// Build an [`MetricsError::InvalidLabel`] with a display-safe copy of
    /// `key`.
    pub(crate) fn invalid_label(key: &str, reason: &'static str) -> Self {
        Self::InvalidLabel {
            key: truncate_for_display(key),
            reason,
        }
    }
}

fn truncate_for_display(s: &str) -> String {
    const LIMIT: usize = 64;
    if s.len() <= LIMIT {
        return s.to_owned();
    }
    let mut end = LIMIT;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    let mut out = String::with_capacity(end + 3);
    out.push_str(&s[..end]);
    out.push_str("...");
    out
}

impl fmt::Display for MetricsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { name, reason } => {
                write!(f, "invalid metric name {name:?}: {reason}")
            }
            Self::InvalidLabel { key, reason } => {
                write!(f, "invalid label {key:?}: {reason}")
            }
            Self::InvalidValue { reason } => write!(f, "invalid value: {reason}"),
            Self::Overflow => f.write_str("operation would overflow"),
            Self::Underflow => f.write_str("operation would underflow"),
            Self::WouldBlock => f.write_str("operation would block: not enough capacity"),
            Self::CircuitOpen => f.write_str("circuit breaker is open"),
            Self::CardinalityExceeded => f.write_str("series cap exceeded"),
            Self::KindMismatch {
                name,
                existing,
                requested,
            } => write!(
                f,
                "metric {name:?} is already registered as a {existing}, cannot register it as a {requested}"
            ),
            Self::AlreadyInitialized => f.write_str("global metrics already initialised"),
            Self::Config(msg) => write!(f, "configuration error: {msg}"),
        }
    }
}

impl std::error::Error for MetricsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_names_every_variant() {
        let cases = [
            (
                MetricsError::invalid_name("bad name", "contains a space"),
                "invalid metric name",
            ),
            (
                MetricsError::invalid_label("k-1", "contains '-'"),
                "invalid label",
            ),
            (
                MetricsError::InvalidValue { reason: "NaN" },
                "invalid value",
            ),
            (MetricsError::Overflow, "overflow"),
            (MetricsError::Underflow, "underflow"),
            (MetricsError::WouldBlock, "would block"),
            (MetricsError::CircuitOpen, "circuit breaker"),
            (MetricsError::CardinalityExceeded, "series cap"),
            (
                MetricsError::KindMismatch {
                    name: "jobs".into(),
                    existing: MetricKind::Counter,
                    requested: MetricKind::Gauge,
                },
                "already registered as a counter",
            ),
            (MetricsError::AlreadyInitialized, "already initialised"),
            (MetricsError::Config("max_series".into()), "max_series"),
        ];
        for (err, needle) in cases {
            let text = err.to_string();
            assert!(text.contains(needle), "{text:?} should contain {needle:?}");
        }
    }

    #[test]
    fn test_invalid_name_truncates_long_names_on_char_boundary() {
        let long = "é".repeat(100);
        let MetricsError::InvalidName { name, .. } = MetricsError::invalid_name(&long, "too long")
        else {
            panic!("expected InvalidName");
        };
        assert!(name.len() <= 67);
        assert!(name.ends_with("..."));
    }

    #[test]
    fn test_error_is_std_error() {
        fn assert_error<E: std::error::Error + Send + Sync + 'static>() {}
        assert_error::<MetricsError>();
    }
}
