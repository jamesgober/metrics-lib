//! Duration statistics: count, sum, minimum and maximum.
//!
//! [`Timer::record_ns`] is two relaxed `fetch_add`s plus a minimum/maximum
//! update that only writes when a new extreme is seen. Exporters render a
//! timer as a Prometheus/OpenMetrics `summary` (`_count`, `_sum` in seconds)
//! plus `_min` / `_max` gauges. For percentiles use a
//! [`Histogram`](crate::Histogram) or a [`Summary`](crate::Summary)
//! (feature `histogram`).
//!
//! # Guards
//!
//! [`Timer::start`] returns a [`RunningTimer`] that borrows the timer and
//! records on drop. When the timer is a temporary — for example
//! `core.timer("x")` — use [`Timer::start_owned`], which keeps the `Arc` alive
//! inside an [`OwnedTimerGuard`]:
//!
//! ```
//! use metrics_lib::MetricsCore;
//!
//! let core = MetricsCore::new();
//! {
//!     let _guard = core.timer("db_query").start_owned();
//!     // ... timed work ...
//! } // recorded here
//! assert_eq!(core.timer("db_query").count(), 1);
//! ```
//!
//! # Overflow
//!
//! The running sum is a 128-bit value (a 64-bit nanosecond sum plus a wrap
//! counter), so it does not overflow in practice: 2^64 ns is 584 years of
//! accumulated time and the wrap counter multiplies that by another 2^64.
//! Durations longer than `u64::MAX` nanoseconds are clamped.

use crate::{MetricsError, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Lock-free duration statistics.
///
/// Cache-line aligned so the hot fields of two timers never share a line.
///
/// # Examples
///
/// ```
/// use metrics_lib::Timer;
/// use std::time::Duration;
///
/// let t = Timer::new();
/// t.record(Duration::from_millis(3));
/// t.record(Duration::from_millis(5));
/// assert_eq!(t.count(), 2);
/// assert_eq!(t.average(), Duration::from_millis(4));
/// ```
#[repr(align(64))]
pub struct Timer {
    count: AtomicU64,
    /// Low 64 bits of the nanosecond sum.
    total_nanos: AtomicU64,
    /// Number of times `total_nanos` wrapped (the high 64 bits of the sum).
    total_wraps: AtomicU64,
    min_nanos: AtomicU64,
    max_nanos: AtomicU64,
    created_at: Instant,
}

/// Point-in-time statistics for a [`Timer`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct TimerStats {
    /// Number of recorded durations.
    pub count: u64,
    /// Sum of recorded durations (saturates at `Duration::MAX`).
    pub total: Duration,
    /// Mean duration (`Duration::ZERO` when empty).
    pub average: Duration,
    /// Smallest duration recorded (`Duration::ZERO` when empty).
    pub min: Duration,
    /// Largest duration recorded (`Duration::ZERO` when empty).
    pub max: Duration,
    /// Time since the timer was created.
    pub age: Duration,
    /// Recordings per second since creation.
    pub rate_per_second: f64,
}

pub(crate) use crate::util::{duration_from_nanos_u128, duration_to_nanos};

impl Timer {
    /// Create an empty timer.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
            total_nanos: AtomicU64::new(0),
            total_wraps: AtomicU64::new(0),
            min_nanos: AtomicU64::new(u64::MAX),
            max_nanos: AtomicU64::new(0),
            created_at: Instant::now(),
        }
    }

    /// Start timing. The guard records the elapsed time when it is dropped
    /// or [stopped](RunningTimer::stop).
    #[inline]
    #[must_use = "dropping the guard immediately records a near-zero duration"]
    pub fn start(&self) -> RunningTimer<'_> {
        RunningTimer {
            timer: self,
            start_time: Instant::now(),
            armed: true,
        }
    }

    /// Start timing with a guard that owns a clone of the `Arc`.
    ///
    /// Use this when the timer is a temporary, such as the value returned by
    /// `MetricsCore::timer`; [`Timer::start`] would borrow from the temporary
    /// and fail to compile (E0716).
    #[inline]
    #[must_use = "dropping the guard immediately records a near-zero duration"]
    pub fn start_owned(self: &Arc<Self>) -> OwnedTimerGuard {
        OwnedTimerGuard {
            timer: Arc::clone(self),
            start_time: Instant::now(),
            armed: true,
        }
    }

    /// Record a duration (clamped to `u64::MAX` nanoseconds).
    #[inline]
    pub fn record(&self, duration: Duration) {
        self.record_ns(duration_to_nanos(duration));
    }

    /// Record a duration given in nanoseconds.
    #[inline]
    pub fn record_ns(&self, duration_ns: u64) {
        self.add_total(duration_ns);
        let _ = self.count.fetch_add(1, Ordering::Relaxed);
        self.update_min(duration_ns);
        self.update_max(duration_ns);
    }

    /// Record a duration, refusing when the sample count would overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when `count` is already
    /// `u64::MAX`; nothing is recorded.
    #[inline]
    pub fn try_record(&self, duration: Duration) -> Result<()> {
        self.try_record_ns(duration_to_nanos(duration))
    }

    /// Nanosecond form of [`Timer::try_record`].
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when `count` is already
    /// `u64::MAX`; nothing is recorded.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::Timer;
    ///
    /// let t = Timer::new();
    /// t.try_record_ns(1_000).unwrap();
    /// assert_eq!(t.count(), 1);
    /// ```
    #[inline]
    pub fn try_record_ns(&self, duration_ns: u64) -> Result<()> {
        // Claim the count first: the check and the increment are one atomic
        // step, so concurrent callers cannot push it past u64::MAX.
        let _ = self
            .count
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |c| c.checked_add(1))
            .map_err(|_| MetricsError::Overflow)?;
        self.add_total(duration_ns);
        self.update_min(duration_ns);
        self.update_max(duration_ns);
        Ok(())
    }

    /// Record several durations, updating each field once.
    #[inline]
    pub fn record_batch(&self, durations: &[Duration]) {
        if durations.is_empty() {
            return;
        }
        let (sum, min, max) = summarize(durations);
        self.add_total_u128(sum);
        let _ = self.count.fetch_add(
            u64::try_from(durations.len()).unwrap_or(u64::MAX),
            Ordering::Relaxed,
        );
        self.update_min(min);
        self.update_max(max);
    }

    /// [`Timer::record_batch`] that refuses when the sample count would
    /// overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when adding `durations.len()` would
    /// overflow `count`; nothing is recorded.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::Timer;
    /// use std::time::Duration;
    ///
    /// let t = Timer::new();
    /// t.try_record_batch(&[Duration::from_micros(5), Duration::from_micros(10)]).unwrap();
    /// assert_eq!(t.count(), 2);
    /// ```
    pub fn try_record_batch(&self, durations: &[Duration]) -> Result<()> {
        if durations.is_empty() {
            return Ok(());
        }
        let n = u64::try_from(durations.len()).map_err(|_| MetricsError::Overflow)?;
        let _ = self
            .count
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |c| c.checked_add(n))
            .map_err(|_| MetricsError::Overflow)?;
        let (sum, min, max) = summarize(durations);
        self.add_total_u128(sum);
        self.update_min(min);
        self.update_max(max);
        Ok(())
    }

    #[inline]
    fn add_total(&self, ns: u64) {
        let prev = self.total_nanos.fetch_add(ns, Ordering::Relaxed);
        if prev.checked_add(ns).is_none() {
            let _ = self.total_wraps.fetch_add(1, Ordering::Release);
        }
    }

    #[inline]
    fn add_total_u128(&self, sum: u128) {
        #[allow(clippy::cast_possible_truncation)] // intentional: keep the low 64 bits
        let low = sum as u64;
        let high = u64::try_from(sum >> 64).unwrap_or(u64::MAX);
        self.add_total(low);
        if high > 0 {
            let _ = self.total_wraps.fetch_add(high, Ordering::Release);
        }
    }

    /// Number of recorded durations.
    #[inline]
    #[must_use]
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Sum of recorded durations in nanoseconds, as a 128-bit value.
    ///
    /// Not linearizable with a concurrent wrap of the low word: in the few
    /// nanoseconds between the low word wrapping and the wrap counter being
    /// bumped, a reader can see a sum 2^64 ns too low. That window opens once
    /// per 584 years of accumulated duration.
    #[must_use]
    pub fn total_nanos(&self) -> u128 {
        // Read the wrap counter on both sides of the low word and retry when
        // it moved in between.
        loop {
            let wraps = self.total_wraps.load(Ordering::Acquire);
            let low = self.total_nanos.load(Ordering::Acquire);
            if self.total_wraps.load(Ordering::Acquire) == wraps {
                return (u128::from(wraps) << 64) | u128::from(low);
            }
        }
    }

    /// Sum of recorded durations (saturates at `Duration::MAX`).
    #[inline]
    #[must_use]
    pub fn total(&self) -> Duration {
        duration_from_nanos_u128(self.total_nanos())
    }

    /// Mean duration, or `Duration::ZERO` when nothing was recorded.
    #[inline]
    #[must_use]
    pub fn average(&self) -> Duration {
        match self.count() {
            0 => Duration::ZERO,
            count => duration_from_nanos_u128(self.total_nanos() / u128::from(count)),
        }
    }

    /// Smallest recorded duration, or `Duration::ZERO` when empty.
    #[inline]
    #[must_use]
    pub fn min(&self) -> Duration {
        match self.min_nanos.load(Ordering::Relaxed) {
            u64::MAX if self.count() == 0 => Duration::ZERO,
            ns => Duration::from_nanos(ns),
        }
    }

    /// Largest recorded duration, or `Duration::ZERO` when empty.
    #[inline]
    #[must_use]
    pub fn max(&self) -> Duration {
        Duration::from_nanos(self.max_nanos.load(Ordering::Relaxed))
    }

    /// Clear every statistic.
    ///
    /// Recordings racing with the reset may be partially kept.
    pub fn reset(&self) {
        self.count.store(0, Ordering::Relaxed);
        self.total_nanos.store(0, Ordering::Relaxed);
        self.total_wraps.store(0, Ordering::Release);
        self.min_nanos.store(u64::MAX, Ordering::Relaxed);
        self.max_nanos.store(0, Ordering::Relaxed);
    }

    /// Point-in-time statistics.
    #[must_use]
    pub fn stats(&self) -> TimerStats {
        let count = self.count();
        let age = self.age();
        TimerStats {
            count,
            total: self.total(),
            average: self.average(),
            min: self.min(),
            max: self.max(),
            age,
            rate_per_second: crate::util::per_second(count, age),
        }
    }

    /// Time since the timer was created.
    #[inline]
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// `true` when nothing was recorded.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Recordings per second since creation.
    #[inline]
    #[must_use]
    pub fn rate_per_second(&self) -> f64 {
        crate::util::per_second(self.count(), self.age())
    }

    #[inline]
    fn update_min(&self, value: u64) {
        if value < self.min_nanos.load(Ordering::Relaxed) {
            let _ = self.min_nanos.fetch_min(value, Ordering::Relaxed);
        }
    }

    #[inline]
    fn update_max(&self, value: u64) {
        if value > self.max_nanos.load(Ordering::Relaxed) {
            let _ = self.max_nanos.fetch_max(value, Ordering::Relaxed);
        }
    }
}

/// `(sum, min, max)` of a non-empty slice, in nanoseconds.
fn summarize(durations: &[Duration]) -> (u128, u64, u64) {
    durations
        .iter()
        .map(|d| duration_to_nanos(*d))
        .fold((0u128, u64::MAX, 0u64), |(sum, min, max), ns| {
            (sum + u128::from(ns), min.min(ns), max.max(ns))
        })
}

impl Default for Timer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timer(count: {}, avg: {:?})",
            self.count(),
            self.average()
        )
    }
}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.stats();
        f.debug_struct("Timer")
            .field("count", &stats.count)
            .field("total", &stats.total)
            .field("average", &stats.average)
            .field("min", &stats.min)
            .field("max", &stats.max)
            .finish()
    }
}

/// Guard returned by [`Timer::start`]; records the elapsed time on drop.
#[must_use = "dropping the guard immediately records a near-zero duration"]
pub struct RunningTimer<'a> {
    timer: &'a Timer,
    start_time: Instant,
    armed: bool,
}

impl RunningTimer<'_> {
    /// Time elapsed so far, without recording.
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Record the elapsed time now and return it.
    #[inline]
    #[allow(clippy::must_use_candidate)] // the duration is optional information
    pub fn stop(mut self) -> Duration {
        let elapsed = self.start_time.elapsed();
        self.timer.record(elapsed);
        self.armed = false;
        elapsed
    }

    /// Drop the guard without recording (for example when the timed
    /// operation was cancelled).
    #[inline]
    pub fn discard(mut self) {
        self.armed = false;
    }
}

impl Drop for RunningTimer<'_> {
    #[inline]
    fn drop(&mut self) {
        if self.armed {
            self.timer.record(self.start_time.elapsed());
        }
    }
}

impl std::fmt::Debug for RunningTimer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningTimer")
            .field("elapsed", &self.elapsed())
            .finish_non_exhaustive()
    }
}

/// Guard returned by [`Timer::start_owned`]; owns an `Arc<Timer>` and
/// records the elapsed time on drop.
#[must_use = "dropping the guard immediately records a near-zero duration"]
pub struct OwnedTimerGuard {
    timer: Arc<Timer>,
    start_time: Instant,
    armed: bool,
}

impl OwnedTimerGuard {
    /// Time elapsed so far, without recording.
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Record the elapsed time now and return it.
    #[inline]
    #[allow(clippy::must_use_candidate)] // the duration is optional information
    pub fn stop(mut self) -> Duration {
        let elapsed = self.start_time.elapsed();
        self.timer.record(elapsed);
        self.armed = false;
        elapsed
    }

    /// Drop the guard without recording.
    #[inline]
    pub fn discard(mut self) {
        self.armed = false;
    }

    /// The timer this guard records into.
    #[inline]
    #[must_use]
    pub fn timer(&self) -> &Arc<Timer> {
        &self.timer
    }
}

impl Drop for OwnedTimerGuard {
    #[inline]
    fn drop(&mut self) {
        if self.armed {
            self.timer.record(self.start_time.elapsed());
        }
    }
}

impl std::fmt::Debug for OwnedTimerGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedTimerGuard")
            .field("elapsed", &self.elapsed())
            .finish_non_exhaustive()
    }
}

/// Free functions for timing closures.
pub mod utils {
    use super::{RunningTimer, Timer};
    use std::time::{Duration, Instant};

    /// Run `f` and return its result together with the elapsed time.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::timer_utils::time_fn;
    ///
    /// let (value, elapsed) = time_fn(|| 6 * 7);
    /// assert_eq!(value, 42);
    /// assert!(elapsed.as_secs() < 1);
    /// ```
    #[inline]
    pub fn time_fn<T>(f: impl FnOnce() -> T) -> (T, Duration) {
        let start = Instant::now();
        let result = f();
        (result, start.elapsed())
    }

    /// Run `f` and record its duration into `timer`.
    #[inline]
    pub fn time_and_record<T>(timer: &Timer, f: impl FnOnce() -> T) -> T {
        let _guard = timer.start();
        f()
    }

    /// Start a guard on `timer` (same as [`Timer::start`]).
    #[inline]
    #[must_use = "dropping the guard immediately records a near-zero duration"]
    pub fn scoped_timer(timer: &Timer) -> RunningTimer<'_> {
        timer.start()
    }
}

/// Time a block and record its duration into a timer.
///
/// The first argument is any expression with a `start()` method returning a
/// guard (a `Timer`, `&Timer`, `Arc<Timer>`, …). Evaluates to the block's
/// value.
///
/// # Examples
///
/// ```
/// use metrics_lib::{time_block, Timer};
///
/// let timer = Timer::new();
/// let answer = time_block!(timer, { 6 * 7 });
/// assert_eq!(answer, 42);
/// assert_eq!(timer.count(), 1);
/// ```
#[macro_export]
macro_rules! time_block {
    ($timer:expr, $block:block) => {{
        let _timing_guard = $timer.start();
        $block
    }};
}

/// Evaluate an expression and return `(value, elapsed)`.
///
/// # Examples
///
/// ```
/// use metrics_lib::time_fn;
///
/// let (result, duration) = time_fn!({
///     std::thread::sleep(std::time::Duration::from_millis(2));
///     "done"
/// });
/// assert!(duration >= std::time::Duration::from_millis(2));
/// assert_eq!(result, "done");
/// ```
#[macro_export]
macro_rules! time_fn {
    ($func:expr) => {{
        let start = ::std::time::Instant::now();
        let result = $func;
        (result, start.elapsed())
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::Barrier;
    use std::thread;

    #[test]
    fn test_timer_basic_operations() {
        let timer = Timer::new();
        assert!(timer.is_empty());
        assert_eq!(timer.average(), Duration::ZERO);
        assert_eq!(timer.min(), Duration::ZERO);
        assert_eq!(timer.max(), Duration::ZERO);
        timer.record(Duration::from_millis(100));
        timer.record(Duration::from_millis(300));
        assert_eq!(timer.count(), 2);
        assert_eq!(timer.total(), Duration::from_millis(400));
        assert_eq!(timer.average(), Duration::from_millis(200));
        assert_eq!(timer.min(), Duration::from_millis(100));
        assert_eq!(timer.max(), Duration::from_millis(300));
        timer.reset();
        assert!(timer.is_empty());
        assert_eq!(timer.total(), Duration::ZERO);
    }

    #[test]
    fn test_timer_zero_duration_min_is_zero() {
        let timer = Timer::new();
        timer.record_ns(0);
        assert_eq!(timer.min(), Duration::ZERO);
        assert_eq!(timer.count(), 1);
    }

    #[test]
    fn test_running_timer_records_on_drop_and_stop() {
        let timer = Timer::new();
        {
            let guard = timer.start();
            assert!(guard.elapsed() < Duration::from_secs(5));
            assert!(format!("{guard:?}").contains("RunningTimer"));
        }
        let elapsed = timer.start().stop();
        assert!(elapsed < Duration::from_secs(5));
        assert_eq!(timer.count(), 2);
        timer.start().discard();
        assert_eq!(timer.count(), 2);
    }

    #[test]
    fn test_owned_guard_works_on_temporaries_regression_ml020() {
        let shared = Arc::new(Timer::new());
        let make = || Arc::clone(&shared);
        {
            // `make().start()` would not compile here (E0716).
            let guard = make().start_owned();
            assert!(Arc::ptr_eq(guard.timer(), &shared));
            assert!(format!("{guard:?}").contains("OwnedTimerGuard"));
            assert!(guard.elapsed() < Duration::from_secs(5));
        }
        assert_eq!(shared.count(), 1);
        let _ = make().start_owned().stop();
        make().start_owned().discard();
        assert_eq!(shared.count(), 2);
    }

    #[test]
    fn test_timer_batch_recording() {
        let timer = Timer::new();
        timer.record_batch(&[
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
        ]);
        timer.record_batch(&[]);
        assert_eq!(timer.count(), 3);
        assert_eq!(timer.average(), Duration::from_millis(20));
        assert_eq!(timer.min(), Duration::from_millis(10));
        assert_eq!(timer.max(), Duration::from_millis(30));
        timer.try_record_batch(&[]).unwrap();
        timer.try_record_batch(&[Duration::from_millis(1)]).unwrap();
        assert_eq!(timer.min(), Duration::from_millis(1));
        assert_eq!(timer.count(), 4);
    }

    #[test]
    fn test_timer_sum_survives_u64_wrap_regression_ml021() {
        // ML-021: the nanosecond sum used to wrap silently, corrupting _sum.
        let timer = Timer::new();
        timer.record_ns(u64::MAX);
        timer.record_ns(10);
        let expected = u128::from(u64::MAX) + 10;
        assert_eq!(timer.total_nanos(), expected);
        assert_eq!(timer.total(), duration_from_nanos_u128(expected));
        assert_eq!(timer.average(), duration_from_nanos_u128(expected / 2));
        timer.record_batch(&[Duration::MAX, Duration::MAX]);
        assert_eq!(timer.total_nanos(), expected + 2 * u128::from(u64::MAX));
    }

    #[test]
    fn test_timer_huge_durations_clamp_instead_of_wrapping() {
        let timer = Timer::new();
        timer.record(Duration::MAX);
        assert_eq!(timer.max(), Duration::from_nanos(u64::MAX));
        assert_eq!(duration_to_nanos(Duration::MAX), u64::MAX);
        assert_eq!(duration_from_nanos_u128(u128::MAX).as_secs(), u64::MAX);
    }

    #[test]
    fn test_timer_try_record_rejects_count_overflow() {
        let timer = Timer::new();
        timer.count.store(u64::MAX, Ordering::Relaxed);
        assert_eq!(timer.try_record_ns(1), Err(MetricsError::Overflow));
        assert_eq!(
            timer.try_record(Duration::from_nanos(1)),
            Err(MetricsError::Overflow)
        );
        assert_eq!(
            timer.try_record_batch(&[Duration::from_nanos(1)]),
            Err(MetricsError::Overflow)
        );
        assert_eq!(timer.total_nanos(), 0);
    }

    #[test]
    fn test_timer_stats_and_formatting() {
        let timer = Timer::new();
        timer.record(Duration::from_millis(5));
        let stats = timer.stats();
        assert_eq!(stats.count, 1);
        assert_eq!(stats.average, Duration::from_millis(5));
        assert!(stats.rate_per_second >= 0.0);
        assert!(timer.rate_per_second() >= 0.0);
        assert!(format!("{timer}").contains("count: 1"));
        assert!(format!("{timer:?}").contains("Timer"));
        assert!(Timer::default().is_empty());
    }

    #[test]
    fn test_timer_utils_and_macros() {
        let timer = Timer::new();
        let (v, d) = utils::time_fn(|| 5);
        assert_eq!(v, 5);
        assert!(d < Duration::from_secs(5));
        assert_eq!(utils::time_and_record(&timer, || 7), 7);
        {
            let _g = utils::scoped_timer(&timer);
        }
        let x = crate::time_block!(timer, { 1 + 1 });
        assert_eq!(x, 2);
        let (y, _) = crate::time_fn!(3 * 3);
        assert_eq!(y, 9);
        assert_eq!(timer.count(), 3);
    }

    #[test]
    fn test_timer_concurrent_recordings_are_exact() {
        let timer = Arc::new(Timer::new());
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8u64)
            .map(|t| {
                let timer = Arc::clone(&timer);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    for i in 1..=10_000u64 {
                        timer.record_ns(t * 100_000 + i);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(timer.count(), 80_000);
        let expected: u128 = (0..8u128)
            .map(|t| (1..=10_000u128).map(|i| t * 100_000 + i).sum::<u128>())
            .sum();
        assert_eq!(timer.total_nanos(), expected);
        assert_eq!(timer.min(), Duration::from_nanos(1));
        assert_eq!(timer.max(), Duration::from_nanos(710_000));
    }

    proptest! {
        #[test]
        fn prop_total_is_exact_sum(values in proptest::collection::vec(any::<u64>(), 0..50)) {
            let timer = Timer::new();
            for v in &values {
                timer.record_ns(*v);
            }
            let expected: u128 = values.iter().map(|v| u128::from(*v)).sum();
            prop_assert_eq!(timer.total_nanos(), expected);
            prop_assert_eq!(timer.count(), values.len() as u64);
            if let (Some(min), Some(max)) = (values.iter().min(), values.iter().max()) {
                prop_assert_eq!(timer.min(), Duration::from_nanos(*min));
                prop_assert_eq!(timer.max(), Duration::from_nanos(*max));
            }
        }

        #[test]
        fn prop_batch_equals_individual(values in proptest::collection::vec(any::<u64>(), 1..30)) {
            let one = Timer::new();
            let batch = Timer::new();
            let durations: Vec<Duration> = values.iter().map(|v| Duration::from_nanos(*v)).collect();
            for d in &durations {
                one.record(*d);
            }
            batch.record_batch(&durations);
            prop_assert_eq!(one.total_nanos(), batch.total_nanos());
            prop_assert_eq!(one.count(), batch.count());
            prop_assert_eq!(one.min(), batch.min());
            prop_assert_eq!(one.max(), batch.max());
        }
    }
}
