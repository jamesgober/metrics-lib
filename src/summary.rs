//! Distribution with streaming quantiles and a relative-error guarantee.
//!
//! [`Summary`] keeps a logarithmically bucketed sketch of every observation
//! (the `DDSketch` scheme): bucket `k` covers `(γ^(k-1), γ^k]` with
//! `γ = (1 + α) / (1 - α)`, so any quantile it reports is within a relative
//! error `α` of the true value. Unlike a [`Histogram`](crate::Histogram) you
//! do not choose bucket bounds; unlike Prometheus client summaries the
//! quantiles are computed at read time from a mergeable sketch, not from a
//! sliding sample.
//!
//! Observations are lock-free: one `ln`, two relaxed `fetch_add`s and three
//! compare-and-swap loops (sum, minimum, maximum) that rarely retry.
//!
//! # Range and memory
//!
//! The sketch covers `[min_value, max_value]` (by default `1e-9 ..= 1e7`,
//! one nanosecond to 115 days when the unit is seconds) with
//! `ln(max/min) / ln(γ)` buckets of 8 bytes each: about 7.4 KiB at the
//! default α = 2 %. Values below `min_value` (including zero) are counted in
//! a dedicated zero bucket and estimated as `0`; values above `max_value`
//! land in the last bucket. Negative values are rejected.
//!
//! The quantiles are cumulative over the summary's lifetime (or since the
//! last [`Summary::reset`]).
//!
//! # Example
//!
//! ```
//! use metrics_lib::Summary;
//! use std::time::Duration;
//!
//! let latency = Summary::new();
//! for ms in 1..=100 {
//!     latency.observe_duration(Duration::from_millis(ms));
//! }
//! let p99 = latency.quantile(0.99);
//! assert!((p99 - 0.099).abs() / 0.099 <= 0.02);
//! ```

use crate::util::CasUpdate;
use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Upper bound on the number of sketch buckets a configuration may need.
pub const MAX_SUMMARY_BUCKETS: usize = 65_536;

/// Quantiles reported by default: median, p90, p95 and p99.
pub const DEFAULT_QUANTILES: &[f64] = &[0.5, 0.9, 0.95, 0.99];

/// Configuration of a [`Summary`].
///
/// # Examples
///
/// ```
/// use metrics_lib::{Summary, SummaryConfig};
///
/// let config = SummaryConfig::default()
///     .with_relative_accuracy(0.01)
///     .with_range(1e-6, 1e3)
///     .with_quantiles([0.5, 0.999]);
/// let summary = Summary::with_config(config).unwrap();
/// summary.observe(0.25);
/// assert_eq!(summary.count(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SummaryConfig {
    /// Maximum relative error of reported quantiles (`α`), in `(0, 0.5]`.
    pub relative_accuracy: f64,
    /// Smallest value tracked with full accuracy (`> 0`).
    pub min_value: f64,
    /// Largest value tracked with full accuracy.
    pub max_value: f64,
    /// Quantiles exporters report, each in `[0, 1]`.
    pub quantiles: Vec<f64>,
}

impl Default for SummaryConfig {
    fn default() -> Self {
        Self {
            relative_accuracy: 0.02,
            min_value: 1e-9,
            max_value: 1e7,
            quantiles: DEFAULT_QUANTILES.to_vec(),
        }
    }
}

impl SummaryConfig {
    /// Set the relative accuracy `α`.
    #[must_use]
    pub fn with_relative_accuracy(mut self, alpha: f64) -> Self {
        self.relative_accuracy = alpha;
        self
    }

    /// Set the accurately tracked value range.
    #[must_use]
    pub fn with_range(mut self, min_value: f64, max_value: f64) -> Self {
        self.min_value = min_value;
        self.max_value = max_value;
        self
    }

    /// Set the quantiles exporters report (sorted and de-duplicated on use).
    #[must_use]
    pub fn with_quantiles(mut self, quantiles: impl IntoIterator<Item = f64>) -> Self {
        self.quantiles = quantiles.into_iter().collect();
        self
    }

    /// Check the configuration.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Config`] when `α` is outside `(0, 0.5]`, the
    /// range is empty, non-finite or not positive, a quantile is outside
    /// `[0, 1]`, or the sketch would need more than [`MAX_SUMMARY_BUCKETS`]
    /// buckets.
    pub fn validate(&self) -> Result<()> {
        let a = self.relative_accuracy;
        if !(a > 0.0 && a <= 0.5) {
            return Err(MetricsError::Config(
                "summary relative_accuracy must be in (0, 0.5]".into(),
            ));
        }
        if !(self.min_value > 0.0 && self.max_value.is_finite() && self.min_value < self.max_value)
        {
            return Err(MetricsError::Config(
                "summary range must satisfy 0 < min_value < max_value < inf".into(),
            ));
        }
        if self.quantiles.iter().any(|q| !(0.0..=1.0).contains(q)) {
            return Err(MetricsError::Config(
                "summary quantiles must be within [0, 1]".into(),
            ));
        }
        if self.bucket_count() > MAX_SUMMARY_BUCKETS {
            return Err(MetricsError::Config(
                "summary configuration needs more than MAX_SUMMARY_BUCKETS buckets".into(),
            ));
        }
        Ok(())
    }

    fn gamma_ln(&self) -> f64 {
        ((1.0 + self.relative_accuracy) / (1.0 - self.relative_accuracy)).ln()
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // validated positive and bounded
    fn bucket_count(&self) -> usize {
        let raw = ((self.max_value.ln() - self.min_value.ln()) / self.gamma_ln()).ceil() + 1.0;
        if raw.is_finite() && raw > 0.0 {
            raw.min(1e9) as usize
        } else {
            usize::MAX
        }
    }
}

/// Lock-free streaming-quantile summary.
#[repr(align(64))]
pub struct Summary {
    gamma_ln: f64,
    /// `ceil(ln(min_value) / ln γ)`: the sketch index of bucket 0.
    offset: i64,
    min_value: f64,
    quantiles: Box<[f64]>,
    buckets: Box<[AtomicU64]>,
    zero_count: AtomicU64,
    count: AtomicU64,
    sum_bits: AtomicU64,
    min_bits: AtomicU64,
    max_bits: AtomicU64,
    created_at: Instant,
}

/// Point-in-time view of a [`Summary`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct SummarySnapshot {
    /// Number of observations.
    pub count: u64,
    /// Sum of observations.
    pub sum: f64,
    /// Smallest observation (`NaN` when empty).
    pub min: f64,
    /// Largest observation (`NaN` when empty).
    pub max: f64,
    /// `(quantile, estimate)` for every configured quantile; estimates are
    /// `NaN` when the summary is empty, as OpenMetrics requires.
    pub quantiles: Vec<(f64, f64)>,
    /// Time since the summary was created.
    pub age: Duration,
}

impl Summary {
    /// Create a summary with [`SummaryConfig::default`].
    #[must_use]
    pub fn new() -> Self {
        Self::build(SummaryConfig::default())
    }

    /// Create a summary with a custom configuration.
    ///
    /// # Errors
    ///
    /// Returns the error of [`SummaryConfig::validate`].
    pub fn with_config(config: SummaryConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self::build(config))
    }

    #[allow(clippy::cast_possible_truncation)] // validated: the index range fits easily
    fn build(config: SummaryConfig) -> Self {
        let gamma_ln = config.gamma_ln();
        let offset = (config.min_value.ln() / gamma_ln).ceil() as i64;
        let len = config.bucket_count().max(1);
        let mut quantiles: Vec<f64> = config.quantiles;
        quantiles.sort_by(f64::total_cmp);
        quantiles.dedup_by(|a, b| a.to_bits() == b.to_bits());
        Self {
            gamma_ln,
            offset,
            min_value: config.min_value,
            quantiles: quantiles.into_boxed_slice(),
            buckets: (0..len).map(|_| AtomicU64::new(0)).collect(),
            zero_count: AtomicU64::new(0),
            count: AtomicU64::new(0),
            sum_bits: AtomicU64::new(0.0_f64.to_bits()),
            min_bits: AtomicU64::new(f64::INFINITY.to_bits()),
            max_bits: AtomicU64::new(f64::NEG_INFINITY.to_bits()),
            created_at: Instant::now(),
        }
    }

    /// The configured export quantiles.
    #[must_use]
    pub fn quantile_targets(&self) -> &[f64] {
        &self.quantiles
    }

    /// Record one observation. Negative, NaN and infinite values are
    /// ignored; use [`Summary::try_observe`] to be told.
    #[inline]
    pub fn observe(&self, value: f64) {
        let _ = self.try_observe(value);
    }

    /// Record a duration, in seconds.
    #[inline]
    pub fn observe_duration(&self, duration: Duration) {
        self.observe(duration.as_secs_f64());
    }

    /// Record one observation.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] for negative, NaN or infinite
    /// values; nothing is recorded.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // index computed from a validated range
    pub fn try_observe(&self, value: f64) -> Result<()> {
        if !(value.is_finite() && value >= 0.0) {
            return Err(MetricsError::InvalidValue {
                reason: "summary values must be finite and non-negative",
            });
        }
        if value < self.min_value {
            let _ = self.zero_count.fetch_add(1, Ordering::Relaxed);
        } else {
            let k = (value.ln() / self.gamma_ln).ceil() as i64;
            let idx = usize::try_from(k.saturating_sub(self.offset)).unwrap_or(0);
            let last = self.buckets.len() - 1;
            let _ = self.buckets[idx.min(last)].fetch_add(1, Ordering::Relaxed);
        }
        let _ = self.count.fetch_add(1, Ordering::Relaxed);
        update_f64(&self.sum_bits, |s| {
            Some(s + value).filter(|n| n.is_finite())
        });
        update_f64(&self.min_bits, |m| (value < m).then_some(value));
        update_f64(&self.max_bits, |m| (value > m).then_some(value));
        Ok(())
    }

    /// Number of observations.
    #[inline]
    #[must_use]
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Sum of observations.
    #[inline]
    #[must_use]
    pub fn sum(&self) -> f64 {
        f64::from_bits(self.sum_bits.load(Ordering::Relaxed))
    }

    /// Smallest observation, or `NaN` when empty.
    #[must_use]
    pub fn min(&self) -> f64 {
        let v = f64::from_bits(self.min_bits.load(Ordering::Relaxed));
        if v.is_finite() { v } else { f64::NAN }
    }

    /// Largest observation, or `NaN` when empty.
    #[must_use]
    pub fn max(&self) -> f64 {
        let v = f64::from_bits(self.max_bits.load(Ordering::Relaxed));
        if v.is_finite() { v } else { f64::NAN }
    }

    /// Estimate the `q`-quantile (`q` clamped to `[0, 1]`).
    ///
    /// Within the configured range the estimate is within the configured
    /// relative accuracy of an observed value of that rank. Returns `NaN`
    /// when the summary is empty.
    #[must_use]
    pub fn quantile(&self, q: f64) -> f64 {
        self.quantiles_of(&[q]).first().copied().unwrap_or(f64::NAN)
    }

    /// Estimate several quantiles from one consistent read of the sketch:
    /// the bucket counts are loaded once and walked once for all targets.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )] // rank arithmetic on counts
    fn quantiles_of(&self, targets: &[f64]) -> Vec<f64> {
        let zero = self.zero_count.load(Ordering::Relaxed);
        let counts: Vec<u64> = self
            .buckets
            .iter()
            .map(|b| b.load(Ordering::Relaxed))
            .collect();
        let total = counts.iter().fold(zero, |acc, c| acc.saturating_add(*c));
        if total == 0 {
            return vec![f64::NAN; targets.len()];
        }
        // 0-based rank of each target, visited in ascending rank order.
        let ranks: Vec<u64> = targets
            .iter()
            .map(|q| {
                let q = if q.is_nan() { 0.5 } else { q.clamp(0.0, 1.0) };
                (q * (total - 1) as f64).floor() as u64
            })
            .collect();
        let mut order: Vec<usize> = (0..targets.len()).collect();
        order.sort_unstable_by_key(|&i| ranks[i]);
        let mut out = vec![f64::NAN; targets.len()];
        let mut next = order.iter().peekable();
        while let Some(&&i) = next.peek() {
            if ranks[i] >= zero {
                break;
            }
            out[i] = 0.0;
            let _ = next.next();
        }
        let mut seen = zero;
        for (bucket, c) in counts.iter().enumerate() {
            if next.peek().is_none() {
                break;
            }
            seen = seen.saturating_add(*c);
            while let Some(&&i) = next.peek() {
                if seen <= ranks[i] {
                    break;
                }
                out[i] = self.bucket_value(bucket);
                let _ = next.next();
            }
        }
        // Exact extremes beat the bucket midpoint at the edges.
        let (min, max) = (self.min(), self.max());
        if !min.is_nan() {
            for v in &mut out {
                if !v.is_nan() {
                    *v = v.clamp(min, max.max(min));
                }
            }
        }
        out
    }

    /// Representative value of bucket `i`: `2 γ^k / (γ + 1)`, which is within
    /// relative error α of every value in `(γ^(k-1), γ^k]`.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_wrap)] // small indices
    fn bucket_value(&self, i: usize) -> f64 {
        let k = i as i64 + self.offset;
        let gamma = self.gamma_ln.exp();
        2.0 * (k as f64 * self.gamma_ln).exp() / (gamma + 1.0)
    }

    /// Point-in-time snapshot including the configured quantiles.
    #[must_use]
    pub fn snapshot(&self) -> SummarySnapshot {
        SummarySnapshot {
            count: self.count(),
            sum: self.sum(),
            min: self.min(),
            max: self.max(),
            quantiles: self
                .quantiles
                .iter()
                .copied()
                .zip(self.quantiles_of(&self.quantiles))
                .collect(),
            age: self.age(),
        }
    }

    /// Time since the summary was created.
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Number of sketch buckets (memory use is 8 bytes per bucket).
    #[must_use]
    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    /// Clear every observation.
    pub fn reset(&self) {
        for b in &*self.buckets {
            b.store(0, Ordering::Relaxed);
        }
        self.zero_count.store(0, Ordering::Relaxed);
        self.count.store(0, Ordering::Relaxed);
        self.sum_bits.store(0.0_f64.to_bits(), Ordering::Relaxed);
        self.min_bits
            .store(f64::INFINITY.to_bits(), Ordering::Relaxed);
        self.max_bits
            .store(f64::NEG_INFINITY.to_bits(), Ordering::Relaxed);
    }
}

#[inline]
fn update_f64(cell: &AtomicU64, f: impl Fn(f64) -> Option<f64>) {
    let _ = cell.cas_update(Ordering::Relaxed, Ordering::Relaxed, |bits| {
        f(f64::from_bits(bits)).map(f64::to_bits)
    });
}

impl Default for Summary {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Summary")
            .field("count", &self.count())
            .field("sum", &self.sum())
            .field("buckets", &self.buckets.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_quantiles_match_single_quantiles_regression_ml2_008() {
        let s = Summary::with_config(
            SummaryConfig::default().with_quantiles([0.99, 0.0, 0.5, 1.0, 0.9]),
        )
        .unwrap();
        for i in 0..10_000u32 {
            s.observe(f64::from(i % 997) * 0.37 + 0.001);
        }
        s.observe(0.0);
        let snap = s.snapshot();
        assert_eq!(snap.quantiles.len(), 5);
        for (q, v) in &snap.quantiles {
            assert_eq!(*v, s.quantile(*q), "q={q}");
        }
        assert!(
            Summary::new()
                .snapshot()
                .quantiles
                .iter()
                .all(|(_, v)| v.is_nan())
        );
    }
    use proptest::prelude::*;
    use std::sync::{Arc, Barrier};

    fn exact_quantile(sorted: &[f64], q: f64) -> f64 {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let rank = (q * (sorted.len() - 1) as f64).floor() as usize;
        sorted[rank]
    }

    #[test]
    fn test_summary_empty_reports_nan() {
        let s = Summary::new();
        assert!(s.quantile(0.5).is_nan());
        assert!(s.min().is_nan());
        assert!(s.max().is_nan());
        let snap = s.snapshot();
        assert_eq!(snap.count, 0);
        assert!(snap.quantiles.iter().all(|(_, v)| v.is_nan()));
        assert_eq!(snap.quantiles.len(), DEFAULT_QUANTILES.len());
    }

    #[test]
    fn test_summary_quantiles_within_relative_error() {
        let s = Summary::new();
        let mut values: Vec<f64> = (1..=10_000).map(|i| f64::from(i) * 1e-4).collect();
        for v in &values {
            s.observe(*v);
        }
        values.sort_by(f64::total_cmp);
        for q in [0.0, 0.1, 0.5, 0.9, 0.95, 0.99, 0.999, 1.0] {
            let exact = exact_quantile(&values, q);
            let est = s.quantile(q);
            assert!(
                (est - exact).abs() <= 0.02 * exact + 1e-12,
                "q={q} exact={exact} est={est}"
            );
        }
        assert_eq!(s.count(), 10_000);
        assert!((s.sum() - values.iter().sum::<f64>()).abs() < 1e-6);
        assert_eq!(s.min(), 1e-4);
        assert_eq!(s.max(), 1.0);
    }

    #[test]
    fn test_summary_rejects_negative_and_non_finite() {
        let s = Summary::new();
        assert!(s.try_observe(-1.0).is_err());
        assert!(s.try_observe(f64::NAN).is_err());
        assert!(s.try_observe(f64::INFINITY).is_err());
        s.observe(-5.0);
        assert_eq!(s.count(), 0);
    }

    #[test]
    fn test_summary_zero_and_out_of_range_values() {
        let s = Summary::with_config(SummaryConfig::default().with_range(1.0, 100.0)).unwrap();
        s.observe(0.0);
        s.observe(0.5);
        s.observe(1e9);
        assert_eq!(s.count(), 3);
        assert_eq!(s.quantile(0.0), 0.0);
        // Above-range values land in the last bucket, then clamp to the max.
        let top = s.quantile(1.0);
        assert!(top >= 99.0 && top <= 1e9, "{top}");
    }

    #[test]
    fn test_summary_config_validation() {
        let base = SummaryConfig::default();
        assert!(base.validate().is_ok());
        for bad in [
            base.clone().with_relative_accuracy(0.0),
            base.clone().with_relative_accuracy(0.9),
            base.clone().with_relative_accuracy(f64::NAN),
            base.clone().with_range(0.0, 1.0),
            base.clone().with_range(2.0, 1.0),
            base.clone().with_range(1.0, f64::INFINITY),
            base.clone().with_quantiles([1.5]),
            base.clone().with_quantiles([f64::NAN]),
            base.clone()
                .with_relative_accuracy(1e-6)
                .with_range(1e-300, 1e300),
        ] {
            assert!(Summary::with_config(bad).is_err());
        }
    }

    #[test]
    fn test_summary_quantile_targets_sorted_and_deduplicated() {
        let s =
            Summary::with_config(SummaryConfig::default().with_quantiles([0.9, 0.5, 0.9])).unwrap();
        assert_eq!(s.quantile_targets(), &[0.5, 0.9]);
    }

    #[test]
    fn test_summary_reset_and_debug() {
        let s = Summary::default();
        s.observe_duration(Duration::from_millis(3));
        s.reset();
        assert_eq!(s.count(), 0);
        assert!(s.quantile(0.5).is_nan());
        assert!(format!("{s:?}").contains("Summary"));
        assert!(s.bucket_count() > 100);
        assert!(s.age() < Duration::from_secs(60));
    }

    #[test]
    fn test_summary_concurrent_observations_are_counted_exactly() {
        let s = Arc::new(Summary::new());
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let s = Arc::clone(&s);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    for i in 1..=5_000u32 {
                        s.observe(f64::from(i) * 1e-3);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(s.count(), 40_000);
        let median = s.quantile(0.5);
        assert!((median - 2.5).abs() <= 0.02 * 2.5 + 1e-3, "{median}");
    }

    proptest! {
        #[test]
        fn prop_quantile_error_is_bounded(
            values in proptest::collection::vec(1e-6f64..1e4, 1..300),
            q in 0.0f64..=1.0
        ) {
            let s = Summary::new();
            for v in &values {
                s.observe(*v);
            }
            let mut sorted = values.clone();
            sorted.sort_by(f64::total_cmp);
            let exact = exact_quantile(&sorted, q);
            let est = s.quantile(q);
            prop_assert!((est - exact).abs() <= 0.02 * exact * 1.0000001, "q={} exact={} est={}", q, exact, est);
        }
    }
}
