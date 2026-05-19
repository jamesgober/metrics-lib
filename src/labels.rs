//! Labels (tags) for metric instances.
//!
//! A [`LabelSet`] is a sorted, deduplicated collection of `(key, value)` pairs
//! that distinguishes one metric *instance* from another sharing the same
//! name. Every exporter in [`crate::exporters`] renders labels in its native
//! format (`name{k="v",k="v"}` for Prometheus / OpenMetrics, `name|#k:v,k:v`
//! for StatsD, `attributes` for OTLP, JSON object for snapshots).
//!
//! # Cardinality
//!
//! Labels are the most common source of unbounded metric growth — a single
//! per-request label value (`user_id`, `request_id`, `trace_id`, …) can blow
//! up the registry within seconds. [`Registry`](crate::Registry) enforces a
//! **per-registry hard cap** on unique `(name, labels)` combinations across
//! all labeled metrics; calls exceeding the cap return `Err(MetricsError::
//! CardinalityExceeded)` from the `try_*` variants and route to a shared
//! `__cardinality_overflow__` sink from the non-`try` variants so the hot
//! path stays panic-free.
//!
//! See `MetricsCore::counter_with`, `MetricsCore::gauge_with`,
//! `MetricsCore::timer_with`, `MetricsCore::rate_with`,
//! `MetricsCore::histogram_with` and their `try_*` counterparts for the
//! labeled lookup entry points.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{init, metrics, LabelSet};
//! init();
//!
//! // Build a label set in a single expression.
//! let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
//! metrics().counter_with("http_requests", &labels).inc();
//!
//! // …or build it incrementally.
//! let mut l = LabelSet::new();
//! l.add("tenant", "acme");
//! l.add("region", "us-east-1");
//! metrics().counter_with("auth_failures", &l).inc();
//! # }
//! ```

use std::borrow::Cow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A single label key/value pair.
///
/// Both halves are `Cow<'static, str>` so static literals stay zero-alloc
/// while runtime-derived values are supported transparently.
pub type Label = (Cow<'static, str>, Cow<'static, str>);

/// An ordered, deduplicated set of label key/value pairs.
///
/// `LabelSet` maintains its inner `Vec<Label>` sorted by key for stable
/// hashing and rendering: two semantically equivalent sets (same pairs in any
/// insertion order) compare equal, hash equal, and render identically.
///
/// Construction is allocation-free for an empty set ([`LabelSet::EMPTY`]) and
/// allocates the backing `Vec` on first insertion.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LabelSet {
    /// Sorted by key (binary-searched on insert).
    pairs: Vec<Label>,
}

impl LabelSet {
    /// The empty label set. Suitable for use as a `const` default.
    pub const EMPTY: LabelSet = LabelSet { pairs: Vec::new() };

    /// Construct a new, empty label set.
    #[inline]
    pub const fn new() -> Self {
        Self::EMPTY
    }

    /// Number of `(key, value)` pairs in the set.
    #[inline]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// `true` when the set has no pairs.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Iterate label pairs as `(&str, &str)` in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }

    /// Insert a `(key, value)` pair, replacing any existing value for `key`.
    ///
    /// Maintains sort order via `binary_search_by`. Returns `&mut self` for
    /// chaining.
    pub fn add(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        let key = key.into();
        let value = value.into();
        match self
            .pairs
            .binary_search_by(|(k, _)| (k.as_ref()).cmp(key.as_ref()))
        {
            Ok(idx) => self.pairs[idx].1 = value,
            Err(idx) => self.pairs.insert(idx, (key, value)),
        }
        self
    }

    /// Builder-style insert: consumes `self` and returns it with `(key,
    /// value)` added or replaced.
    #[must_use]
    pub fn with(
        mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.add(key, value);
        self
    }

    /// Lookup a label value by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.pairs
            .binary_search_by(|(k, _)| (k.as_ref()).cmp(key))
            .ok()
            .map(|idx| self.pairs[idx].1.as_ref())
    }

    /// Remove a label by key. Returns `true` if a pair was removed.
    pub fn remove(&mut self, key: &str) -> bool {
        if let Ok(idx) = self.pairs.binary_search_by(|(k, _)| (k.as_ref()).cmp(key)) {
            self.pairs.remove(idx);
            true
        } else {
            false
        }
    }

    /// Render labels in Prometheus / OpenMetrics format: `{k="v",k="v"}`.
    ///
    /// Returns an empty string for an empty set. Label values are escaped per
    /// the Prometheus exposition format: `\` → `\\`, `"` → `\"`, `\n` → `\n`.
    /// Label keys are emitted verbatim (Prometheus requires `[a-zA-Z_][a-zA-Z0-9_]*`);
    /// callers are responsible for choosing valid keys.
    pub fn to_prometheus(&self) -> String {
        if self.pairs.is_empty() {
            return String::new();
        }
        let mut out = String::with_capacity(2 + self.pairs.len() * 16);
        out.push('{');
        for (i, (k, v)) in self.pairs.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(k);
            out.push_str("=\"");
            escape_prometheus_value(&mut out, v);
            out.push('"');
        }
        out.push('}');
        out
    }

    /// Render labels in StatsD DogStatsD format: `|#k:v,k:v`.
    ///
    /// Returns an empty string for an empty set. Values are sanitised by
    /// replacing `|`, `,`, `\n`, and `:` with `_` (the format has no escape
    /// sequence; sanitisation is the standard practice).
    pub fn to_statsd(&self) -> String {
        if self.pairs.is_empty() {
            return String::new();
        }
        let mut out = String::with_capacity(2 + self.pairs.len() * 16);
        out.push_str("|#");
        for (i, (k, v)) in self.pairs.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(k);
            out.push(':');
            for c in v.chars() {
                if matches!(c, '|' | ',' | '\n' | ':') {
                    out.push('_');
                } else {
                    out.push(c);
                }
            }
        }
        out
    }
}

impl Hash for LabelSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the length explicitly so [("a","b")] doesn't collide with
        // [("a","b"), ()] etc.
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
        let mut s = Self::new();
        for (k, v) in iter {
            s.add(k, v);
        }
        s
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
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.pairs.len()))?;
        for (k, v) in &self.pairs {
            map.serialize_entry(k.as_ref(), v.as_ref())?;
        }
        map.end()
    }
}

fn escape_prometheus_value(out: &mut String, v: &str) {
    for c in v.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_is_empty_and_renders_empty() {
        let l = LabelSet::EMPTY;
        assert!(l.is_empty());
        assert_eq!(l.len(), 0);
        assert_eq!(l.to_prometheus(), "");
        assert_eq!(l.to_statsd(), "");
    }

    #[test]
    fn add_keeps_sorted_and_deduplicates() {
        let mut l = LabelSet::new();
        l.add("status", "200");
        l.add("method", "GET");
        l.add("region", "us");
        // Insertion order: status, method, region.
        // Sorted order:    method, region, status.
        let pairs: Vec<_> = l.iter().collect();
        assert_eq!(
            pairs,
            vec![("method", "GET"), ("region", "us"), ("status", "200")]
        );

        // Replacing a key updates the value in place.
        l.add("status", "500");
        assert_eq!(l.get("status"), Some("500"));
        assert_eq!(l.len(), 3);
    }

    #[test]
    fn equal_sets_hash_equal_regardless_of_insertion_order() {
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
    fn remove_keeps_invariants() {
        let mut l = LabelSet::from([("a", "1"), ("b", "2"), ("c", "3")]);
        assert!(l.remove("b"));
        assert!(!l.remove("b"));
        assert_eq!(l.iter().collect::<Vec<_>>(), vec![("a", "1"), ("c", "3")]);
    }

    #[test]
    fn prometheus_rendering_escapes_correctly() {
        let l = LabelSet::from([("path", r#"/foo "bar"\baz"#), ("note", "line1\nline2")]);
        let s = l.to_prometheus();
        // Sorted alphabetically: note, path.
        assert_eq!(s, r#"{note="line1\nline2",path="/foo \"bar\"\\baz"}"#);
    }

    #[test]
    fn statsd_rendering_sanitises_specials() {
        let l = LabelSet::from([("k1", "with|pipe"), ("k2", "with,comma:colon")]);
        let s = l.to_statsd();
        assert_eq!(s, "|#k1:with_pipe,k2:with_comma_colon");
    }

    #[test]
    fn ordering_is_lexicographic_over_pairs() {
        let a = LabelSet::from([("a", "1")]);
        let b = LabelSet::from([("a", "2")]);
        let c = LabelSet::from([("b", "0")]);
        let mut v = vec![c.clone(), b.clone(), a.clone()];
        v.sort();
        assert_eq!(v, vec![a, b, c]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_map() {
        let l = LabelSet::from([("method", "GET"), ("status", "200")]);
        let j = serde_json::to_string(&l).unwrap();
        assert_eq!(j, r#"{"method":"GET","status":"200"}"#);
    }
}
