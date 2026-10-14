//! Signed counter that may go up or down.
//!
//! [`UpDownCounter`] tracks an integer quantity that rises and falls: items
//! in a queue, open connections, in-flight requests. Unlike a
//! [`Gauge`](crate::Gauge) it is integral and exact, and unlike a
//! [`Counter`](crate::Counter) it may decrease. Exporters render it as a
//! gauge (Prometheus/OpenMetrics) or a non-monotonic sum (OTLP).

use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicI64, Ordering};

/// Lock-free signed `i64` counter.
///
/// The unchecked operations wrap on overflow; the `try_*` operations return
/// [`MetricsError::Overflow`] / [`MetricsError::Underflow`] instead.
///
/// # Examples
///
/// ```
/// use metrics_lib::UpDownCounter;
///
/// let in_flight = UpDownCounter::new();
/// in_flight.inc();
/// in_flight.inc();
/// in_flight.dec();
/// assert_eq!(in_flight.get(), 1);
/// ```
#[repr(align(64))]
#[derive(Default)]
pub struct UpDownCounter {
    value: AtomicI64,
}

impl UpDownCounter {
    /// Create a counter at zero.
    #[must_use]
    pub const fn new() -> Self {
        Self::with_value(0)
    }

    /// Create a counter at `initial`.
    #[must_use]
    pub const fn with_value(initial: i64) -> Self {
        Self {
            value: AtomicI64::new(initial),
        }
    }

    /// Increment by one. Wraps on overflow.
    #[inline]
    pub fn inc(&self) {
        let _ = self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement by one. Wraps on underflow.
    #[inline]
    pub fn dec(&self) {
        let _ = self.value.fetch_sub(1, Ordering::Relaxed);
    }

    /// Add `delta` (may be negative). Wraps on overflow.
    #[inline]
    pub fn add(&self, delta: i64) {
        let _ = self.value.fetch_add(delta, Ordering::Relaxed);
    }

    /// Subtract `delta` (may be negative). Wraps on overflow.
    #[inline]
    pub fn sub(&self, delta: i64) {
        let _ = self.value.fetch_sub(delta, Ordering::Relaxed);
    }

    /// Add `delta` unless the result would leave the `i64` range.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when the sum would exceed
    /// `i64::MAX` and [`MetricsError::Underflow`] when it would go below
    /// `i64::MIN`. The value is unchanged on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{MetricsError, UpDownCounter};
    ///
    /// let c = UpDownCounter::with_value(i64::MIN + 1);
    /// assert!(c.try_add(-1).is_ok());
    /// assert_eq!(c.try_add(-1), Err(MetricsError::Underflow));
    /// ```
    pub fn try_add(&self, delta: i64) -> Result<()> {
        self.value
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                v.checked_add(delta)
            })
            .map(|_| ())
            .map_err(|_| {
                if delta > 0 {
                    MetricsError::Overflow
                } else {
                    MetricsError::Underflow
                }
            })
    }

    /// Subtract `delta` unless the result would leave the `i64` range.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Underflow`] when the result would go below
    /// `i64::MIN` and [`MetricsError::Overflow`] when it would exceed
    /// `i64::MAX` (subtracting a negative delta). The value is unchanged on
    /// error.
    pub fn try_sub(&self, delta: i64) -> Result<()> {
        self.value
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                v.checked_sub(delta)
            })
            .map(|_| ())
            .map_err(|_| {
                if delta > 0 {
                    MetricsError::Underflow
                } else {
                    MetricsError::Overflow
                }
            })
    }

    /// Current value.
    #[inline]
    #[must_use]
    pub fn get(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Overwrite the value.
    #[inline]
    pub fn set(&self, value: i64) {
        self.value.store(value, Ordering::Relaxed);
    }

    /// Reset to zero.
    #[inline]
    pub fn reset(&self) {
        self.set(0);
    }
}

impl std::fmt::Debug for UpDownCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpDownCounter")
            .field("value", &self.get())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::{Arc, Barrier};

    #[test]
    fn test_updown_basic_ops() {
        let c = UpDownCounter::new();
        c.inc();
        c.add(10);
        c.dec();
        c.sub(3);
        assert_eq!(c.get(), 7);
        c.add(-20);
        assert_eq!(c.get(), -13);
        c.set(5);
        assert_eq!(c.get(), 5);
        c.reset();
        assert_eq!(c.get(), 0);
        assert!(format!("{c:?}").contains("UpDownCounter"));
        assert_eq!(UpDownCounter::default().get(), 0);
    }

    #[test]
    fn test_updown_checked_bounds() {
        let c = UpDownCounter::with_value(i64::MAX);
        assert_eq!(c.try_add(1), Err(MetricsError::Overflow));
        assert_eq!(c.try_sub(-1), Err(MetricsError::Overflow));
        assert_eq!(c.get(), i64::MAX);
        let c = UpDownCounter::with_value(i64::MIN);
        assert_eq!(c.try_add(-1), Err(MetricsError::Underflow));
        assert_eq!(c.try_sub(1), Err(MetricsError::Underflow));
        assert!(c.try_sub(-1).is_ok());
        assert_eq!(c.get(), i64::MIN + 1);
    }

    #[test]
    fn test_updown_concurrent_balanced_updates_return_to_zero() {
        let c = Arc::new(UpDownCounter::new());
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|i| {
                let c = Arc::clone(&c);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    for _ in 0..20_000 {
                        if i % 2 == 0 { c.inc() } else { c.dec() }
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(c.get(), 0);
    }

    proptest! {
        #[test]
        fn prop_try_add_matches_checked_add(start in any::<i64>(), delta in any::<i64>()) {
            let c = UpDownCounter::with_value(start);
            let result = c.try_add(delta);
            match start.checked_add(delta) {
                Some(v) => { prop_assert!(result.is_ok()); prop_assert_eq!(c.get(), v); }
                None => { prop_assert!(result.is_err()); prop_assert_eq!(c.get(), start); }
            }
        }

        #[test]
        fn prop_try_sub_matches_checked_sub(start in any::<i64>(), delta in any::<i64>()) {
            let c = UpDownCounter::with_value(start);
            let result = c.try_sub(delta);
            match start.checked_sub(delta) {
                Some(v) => { prop_assert!(result.is_ok()); prop_assert_eq!(c.get(), v); }
                None => { prop_assert!(result.is_err()); prop_assert_eq!(c.get(), start); }
            }
        }
    }
}
