//! # Ultra-Fast Atomic Counter
//!
//! The fastest counter implementation possible - sub-3ns increments.
//!
//! ## Features
//!
//! - **Sub-3ns increments** - Single atomic instruction
//! - **Zero allocations** - Pure stack operations
//! - **Lock-free** - Never blocks, never waits
//! - **Cache optimized** - Aligned to prevent false sharing
//! - **Overflow safe** - Handles u64::MAX gracefully

use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Ultra-fast atomic counter
///
/// Optimized for maximum throughput with minimal memory overhead.
/// Cache-line aligned to prevent false sharing.
#[repr(align(64))]
pub struct Counter {
    /// Main counter value
    value: AtomicU64,
    /// Creation timestamp for rate calculations
    created_at: Instant,
}

/// Counter statistics
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CounterStats {
    /// Current counter value
    pub value: u64,
    /// Time since counter creation
    pub age: Duration,
    /// Average increments per second since creation
    pub rate_per_second: f64,
    /// Total increments (same as value for basic counter)
    pub total: u64,
}

impl Counter {
    /// Create new counter starting at zero
    #[inline]
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
            created_at: Instant::now(),
        }
    }

    /// Create counter with initial value
    #[inline]
    pub fn with_value(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
            created_at: Instant::now(),
        }
    }

    /// Increment by 1 - THE FASTEST PATH
    ///
    /// This is optimized to be as fast as physically possible:
    /// - Single atomic fetch_add instruction
    /// - Relaxed memory ordering for maximum speed
    /// - Inlined for zero function call overhead
    #[inline(always)]
    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Try to increment by 1 with overflow check
    ///
    /// Returns `Ok(())` on success, or `Err(MetricsError::Overflow)` if the
    /// increment would overflow `u64::MAX`.
    ///
    /// Example
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    /// let c = Counter::with_value(u64::MAX - 1);
    /// c.try_inc().unwrap();
    /// assert_eq!(c.get(), u64::MAX);
    /// assert!(matches!(c.try_inc(), Err(MetricsError::Overflow)));
    /// ```
    #[inline(always)]
    pub fn try_inc(&self) -> Result<()> {
        // CAS loop: the overflow check and the increment are together atomic,
        // so this is safe under concurrent access (no TOCTOU race).
        loop {
            let current = self.value.load(Ordering::Relaxed);
            if current == u64::MAX {
                return Err(MetricsError::Overflow);
            }
            match self.value.compare_exchange_weak(
                current,
                current + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Ok(()),
                Err(_) => continue,
            }
        }
    }

    /// Add arbitrary amount - also blazingly fast
    ///
    /// Zero branch optimization - if amount is 0, still does the atomic
    /// operation to maintain consistent performance characteristics
    #[inline(always)]
    pub fn add(&self, amount: u64) {
        self.value.fetch_add(amount, Ordering::Relaxed);
    }

    /// Try to add an arbitrary amount with overflow check
    ///
    /// Returns `Ok(())` on success, or `Err(MetricsError::Overflow)` if the
    /// addition would overflow `u64::MAX`.
    ///
    /// Example
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    /// let c = Counter::with_value(u64::MAX - 5);
    /// assert!(c.try_add(4).is_ok());
    /// assert!(matches!(c.try_add(2), Err(MetricsError::Overflow)));
    /// ```
    #[inline(always)]
    pub fn try_add(&self, amount: u64) -> Result<()> {
        if amount == 0 {
            return Ok(());
        }
        // CAS loop: overflow check and addition are together atomic.
        loop {
            let current = self.value.load(Ordering::Relaxed);
            let new_val = current.checked_add(amount).ok_or(MetricsError::Overflow)?;
            match self.value.compare_exchange_weak(
                current,
                new_val,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Ok(()),
                Err(_) => continue,
            }
        }
    }

    /// Get current value - single atomic load
    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Reset to zero - use sparingly
    ///
    /// Note: This uses SeqCst ordering to ensure all threads see the reset
    #[inline]
    pub fn reset(&self) {
        self.value.store(0, Ordering::SeqCst);
    }

    /// Set to specific value - use sparingly
    ///
    /// Note: This uses SeqCst ordering for consistency
    #[inline]
    pub fn set(&self, value: u64) {
        self.value.store(value, Ordering::SeqCst);
    }

    /// Try to set to a specific value (always succeeds for `u64`)
    ///
    /// This method never fails and always returns `Ok(())`.
    #[inline]
    pub fn try_set(&self, value: u64) -> Result<()> {
        self.set(value);
        Ok(())
    }

    /// Atomic compare-and-swap
    ///
    /// Returns Ok(previous_value) if successful, Err(current_value) if failed
    #[inline]
    pub fn compare_and_swap(&self, expected: u64, new: u64) -> core::result::Result<u64, u64> {
        match self
            .value
            .compare_exchange(expected, new, Ordering::SeqCst, Ordering::SeqCst)
        {
            Ok(prev) => Ok(prev),
            Err(current) => Err(current),
        }
    }

    /// Add amount and return previous value
    #[must_use]
    #[inline]
    pub fn fetch_add(&self, amount: u64) -> u64 {
        self.value.fetch_add(amount, Ordering::Relaxed)
    }

    /// Checked fetch_add that returns the previous value or error on overflow
    ///
    /// Returns the previous value on success, or `Err(MetricsError::Overflow)`
    /// if adding `amount` would overflow `u64::MAX`.
    ///
    /// Example
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    /// let c = Counter::with_value(u64::MAX - 1);
    /// assert_eq!(c.try_fetch_add(1).unwrap(), u64::MAX - 1);
    /// assert!(matches!(c.try_fetch_add(1), Err(MetricsError::Overflow)));
    /// ```
    #[inline]
    pub fn try_fetch_add(&self, amount: u64) -> Result<u64> {
        if amount == 0 {
            return Ok(self.get());
        }
        // CAS loop: overflow check and fetch_add are together atomic.
        loop {
            let current = self.value.load(Ordering::Relaxed);
            let new_val = current.checked_add(amount).ok_or(MetricsError::Overflow)?;
            match self.value.compare_exchange_weak(
                current,
                new_val,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(prev) => return Ok(prev),
                Err(_) => continue,
            }
        }
    }

    /// Add amount and return new value
    ///
    /// On overflow the returned value wraps modulo `u64::MAX + 1`, matching
    /// the underlying [`AtomicU64::fetch_add`] semantics (no panic in debug or
    /// release builds). Use [`Counter::try_fetch_add`] for an explicit
    /// `MetricsError::Overflow` instead.
    #[must_use]
    #[inline]
    pub fn add_and_get(&self, amount: u64) -> u64 {
        self.value
            .fetch_add(amount, Ordering::Relaxed)
            .wrapping_add(amount)
    }

    /// Increment and return new value
    ///
    /// On overflow the returned value wraps to `0`, matching the underlying
    /// [`AtomicU64::fetch_add`] semantics (no panic in debug or release
    /// builds). Use [`Counter::try_inc_and_get`] for an explicit
    /// `MetricsError::Overflow` instead.
    #[must_use]
    #[inline]
    pub fn inc_and_get(&self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
    }

    /// Checked increment that returns new value or error on overflow
    ///
    /// Returns the new value, or `Err(MetricsError::Overflow)` if the
    /// increment would overflow `u64::MAX`.
    ///
    /// Example
    /// ```
    /// use metrics_lib::{Counter, MetricsError};
    /// let c = Counter::with_value(u64::MAX - 1);
    /// assert_eq!(c.try_inc_and_get().unwrap(), u64::MAX);
    /// assert!(matches!(c.try_inc_and_get(), Err(MetricsError::Overflow)));
    /// ```
    #[inline]
    pub fn try_inc_and_get(&self) -> Result<u64> {
        // CAS loop: the returned value is the exact new value after the atomic
        // increment — correct under concurrent access, unlike a load+store pattern.
        loop {
            let current = self.value.load(Ordering::Relaxed);
            let new_val = current.checked_add(1).ok_or(MetricsError::Overflow)?;
            match self.value.compare_exchange_weak(
                current,
                new_val,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Ok(new_val),
                Err(_) => continue,
            }
        }
    }

    /// Get comprehensive statistics
    #[must_use]
    pub fn stats(&self) -> CounterStats {
        let value = self.get();
        let age = self.created_at.elapsed();
        let age_seconds = age.as_secs_f64();

        let rate_per_second = if age_seconds > 0.0 {
            value as f64 / age_seconds
        } else {
            0.0
        };

        CounterStats {
            value,
            age,
            rate_per_second,
            total: value,
        }
    }

    /// Get age since creation
    #[must_use]
    #[inline]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Check if counter is zero
    #[must_use]
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.get() == 0
    }

    /// Get rate per second since creation
    #[must_use]
    #[inline]
    pub fn rate_per_second(&self) -> f64 {
        let age_seconds = self.age().as_secs_f64();
        if age_seconds > 0.0 {
            self.get() as f64 / age_seconds
        } else {
            0.0
        }
    }

    /// Saturating add - won't overflow past u64::MAX
    ///
    /// Uses a `Relaxed` CAS loop on the hot path; no `SeqCst` cost. The loop
    /// terminates either when the value is already at `u64::MAX` or when a CAS
    /// succeeds.
    #[inline]
    pub fn saturating_add(&self, amount: u64) {
        loop {
            let current = self.value.load(Ordering::Relaxed);
            let new_value = current.saturating_add(amount);

            // If no change needed (already at max), break
            if new_value == current {
                break;
            }

            match self.value.compare_exchange_weak(
                current,
                new_value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
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
            .field("rate_per_second", &self.rate_per_second())
            .finish()
    }
}

// Counter is composed solely of `AtomicU64` (Send + Sync) and `Instant`
// (Send + Sync). The compiler derives Send + Sync automatically; no
// explicit unsafe impl is needed.

/// Batch counter operations for even better performance
impl Counter {
    /// Batch increment - for very high throughput scenarios
    ///
    /// When you have many increments to do, batch them for better performance
    #[inline]
    pub fn batch_inc(&self, count: usize) {
        if count > 0 {
            self.add(count as u64);
        }
    }

    /// Conditional increment - only increment if condition is true
    #[inline]
    pub fn inc_if(&self, condition: bool) {
        if condition {
            self.inc();
        }
    }

    /// Increment with maximum value
    ///
    /// Uses a `Relaxed` CAS loop on the hot path; no `SeqCst` cost. Returns
    /// `true` if the increment was applied (counter was below `max_value`),
    /// `false` if the counter was already at or above the limit.
    #[inline]
    pub fn inc_max(&self, max_value: u64) -> bool {
        loop {
            let current = self.value.load(Ordering::Relaxed);
            if current >= max_value {
                return false;
            }
            // Safe: current < max_value <= u64::MAX implies current < u64::MAX.
            let new_value = current + 1;

            match self.value.compare_exchange_weak(
                current,
                new_value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(_) => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic_operations() {
        let counter = Counter::new();

        assert_eq!(counter.get(), 0);
        assert!(counter.is_zero());

        counter.inc();
        assert_eq!(counter.get(), 1);
        assert!(!counter.is_zero());

        counter.add(5);
        assert_eq!(counter.get(), 6);

        counter.reset();
        assert_eq!(counter.get(), 0);

        counter.set(42);
        assert_eq!(counter.get(), 42);
    }

    #[test]
    fn test_fetch_operations() {
        let counter = Counter::new();

        assert_eq!(counter.fetch_add(10), 0);
        assert_eq!(counter.get(), 10);

        assert_eq!(counter.inc_and_get(), 11);
        assert_eq!(counter.add_and_get(5), 16);
    }

    #[test]
    fn test_compare_and_swap() {
        let counter = Counter::new();
        counter.set(10);

        // Successful swap
        assert_eq!(counter.compare_and_swap(10, 20), Ok(10));
        assert_eq!(counter.get(), 20);

        // Failed swap
        assert_eq!(counter.compare_and_swap(10, 30), Err(20));
        assert_eq!(counter.get(), 20);
    }

    #[test]
    fn test_saturating_add() {
        let counter = Counter::new();
        counter.set(u64::MAX - 5);

        counter.saturating_add(10);
        assert_eq!(counter.get(), u64::MAX);

        // Should not overflow
        counter.saturating_add(100);
        assert_eq!(counter.get(), u64::MAX);
    }

    #[test]
    fn test_conditional_operations() {
        let counter = Counter::new();

        counter.inc_if(true);
        assert_eq!(counter.get(), 1);

        counter.inc_if(false);
        assert_eq!(counter.get(), 1);

        // Test inc_max
        assert!(counter.inc_max(5));
        assert_eq!(counter.get(), 2);

        counter.set(5);
        assert!(!counter.inc_max(5));
        assert_eq!(counter.get(), 5);
    }

    #[test]
    fn test_statistics() {
        let counter = Counter::new();
        counter.add(100);

        let stats = counter.stats();
        assert_eq!(stats.value, 100);
        assert_eq!(stats.total, 100);
        assert!(stats.age > Duration::from_nanos(0));
        // Rate might be 0 if test runs too fast
        assert!(stats.rate_per_second >= 0.0);
    }

    #[test]
    fn test_high_concurrency() {
        let counter = Arc::new(Counter::new());
        let num_threads = 100;
        let increments_per_thread = 1000;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..increments_per_thread {
                        counter.inc();
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), num_threads * increments_per_thread);

        let stats = counter.stats();
        assert!(stats.rate_per_second > 0.0);
    }

    #[test]
    fn test_batch_operations() {
        let counter = Counter::new();

        counter.batch_inc(1000);
        assert_eq!(counter.get(), 1000);

        counter.batch_inc(0); // Should be no-op
        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_display_and_debug() {
        let counter = Counter::new();
        counter.set(42);

        let display_str = format!("{counter}");
        assert!(display_str.contains("42"));

        let debug_str = format!("{counter:?}");
        assert!(debug_str.contains("Counter"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_add_and_get_wraps_on_overflow_without_panic() {
        // 0.9.2 regression: add_and_get and inc_and_get used `+` which would
        // panic in debug builds when crossing u64::MAX. They now wrap, matching
        // the underlying fetch_add semantics.
        let counter = Counter::with_value(u64::MAX);
        let next = counter.inc_and_get();
        assert_eq!(next, 0, "inc_and_get should wrap to 0 past u64::MAX");
        // Subsequent inc lands on 1
        assert_eq!(counter.inc_and_get(), 1);

        let counter = Counter::with_value(u64::MAX - 2);
        let next = counter.add_and_get(5);
        assert_eq!(
            next, 2,
            "add_and_get should wrap: (u64::MAX-2) + 5 == 2 mod 2^64"
        );
    }

    #[test]
    fn test_saturating_add_terminates_at_max() {
        // 0.9.2 regression: saturating_add previously used SeqCst CAS in a
        // tight loop. Now uses Relaxed compare_exchange_weak. Behavior must
        // remain identical.
        let counter = Counter::with_value(u64::MAX - 3);
        counter.saturating_add(100);
        assert_eq!(counter.get(), u64::MAX);

        // Already-at-max no-op short-circuit
        counter.saturating_add(1);
        assert_eq!(counter.get(), u64::MAX);
    }

    #[test]
    fn test_inc_max_relaxed_cas_still_correct() {
        // 0.9.2 regression: inc_max switched from SeqCst CAS to Relaxed.
        // Single-threaded behavior must remain identical.
        let counter = Counter::with_value(5);
        assert!(counter.inc_max(7));
        assert_eq!(counter.get(), 6);
        assert!(counter.inc_max(7));
        assert_eq!(counter.get(), 7);
        assert!(!counter.inc_max(7));
        assert_eq!(counter.get(), 7);
    }

    #[test]
    fn test_checked_operations_and_overflow_paths() {
        let counter = Counter::new();

        counter.try_set(3).unwrap();
        assert_eq!(counter.get(), 3);

        counter.try_inc().unwrap();
        assert_eq!(counter.get(), 4);

        counter.try_add(0).unwrap();
        assert_eq!(counter.get(), 4);

        assert_eq!(counter.try_fetch_add(2).unwrap(), 4);
        assert_eq!(counter.get(), 6);

        assert_eq!(counter.try_fetch_add(0).unwrap(), 6);
        assert_eq!(counter.try_inc_and_get().unwrap(), 7);

        let overflow = Counter::with_value(u64::MAX);
        assert!(matches!(overflow.try_inc(), Err(MetricsError::Overflow)));
        assert!(matches!(overflow.try_add(1), Err(MetricsError::Overflow)));
        assert!(matches!(
            overflow.try_fetch_add(1),
            Err(MetricsError::Overflow)
        ));
        assert!(matches!(
            overflow.try_inc_and_get(),
            Err(MetricsError::Overflow)
        ));
    }
}

#[cfg(all(test, feature = "bench-tests", not(tarpaulin), not(coverage)))]
#[allow(unused_imports)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[cfg_attr(not(feature = "bench-tests"), ignore)]
    #[test]
    fn bench_counter_increment() {
        let counter = Counter::new();
        let iterations = 10_000_000;

        let start = Instant::now();
        for _ in 0..iterations {
            counter.inc();
        }
        let elapsed = start.elapsed();

        println!(
            "Counter increment: {:.2} ns/op",
            elapsed.as_nanos() as f64 / iterations as f64
        );

        // Throughput-only smoke check. Criterion's `metrics_bench` is the
        // authoritative regression detector — see CONTRIBUTING.md.
        assert_eq!(counter.get(), iterations as u64);
    }

    #[cfg_attr(not(feature = "bench-tests"), ignore)]
    #[test]
    fn bench_counter_add() {
        let counter = Counter::new();
        let iterations = 1_000_000;

        let start = Instant::now();
        for i in 0..iterations {
            counter.add(i + 1);
        }
        let elapsed = start.elapsed();

        println!(
            "Counter add: {:.2} ns/op",
            elapsed.as_nanos() as f64 / iterations as f64
        );

        // Throughput-only smoke check. Criterion catches regressions.
    }

    #[cfg_attr(not(feature = "bench-tests"), ignore)]
    #[test]
    fn bench_counter_get() {
        let counter = Counter::new();
        counter.set(42);
        let iterations = 100_000_000;

        let start = Instant::now();
        let mut sum = 0;
        for _ in 0..iterations {
            sum += counter.get();
        }
        let elapsed = start.elapsed();

        println!(
            "Counter get: {:.2} ns/op",
            elapsed.as_nanos() as f64 / iterations as f64
        );

        // Prevent optimization elision; Criterion is the regression detector.
        assert_eq!(sum, 42 * iterations);
    }
}
