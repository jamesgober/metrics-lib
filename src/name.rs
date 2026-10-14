//! Metric-name and label validation.
//!
//! Every name and label that reaches an exporter goes through this module
//! first, so no exporter ever has to trust caller input. The rules are the
//! intersection of what Prometheus, OpenMetrics, StatsD and OTLP accept, plus
//! `.` (common in StatsD and OTLP names), which the Prometheus and
//! OpenMetrics exporters render as `_`. A leading `_` is refused because
//! OpenMetrics reserves it, and `:` is refused because Prometheus reserves it
//! for recording rules.
//!
//! | Item | Rule |
//! |---|---|
//! | metric name | 1..=[`MAX_NAME_LEN`] bytes of `[A-Za-z0-9_.]`, starting with a letter |
//! | label key | 1..=[`MAX_LABEL_KEY_LEN`] bytes of `[A-Za-z0-9_]`, starting with a letter |
//! | label value | any UTF-8, at most [`MAX_LABEL_VALUE_LEN`] bytes |
//! | label count | at most [`MAX_LABELS`] pairs per set |
//!
//! The checked registry methods (`try_*`) reject invalid input with
//! [`MetricsError::InvalidName`] / [`MetricsError::InvalidLabel`]. The
//! unchecked methods repair it with [`sanitize_metric_name`] and
//! [`sanitize_label_key`] so a typo can never take down the hot path, and
//! count each repair in [`Registry::invalid_inputs`](crate::Registry::invalid_inputs).

use crate::{MetricsError, Result};
use std::borrow::Cow;

/// Maximum length of a metric name, in bytes.
pub const MAX_NAME_LEN: usize = 200;
/// Maximum length of a label key, in bytes.
pub const MAX_LABEL_KEY_LEN: usize = 100;
/// Maximum length of a label value, in bytes.
pub const MAX_LABEL_VALUE_LEN: usize = 1024;
/// Maximum number of label pairs in one [`LabelSet`](crate::LabelSet).
pub const MAX_LABELS: usize = 32;

#[inline]
const fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.')
}

#[inline]
const fn is_key_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Check `name` against the metric-name rules.
///
/// # Errors
///
/// Returns [`MetricsError::InvalidName`] naming the first rule `name` breaks.
///
/// # Examples
///
/// ```
/// use metrics_lib::validate_metric_name;
///
/// assert!(validate_metric_name("http_requests").is_ok());
/// assert!(validate_metric_name("http.server.duration").is_ok());
/// assert!(validate_metric_name("1st").is_err());
/// assert!(validate_metric_name("bad name").is_err());
/// assert!(validate_metric_name("inj\n# TYPE x gauge").is_err());
/// ```
pub fn validate_metric_name(name: &str) -> Result<()> {
    let bytes = name.as_bytes();
    let Some(&first) = bytes.first() else {
        return Err(MetricsError::invalid_name(name, "name is empty"));
    };
    if bytes.len() > MAX_NAME_LEN {
        return Err(MetricsError::invalid_name(
            name,
            "name is longer than MAX_NAME_LEN bytes",
        ));
    }
    if !first.is_ascii_alphabetic() {
        return Err(MetricsError::invalid_name(
            name,
            "name must start with an ASCII letter",
        ));
    }
    if !bytes.iter().all(|&b| is_name_byte(b)) {
        return Err(MetricsError::invalid_name(
            name,
            "name may only contain [A-Za-z0-9_.]",
        ));
    }
    Ok(())
}

/// Check `key` against the label-key rules.
///
/// # Errors
///
/// Returns [`MetricsError::InvalidLabel`] naming the first rule `key` breaks.
///
/// # Examples
///
/// ```
/// use metrics_lib::validate_label_key;
///
/// assert!(validate_label_key("status_code").is_ok());
/// assert!(validate_label_key("bad\"key}").is_err());
/// assert!(validate_label_key("__name__").is_err());
/// ```
pub fn validate_label_key(key: &str) -> Result<()> {
    let bytes = key.as_bytes();
    let Some(&first) = bytes.first() else {
        return Err(MetricsError::invalid_label(key, "label key is empty"));
    };
    if bytes.len() > MAX_LABEL_KEY_LEN {
        return Err(MetricsError::invalid_label(
            key,
            "label key is longer than MAX_LABEL_KEY_LEN bytes",
        ));
    }
    if !first.is_ascii_alphabetic() {
        return Err(MetricsError::invalid_label(
            key,
            "label key must start with an ASCII letter",
        ));
    }
    if !bytes.iter().all(|&b| is_key_byte(b)) {
        return Err(MetricsError::invalid_label(
            key,
            "label key may only contain [A-Za-z0-9_]",
        ));
    }
    Ok(())
}

/// Check a label value against the length limit.
///
/// Any UTF-8 is accepted; exporters escape or replace the characters their
/// format reserves.
///
/// # Errors
///
/// Returns [`MetricsError::InvalidLabel`] when the value is longer than
/// [`MAX_LABEL_VALUE_LEN`] bytes. `key` is only used for the error message.
///
/// # Examples
///
/// ```
/// use metrics_lib::{validate_label_value, MAX_LABEL_VALUE_LEN};
///
/// assert!(validate_label_value("path", "/users/{id}").is_ok());
/// assert!(validate_label_value("path", &"x".repeat(MAX_LABEL_VALUE_LEN + 1)).is_err());
/// ```
pub fn validate_label_value(key: &str, value: &str) -> Result<()> {
    if value.len() > MAX_LABEL_VALUE_LEN {
        return Err(MetricsError::invalid_label(
            key,
            "label value is longer than MAX_LABEL_VALUE_LEN bytes",
        ));
    }
    Ok(())
}

/// Repair `name` so it satisfies [`validate_metric_name`].
///
/// Every character outside `[A-Za-z0-9_.]` becomes `_`, a name that does not
/// start with a letter gets an `m_` prefix (so an empty name becomes `m_`),
/// and the result is truncated to [`MAX_NAME_LEN`] bytes. Valid names are
/// returned unchanged without allocating.
///
/// # Examples
///
/// ```
/// use metrics_lib::sanitize_metric_name;
///
/// assert_eq!(sanitize_metric_name("http_requests"), "http_requests");
/// assert_eq!(sanitize_metric_name("http-requests"), "http_requests");
/// assert_eq!(sanitize_metric_name("9lives"), "m_9lives");
/// assert_eq!(sanitize_metric_name("a\nb"), "a_b");
/// ```
#[must_use]
pub fn sanitize_metric_name(name: &str) -> Cow<'_, str> {
    if validate_metric_name(name).is_ok() {
        return Cow::Borrowed(name);
    }
    Cow::Owned(sanitize_with(name, MAX_NAME_LEN, "m_", is_name_byte))
}

/// Repair `key` so it satisfies [`validate_label_key`].
///
/// Same strategy as [`sanitize_metric_name`], with the label-key alphabet and
/// an `l_` prefix.
///
/// # Examples
///
/// ```
/// use metrics_lib::sanitize_label_key;
///
/// assert_eq!(sanitize_label_key("status"), "status");
/// assert_eq!(sanitize_label_key("bad\"key}"), "bad_key_");
/// assert_eq!(sanitize_label_key("_private"), "l__private");
/// ```
#[must_use]
pub fn sanitize_label_key(key: &str) -> Cow<'_, str> {
    if validate_label_key(key).is_ok() {
        return Cow::Borrowed(key);
    }
    Cow::Owned(sanitize_with(key, MAX_LABEL_KEY_LEN, "l_", is_key_byte))
}

/// Truncate `value` to [`MAX_LABEL_VALUE_LEN`] bytes on a character boundary.
#[must_use]
pub(crate) fn sanitize_label_value(value: &str) -> Cow<'_, str> {
    if value.len() <= MAX_LABEL_VALUE_LEN {
        return Cow::Borrowed(value);
    }
    let mut end = MAX_LABEL_VALUE_LEN;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    Cow::Owned(value[..end].to_owned())
}

fn sanitize_with(input: &str, max_len: usize, prefix: &str, allowed: fn(u8) -> bool) -> String {
    let mut out = String::with_capacity(input.len().min(max_len) + prefix.len());
    if !input
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphabetic)
    {
        out.push_str(prefix);
    }
    for ch in input.chars() {
        let keep = u8::try_from(ch).is_ok_and(allowed);
        out.push(if keep { ch } else { '_' });
    }
    // Every byte is ASCII at this point, so any index is a char boundary.
    out.truncate(max_len);
    out
}

/// Render `name` with every `.` replaced by `_`, as Prometheus and
/// OpenMetrics require. Borrows when there is nothing to replace.
pub(crate) fn prometheus_name(name: &str) -> Cow<'_, str> {
    if name.contains('.') {
        Cow::Owned(name.replace('.', "_"))
    } else {
        Cow::Borrowed(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_validate_metric_name_accepts_valid_names() {
        for name in ["a", "a_b", "http.server.duration", "x_1", "A9", "a__b"] {
            assert!(validate_metric_name(name).is_ok(), "{name}");
        }
    }

    #[test]
    fn test_validate_metric_name_rejects_each_rule() {
        let cases = [
            ("", "empty"),
            ("1abc", "start with an ASCII letter"),
            ("_leading", "start with an ASCII letter"),
            ("__reserved", "start with an ASCII letter"),
            (":colon", "start with an ASCII letter"),
            ("a:b", "may only contain"),
            ("has space", "may only contain"),
            ("quote\"", "may only contain"),
            ("brace}", "may only contain"),
            ("new\nline", "may only contain"),
            ("dash-ed", "may only contain"),
            ("pipe|", "may only contain"),
            ("hash#", "may only contain"),
            ("unicodé", "may only contain"),
        ];
        for (name, needle) in cases {
            let err = validate_metric_name(name).unwrap_err().to_string();
            assert!(err.contains(needle), "{name:?}: {err}");
        }
        let long = "a".repeat(MAX_NAME_LEN + 1);
        assert!(validate_metric_name(&long).is_err());
        assert!(validate_metric_name(&long[..MAX_NAME_LEN]).is_ok());
    }

    #[test]
    fn test_validate_label_key_rejects_dots_and_specials() {
        assert!(validate_label_key("ok_key").is_ok());
        for key in [
            "", "a.b", "a:b", "1a", "_x", "__x", "a b", "k=v", "k\"", "k}",
        ] {
            assert!(validate_label_key(key).is_err(), "{key:?}");
        }
        let long = "k".repeat(MAX_LABEL_KEY_LEN + 1);
        assert!(validate_label_key(&long).is_err());
    }

    #[test]
    fn test_validate_label_value_enforces_length() {
        assert!(validate_label_value("k", "any \"value\" \n is fine").is_ok());
        let long = "v".repeat(MAX_LABEL_VALUE_LEN + 1);
        assert!(validate_label_value("k", &long).is_err());
    }

    #[test]
    fn test_sanitize_metric_name_repairs_each_rule() {
        assert_eq!(sanitize_metric_name(""), "m_");
        assert_eq!(sanitize_metric_name("1a"), "m_1a");
        assert_eq!(sanitize_metric_name("__a"), "m___a");
        assert_eq!(sanitize_metric_name("a:b"), "a_b");
        assert_eq!(sanitize_metric_name("a b\"c}\n"), "a_b_c__");
        assert_eq!(sanitize_metric_name("é"), "m__");
        assert_eq!(sanitize_label_key(""), "l_");
        assert_eq!(sanitize_label_key("_k"), "l__k");
        assert!(matches!(sanitize_metric_name("ok"), Cow::Borrowed(_)));
        let dashes = "-".repeat(MAX_NAME_LEN * 2);
        assert_eq!(sanitize_metric_name(&dashes).len(), MAX_NAME_LEN);
    }

    #[test]
    fn test_sanitize_label_value_truncates_on_char_boundary() {
        let value = "é".repeat(MAX_LABEL_VALUE_LEN);
        let out = sanitize_label_value(&value);
        assert!(out.len() <= MAX_LABEL_VALUE_LEN);
        assert!(out.chars().all(|c| c == 'é'));
        assert!(matches!(sanitize_label_value("short"), Cow::Borrowed(_)));
    }

    #[test]
    fn test_prometheus_name_replaces_dots_only_when_needed() {
        assert_eq!(prometheus_name("a.b.c"), "a_b_c");
        assert!(matches!(prometheus_name("a_b"), Cow::Borrowed(_)));
    }

    proptest! {
        #[test]
        fn prop_sanitized_name_is_always_valid(input in ".{0,300}") {
            let out = sanitize_metric_name(&input);
            prop_assert!(validate_metric_name(&out).is_ok(), "{:?} -> {:?}", input, out);
        }

        #[test]
        fn prop_sanitized_key_is_always_valid(input in ".{0,200}") {
            let out = sanitize_label_key(&input);
            prop_assert!(validate_label_key(&out).is_ok(), "{:?} -> {:?}", input, out);
        }

        #[test]
        fn prop_valid_names_survive_sanitising_unchanged(name in "[A-Za-z][A-Za-z0-9_.]{0,50}") {
            prop_assert_eq!(sanitize_metric_name(&name), name.as_str());
        }

        #[test]
        fn prop_prometheus_name_of_valid_name_matches_prometheus_grammar(
            name in "[A-Za-z][A-Za-z0-9_.]{0,50}"
        ) {
            let out = prometheus_name(&name);
            let bytes = out.as_bytes();
            prop_assert!(bytes[0].is_ascii_alphabetic());
            prop_assert!(bytes.iter().all(|b| b.is_ascii_alphanumeric() || *b == b'_'));
        }
    }
}
