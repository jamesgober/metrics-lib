//! Labels (tags) that distinguish series sharing one metric name.
//!
//! A [`LabelSet`] is a sorted, de-duplicated collection of `(key, value)`
//! pairs. Two sets holding the same pairs compare equal, hash equal and
//! render identically, whatever order the pairs were added in.
//!
//! # Validation
//!
//! [`LabelSet::try_add`] checks keys and values against the rules in
//! [`validate_label_key`](crate::validate_label_key) and
//! [`validate_label_value`](crate::validate_label_value). [`LabelSet::add`]
//! accepts anything; the registry repairs invalid keys and over-long values
//! when such a set is first registered (see
//! [`Registry::invalid_inputs`](crate::Registry::invalid_inputs)), so an
//! exporter never sees an invalid label.
//!
//! # Cardinality
//!
//! Labels are the most common source of unbounded metric growth: one
//! per-request value (`user_id`, `request_id`, …) can create a new series on
//! every request. The [`Registry`](crate::Registry) caps the total number of
//! series. Past the cap, the `try_*` lookups return
//! [`MetricsError::CardinalityExceeded`](crate::MetricsError::CardinalityExceeded)
//! and the other lookups hand out a detached overflow metric that is never
//! exported, so the hot path stays panic-free.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{LabelSet, MetricsCore};
//!
//! let core = MetricsCore::new();
//!
//! // Build a label set in one expression…
//! let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
//! core.counter_with("http_requests", &labels).inc();
//!
//! // …or incrementally, with validation.
//! let mut tenant = LabelSet::new();
//! tenant.try_add("tenant", "acme").unwrap();
//! tenant.try_add("region", "us-east-1").unwrap();
//! core.counter_with("auth_failures", &tenant).inc();
//! assert!(tenant.clone().try_add("bad key", "x").is_err());
//! # }
//! ```

use crate::name::{MAX_LABELS, sanitize_label_key, validate_label_key, validate_label_value};
use crate::{MetricsError, Result};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A single label key/value pair.
///
/// Both halves are `Cow<'static, str>`, so string literals cost no
/// allocation and runtime values are supported transparently.
pub type Label = (Cow<'static, str>, Cow<'static, str>);

/// An ordered, de-duplicated set of label key/value pairs.
///
/// The pairs are kept sorted by key, so equality, hashing and rendering are
/// independent of insertion order. The empty set ([`LabelSet::EMPTY`]) does
/// not allocate.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LabelSet {
    /// Sorted by key (binary-searched on insert).
    pairs: Vec<Label>,
}

impl LabelSet {
    /// The empty label set. Usable in `const` contexts.
    pub const EMPTY: LabelSet = LabelSet { pairs: Vec::new() };

    /// Construct a new, empty label set.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::EMPTY
    }

    /// Number of `(key, value)` pairs in the set.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// `true` when the set has no pairs.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Iterate the pairs as `(&str, &str)`, sorted by key.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }

    /// Insert a pair, replacing any existing value for `key`.
    ///
    /// No validation happens here; see [`LabelSet::try_add`]. Returns
    /// `&mut self` for chaining.
    pub fn add(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        let key = key.into();
        let value = value.into();
        match self
            .pairs
            .binary_search_by(|(k, _)| k.as_ref().cmp(key.as_ref()))
        {
            Ok(idx) => self.pairs[idx].1 = value,
            Err(idx) => self.pairs.insert(idx, (key, value)),
        }
        self
    }

    /// Insert a pair after validating it.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidLabel`] when the key breaks the
    /// label-key rules, the value is longer than
    /// [`MAX_LABEL_VALUE_LEN`](crate::MAX_LABEL_VALUE_LEN) bytes, or adding a
    /// new key would exceed [`MAX_LABELS`](crate::MAX_LABELS) pairs. The set
    /// is unchanged on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::LabelSet;
    ///
    /// let mut labels = LabelSet::new();
    /// labels.try_add("route", "/users/{id}").unwrap();
    /// assert!(labels.try_add("route\n", "x").is_err());
    /// assert_eq!(labels.len(), 1);
    /// ```
    pub fn try_add(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Result<&mut Self> {
        let key = key.into();
        let value = value.into();
        validate_label_key(&key)?;
        validate_label_value(&key, &value)?;
        if self.pairs.len() >= MAX_LABELS && !self.contains_key(&key) {
            return Err(MetricsError::invalid_label(
                &key,
                "label set already holds MAX_LABELS pairs",
            ));
        }
        Ok(self.add(key, value))
    }

    /// Builder-style insert: consumes `self` and returns it with the pair
    /// added or replaced. No validation; see [`LabelSet::try_add`].
    #[must_use]
    pub fn with(
        mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        let _ = self.add(key, value);
        self
    }

    /// Look up the value stored for `key`.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.pairs
            .binary_search_by(|(k, _)| k.as_ref().cmp(key))
            .ok()
            .map(|idx| self.pairs[idx].1.as_ref())
    }

    /// `true` when the set holds a pair for `key`.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Remove the pair for `key`. Returns `true` if a pair was removed.
    pub fn remove(&mut self, key: &str) -> bool {
        if let Ok(idx) = self.pairs.binary_search_by(|(k, _)| k.as_ref().cmp(key)) {
            let _ = self.pairs.remove(idx);
            true
        } else {
            false
        }
    }

    /// Check every pair against the label rules.
    ///
    /// # Errors
    ///
    /// Returns the first [`MetricsError::InvalidLabel`] found, or one for the
    /// set itself when it holds more than [`MAX_LABELS`](crate::MAX_LABELS)
    /// pairs.
    pub fn validate(&self) -> Result<()> {
        if self.pairs.len() > MAX_LABELS {
            return Err(MetricsError::invalid_label(
                "",
                "label set holds more than MAX_LABELS pairs",
            ));
        }
        for (k, v) in &self.pairs {
            validate_label_key(k)?;
            validate_label_value(k, v)?;
        }
        Ok(())
    }

    /// Return a copy of this set that passes [`LabelSet::validate`].
    ///
    /// Invalid keys are repaired with
    /// [`sanitize_label_key`](crate::sanitize_label_key), over-long values
    /// are truncated on a character boundary, and pairs past
    /// [`MAX_LABELS`](crate::MAX_LABELS) are dropped. When repairing makes two
    /// keys equal, the later value wins. Borrows when the set is already
    /// valid.
    #[must_use]
    pub fn sanitized(&self) -> Cow<'_, LabelSet> {
        if self.validate().is_ok() {
            return Cow::Borrowed(self);
        }
        let mut out = LabelSet::new();
        for (k, v) in self.pairs.iter().take(MAX_LABELS) {
            let key = sanitize_label_key(k).into_owned();
            let value = crate::name::sanitize_label_value(v).into_owned();
            let _ = out.add(key, value);
        }
        Cow::Owned(out)
    }

    /// Render the set in Prometheus / OpenMetrics form: `{k="v",k="v"}`.
    ///
    /// Returns an empty string for an empty set. Values are escaped (`\` as
    /// `\\`, `"` as `\"`, line feed as `\n`); keys that break the label-key
    /// rules are repaired with
    /// [`sanitize_label_key`](crate::sanitize_label_key), so the output is
    /// always well-formed.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::LabelSet;
    ///
    /// let labels = LabelSet::from([("path", "/a\"b"), ("method", "GET")]);
    /// assert_eq!(labels.to_prometheus(), r#"{method="GET",path="/a\"b"}"#);
    /// ```
    #[must_use]
    pub fn to_prometheus(&self) -> String {
        let mut out = String::new();
        self.write_prometheus(&mut out, None);
        out
    }

    /// Append the Prometheus rendering to `out`, optionally followed by one
    /// extra pair (`le` / `quantile`) whose value is written verbatim.
    pub(crate) fn write_prometheus(&self, out: &mut String, extra: Option<(&str, &str)>) {
        if self.pairs.is_empty() && extra.is_none() {
            return;
        }
        out.push('{');
        let mut first = true;
        for (k, v) in &self.pairs {
            if !first {
                out.push(',');
            }
            first = false;
            out.push_str(&sanitize_label_key(k));
            out.push_str("=\"");
            escape_label_value(out, v);
            out.push('"');
        }
        if let Some((k, v)) = extra {
            if !first {
                out.push(',');
            }
            out.push_str(k);
            out.push_str("=\"");
            out.push_str(v);
            out.push('"');
        }
        out.push('}');
    }

    /// Render the set as `DogStatsD` tags: `|#k:v,k:v`.
    ///
    /// Returns an empty string for an empty set. The format has no escape
    /// sequences, so every character that would end a tag, a line or the
    /// metric (`|`, `,`, `:`, `#`, `@`, line feed, carriage return) is replaced
    /// by `_`. Keys are repaired as in [`LabelSet::to_prometheus`].
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::LabelSet;
    ///
    /// let labels = LabelSet::from([("k", "a|b,c\nd")]);
    /// assert_eq!(labels.to_statsd(), "|#k:a_b_c_d");
    /// ```
    #[must_use]
    pub fn to_statsd(&self) -> String {
        let mut out = String::new();
        self.write_statsd(&mut out);
        out
    }

    pub(crate) fn write_statsd(&self, out: &mut String) {
        if self.pairs.is_empty() {
            return;
        }
        out.push_str("|#");
        for (i, (k, v)) in self.pairs.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&sanitize_label_key(k));
            out.push(':');
            push_statsd_safe(out, v);
        }
    }
}

/// Append `s` with every `StatsD`-reserved character replaced by `_`.
pub(crate) fn push_statsd_safe(out: &mut String, s: &str) {
    for c in s.chars() {
        if matches!(c, '|' | ',' | ':' | '#' | '@' | '\n' | '\r') {
            out.push('_');
        } else {
            out.push(c);
        }
    }
}

impl Hash for LabelSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pairs.len().hash(state);
        for (k, v) in &self.pairs {
            k.as_ref().hash(state);
            v.as_ref().hash(state);
        }
    }
}

impl PartialOrd for LabelSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LabelSet {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()));
        let b = other.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()));
        a.cmp(b)
    }
}

impl<K, V> FromIterator<(K, V)> for LabelSet
where
    K: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut set = Self::new();
        for (k, v) in iter {
            let _ = set.add(k, v);
        }
        set
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for LabelSet
where
    K: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LabelSet {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.pairs.len()))?;
        for (k, v) in &self.pairs {
            map.serialize_entry(k.as_ref(), v.as_ref())?;
        }
        map.end()
    }
}

/// Escape a label value (or `OpenMetrics` HELP text) for the text formats:
/// `\` as `\\`, `"` as `\"`, line feed as `\n`. The formats have no escape
/// for a carriage return and forbid it in the output, so it becomes a space.
pub(crate) fn escape_label_value(out: &mut String, v: &str) {
    if !v.bytes().any(|b| matches!(b, b'\\' | b'"' | b'\n' | b'\r')) {
        out.push_str(v);
        return;
    }
    for c in v.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push(' '),
            c => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_empty_set_renders_empty() {
        let l = LabelSet::EMPTY;
        assert!(l.is_empty());
        assert_eq!(l.len(), 0);
        assert_eq!(l.to_prometheus(), "");
        assert_eq!(l.to_statsd(), "");
    }

    #[test]
    fn test_add_keeps_sorted_and_deduplicates() {
        let mut l = LabelSet::new();
        let _ = l
            .add("status", "200")
            .add("method", "GET")
            .add("region", "us");
        let pairs: Vec<_> = l.iter().collect();
        assert_eq!(
            pairs,
            vec![("method", "GET"), ("region", "us"), ("status", "200")]
        );
        let _ = l.add("status", "500");
        assert_eq!(l.get("status"), Some("500"));
        assert_eq!(l.len(), 3);
        assert!(l.contains_key("method"));
        assert!(!l.contains_key("nope"));
    }

    #[test]
    fn test_equal_sets_hash_equal_regardless_of_insertion_order() {
        use std::collections::hash_map::DefaultHasher;
        fn hash(l: &LabelSet) -> u64 {
            let mut h = DefaultHasher::new();
            l.hash(&mut h);
            h.finish()
        }
        let a = LabelSet::from([("a", "1"), ("b", "2"), ("c", "3")]);
        let b = LabelSet::from([("c", "3"), ("a", "1"), ("b", "2")]);
        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));
    }

    #[test]
    fn test_remove_keeps_invariants() {
        let mut l = LabelSet::from([("a", "1"), ("b", "2"), ("c", "3")]);
        assert!(l.remove("b"));
        assert!(!l.remove("b"));
        assert_eq!(l.iter().collect::<Vec<_>>(), vec![("a", "1"), ("c", "3")]);
    }

    #[test]
    fn test_prometheus_rendering_escapes_values() {
        let l = LabelSet::from([("path", r#"/foo "bar"\baz"#), ("note", "line1\nline2")]);
        assert_eq!(
            l.to_prometheus(),
            r#"{note="line1\nline2",path="/foo \"bar\"\\baz"}"#
        );
    }

    #[test]
    fn test_prometheus_rendering_repairs_hostile_keys() {
        // Regression for ML-009: a key could inject a new exposition line.
        let l = LabelSet::from([("bad\"key}\n", "v")]);
        let out = l.to_prometheus();
        assert!(!out.contains('\n'));
        assert_eq!(out, r#"{bad_key__="v"}"#);
    }

    #[test]
    fn test_prometheus_rendering_appends_extra_pair_last() {
        let l = LabelSet::from([("a", "1")]);
        let mut out = String::new();
        l.write_prometheus(&mut out, Some(("le", "+Inf")));
        assert_eq!(out, r#"{a="1",le="+Inf"}"#);
        let mut only_extra = String::new();
        LabelSet::EMPTY.write_prometheus(&mut only_extra, Some(("quantile", "0.5")));
        assert_eq!(only_extra, r#"{quantile="0.5"}"#);
    }

    #[test]
    fn test_statsd_rendering_replaces_every_reserved_character() {
        let l = LabelSet::from([("k1", "a|b"), ("k2", "c,d:e#f@g\nh\ri")]);
        assert_eq!(l.to_statsd(), "|#k1:a_b,k2:c_d_e_f_g_h_i");
        let hostile = LabelSet::from([("bad\"key}\n", "v")]);
        assert!(!hostile.to_statsd().contains('\n'));
    }

    #[test]
    fn test_try_add_validates_key_value_and_count() {
        let mut l = LabelSet::new();
        assert!(l.try_add("ok", "v").is_ok());
        assert!(l.try_add("bad key", "v").is_err());
        assert!(
            l.try_add("big", "x".repeat(crate::MAX_LABEL_VALUE_LEN + 1))
                .is_err()
        );
        let mut full = LabelSet::new();
        for i in 0..MAX_LABELS {
            let _ = full.try_add(format!("k{i}"), "v").unwrap();
        }
        assert!(full.try_add("one_more", "v").is_err());
        // Replacing an existing key is fine even when full.
        assert!(full.try_add("k0", "v2").is_ok());
    }

    #[test]
    fn test_validate_and_sanitized_round_trip() {
        let valid = LabelSet::from([("a", "1")]);
        assert!(valid.validate().is_ok());
        assert!(matches!(valid.sanitized(), Cow::Borrowed(_)));

        let invalid = LabelSet::from([("bad key", "1".to_owned()), ("ok", "x".repeat(5000))]);
        assert!(invalid.validate().is_err());
        let fixed = invalid.sanitized().into_owned();
        assert!(fixed.validate().is_ok());
        assert_eq!(fixed.get("bad_key"), Some("1"));
        assert_eq!(
            fixed.get("ok").map(str::len),
            Some(crate::MAX_LABEL_VALUE_LEN)
        );

        let mut many = LabelSet::new();
        for i in 0..(MAX_LABELS + 5) {
            let _ = many.add(format!("k{i:02}"), "v");
        }
        assert!(many.validate().is_err());
        assert_eq!(many.sanitized().len(), MAX_LABELS);
    }

    #[test]
    fn test_ordering_is_lexicographic_over_pairs() {
        let a = LabelSet::from([("a", "1")]);
        let b = LabelSet::from([("a", "2")]);
        let c = LabelSet::from([("b", "0")]);
        let mut v = vec![c.clone(), b.clone(), a.clone()];
        v.sort();
        assert_eq!(v, vec![a, b, c]);
    }

    #[test]
    fn test_with_builder_adds_pairs() {
        let l = LabelSet::new().with("b", "2").with("a", "1");
        assert_eq!(l.iter().next(), Some(("a", "1")));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serializes_as_map() {
        let l = LabelSet::from([("method", "GET"), ("status", "200")]);
        let j = serde_json::to_string(&l).unwrap();
        assert_eq!(j, r#"{"method":"GET","status":"200"}"#);
    }

    proptest! {
        #[test]
        fn prop_sanitized_sets_always_validate(
            pairs in proptest::collection::vec((".{0,40}", ".{0,40}"), 0..50)
        ) {
            let set: LabelSet = pairs.into_iter().collect();
            prop_assert!(set.sanitized().validate().is_ok());
        }

        #[test]
        fn prop_rendering_never_breaks_lines(
            pairs in proptest::collection::vec((".{0,20}", ".{0,20}"), 0..8)
        ) {
            let set: LabelSet = pairs.into_iter().collect();
            prop_assert!(!set.to_prometheus().contains('\n'));
            prop_assert!(!set.to_statsd().contains('\n'));
        }
    }
}
