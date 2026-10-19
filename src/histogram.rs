//! Bucketed distribution with sum, count, exemplars and estimated quantiles.
//!
//! Prometheus / OpenMetrics semantics:
//!
//! - each bucket counts observations `<= upper_bound`;
//! - buckets are reported *cumulatively*, ending with an implicit `+Inf`
//!   bucket equal to the total count;
//! - `sum` and `count` are tracked for the `_sum` / `_count` series.
//!
//! Build one with [`Histogram::default_seconds`] (the standard Prometheus
//! latency buckets), [`Histogram::linear`] / [`Histogram::exponential`], or
//! [`Histogram::with_buckets`] / [`Histogram::try_with_buckets`] for explicit
//! bounds.
//!
//! # Concurrency
//!
//! [`Histogram::observe`] is lock-free: a binary search for the bucket,
//! two relaxed `fetch_add`s and one compare-and-swap loop on the `f64` sum.
//! Snapshots read the buckets before the total, so a snapshot taken during
//! concurrent writes is still monotonic (`le="+Inf"` is never below a lower
//! bucket).
//!
//! # Exemplars
//!
//! [`Histogram::observe_with_exemplar`] additionally stores the observation's
//! trace context (for example a trace or span id) in its bucket. The
//! OpenMetrics exporter renders the latest exemplar of each bucket. Exemplar
//! storage is allocated on first use; histograms that never see an exemplar
//! pay nothing for it.
//!
//! # Example
//!
//! ```
//! use metrics_lib::Histogram;
//!
//! let latency = Histogram::default_seconds();
//! latency.observe(0.012);
//! latency.observe(0.085);
//! latency.observe(0.250);
//! assert_eq!(latency.count(), 3);
//! assert!(latency.quantile(0.5) > 0.0);
//! ```

use crate::util::CasUpdate;
use crate::{LabelSet, MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError, TryLockError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Standard Prometheus latency buckets, in seconds.
///
/// The same bounds as `prometheus.DefBuckets` in the Go client, suitable for
/// HTTP, RPC and database latencies.
pub const DEFAULT_SECONDS_BUCKETS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

/// Upper bound on the number of explicit buckets in one histogram.
pub const MAX_HISTOGRAM_BUCKETS: usize = 1024;

/// Upper bound on the combined length (in characters) of an exemplar's label
/// names and values, as fixed by the OpenMetrics specification.
pub const MAX_EXEMPLAR_LABEL_CHARS: usize = 128;

/// A sample observation attached to a histogram bucket, carrying trace
/// context.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct Exemplar {
    /// Labels identifying the sample (for example `trace_id`).
    pub labels: LabelSet,
    /// The observed value.
    pub value: f64,
    /// When the observation was made, in seconds since the Unix epoch.
    pub timestamp: f64,
}

/// Lock-free bucketed histogram.
#[repr(align(64))]
pub struct Histogram {
    /// Explicit upper bounds, ascending. The `+Inf` bucket is implicit.
    bucket_bounds: Box<[f64]>,
    /// Non-cumulative count per explicit bucket.
    bucket_counts: Box<[AtomicU64]>,
    /// Sum of observations as `f64` bits.
    sum_bits: AtomicU64,
    /// Total observations (explicit buckets plus `+Inf`).
    total: AtomicU64,
    /// One exemplar slot per bucket including `+Inf`, allocated on first use.
    exemplars: OnceLock<Box<[Mutex<Option<Exemplar>>]>>,
    created_at: Instant,
}

/// Immutable snapshot of a [`Histogram`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct HistogramSnapshot {
    /// Cumulative buckets. The last one has `upper_bound = +Inf` and
    /// `count = self.count`.
    pub buckets: Vec<HistogramBucket>,
    /// Sum of observations.
    pub sum: f64,
    /// Total number of observations.
    pub count: u64,
    /// Time since the histogram was created.
    pub age: Duration,
    /// The most recent exemplar of each bucket, parallel to `buckets`.
    ///
    /// Empty (no allocation) when the histogram has never received an
    /// exemplar, so histograms without exemplars pay nothing for them.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub exemplars: Vec<Option<Exemplar>>,
}

impl HistogramSnapshot {
    /// The exemplar of bucket `index`, if one was recorded.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Histogram, LabelSet};
    ///
    /// let h = Histogram::with_buckets([0.1, 1.0]);
    /// h.observe_with_exemplar(0.05, &LabelSet::from([("trace_id", "4bf92f35")]));
    /// let snap = h.snapshot();
    /// assert_eq!(snap.exemplar(0).map(|e| e.value), Some(0.05));
    /// assert!(snap.exemplar(1).is_none());
    /// ```
    #[must_use]
    pub fn exemplar(&self, index: usize) -> Option<&Exemplar> {
        self.exemplars.get(index).and_then(Option::as_ref)
    }
}

/// One cumulative bucket of a [`HistogramSnapshot`].
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct HistogramBucket {
    /// Inclusive upper bound; `f64::INFINITY` for the last bucket. Serialised
    /// as the string `"+Inf"` in that case, since JSON has no infinity.
    pub upper_bound: f64,
    /// Number of observations `<= upper_bound`.
    pub count: u64,
}

#[cfg(feature = "serde")]
impl serde::Serialize for HistogramBucket {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("HistogramBucket", 2)?;
        if self.upper_bound.is_finite() {
            s.serialize_field("upper_bound", &self.upper_bound)?;
        } else {
            s.serialize_field("upper_bound", "+Inf")?;
        }
        s.serialize_field("count", &self.count)?;
        s.end()
    }
}

/// Sort, de-duplicate and validate user-supplied bucket bounds.
fn clean_bounds(bounds: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut cleaned: Vec<f64> = bounds.into_iter().filter(|b| b.is_finite()).collect();
    cleaned.sort_by(f64::total_cmp);
    cleaned.dedup_by(|a, b| a.to_bits() == b.to_bits() || (*a == 0.0 && *b == 0.0));
    cleaned.truncate(MAX_HISTOGRAM_BUCKETS);
    cleaned
}

impl Histogram {
    /// Build a histogram from explicit upper bounds.
    ///
    /// Non-finite bounds are dropped, the rest are sorted and de-duplicated,
    /// and at most [`MAX_HISTOGRAM_BUCKETS`] are kept (the smallest ones).
    /// When nothing is left, a single `1.0` bucket is used; call
    /// [`Histogram::try_with_buckets`] to get an error instead. The implicit
    /// `+Inf` bucket is always present.
    #[must_use]
    pub fn with_buckets(bounds: impl IntoIterator<Item = f64>) -> Self {
        let mut cleaned = clean_bounds(bounds);
        if cleaned.is_empty() {
            cleaned.push(1.0);
        }
        Self::from_clean(cleaned)
    }

    /// Like [`Histogram::with_buckets`] but refuses an empty bucket list.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] when no finite bound remains
    /// after cleaning.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::Histogram;
    ///
    /// assert!(Histogram::try_with_buckets([0.1, 1.0]).is_ok());
    /// assert!(Histogram::try_with_buckets([]).is_err());
    /// assert!(Histogram::try_with_buckets([f64::NAN]).is_err());
    /// ```
    pub fn try_with_buckets(bounds: impl IntoIterator<Item = f64>) -> Result<Self> {
        let cleaned = clean_bounds(bounds);
        if cleaned.is_empty() {
            return Err(MetricsError::InvalidValue {
                reason: "a histogram needs at least one finite bucket bound",
            });
        }
        Ok(Self::from_clean(cleaned))
    }

    /// Validate bucket bounds the way [`Histogram::try_with_buckets`] does and
    /// return them cleaned.
    pub(crate) fn validate_bounds(bounds: impl IntoIterator<Item = f64>) -> Result<Vec<f64>> {
        let cleaned = clean_bounds(bounds);
        if cleaned.is_empty() {
            return Err(MetricsError::InvalidValue {
                reason: "a histogram needs at least one finite bucket bound",
            });
        }
        Ok(cleaned)
    }

    fn from_clean(cleaned: Vec<f64>) -> Self {
        let bucket_counts: Box<[AtomicU64]> = cleaned.iter().map(|_| AtomicU64::new(0)).collect();
        Self {
            bucket_bounds: cleaned.into_boxed_slice(),
            bucket_counts,
            sum_bits: AtomicU64::new(0.0_f64.to_bits()),
            total: AtomicU64::new(0),
            exemplars: OnceLock::new(),
            created_at: Instant::now(),
        }
    }

    /// Histogram with [`DEFAULT_SECONDS_BUCKETS`].
    #[must_use]
    pub fn default_seconds() -> Self {
        Self::from_clean(DEFAULT_SECONDS_BUCKETS.to_vec())
    }

    /// Linear buckets: `start, start + width, …, start + (count - 1) * width`.
    ///
    /// Falls back to a single `1.0` bucket when `count == 0`, `width <= 0`,
    /// or an input is non-finite.
    #[must_use]
    pub fn linear(start: f64, width: f64, count: usize) -> Self {
        if count == 0 || !width.is_finite() || width <= 0.0 || !start.is_finite() {
            return Self::with_buckets([1.0]);
        }
        let count = count.min(MAX_HISTOGRAM_BUCKETS);
        let mut bound = start;
        let mut bounds = Vec::with_capacity(count);
        for _ in 0..count {
            bounds.push(bound);
            bound += width;
        }
        Self::with_buckets(bounds)
    }

    /// Exponential buckets: `start, start * factor, …, start * factor^(count - 1)`.
    ///
    /// Falls back to a single `1.0` bucket when `count == 0`, `start <= 0`,
    /// `factor <= 1`, or an input is non-finite.
    #[must_use]
    pub fn exponential(start: f64, factor: f64, count: usize) -> Self {
        if count == 0 || !start.is_finite() || !factor.is_finite() || start <= 0.0 || factor <= 1.0
        {
            return Self::with_buckets([1.0]);
        }
        let count = count.min(MAX_HISTOGRAM_BUCKETS);
        let mut bound = start;
        let mut bounds = Vec::with_capacity(count);
        for _ in 0..count {
            bounds.push(bound);
            bound *= factor;
        }
        Self::with_buckets(bounds)
    }

    /// The explicit upper bounds (without `+Inf`).
    #[must_use]
    pub fn bounds(&self) -> &[f64] {
        &self.bucket_bounds
    }

    /// Record one observation. NaN and ±∞ are ignored; use
    /// [`Histogram::try_observe`] to be told.
    #[inline]
    pub fn observe(&self, value: f64) {
        if value.is_finite() {
            let _ = self.observe_finite(value);
        }
    }

    /// Record a duration, in seconds.
    #[inline]
    pub fn observe_duration(&self, duration: Duration) {
        self.observe(duration.as_secs_f64());
    }

    /// Record one observation, rejecting NaN and ±∞.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] for a non-finite value; nothing
    /// is recorded.
    #[inline]
    pub fn try_observe(&self, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(MetricsError::InvalidValue {
                reason: "value is not finite",
            });
        }
        let _ = self.observe_finite(value);
        Ok(())
    }

    /// Record one observation and attach `labels` as its exemplar.
    ///
    /// The exemplar replaces the previous one of the same bucket. It is
    /// dropped (the observation is still recorded) when `labels` breaks the
    /// label rules or exceeds [`MAX_EXEMPLAR_LABEL_CHARS`], or when another
    /// thread is writing that bucket's exemplar at the same moment.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Histogram, LabelSet};
    ///
    /// let h = Histogram::with_buckets([0.1, 1.0]);
    /// h.observe_with_exemplar(0.05, &LabelSet::from([("trace_id", "4bf92f35")]));
    /// let snap = h.snapshot();
    /// assert_eq!(snap.exemplar(0).map(|e| e.value), Some(0.05));
    /// ```
    pub fn observe_with_exemplar(&self, value: f64, labels: &LabelSet) {
        let _ = self.try_observe_with_exemplar(value, labels);
    }

    /// Like [`Histogram::observe_with_exemplar`] but reports why an exemplar
    /// was not stored.
    ///
    /// # Errors
    ///
    /// - [`MetricsError::InvalidValue`] for a non-finite value (nothing is
    ///   recorded).
    /// - [`MetricsError::InvalidLabel`] when `labels` is invalid or too long
    ///   (the observation is recorded, the exemplar is not).
    /// - [`MetricsError::WouldBlock`] when another thread was storing an
    ///   exemplar in the same bucket (the observation is recorded).
    pub fn try_observe_with_exemplar(&self, value: f64, labels: &LabelSet) -> Result<()> {
        if !value.is_finite() {
            return Err(MetricsError::InvalidValue {
                reason: "value is not finite",
            });
        }
        let idx = self.observe_finite(value);
        validate_exemplar_labels(labels)?;
        let slots = self.exemplars.get_or_init(|| {
            (0..=self.bucket_bounds.len())
                .map(|_| Mutex::new(None))
                .collect()
        });
        let Some(slot) = slots.get(idx) else {
            return Ok(());
        };
        let exemplar = Exemplar {
            labels: labels.clone(),
            value,
            timestamp: unix_seconds_now(),
        };
        match slot.try_lock() {
            Ok(mut guard) => {
                *guard = Some(exemplar);
                Ok(())
            }
            Err(TryLockError::Poisoned(poisoned)) => {
                *poisoned.into_inner() = Some(exemplar);
                Ok(())
            }
            Err(TryLockError::WouldBlock) => Err(MetricsError::WouldBlock),
        }
    }

    /// Record a finite value and return its bucket index (`len` = `+Inf`).
    #[inline]
    fn observe_finite(&self, value: f64) -> usize {
        let idx = self.bucket_bounds.partition_point(|&b| b < value);
        if let Some(slot) = self.bucket_counts.get(idx) {
            let _ = slot.fetch_add(1, Ordering::Relaxed);
        }
        let _ = self.total.fetch_add(1, Ordering::Relaxed);
        let _ = self
            .sum_bits
            .cas_update(Ordering::Relaxed, Ordering::Relaxed, |bits| {
                let next = f64::from_bits(bits) + value;
                // Leave the sum untouched rather than store ±∞; the bucket
                // and count updates above still stand.
                next.is_finite().then(|| next.to_bits())
            });
        idx
    }

    /// Number of observations.
    #[inline]
    #[must_use]
    pub fn count(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }

    /// Sum of observations.
    #[inline]
    #[must_use]
    pub fn sum(&self) -> f64 {
        f64::from_bits(self.sum_bits.load(Ordering::Relaxed))
    }

    /// Mean observation, or `0.0` when empty.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // statistics: f64 precision is the contract
    pub fn mean(&self) -> f64 {
        match self.count() {
            0 => 0.0,
            c => self.sum() / c as f64,
        }
    }

    /// Time since the histogram was created.
    #[inline]
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Clear every bucket, the sum, the count and stored exemplars.
    pub fn reset(&self) {
        for b in &*self.bucket_counts {
            b.store(0, Ordering::Relaxed);
        }
        self.sum_bits.store(0.0_f64.to_bits(), Ordering::Relaxed);
        self.total.store(0, Ordering::Relaxed);
        if let Some(slots) = self.exemplars.get() {
            for slot in &**slots {
                *slot.lock().unwrap_or_else(PoisonError::into_inner) = None;
            }
        }
    }

    /// Capture a cumulative snapshot, including the latest exemplar of each
    /// bucket when exemplars have been recorded.
    ///
    /// Buckets are read before the total, and `count` is raised to the
    /// cumulative bucket sum if a concurrent writer had bumped a bucket but
    /// not yet the total, so the snapshot always satisfies
    /// `bucket[i] <= bucket[i + 1] <= count`.
    #[must_use]
    pub fn snapshot(&self) -> HistogramSnapshot {
        let mut snap = self.snapshot_counts();
        if let Some(slots) = self.exemplars.get() {
            snap.exemplars = slots
                .iter()
                .map(|slot| slot.lock().unwrap_or_else(PoisonError::into_inner).clone())
                .collect();
        }
        snap
    }

    /// Snapshot without exemplars, for exporters that cannot carry them.
    pub(crate) fn snapshot_counts(&self) -> HistogramSnapshot {
        let mut buckets = Vec::with_capacity(self.bucket_bounds.len() + 1);
        let mut cumulative: u64 = 0;
        // `extend` over zipped slices writes without per-element capacity
        // checks.
        buckets.extend(
            self.bucket_bounds
                .iter()
                .zip(self.bucket_counts.iter())
                .map(|(bound, counter)| {
                    cumulative = cumulative.saturating_add(counter.load(Ordering::Relaxed));
                    HistogramBucket {
                        upper_bound: *bound,
                        count: cumulative,
                    }
                }),
        );
        let count = self.count().max(cumulative);
        buckets.push(HistogramBucket {
            upper_bound: f64::INFINITY,
            count,
        });
        HistogramSnapshot {
            buckets,
            sum: self.sum(),
            count,
            age: self.age(),
            exemplars: Vec::new(),
        }
    }

    /// Estimate the `q`-quantile by linear interpolation inside buckets.
    ///
    /// `q` is clamped to `0.0..=1.0` (NaN is treated as `0.5`). Returns `0.0`
    /// when empty. The lower edge of the first bucket is taken as
    /// `min(0, first bound)`; a quantile falling into the `+Inf` bucket
    /// returns the largest explicit bound.
    #[must_use]
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )] // rank arithmetic on counts
    pub fn quantile(&self, q: f64) -> f64 {
        let q = if q.is_nan() { 0.5 } else { q.clamp(0.0, 1.0) };
        let count = self.count();
        if count == 0 {
            return 0.0;
        }
        let target = ((q * count as f64).ceil() as u64).max(1);
        let mut cumulative: u64 = 0;
        let mut lower = self.bucket_bounds.first().map_or(0.0, |b| b.min(0.0));
        for (bound, counter) in self.bucket_bounds.iter().zip(self.bucket_counts.iter()) {
            let in_bucket = counter.load(Ordering::Relaxed);
            let next = cumulative.saturating_add(in_bucket);
            if next >= target && in_bucket > 0 {
                let within = (target - cumulative) as f64 / in_bucket as f64;
                return lower + (bound - lower) * within;
            }
            cumulative = next;
            lower = *bound;
        }
        self.bucket_bounds.last().copied().unwrap_or(0.0)
    }
}

/// Check exemplar labels against the label rules and the OpenMetrics length
/// limit.
fn validate_exemplar_labels(labels: &LabelSet) -> Result<()> {
    labels.validate()?;
    let chars: usize = labels
        .iter()
        .map(|(k, v)| k.chars().count() + v.chars().count())
        .sum();
    if chars > MAX_EXEMPLAR_LABEL_CHARS {
        return Err(MetricsError::invalid_label(
            "",
            "exemplar labels exceed MAX_EXEMPLAR_LABEL_CHARS characters",
        ));
    }
    Ok(())
}

fn unix_seconds_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

impl Default for Histogram {
    fn default() -> Self {
        Self::default_seconds()
    }
}

impl std::fmt::Debug for Histogram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Histogram")
            .field("buckets", &(self.bucket_bounds.len() + 1))
            .field("count", &self.count())
            .field("sum", &self.sum())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact comparisons of exactly representable values
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, Barrier};

    #[test]
    fn test_histogram_observe_counts_and_sums() {
        let h = Histogram::with_buckets([0.1, 1.0, 10.0]);
        h.observe(0.5);
        h.observe(0.5);
        h.observe(5.0);
        assert_eq!(h.count(), 3);
        assert!((h.sum() - 6.0).abs() < 1e-9);
        assert!((h.mean() - 2.0).abs() < 1e-9);
        h.observe_duration(Duration::from_millis(500));
        assert_eq!(h.count(), 4);
    }

    #[test]
    fn test_histogram_cumulative_buckets() {
        let h = Histogram::with_buckets([1.0, 2.0, 3.0]);
        for v in [0.5, 1.5, 2.5, 10.0, 1.0] {
            h.observe(v);
        }
        let snap = h.snapshot();
        let counts: Vec<u64> = snap.buckets.iter().map(|b| b.count).collect();
        assert_eq!(counts, vec![2, 3, 4, 5]);
        assert!(snap.buckets[3].upper_bound.is_infinite());
        assert_eq!(snap.count, 5);
    }

    #[test]
    fn test_histogram_non_finite_inputs() {
        let h = Histogram::with_buckets([1.0]);
        assert!(h.try_observe(f64::NAN).is_err());
        assert!(h.try_observe(f64::INFINITY).is_err());
        h.observe(f64::NAN);
        h.observe(f64::NEG_INFINITY);
        h.observe(2.0);
        assert_eq!(h.count(), 1);
        assert!(h.try_observe(0.5).is_ok());
    }

    #[test]
    fn test_histogram_sum_overflow_keeps_finite_sum() {
        let h = Histogram::with_buckets([1.0]);
        h.observe(f64::MAX);
        h.observe(f64::MAX);
        assert_eq!(h.count(), 2);
        assert!(h.sum().is_finite());
    }

    #[test]
    fn test_histogram_empty_bounds_do_not_panic_regression_ml011() {
        // ML-011: `with_buckets([])` panicked in debug builds.
        let h = Histogram::with_buckets(std::iter::empty());
        assert_eq!(h.bounds(), &[1.0]);
        assert!(Histogram::try_with_buckets(std::iter::empty()).is_err());
        assert!(Histogram::try_with_buckets([f64::NAN, f64::INFINITY]).is_err());
        assert!(Histogram::validate_bounds([]).is_err());
        assert_eq!(
            Histogram::validate_bounds([2.0, 1.0, 2.0]).unwrap(),
            vec![1.0, 2.0]
        );
    }

    #[test]
    fn test_histogram_dedup_is_exact_not_epsilon() {
        let h = Histogram::with_buckets([1e-17, 2e-17, 0.0, -0.0]);
        assert_eq!(h.bounds().len(), 3);
    }

    #[test]
    fn test_histogram_bucket_count_is_capped() {
        let h = Histogram::with_buckets((0..5000).map(f64::from));
        assert_eq!(h.bounds().len(), MAX_HISTOGRAM_BUCKETS);
        let l = Histogram::linear(0.0, 1.0, 5000);
        assert_eq!(l.bounds().len(), MAX_HISTOGRAM_BUCKETS);
    }

    #[test]
    fn test_histogram_quantiles() {
        let h = Histogram::with_buckets([1.0, 2.0]);
        assert_eq!(h.quantile(0.5), 0.0);
        for _ in 0..10 {
            h.observe(0.5);
            h.observe(1.5);
        }
        let p50 = h.quantile(0.5);
        assert!(p50 > 0.0 && p50 <= 1.0);
        assert!((h.quantile(1.0) - 2.0).abs() < 1e-9);
        assert!(h.quantile(f64::NAN) > 0.0);
        assert!(h.quantile(0.0) > 0.0);
        h.observe(100.0);
        assert_eq!(h.quantile(1.0), 2.0);
    }

    #[test]
    fn test_histogram_quantile_with_negative_bounds() {
        let h = Histogram::with_buckets([-10.0, -5.0, 0.0]);
        for _ in 0..4 {
            h.observe(-7.0);
        }
        let q = h.quantile(0.5);
        assert!((-10.0..=-5.0).contains(&q), "{q}");
    }

    #[test]
    fn test_histogram_constructors() {
        let l = Histogram::linear(0.0, 0.5, 5);
        assert_eq!(l.bounds(), &[0.0, 0.5, 1.0, 1.5, 2.0]);
        let e = Histogram::exponential(0.1, 2.0, 4);
        assert_eq!(e.bounds().len(), 4);
        assert!((e.bounds()[3] - 0.8).abs() < 1e-12);
        for h in [
            Histogram::linear(0.0, -1.0, 5),
            Histogram::linear(0.0, 1.0, 0),
            Histogram::exponential(0.0, 2.0, 5),
            Histogram::exponential(1.0, 1.0, 5),
            Histogram::exponential(1.0, f64::NAN, 5),
        ] {
            assert_eq!(h.bounds(), &[1.0]);
        }
        assert_eq!(Histogram::default().bounds(), DEFAULT_SECONDS_BUCKETS);
    }

    #[test]
    fn test_histogram_reset_clears_state_and_exemplars() {
        let h = Histogram::with_buckets([1.0]);
        h.observe_with_exemplar(0.5, &LabelSet::from([("trace_id", "a")]));
        h.reset();
        assert_eq!(h.count(), 0);
        assert_eq!(h.sum(), 0.0);
        assert!(h.snapshot().exemplars.iter().all(Option::is_none));
        assert!(format!("{h:?}").contains("Histogram"));
    }

    #[test]
    fn test_histogram_exemplars_land_in_matching_bucket() {
        let h = Histogram::with_buckets([0.1, 1.0]);
        let trace = LabelSet::from([("trace_id", "abc")]);
        h.observe_with_exemplar(0.05, &trace);
        h.observe_with_exemplar(0.5, &trace);
        h.observe_with_exemplar(50.0, &trace);
        let snap = h.snapshot();
        let values: Vec<Option<f64>> = snap
            .exemplars
            .iter()
            .map(|e| e.as_ref().map(|e| e.value))
            .collect();
        assert_eq!(values, vec![Some(0.05), Some(0.5), Some(50.0)]);
        assert!(snap.exemplar(0).unwrap().timestamp > 0.0);
        // Exporters that drop exemplars never copy them.
        assert_eq!(h.snapshot_counts().exemplars.len(), 0);
        // A histogram without exemplars allocates none.
        let plain = Histogram::with_buckets([1.0]).snapshot();
        assert_eq!(plain.exemplars.capacity(), 0);
        assert_eq!(h.count(), 3);
    }

    #[test]
    fn test_histogram_exemplar_validation() {
        let h = Histogram::with_buckets([1.0]);
        assert!(
            h.try_observe_with_exemplar(f64::NAN, &LabelSet::EMPTY)
                .is_err()
        );
        let long = LabelSet::from([("trace_id", "x".repeat(200))]);
        assert!(matches!(
            h.try_observe_with_exemplar(0.5, &long),
            Err(MetricsError::InvalidLabel { .. })
        ));
        let bad = LabelSet::from([("bad key", "x")]);
        assert!(h.try_observe_with_exemplar(0.5, &bad).is_err());
        // The observations were still recorded.
        assert_eq!(h.count(), 2);
        assert!(h.snapshot().exemplar(0).is_none());
    }

    #[test]
    fn test_histogram_snapshot_is_monotonic_under_concurrent_writes_regression_ml017() {
        let h = Arc::new(Histogram::with_buckets([1.0, 2.0, 4.0, 8.0]));
        let stop = Arc::new(AtomicBool::new(false));
        let writers: Vec<_> = (0..4)
            .map(|t| {
                let h = Arc::clone(&h);
                let stop = Arc::clone(&stop);
                std::thread::spawn(move || {
                    let mut i = 0u32;
                    while !stop.load(Ordering::Relaxed) {
                        h.observe(f64::from((i + t) % 10));
                        i = i.wrapping_add(1);
                    }
                })
            })
            .collect();
        for _ in 0..20_000 {
            let snap = h.snapshot();
            for pair in snap.buckets.windows(2) {
                assert!(pair[0].count <= pair[1].count);
            }
            assert_eq!(snap.buckets.last().map(|b| b.count), Some(snap.count));
        }
        stop.store(true, Ordering::Relaxed);
        for w in writers {
            w.join().unwrap();
        }
    }

    #[test]
    fn test_histogram_concurrent_observations_are_exact() {
        let h = Arc::new(Histogram::with_buckets([10.0, 100.0]));
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let h = Arc::clone(&h);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    for i in 0..10_000u32 {
                        h.observe(f64::from(i % 200));
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        let snap = h.snapshot();
        assert_eq!(snap.count, 80_000);
        // Values 0..=10 land in the first bucket: 11 of every 200.
        assert_eq!(snap.buckets[0].count, 8 * 50 * 11);
        let expected_sum: f64 = 8.0 * 50.0 * (0..200).map(f64::from).sum::<f64>();
        assert!((snap.sum - expected_sum).abs() < 1e-6);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_histogram_bucket_serializes_inf_as_string_regression_ml024() {
        let h = Histogram::with_buckets([1.0]);
        h.observe(5.0);
        let json = serde_json::to_string(&h.snapshot().buckets).unwrap();
        assert!(json.contains(r#""upper_bound":"+Inf""#), "{json}");
        assert!(!json.contains("null,\"count\""));
    }

    proptest! {
        #[test]
        fn prop_snapshot_invariants(
            bounds in proptest::collection::vec(-1e6f64..1e6, 0..20),
            values in proptest::collection::vec(-2e6f64..2e6, 0..200)
        ) {
            let h = Histogram::with_buckets(bounds);
            for v in &values {
                h.observe(*v);
            }
            let snap = h.snapshot();
            prop_assert_eq!(snap.count, values.len() as u64);
            prop_assert_eq!(snap.buckets.last().map(|b| b.count), Some(snap.count));
            for pair in snap.buckets.windows(2) {
                prop_assert!(pair[0].count <= pair[1].count);
                prop_assert!(pair[0].upper_bound < pair[1].upper_bound);
            }
            for b in &snap.buckets {
                let expected = values.iter().filter(|v| **v <= b.upper_bound).count() as u64;
                prop_assert_eq!(b.count, expected);
            }
        }

        #[test]
        fn prop_quantile_is_within_bounds(
            values in proptest::collection::vec(0.0f64..100.0, 1..200),
            q in any::<f64>()
        ) {
            let h = Histogram::linear(0.0, 10.0, 11);
            for v in &values {
                h.observe(*v);
            }
            let est = h.quantile(q);
            prop_assert!((0.0..=100.0).contains(&est), "{}", est);
        }
    }
}
