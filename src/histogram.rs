//! Histogram metric: bucketed observations with sum/count and approximate
//! quantile estimation.
//!
//! Compatible with Prometheus and OpenMetrics histogram semantics:
//! - Each bucket counts observations whose value is `<= upper_bound`.
//! - Buckets are *cumulative* on export (Prometheus convention).
//! - `+Inf` is an implicit final bucket equal to the total count.
//! - `sum` and `count` are tracked separately for downstream `_sum` /
//!   `_count` series.
//!
//! Use [`Histogram::default_seconds`] for the standard Prometheus latency
//! buckets, [`Histogram::linear`] / [`Histogram::exponential`] for
//! parameterized layouts, or [`Histogram::with_buckets`] for explicit
//! boundaries.
//!
//! # Concurrency
//!
//! All update operations are lock-free atomics on the hot path. `observe()`
//! does a binary search to find the destination bucket (O(log B)), then three
//! `Relaxed` operations (bucket counter +1, sum CAS-loop, count +1). The CAS
//! loop on `sum` is the only retry path under contention.
//!
//! # Example
//!
//! ```
//! use metrics_lib::Histogram;
//! let h = Histogram::default_seconds();
//! h.observe(0.012);   // 12 ms
//! h.observe(0.085);
//! h.observe(0.250);
//! assert_eq!(h.count(), 3);
//! assert!(h.quantile(0.5) > 0.0);
//! ```

use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Standard Prometheus latency buckets (seconds).
///
/// Equivalent to `prometheus::default_buckets()` and `DefaultObjectives` in
/// the Go/Java client libraries. Suitable for HTTP request / RPC / DB query
/// latency dashboards out of the box.
pub const DEFAULT_SECONDS_BUCKETS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

/// Lock-free bucketed histogram.
///
/// Cache-line aligned to prevent false sharing between the
/// frequently-touched `sum` / `count` fields and the colder bucket array.
#[repr(align(64))]
pub struct Histogram {
    /// Bucket upper bounds, sorted ascending. The implicit `+Inf` bucket is
    /// not stored here; it is reconstructed from the difference between
    /// `count` and `bucket_counts.last()` at snapshot time.
    bucket_bounds: Box<[f64]>,
    /// One atomic counter per explicit bucket. `bucket_counts[i]` counts
    /// observations `<= bucket_bounds[i]` (non-cumulative storage — the
    /// `snapshot()` API converts to cumulative form).
    bucket_counts: Box<[AtomicU64]>,
    /// Sum of all finite observations, stored as `f64::to_bits` for atomic
    /// CAS updates.
    sum_bits: AtomicU64,
    /// Total number of observations recorded (finite + the implicit `+Inf`
    /// bucket).
    total: AtomicU64,
    /// Creation timestamp (for `age()` / rate calculations).
    created_at: Instant,
}

/// Immutable snapshot of a [`Histogram`]'s state.
///
/// Suitable for passing to exporters. `buckets` is cumulative: each entry's
/// `count` is the total number of observations `<= upper_bound`. A trailing
/// entry with `upper_bound = f64::INFINITY` matches `count`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HistogramSnapshot {
    /// Cumulative bucket counts. The final element always has
    /// `upper_bound = +Inf` and `count = total`.
    pub buckets: Vec<HistogramBucket>,
    /// Sum of all finite observations.
    pub sum: f64,
    /// Total number of observations.
    pub count: u64,
    /// Time since the histogram was created.
    pub age: Duration,
}

/// One row of a [`HistogramSnapshot::buckets`].
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HistogramBucket {
    /// Inclusive upper bound for this bucket. The final bucket has
    /// `upper_bound = f64::INFINITY`.
    pub upper_bound: f64,
    /// Cumulative number of observations `<= upper_bound`.
    pub count: u64,
}

impl Histogram {
    /// Construct a histogram with the supplied explicit bucket upper bounds.
    ///
    /// `bounds` must be finite and strictly ascending. Duplicates and NaN
    /// are silently dropped before validation; an empty result after
    /// cleaning panics in debug builds and uses a single `1.0` bucket in
    /// release builds. The implicit `+Inf` bucket is always present.
    pub fn with_buckets(bounds: impl IntoIterator<Item = f64>) -> Self {
        let mut cleaned: Vec<f64> = bounds
            .into_iter()
            .filter(|b| b.is_finite())
            .collect::<Vec<_>>();
        cleaned.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        cleaned.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
        if cleaned.is_empty() {
            debug_assert!(
                false,
                "Histogram::with_buckets requires at least one bucket"
            );
            cleaned.push(1.0);
        }
        let bucket_counts: Box<[AtomicU64]> = cleaned.iter().map(|_| AtomicU64::new(0)).collect();
        Self {
            bucket_bounds: cleaned.into_boxed_slice(),
            bucket_counts,
            sum_bits: AtomicU64::new(0.0_f64.to_bits()),
            total: AtomicU64::new(0),
            created_at: Instant::now(),
        }
    }

    /// Convenience: histogram with the standard Prometheus latency-seconds
    /// buckets ([`crate::DEFAULT_SECONDS_BUCKETS`]).
    pub fn default_seconds() -> Self {
        Self::with_buckets(DEFAULT_SECONDS_BUCKETS.iter().copied())
    }

    /// Linear buckets: `start, start+width, …, start+(count-1)*width`.
    ///
    /// Returns an empty histogram (single `1.0` bucket) when `count == 0`,
    /// `width <= 0.0`, or any computed boundary is non-finite.
    pub fn linear(start: f64, width: f64, count: usize) -> Self {
        if count == 0 || !width.is_finite() || width <= 0.0 || !start.is_finite() {
            return Self::with_buckets(std::iter::once(1.0));
        }
        let bounds = (0..count).map(|i| start + width * (i as f64));
        Self::with_buckets(bounds)
    }

    /// Exponential buckets: `start, start*factor, …, start*factor^(count-1)`.
    ///
    /// Returns a fallback single-bucket histogram when inputs are invalid
    /// (`start <= 0`, `factor <= 1`, non-finite, `count == 0`).
    pub fn exponential(start: f64, factor: f64, count: usize) -> Self {
        if count == 0 || !start.is_finite() || !factor.is_finite() || start <= 0.0 || factor <= 1.0
        {
            return Self::with_buckets(std::iter::once(1.0));
        }
        let mut cur = start;
        let bounds = (0..count).map(|i| {
            let v = cur;
            if i + 1 < count {
                cur *= factor;
            }
            v
        });
        Self::with_buckets(bounds)
    }

    /// Record one observation. Non-finite values are silently dropped; use
    /// [`Self::try_observe`] to receive an explicit error instead.
    #[inline]
    pub fn observe(&self, value: f64) {
        if !value.is_finite() {
            return;
        }
        self.observe_finite(value);
    }

    /// Record one observation; rejects non-finite values.
    ///
    /// Returns `Err(MetricsError::InvalidValue)` for NaN / ±Inf.
    #[inline]
    pub fn try_observe(&self, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(MetricsError::InvalidValue {
                reason: "value is not finite",
            });
        }
        self.observe_finite(value);
        Ok(())
    }

    #[inline]
    fn observe_finite(&self, value: f64) {
        // Binary-search for the matching bucket. Anything larger than the
        // last explicit bound lands in the implicit +Inf bucket — that is,
        // it bumps `total` but not any `bucket_counts[i]`.
        let idx = self.bucket_bounds.partition_point(|&b| b < value);
        if idx < self.bucket_counts.len() {
            self.bucket_counts[idx].fetch_add(1, Ordering::Relaxed);
        }
        self.total.fetch_add(1, Ordering::Relaxed);

        // Atomic-f64 sum update.
        loop {
            let prev_bits = self.sum_bits.load(Ordering::Relaxed);
            let new = f64::from_bits(prev_bits) + value;
            if !new.is_finite() {
                // Overflow to ±Inf — leave sum unchanged rather than poison
                // the stored value. The bucket+count updates above stand.
                return;
            }
            if self
                .sum_bits
                .compare_exchange_weak(
                    prev_bits,
                    new.to_bits(),
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return;
            }
        }
    }

    /// Number of recorded observations (across all buckets, including +Inf).
    #[must_use]
    #[inline]
    pub fn count(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }

    /// Sum of recorded observations.
    #[must_use]
    #[inline]
    pub fn sum(&self) -> f64 {
        f64::from_bits(self.sum_bits.load(Ordering::Relaxed))
    }

    /// Mean of recorded observations. Returns `0.0` when `count == 0`.
    #[must_use]
    #[inline]
    pub fn mean(&self) -> f64 {
        let c = self.count();
        if c == 0 {
            0.0
        } else {
            self.sum() / (c as f64)
        }
    }

    /// Time since the histogram was created.
    #[must_use]
    #[inline]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Reset every bucket counter, the sum, and the total count.
    pub fn reset(&self) {
        for b in self.bucket_counts.iter() {
            b.store(0, Ordering::SeqCst);
        }
        self.sum_bits.store(0.0_f64.to_bits(), Ordering::SeqCst);
        self.total.store(0, Ordering::SeqCst);
    }

    /// Capture an immutable snapshot of the histogram's current state.
    ///
    /// `buckets` is rendered cumulatively: each entry's `count` is the total
    /// observations `<= upper_bound`. A trailing `+Inf` bucket is appended
    /// matching the total count.
    pub fn snapshot(&self) -> HistogramSnapshot {
        let count = self.count();
        let sum = self.sum();
        let mut buckets = Vec::with_capacity(self.bucket_bounds.len() + 1);
        let mut cumulative: u64 = 0;
        for (bound, counter) in self.bucket_bounds.iter().zip(self.bucket_counts.iter()) {
            cumulative = cumulative.saturating_add(counter.load(Ordering::Relaxed));
            buckets.push(HistogramBucket {
                upper_bound: *bound,
                count: cumulative,
            });
        }
        // Implicit +Inf bucket holds count - cumulative observations.
        buckets.push(HistogramBucket {
            upper_bound: f64::INFINITY,
            count,
        });
        HistogramSnapshot {
            buckets,
            sum,
            count,
            age: self.age(),
        }
    }

    /// Approximate quantile via bucket interpolation.
    ///
    /// `q` is clamped to `0.0..=1.0`. Returns `0.0` when the histogram is
    /// empty. The estimate assumes a uniform distribution of observations
    /// within each bucket; for the implicit `+Inf` bucket the largest
    /// explicit upper bound is returned.
    #[must_use]
    pub fn quantile(&self, q: f64) -> f64 {
        let q = q.clamp(0.0, 1.0);
        let count = self.count();
        if count == 0 {
            return 0.0;
        }
        let target = (q * count as f64).ceil() as u64;
        let mut cumulative: u64 = 0;
        let mut prev_bound = 0.0;
        for (i, (bound, counter)) in self
            .bucket_bounds
            .iter()
            .zip(self.bucket_counts.iter())
            .enumerate()
        {
            let bucket_count = counter.load(Ordering::Relaxed);
            let next_cum = cumulative.saturating_add(bucket_count);
            if next_cum >= target {
                if bucket_count == 0 {
                    return *bound;
                }
                // Linear interpolation inside the bucket.
                let lower = if i == 0 { 0.0 } else { prev_bound };
                let within = (target - cumulative) as f64 / bucket_count as f64;
                return lower + (bound - lower) * within;
            }
            cumulative = next_cum;
            prev_bound = *bound;
        }
        // Quantile lies in the +Inf bucket; return the highest explicit
        // bound rather than infinity.
        self.bucket_bounds.last().copied().unwrap_or(f64::INFINITY)
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::default_seconds()
    }
}

impl std::fmt::Debug for Histogram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let snap = self.snapshot();
        f.debug_struct("Histogram")
            .field("buckets", &snap.buckets.len())
            .field("count", &snap.count)
            .field("sum", &snap.sum)
            .field(
                "mean",
                &(if snap.count > 0 {
                    snap.sum / snap.count as f64
                } else {
                    0.0
                }),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observe_increments_count_and_sum() {
        let h = Histogram::with_buckets([0.1, 1.0, 10.0]);
        h.observe(0.5);
        h.observe(0.5);
        h.observe(5.0);
        assert_eq!(h.count(), 3);
        assert!((h.sum() - 6.0).abs() < 1e-9);
        assert!((h.mean() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn cumulative_buckets_reported_correctly() {
        let h = Histogram::with_buckets([1.0, 2.0, 3.0]);
        h.observe(0.5); // → bucket[0]
        h.observe(1.5); // → bucket[1]
        h.observe(2.5); // → bucket[2]
        h.observe(10.0); // → +Inf
        let snap = h.snapshot();
        assert_eq!(snap.buckets.len(), 4);
        assert_eq!(snap.buckets[0].upper_bound, 1.0);
        assert_eq!(snap.buckets[0].count, 1); // ≤ 1.0
        assert_eq!(snap.buckets[1].count, 2); // ≤ 2.0
        assert_eq!(snap.buckets[2].count, 3); // ≤ 3.0
        assert!(snap.buckets[3].upper_bound.is_infinite());
        assert_eq!(snap.buckets[3].count, 4); // ≤ +Inf
        assert_eq!(snap.count, 4);
    }

    #[test]
    fn try_observe_rejects_non_finite() {
        let h = Histogram::with_buckets([1.0]);
        assert!(matches!(
            h.try_observe(f64::NAN),
            Err(MetricsError::InvalidValue { .. })
        ));
        assert!(matches!(
            h.try_observe(f64::INFINITY),
            Err(MetricsError::InvalidValue { .. })
        ));
        assert_eq!(h.count(), 0);
    }

    #[test]
    fn observe_silently_drops_non_finite() {
        let h = Histogram::with_buckets([1.0]);
        h.observe(f64::NAN);
        h.observe(f64::INFINITY);
        h.observe(2.0);
        assert_eq!(h.count(), 1);
    }

    #[test]
    fn quantile_zero_on_empty() {
        let h = Histogram::with_buckets([1.0]);
        assert_eq!(h.quantile(0.5), 0.0);
    }

    #[test]
    fn quantile_estimates_within_bucket() {
        let h = Histogram::with_buckets([1.0, 2.0]);
        for _ in 0..10 {
            h.observe(0.5); // 10 obs in [0, 1.0]
        }
        for _ in 0..10 {
            h.observe(1.5); // 10 obs in (1.0, 2.0]
        }
        // p50 should land roughly mid-way in the first bucket.
        let p50 = h.quantile(0.5);
        assert!(p50 > 0.0 && p50 <= 1.0);
        // p100 sits at the top explicit bound.
        let p100 = h.quantile(1.0);
        assert!((p100 - 2.0).abs() < 1e-9);
    }

    #[test]
    fn linear_and_exponential_constructors() {
        let l = Histogram::linear(0.0, 0.5, 5);
        let snap = l.snapshot();
        // 5 explicit + +Inf = 6
        assert_eq!(snap.buckets.len(), 6);
        assert_eq!(snap.buckets[0].upper_bound, 0.0);
        assert_eq!(snap.buckets[1].upper_bound, 0.5);

        let e = Histogram::exponential(0.1, 2.0, 4);
        let snap = e.snapshot();
        assert_eq!(snap.buckets.len(), 5);
        assert!((snap.buckets[0].upper_bound - 0.1).abs() < 1e-9);
        assert!((snap.buckets[1].upper_bound - 0.2).abs() < 1e-9);
        assert!((snap.buckets[2].upper_bound - 0.4).abs() < 1e-9);
        assert!((snap.buckets[3].upper_bound - 0.8).abs() < 1e-9);
    }

    #[test]
    fn invalid_constructor_inputs_fall_back_to_single_bucket() {
        let h = Histogram::linear(0.0, -1.0, 5);
        assert_eq!(h.bucket_bounds.len(), 1);
        let h = Histogram::exponential(0.0, 2.0, 5);
        assert_eq!(h.bucket_bounds.len(), 1);
        let h = Histogram::exponential(1.0, 1.0, 5);
        assert_eq!(h.bucket_bounds.len(), 1);
    }

    #[test]
    fn reset_clears_state() {
        let h = Histogram::with_buckets([1.0]);
        h.observe(0.5);
        h.observe(0.5);
        assert_eq!(h.count(), 2);
        h.reset();
        assert_eq!(h.count(), 0);
        assert_eq!(h.sum(), 0.0);
    }

    #[test]
    fn debug_impl_does_not_panic() {
        let h = Histogram::default_seconds();
        h.observe(0.5);
        let _ = format!("{h:?}");
    }
}
