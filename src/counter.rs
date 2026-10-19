//! Monotonic `u64` counter.
//!
//! [`Counter::inc`] and [`Counter::add`] compile to one atomic
//! read-modify-write (`lock xadd` on x86-64) with `Relaxed` ordering. On the
//! reference machine (see `docs/PERFORMANCE.md`) an uncontended `inc` costs
//! about 4.5 ns; under heavy multi-thread contention on one counter, prefer
//! [`ShardedCounter`](crate::ShardedCounter).
//!
//! # Overflow
//!
//! The unchecked operations wrap at `u64::MAX`, exactly like
//! [`AtomicU64::fetch_add`]. At one billion increments per second that takes
//! more than 580 years. Use the `try_*` methods for an explicit
//! [`MetricsError::Overflow`], or [`Counter::saturating_add`] to clamp.
//!
//! # Memory ordering
//!
//! A counter is a statistic, not a synchronisation primitive: every
//! operation uses `Relaxed` ordering and does not order any other memory
//! access.

use crate::util::CasUpdate;
use crate::util::per_second;
use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Lock-free monotonic counter.
///
/// Cache-line aligned so two counters never share a line (no false sharing).
///
/// # Examples
///
/// ```
/// use metrics_lib::Counter;
///
/// let requests = Counter::new();
/// requests.inc();
/// requests.add(4);
/// assert_eq!(requests.get(), 5);
/// ```
#[repr(align(64))]
pub struct Counter {
    value: AtomicU64,
    created_at: Instant,
}

/// Point-in-time statistics for a [`Counter`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct CounterStats {
    /// Current value.
    pub value: u64,
    /// Time since the counter was created.
    pub age: Duration,
    /// Average increments per second since creation.
    pub rate_per_second: f64,
}

impl Counter {
    /// Create a counter starting at zero.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::with_value(0)
    }

    /// Create a counter starting at `initial`.
    #[inline]
    #[must_use]
    pub fn with_value(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
            created_at: Instant::now(),
        }
    }

    /// Increment by one. Wraps at `u64::MAX`.
    #[inline]
    pub fn inc(&self) {
        let _ = self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Add `amount`. Wraps at `u64::MAX`.
    #[inline]
    pub fn add(&self, amount: u64) {
        let _ = self.value.fetch_add(amount, Ordering::Relaxed);
    }

    /// Increment by one unless that would overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when the counter is at `u64::MAX`;
    /// the value is unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    ///
    /// let c = Counter::with_value(u64::MAX - 1);
    /// c.try_inc().unwrap();
    /// assert_eq!(c.get(), u64::MAX);
    /// assert_eq!(c.try_inc(), Err(MetricsError::Overflow));
    /// ```
    #[inline]
    pub fn try_inc(&self) -> Result<()> {
        self.try_add(1)
    }

    /// Add `amount` unless that would overflow.
    ///
    /// The overflow check and the addition form one atomic step (a CAS loop),
    /// so concurrent callers can never push the value past `u64::MAX`.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when `value + amount` would exceed
    /// `u64::MAX`; the value is unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    ///
    /// let c = Counter::with_value(u64::MAX - 5);
    /// assert!(c.try_add(4).is_ok());
    /// assert_eq!(c.try_add(2), Err(MetricsError::Overflow));
    /// ```
    #[inline]
    pub fn try_add(&self, amount: u64) -> Result<()> {
        self.try_fetch_add(amount).map(|_| ())
    }

    /// Add `amount` and return the previous value, unless that would
    /// overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when `value + amount` would exceed
    /// `u64::MAX`; the value is unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    ///
    /// let c = Counter::with_value(u64::MAX - 1);
    /// assert_eq!(c.try_fetch_add(1), Ok(u64::MAX - 1));
    /// assert_eq!(c.try_fetch_add(1), Err(MetricsError::Overflow));
    /// ```
    #[inline]
    pub fn try_fetch_add(&self, amount: u64) -> Result<u64> {
        self.value
            .cas_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(amount)
            })
            .map_err(|_| MetricsError::Overflow)
    }

    /// Increment by one and return the new value, unless that would
    /// overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when the counter is at `u64::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    ///
    /// let c = Counter::with_value(u64::MAX - 1);
    /// assert_eq!(c.try_inc_and_get(), Ok(u64::MAX));
    /// assert_eq!(c.try_inc_and_get(), Err(MetricsError::Overflow));
    /// ```
    #[inline]
    pub fn try_inc_and_get(&self) -> Result<u64> {
        self.try_fetch_add(1).map(|prev| prev + 1)
    }

    /// Current value.
    #[inline]
    #[must_use]
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Reset to zero.
    #[inline]
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }

    /// Overwrite the value.
    ///
    /// A counter that moves backwards looks like a reset to Prometheus and
    /// OTLP consumers; use a [`Gauge`](crate::Gauge) or an
    /// [`UpDownCounter`](crate::UpDownCounter) for values that go down.
    #[inline]
    pub fn set(&self, value: u64) {
        self.value.store(value, Ordering::Relaxed);
    }

    /// Raise the value to `value` if it is currently lower; never lowers it.
    ///
    /// Useful for mirroring an external monotonic counter from several
    /// threads that may observe it out of order.
    #[inline]
    pub fn set_max(&self, value: u64) {
        let _ = self.value.fetch_max(value, Ordering::Relaxed);
    }

    /// Replace the value with `new` if it currently equals `expected`.
    ///
    /// # Errors
    ///
    /// Returns `Err(current)` with the actual value when it did not equal
    /// `expected`.
    #[inline]
    pub fn compare_and_swap(&self, expected: u64, new: u64) -> core::result::Result<u64, u64> {
        self.value
            .compare_exchange(expected, new, Ordering::Relaxed, Ordering::Relaxed)
    }

    /// Add `amount` and return the previous value. Wraps at `u64::MAX`.
    #[inline]
    #[must_use = "use `add` when the previous value is not needed"]
    pub fn fetch_add(&self, amount: u64) -> u64 {
        self.value.fetch_add(amount, Ordering::Relaxed)
    }

    /// Add `amount` and return the new value. Wraps at `u64::MAX`.
    #[inline]
    #[must_use = "use `add` when the new value is not needed"]
    pub fn add_and_get(&self, amount: u64) -> u64 {
        self.value
            .fetch_add(amount, Ordering::Relaxed)
            .wrapping_add(amount)
    }

    /// Increment by one and return the new value. Wraps at `u64::MAX`.
    #[inline]
    #[must_use = "use `inc` when the new value is not needed"]
    pub fn inc_and_get(&self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
    }

    /// Add `amount`, clamping at `u64::MAX` instead of wrapping.
    #[inline]
    pub fn saturating_add(&self, amount: u64) {
        let _ = self
            .value
            .cas_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                let next = current.saturating_add(amount);
                (next != current).then_some(next)
            });
    }

    /// Add `count` events (a `usize`, as returned by `len()`). Wraps at
    /// `u64::MAX`.
    #[inline]
    pub fn batch_inc(&self, count: usize) {
        if count > 0 {
            self.add(u64::try_from(count).unwrap_or(u64::MAX));
        }
    }

    /// Increment by one when `condition` is `true`.
    #[inline]
    pub fn inc_if(&self, condition: bool) {
        if condition {
            self.inc();
        }
    }

    /// Increment by one only while the value is below `max_value`.
    ///
    /// Returns `true` when the increment was applied. The check and the
    /// increment are one atomic step.
    #[inline]
    pub fn inc_max(&self, max_value: u64) -> bool {
        self.value
            .cas_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                (current < max_value).then(|| current + 1)
            })
            .is_ok()
    }

    /// Point-in-time statistics.
    #[must_use]
    pub fn stats(&self) -> CounterStats {
        let value = self.get();
        let age = self.age();
        CounterStats {
            value,
            age,
            rate_per_second: per_second(value, age),
        }
    }

    /// Time since the counter was created.
    #[inline]
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// `true` when the value is zero.
    #[inline]
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.get() == 0
    }

    /// Average increments per second since creation.
    #[inline]
    #[must_use]
    pub fn rate_per_second(&self) -> f64 {
        per_second(self.get(), self.age())
    }
}

impl Default for Counter {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Counter({})", self.get())
    }
}

impl std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Counter")
            .field("value", &self.get())
            .field("age", &self.age())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_counter_basic_operations() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
        assert!(counter.is_zero());
        counter.inc();
        counter.add(5);
        assert_eq!(counter.get(), 6);
        counter.reset();
        assert_eq!(counter.get(), 0);
        counter.set(42);
        assert_eq!(counter.get(), 42);
        assert!(!counter.is_zero());
    }

    #[test]
    fn test_counter_fetch_variants_return_previous_and_new() {
        let counter = Counter::new();
        assert_eq!(counter.fetch_add(10), 0);
        assert_eq!(counter.inc_and_get(), 11);
        assert_eq!(counter.add_and_get(5), 16);
    }

    #[test]
    fn test_counter_set_max_only_raises() {
        let counter = Counter::with_value(10);
        counter.set_max(5);
        assert_eq!(counter.get(), 10);
        counter.set_max(15);
        assert_eq!(counter.get(), 15);
    }

    #[test]
    fn test_counter_compare_and_swap() {
        let counter = Counter::with_value(10);
        assert_eq!(counter.compare_and_swap(10, 20), Ok(10));
        assert_eq!(counter.compare_and_swap(10, 30), Err(20));
        assert_eq!(counter.get(), 20);
    }

    #[test]
    fn test_counter_saturating_add_clamps_at_max() {
        let counter = Counter::with_value(u64::MAX - 5);
        counter.saturating_add(10);
        assert_eq!(counter.get(), u64::MAX);
        counter.saturating_add(100);
        assert_eq!(counter.get(), u64::MAX);
        counter.saturating_add(0);
        assert_eq!(counter.get(), u64::MAX);
    }

    #[test]
    fn test_counter_unchecked_ops_wrap_at_max() {
        let counter = Counter::with_value(u64::MAX);
        assert_eq!(counter.inc_and_get(), 0);
        let counter = Counter::with_value(u64::MAX - 2);
        assert_eq!(counter.add_and_get(5), 2);
    }

    #[test]
    fn test_counter_conditional_operations() {
        let counter = Counter::new();
        counter.inc_if(true);
        counter.inc_if(false);
        assert_eq!(counter.get(), 1);
        assert!(counter.inc_max(5));
        assert_eq!(counter.get(), 2);
        counter.set(5);
        assert!(!counter.inc_max(5));
        assert_eq!(counter.get(), 5);
        counter.set(u64::MAX);
        assert!(!counter.inc_max(u64::MAX));
    }

    #[test]
    fn test_counter_batch_inc() {
        let counter = Counter::new();
        counter.batch_inc(1000);
        counter.batch_inc(0);
        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_counter_checked_operations_and_overflow_paths() {
        let counter = Counter::new();
        counter.try_inc().unwrap();
        counter.try_add(0).unwrap();
        assert_eq!(counter.try_fetch_add(2), Ok(1));
        assert_eq!(counter.try_inc_and_get(), Ok(4));

        let full = Counter::with_value(u64::MAX);
        assert_eq!(full.try_inc(), Err(MetricsError::Overflow));
        assert_eq!(full.try_add(1), Err(MetricsError::Overflow));
        assert_eq!(full.try_fetch_add(1), Err(MetricsError::Overflow));
        assert_eq!(full.try_inc_and_get(), Err(MetricsError::Overflow));
        assert_eq!(full.try_add(0), Ok(()));
        assert_eq!(full.get(), u64::MAX);
    }

    #[test]
    fn test_counter_stats_and_formatting() {
        let counter = Counter::new();
        counter.add(100);
        let stats = counter.stats();
        assert_eq!(stats.value, 100);
        assert!(stats.rate_per_second >= 0.0);
        assert!(counter.rate_per_second() >= 0.0);
        assert!(format!("{counter}").contains("100"));
        assert!(format!("{counter:?}").contains("Counter"));
        assert_eq!(Counter::default().get(), 0);
    }

    #[test]
    fn test_counter_concurrent_increments_are_exact() {
        let counter = Arc::new(Counter::new());
        let threads = 8;
        let per_thread = 50_000u64;
        let barrier = Arc::new(Barrier::new(threads));
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let counter = Arc::clone(&counter);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    for _ in 0..per_thread {
                        counter.inc();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.get(), threads as u64 * per_thread);
    }

    #[test]
    fn test_counter_concurrent_try_add_never_passes_max() {
        let counter = Arc::new(Counter::with_value(u64::MAX - 1000));
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let counter = Arc::clone(&counter);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    (0..500).filter(|_| counter.try_add(1).is_ok()).count()
                })
            })
            .collect();
        let applied: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert_eq!(applied, 1000);
        assert_eq!(counter.get(), u64::MAX);
    }

    proptest! {
        #[test]
        fn prop_try_add_matches_checked_add(start in any::<u64>(), amount in any::<u64>()) {
            let counter = Counter::with_value(start);
            match start.checked_add(amount) {
                Some(sum) => {
                    prop_assert_eq!(counter.try_add(amount), Ok(()));
                    prop_assert_eq!(counter.get(), sum);
                }
                None => {
                    prop_assert_eq!(counter.try_add(amount), Err(MetricsError::Overflow));
                    prop_assert_eq!(counter.get(), start);
                }
            }
        }

        #[test]
        fn prop_saturating_add_matches_std(start in any::<u64>(), amount in any::<u64>()) {
            let counter = Counter::with_value(start);
            counter.saturating_add(amount);
            prop_assert_eq!(counter.get(), start.saturating_add(amount));
        }

        #[test]
        fn prop_add_wraps_like_std(start in any::<u64>(), amount in any::<u64>()) {
            let counter = Counter::with_value(start);
            counter.add(amount);
            prop_assert_eq!(counter.get(), start.wrapping_add(amount));
        }
    }
}
