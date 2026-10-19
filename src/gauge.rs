//! Lock-free `f64` gauge.
//!
//! The value is stored as IEEE-754 bits in an [`AtomicU64`]. [`Gauge::set`]
//! and [`Gauge::get`] are a single relaxed store / load; read-modify-write
//! operations (`add`, `set_max`, `update_ema`, …) are compare-and-swap loops
//! that only retry under contention.
//!
//! # Non-finite values
//!
//! - [`Gauge::set`] stores its argument as-is, including NaN and ±∞, because
//!   a NaN gauge is a legitimate "no data" signal and both Prometheus and
//!   OpenMetrics can carry it. Use [`Gauge::try_set`] to reject non-finite
//!   values.
//! - Every other unchecked method ignores a non-finite argument and never
//!   writes a non-finite result: `add`, `sub`, `set_max`, `set_min`,
//!   `multiply`, `divide`, `clamp` and `update_ema` leave the gauge unchanged
//!   in those cases. The `try_*` variants report the same situations as
//!   errors.

use crate::util::CasUpdate;
use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Lock-free floating-point gauge.
///
/// Cache-line aligned so two gauges never share a line.
///
/// # Examples
///
/// ```
/// use metrics_lib::Gauge;
///
/// let temperature = Gauge::new();
/// temperature.set(21.5);
/// temperature.add(0.5);
/// assert_eq!(temperature.get(), 22.0);
/// ```
#[repr(align(64))]
pub struct Gauge {
    bits: AtomicU64,
    created_at: Instant,
}

/// Point-in-time statistics for a [`Gauge`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct GaugeStats {
    /// Current value.
    pub value: f64,
    /// Time since the gauge was created.
    pub age: Duration,
}

const NOT_FINITE: MetricsError = MetricsError::InvalidValue {
    reason: "value is not finite",
};

impl Gauge {
    /// Create a gauge at `0.0`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::with_value(0.0)
    }

    /// Create a gauge at `initial` (stored as-is, see [`Gauge::set`]).
    #[inline]
    #[must_use]
    pub fn with_value(initial: f64) -> Self {
        Self {
            bits: AtomicU64::new(initial.to_bits()),
            created_at: Instant::now(),
        }
    }

    /// Store `value` as-is. NaN and ±∞ are stored too; see the module docs.
    #[inline]
    pub fn set(&self, value: f64) {
        self.bits.store(value.to_bits(), Ordering::Relaxed);
    }

    /// Store `value` if it is finite.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] for NaN or ±∞; the gauge is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Gauge, MetricsError};
    ///
    /// let g = Gauge::new();
    /// assert!(g.try_set(42.0).is_ok());
    /// assert!(matches!(g.try_set(f64::NAN), Err(MetricsError::InvalidValue { .. })));
    /// assert_eq!(g.get(), 42.0);
    /// ```
    #[inline]
    pub fn try_set(&self, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(NOT_FINITE);
        }
        self.set(value);
        Ok(())
    }

    /// Current value.
    #[inline]
    #[must_use]
    pub fn get(&self) -> f64 {
        f64::from_bits(self.bits.load(Ordering::Relaxed))
    }

    /// Apply `f` atomically. `f` returns `None` to leave the value as-is.
    /// Returns `true` when a new value was written.
    #[inline]
    fn update(&self, f: impl Fn(f64) -> Option<f64>) -> bool {
        self.bits
            .cas_update(Ordering::Relaxed, Ordering::Relaxed, |bits| {
                f(f64::from_bits(bits)).map(f64::to_bits)
            })
            .is_ok()
    }

    /// Add `delta`. Ignored when `delta` is non-finite or the sum would be.
    #[inline]
    pub fn add(&self, delta: f64) {
        if delta == 0.0 || !delta.is_finite() {
            return;
        }
        let _ = self.update(|v| Some(v + delta).filter(|n| n.is_finite()));
    }

    /// Subtract `delta`. Same rules as [`Gauge::add`].
    #[inline]
    pub fn sub(&self, delta: f64) {
        self.add(-delta);
    }

    /// Add `delta`, reporting invalid input.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] when `delta` is non-finite and
    /// [`MetricsError::Overflow`] when the sum would be non-finite (including
    /// when the gauge already holds NaN or ±∞). The gauge is unchanged on
    /// error.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Gauge, MetricsError};
    ///
    /// let g = Gauge::with_value(1.5);
    /// g.try_add(2.5).unwrap();
    /// assert_eq!(g.get(), 4.0);
    /// assert!(matches!(g.try_add(f64::INFINITY), Err(MetricsError::InvalidValue { .. })));
    /// ```
    #[inline]
    pub fn try_add(&self, delta: f64) -> Result<()> {
        if !delta.is_finite() {
            return Err(MetricsError::InvalidValue {
                reason: "delta is not finite",
            });
        }
        if self.update(|v| Some(v + delta).filter(|n| n.is_finite())) {
            Ok(())
        } else {
            Err(MetricsError::Overflow)
        }
    }

    /// Subtract `delta`, reporting invalid input. Same rules as
    /// [`Gauge::try_add`].
    ///
    /// # Errors
    ///
    /// See [`Gauge::try_add`].
    #[inline]
    pub fn try_sub(&self, delta: f64) -> Result<()> {
        self.try_add(-delta)
    }

    /// Raise the value to `value` if `value` is larger. Ignored when `value`
    /// is non-finite. A NaN gauge is replaced.
    #[inline]
    pub fn set_max(&self, value: f64) {
        if value.is_finite() {
            let _ = self.update(|v| (v.is_nan() || value > v).then_some(value));
        }
    }

    /// Like [`Gauge::set_max`] but reports a non-finite `value`.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] for NaN or ±∞.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::Gauge;
    ///
    /// let g = Gauge::with_value(5.0);
    /// g.try_set_max(10.0).unwrap();
    /// assert_eq!(g.get(), 10.0);
    /// assert!(g.try_set_max(f64::NAN).is_err());
    /// ```
    #[inline]
    pub fn try_set_max(&self, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(NOT_FINITE);
        }
        self.set_max(value);
        Ok(())
    }

    /// Lower the value to `value` if `value` is smaller. Ignored when `value`
    /// is non-finite. A NaN gauge is replaced.
    #[inline]
    pub fn set_min(&self, value: f64) {
        if value.is_finite() {
            let _ = self.update(|v| (v.is_nan() || value < v).then_some(value));
        }
    }

    /// Like [`Gauge::set_min`] but reports a non-finite `value`.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidValue`] for NaN or ±∞.
    #[inline]
    pub fn try_set_min(&self, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(NOT_FINITE);
        }
        self.set_min(value);
        Ok(())
    }

    /// Replace the value with `new` if its bits equal `expected`'s.
    ///
    /// # Errors
    ///
    /// Returns `Err(current)` with the actual value when it did not match.
    #[inline]
    pub fn compare_and_swap(&self, expected: f64, new: f64) -> core::result::Result<f64, f64> {
        self.bits
            .compare_exchange(
                expected.to_bits(),
                new.to_bits(),
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .map(f64::from_bits)
            .map_err(f64::from_bits)
    }

    /// Reset to `0.0`.
    #[inline]
    pub fn reset(&self) {
        self.set(0.0);
    }

    /// Multiply by `factor`. Ignored when `factor` is non-finite or the
    /// product would be.
    #[inline]
    pub fn multiply(&self, factor: f64) {
        if factor.is_finite() {
            let _ = self.update(|v| Some(v * factor).filter(|n| n.is_finite()));
        }
    }

    /// Divide by `divisor`. Ignored when `divisor` is zero or non-finite, or
    /// the quotient would be non-finite.
    #[inline]
    pub fn divide(&self, divisor: f64) {
        if divisor.is_finite() && divisor != 0.0 {
            let _ = self.update(|v| Some(v / divisor).filter(|n| n.is_finite()));
        }
    }

    /// Replace the value with its absolute value.
    #[inline]
    pub fn abs(&self) {
        let _ = self.update(|v| v.is_sign_negative().then_some(v.abs()));
    }

    /// Clamp the value into `[min, max]`.
    ///
    /// Ignored when either bound is NaN or `min > max` (where
    /// [`f64::clamp`] would panic). A NaN gauge is left unchanged.
    #[inline]
    pub fn clamp(&self, min: f64, max: f64) {
        if min.is_nan() || max.is_nan() || min > max {
            return;
        }
        let _ = self.update(|v| {
            let clamped = v.clamp(min, max);
            (clamped.to_bits() != v.to_bits() && !v.is_nan()).then_some(clamped)
        });
    }

    /// Exponential moving average: `alpha * sample + (1 - alpha) * current`.
    ///
    /// `alpha` is clamped to `[0, 1]`. Ignored when `sample` or `alpha` is
    /// non-finite, or the result would be. When the gauge holds NaN or ±∞ the
    /// sample replaces it.
    #[inline]
    pub fn update_ema(&self, sample: f64, alpha: f64) {
        if !sample.is_finite() || !alpha.is_finite() {
            return;
        }
        let alpha = alpha.clamp(0.0, 1.0);
        let _ = self.update(|v| {
            if v.is_finite() {
                Some(alpha.mul_add(sample, (1.0 - alpha) * v)).filter(|n| n.is_finite())
            } else {
                Some(sample)
            }
        });
    }

    /// Point-in-time statistics.
    #[must_use]
    pub fn stats(&self) -> GaugeStats {
        GaugeStats {
            value: self.get(),
            age: self.age(),
        }
    }

    /// Time since the gauge was created.
    #[inline]
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// `true` when the value is exactly `0.0` or `-0.0`.
    #[inline]
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.get() == 0.0
    }

    /// `true` when the value is greater than zero.
    #[inline]
    #[must_use]
    pub fn is_positive(&self) -> bool {
        self.get() > 0.0
    }

    /// `true` when the value is less than zero.
    #[inline]
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.get() < 0.0
    }

    /// `true` when the value is neither NaN nor infinite.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.get().is_finite()
    }
}

impl Default for Gauge {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Gauge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gauge({})", self.get())
    }
}

impl std::fmt::Debug for Gauge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gauge")
            .field("value", &self.get())
            .field("age", &self.age())
            .finish()
    }
}

/// Gauges specialised for percentages and byte counts.
pub mod specialized {
    use super::{Gauge, GaugeStats};

    /// Gauge whose value is always within `0.0..=100.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::gauge_specialized::PercentageGauge;
    ///
    /// let cpu = PercentageGauge::new();
    /// cpu.set_percentage(150.0);
    /// assert_eq!(cpu.get_percentage(), 100.0);
    /// cpu.set_ratio(0.25);
    /// assert_eq!(cpu.get_percentage(), 25.0);
    /// ```
    #[repr(align(64))]
    #[derive(Default)]
    pub struct PercentageGauge {
        inner: Gauge,
    }

    impl PercentageGauge {
        /// Create a gauge at 0 %.
        #[must_use]
        pub fn new() -> Self {
            Self {
                inner: Gauge::new(),
            }
        }

        /// Set the percentage, clamped to `0.0..=100.0`. Ignored for NaN.
        #[inline]
        pub fn set_percentage(&self, percentage: f64) {
            if !percentage.is_nan() {
                self.inner.set(percentage.clamp(0.0, 100.0));
            }
        }

        /// Current percentage.
        #[inline]
        #[must_use]
        pub fn get_percentage(&self) -> f64 {
            self.inner.get()
        }

        /// Set from a ratio (`0.0..=1.0` maps to `0..=100` %). Ignored for NaN.
        #[inline]
        pub fn set_ratio(&self, ratio: f64) {
            self.set_percentage(ratio * 100.0);
        }

        /// Current value as a ratio (`0.0..=1.0`).
        #[inline]
        #[must_use]
        pub fn get_ratio(&self) -> f64 {
            self.inner.get() / 100.0
        }

        /// `true` at exactly 100 %.
        #[inline]
        #[must_use]
        pub fn is_full(&self) -> bool {
            self.inner.get() >= 100.0
        }

        /// `true` at exactly 0 %.
        #[inline]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.inner.get() <= 0.0
        }

        /// Add `delta` percentage points, clamping the result to
        /// `0.0..=100.0`. Ignored for a non-finite `delta`.
        #[inline]
        pub fn add_percentage(&self, delta: f64) {
            if delta.is_finite() {
                let _ = self.inner.update(|v| {
                    Some((v + delta).clamp(0.0, 100.0)).filter(|n| n.to_bits() != v.to_bits())
                });
            }
        }

        /// Statistics of the underlying gauge.
        #[must_use]
        pub fn stats(&self) -> GaugeStats {
            self.inner.stats()
        }
    }

    impl std::fmt::Display for PercentageGauge {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PercentageGauge({}%)", self.get_percentage())
        }
    }

    /// CPU usage gauge (a [`PercentageGauge`]).
    pub type CpuGauge = PercentageGauge;

    /// Gauge holding a byte count.
    ///
    /// Stored as `f64`: exact up to 2^53 bytes (8 PiB).
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::gauge_specialized::MemoryGauge;
    ///
    /// let heap = MemoryGauge::new();
    /// heap.set_bytes(3 * 1024 * 1024);
    /// assert_eq!(heap.get_mb(), 3.0);
    /// ```
    #[repr(align(64))]
    #[derive(Default)]
    pub struct MemoryGauge {
        bytes: Gauge,
    }

    impl MemoryGauge {
        /// Create a gauge at zero bytes.
        #[must_use]
        pub fn new() -> Self {
            Self {
                bytes: Gauge::new(),
            }
        }

        /// Set the byte count.
        #[inline]
        #[allow(clippy::cast_precision_loss)] // documented: exact up to 2^53 bytes
        pub fn set_bytes(&self, bytes: u64) {
            self.bytes.set(bytes as f64);
        }

        /// Current byte count (saturating at `0` and `u64::MAX`).
        #[inline]
        #[must_use]
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // saturating float-to-int cast is the intent
        pub fn get_bytes(&self) -> u64 {
            self.bytes.get() as u64
        }

        /// Current size in KiB.
        #[inline]
        #[must_use]
        pub fn get_kb(&self) -> f64 {
            self.bytes.get() / 1024.0
        }

        /// Current size in MiB.
        #[inline]
        #[must_use]
        pub fn get_mb(&self) -> f64 {
            self.bytes.get() / (1024.0 * 1024.0)
        }

        /// Current size in GiB.
        #[inline]
        #[must_use]
        pub fn get_gb(&self) -> f64 {
            self.bytes.get() / (1024.0 * 1024.0 * 1024.0)
        }

        /// Add (or, when negative, subtract) `bytes`.
        #[inline]
        #[allow(clippy::cast_precision_loss)] // documented: exact up to 2^53 bytes
        pub fn add_bytes(&self, bytes: i64) {
            self.bytes.add(bytes as f64);
        }

        /// Statistics of the underlying gauge.
        #[must_use]
        pub fn stats(&self) -> GaugeStats {
            self.bytes.stats()
        }
    }

    impl std::fmt::Display for MemoryGauge {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mb = self.get_mb();
            if mb >= 1024.0 {
                write!(f, "MemoryGauge({:.2} GB)", self.get_gb())
            } else {
                write!(f, "MemoryGauge({mb:.2} MB)")
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact comparisons of exactly representable values
mod tests {
    use super::specialized::{MemoryGauge, PercentageGauge};
    use super::*;
    use proptest::prelude::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_gauge_basic_operations() {
        let gauge = Gauge::new();
        assert!(gauge.is_zero());
        gauge.set(42.5);
        assert!(gauge.is_positive());
        gauge.add(7.5);
        gauge.sub(10.0);
        assert_eq!(gauge.get(), 40.0);
        gauge.reset();
        assert_eq!(gauge.get(), 0.0);
        gauge.set(-1.0);
        assert!(gauge.is_negative());
    }

    #[test]
    fn test_gauge_arithmetic() {
        let gauge = Gauge::with_value(10.0);
        gauge.multiply(2.0);
        gauge.divide(4.0);
        assert_eq!(gauge.get(), 5.0);
        gauge.set(-15.0);
        gauge.abs();
        assert_eq!(gauge.get(), 15.0);
        gauge.clamp(5.0, 10.0);
        assert_eq!(gauge.get(), 10.0);
        gauge.multiply(1.0);
        gauge.divide(1.0);
        assert_eq!(gauge.get(), 10.0);
    }

    #[test]
    fn test_gauge_min_max() {
        let gauge = Gauge::with_value(10.0);
        gauge.set_max(15.0);
        gauge.set_max(12.0);
        assert_eq!(gauge.get(), 15.0);
        gauge.set_min(8.0);
        gauge.set_min(12.0);
        assert_eq!(gauge.get(), 8.0);
        assert!(gauge.try_set_min(1.0).is_ok());
        assert_eq!(gauge.get(), 1.0);
        assert!(gauge.try_set_max(f64::INFINITY).is_err());
        assert!(gauge.try_set_min(f64::NAN).is_err());
    }

    #[test]
    fn test_gauge_set_max_min_ignore_nan_regression_ml015() {
        // ML-015: set_max(NaN) used to store NaN and poison every later add.
        let gauge = Gauge::with_value(1.0);
        gauge.set_max(f64::NAN);
        gauge.set_min(f64::NAN);
        gauge.set_max(f64::INFINITY);
        gauge.set_min(f64::NEG_INFINITY);
        assert_eq!(gauge.get(), 1.0);
        gauge.add(1.0);
        assert_eq!(gauge.get(), 2.0);
    }

    #[test]
    fn test_gauge_update_ema_ignores_non_finite_regression_ml015() {
        let gauge = Gauge::with_value(10.0);
        gauge.update_ema(f64::NAN, 0.5);
        gauge.update_ema(f64::INFINITY, 0.5);
        gauge.update_ema(20.0, f64::NAN);
        assert_eq!(gauge.get(), 10.0);
        gauge.update_ema(20.0, 0.5);
        assert_eq!(gauge.get(), 15.0);
        gauge.update_ema(30.0, 0.3);
        assert_eq!(gauge.get(), 19.5);
        // A NaN gauge is replaced by the sample.
        gauge.set(f64::NAN);
        gauge.update_ema(3.0, 0.1);
        assert_eq!(gauge.get(), 3.0);
    }

    #[test]
    fn test_gauge_ema_alpha_clamped() {
        let gauge = Gauge::with_value(10.0);
        gauge.update_ema(100.0, -1.0);
        assert_eq!(gauge.get(), 10.0);
        gauge.update_ema(100.0, 2.0);
        assert_eq!(gauge.get(), 100.0);
    }

    #[test]
    fn test_gauge_nan_is_replaced_by_set_max_and_set_min() {
        let gauge = Gauge::with_value(f64::NAN);
        gauge.set_max(3.0);
        assert_eq!(gauge.get(), 3.0);
        gauge.set(f64::NAN);
        gauge.set_min(4.0);
        assert_eq!(gauge.get(), 4.0);
    }

    #[test]
    fn test_gauge_clamp_invalid_bounds_do_not_panic_regression_ml011() {
        let gauge = Gauge::with_value(5.0);
        gauge.clamp(10.0, 1.0);
        gauge.clamp(f64::NAN, 1.0);
        gauge.clamp(0.0, f64::NAN);
        assert_eq!(gauge.get(), 5.0);
        let nan = Gauge::with_value(f64::NAN);
        nan.clamp(0.0, 1.0);
        assert!(nan.get().is_nan());
    }

    #[test]
    fn test_gauge_clamp_handles_tiny_magnitudes_regression_ml022() {
        let gauge = Gauge::with_value(1e-20);
        gauge.clamp(1e-17, 1e-16);
        assert_eq!(gauge.get(), 1e-17);
    }

    #[test]
    fn test_gauge_compare_and_swap() {
        let gauge = Gauge::with_value(10.0);
        assert_eq!(gauge.compare_and_swap(10.0, 20.0), Ok(10.0));
        assert_eq!(gauge.compare_and_swap(10.0, 30.0), Err(20.0));
    }

    #[test]
    fn test_gauge_try_add_validation_and_overflow() {
        let gauge = Gauge::new();
        assert!(matches!(
            gauge.try_add(f64::NAN),
            Err(MetricsError::InvalidValue { .. })
        ));
        let big = Gauge::with_value(f64::MAX / 2.0);
        assert_eq!(big.try_add(f64::MAX), Err(MetricsError::Overflow));
        assert!(gauge.try_sub(1.0).is_ok());
        assert_eq!(gauge.get(), -1.0);
        assert!(gauge.try_set(f64::NAN).is_err());
    }

    #[test]
    fn test_gauge_set_stores_non_finite_values_as_documented() {
        let gauge = Gauge::new();
        gauge.set(f64::INFINITY);
        assert!(!gauge.is_finite());
        gauge.set(f64::NAN);
        assert!(gauge.get().is_nan());
        gauge.add(1.0);
        assert!(gauge.get().is_nan());
    }

    #[test]
    fn test_gauge_non_finite_helpers_are_noops() {
        let gauge = Gauge::with_value(12.0);
        gauge.add(f64::NAN);
        gauge.multiply(f64::INFINITY);
        gauge.divide(0.0);
        gauge.divide(f64::NAN);
        assert_eq!(gauge.get(), 12.0);
        let huge = Gauge::with_value(f64::MAX / 2.0);
        huge.multiply(4.0);
        assert_eq!(huge.get(), f64::MAX / 2.0);
    }

    #[test]
    fn test_gauge_stats_and_formatting() {
        let gauge = Gauge::with_value(42.5);
        assert_eq!(gauge.stats().value, 42.5);
        assert!(format!("{gauge}").contains("42.5"));
        assert!(format!("{gauge:?}").contains("Gauge"));
        assert_eq!(Gauge::default().get(), 0.0);
    }

    #[test]
    fn test_percentage_gauge() {
        let gauge = PercentageGauge::new();
        gauge.set_percentage(75.5);
        assert!((gauge.get_ratio() - 0.755).abs() < 1e-12);
        gauge.set_ratio(0.9);
        assert_eq!(gauge.get_percentage(), 90.0);
        gauge.set_percentage(150.0);
        assert!(gauge.is_full());
        gauge.set_percentage(-10.0);
        assert!(gauge.is_empty());
        gauge.set_percentage(f64::NAN);
        assert_eq!(gauge.get_percentage(), 0.0);
        gauge.set_percentage(95.0);
        gauge.add_percentage(10.0);
        assert_eq!(gauge.get_percentage(), 100.0);
        gauge.add_percentage(f64::NAN);
        assert_eq!(gauge.get_percentage(), 100.0);
        assert!(gauge.to_string().contains("100"));
        assert_eq!(gauge.stats().value, 100.0);
    }

    #[test]
    fn test_percentage_gauge_add_handles_tiny_deltas_regression_ml022() {
        let gauge = PercentageGauge::new();
        gauge.add_percentage(1e-20);
        assert_eq!(gauge.get_percentage(), 1e-20);
    }

    #[test]
    fn test_memory_gauge() {
        let gauge = MemoryGauge::new();
        gauge.set_bytes(1024 * 1024 * 1024);
        assert_eq!(gauge.get_bytes(), 1024 * 1024 * 1024);
        assert_eq!(gauge.get_gb(), 1.0);
        assert_eq!(gauge.get_mb(), 1024.0);
        assert_eq!(gauge.get_kb(), 1024.0 * 1024.0);
        assert!(gauge.to_string().contains("GB"));
        gauge.add_bytes(-(1024 * 1024 * 1023));
        assert!(gauge.to_string().contains("MB"));
        assert_eq!(gauge.stats().value, 1024.0 * 1024.0);
    }

    #[test]
    fn test_gauge_concurrent_adds_are_exact() {
        let gauge = Arc::new(Gauge::new());
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let gauge = Arc::clone(&gauge);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    for _ in 0..10_000 {
                        gauge.add(1.0);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(gauge.get(), 80_000.0);
    }

    proptest! {
        #[test]
        fn prop_unchecked_ops_never_produce_non_finite_from_finite(
            start in -1e300f64..1e300,
            args in proptest::collection::vec(any::<f64>(), 1..20)
        ) {
            let g = Gauge::with_value(start);
            for (i, a) in args.iter().enumerate() {
                match i % 7 {
                    0 => g.add(*a),
                    1 => g.set_max(*a),
                    2 => g.set_min(*a),
                    3 => g.multiply(*a),
                    4 => g.divide(*a),
                    5 => g.update_ema(*a, 0.5),
                    _ => g.clamp(*a, a.abs()),
                }
                prop_assert!(g.get().is_finite());
            }
        }

        #[test]
        fn prop_clamp_never_panics(v in any::<f64>(), lo in any::<f64>(), hi in any::<f64>()) {
            let g = Gauge::with_value(v);
            g.clamp(lo, hi);
            if !lo.is_nan() && !hi.is_nan() && lo <= hi && !v.is_nan() {
                prop_assert!(g.get() >= lo && g.get() <= hi);
            }
        }
    }
}
