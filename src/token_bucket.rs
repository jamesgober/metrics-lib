//! Strict-admission token bucket.
//!
//! [`TokenBucket`] admits work at a sustained rate with a bounded burst, and
//! never over-admits, even under contention: every acquisition is a single
//! compare-and-swap on one `AtomicU64`.
//!
//! # Algorithm
//!
//! The bucket is implemented as GCRA (the generic cell rate algorithm), the
//! timestamp form of a token bucket. The only state is the *theoretical
//! arrival time* (TAT) in nanoseconds since the bucket was created. One token
//! is worth `interval = 1 s / rate` of time; the burst tolerance is
//! `capacity × interval`. Acquiring `n` tokens at time `now` moves the TAT to
//! `max(TAT, now) + n × interval`, and is refused when that lands more than
//! the tolerance ahead of `now`.
//!
//! Time is a monotonic `u64` nanosecond count, so the bucket keeps refilling
//! for 584 years of uptime (the 1.x bucket stopped refilling after 49.7 days).
//!
//! # Limits
//!
//! - The refill interval is rounded to whole nanoseconds, so rates above
//!   `1e9` tokens/s are clamped to `1e9`, and very high rates are rounded
//!   (at `1e8` tokens/s the interval is exactly 10 ns). Read the effective
//!   rate back with [`TokenBucket::refill_per_second`].
//! - The burst tolerance is a `u64` nanosecond span, so the effective
//!   capacity is `min(capacity, u64::MAX / interval)` — about 18 billion
//!   tokens at one token per second. Read it back with
//!   [`TokenBucket::capacity`].
//! - A rate of `0` (or a negative or non-finite rate) makes a bucket that
//!   never refills.
//!
//! # Example
//!
//! ```
//! use metrics_lib::TokenBucket;
//!
//! // 10 tokens per second sustained, bursts of up to 50.
//! let bucket = TokenBucket::new(50, 10.0);
//! if bucket.try_acquire(1).is_ok() {
//!     // serve the request
//! }
//! assert!(bucket.acquire(5));
//! ```

use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const NANOS_PER_SEC: f64 = 1_000_000_000.0;

/// Highest supported refill rate, in tokens per second.
pub const MAX_REFILL_PER_SECOND: f64 = 1e9;

#[derive(Debug, Clone, Copy)]
enum Mode {
    /// `state` is the theoretical arrival time in ns since `created_at`.
    Gcra { interval_ns: u64, tolerance_ns: u64 },
    /// `state` is the number of tokens left; nothing refills.
    Static,
}

/// Lock-free token bucket with strict admission (GCRA).
///
/// Cache-line aligned to keep concurrent admission pipelines from false
/// sharing.
#[repr(align(64))]
pub struct TokenBucket {
    state: AtomicU64,
    capacity: u64,
    mode: Mode,
    created_at: Instant,
}

impl TokenBucket {
    /// Build a bucket that starts full.
    ///
    /// * `capacity` — burst size in tokens.
    /// * `refill_per_second` — sustained rate. `0`, negative and non-finite
    ///   rates produce a bucket that never refills; rates above
    ///   [`MAX_REFILL_PER_SECOND`] are clamped.
    ///
    /// See the module docs for how the effective capacity and rate are
    /// derived; [`TokenBucket::try_new`] refuses inputs that would be
    /// adjusted.
    #[must_use]
    pub fn new(capacity: u64, refill_per_second: f64) -> Self {
        let rate = if refill_per_second.is_finite() && refill_per_second > 0.0 {
            refill_per_second.min(MAX_REFILL_PER_SECOND)
        } else {
            0.0
        };
        let (mode, capacity, state) = if rate > 0.0 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            // positive, at most ~1.8e19
            let interval_ns = ((NANOS_PER_SEC / rate).round() as u64).max(1);
            let effective = capacity.min(u64::MAX / interval_ns);
            let tolerance_ns = effective * interval_ns;
            (
                Mode::Gcra {
                    interval_ns,
                    tolerance_ns,
                },
                effective,
                0,
            )
        } else {
            (Mode::Static, capacity, capacity)
        };
        Self {
            state: AtomicU64::new(state),
            capacity,
            mode,
            created_at: Instant::now(),
        }
    }

    /// Build a bucket, refusing inputs that [`TokenBucket::new`] would adjust.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Config`] when `refill_per_second` is negative,
    /// NaN, infinite or above [`MAX_REFILL_PER_SECOND`], or when `capacity`
    /// exceeds what the burst tolerance can represent at that rate.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::TokenBucket;
    ///
    /// assert!(TokenBucket::try_new(100, 10.0).is_ok());
    /// assert!(TokenBucket::try_new(100, f64::NAN).is_err());
    /// assert!(TokenBucket::try_new(100, -1.0).is_err());
    /// ```
    pub fn try_new(capacity: u64, refill_per_second: f64) -> Result<Self> {
        if !refill_per_second.is_finite() || refill_per_second < 0.0 {
            return Err(MetricsError::Config(
                "token bucket refill rate must be finite and non-negative".into(),
            ));
        }
        if refill_per_second > MAX_REFILL_PER_SECOND {
            return Err(MetricsError::Config(
                "token bucket refill rate exceeds MAX_REFILL_PER_SECOND".into(),
            ));
        }
        let bucket = Self::new(capacity, refill_per_second);
        if bucket.capacity != capacity {
            return Err(MetricsError::Config(
                "token bucket capacity is too large for its refill rate".into(),
            ));
        }
        Ok(bucket)
    }

    /// Effective burst capacity in tokens.
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    /// Effective refill rate in tokens per second (`0.0` for a static bucket).
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // interval is at most ~1.8e19 ns
    pub fn refill_per_second(&self) -> f64 {
        match self.mode {
            Mode::Gcra { interval_ns, .. } => NANOS_PER_SEC / interval_ns as f64,
            Mode::Static => 0.0,
        }
    }

    #[inline]
    fn now_ns(&self) -> u64 {
        crate::util::duration_to_nanos(self.created_at.elapsed())
    }

    /// Tokens available right now (advisory: another thread may take them
    /// before you do).
    #[must_use]
    pub fn available(&self) -> u64 {
        self.available_at(self.now_ns())
    }

    fn available_at(&self, now: u64) -> u64 {
        let state = self.state.load(Ordering::Acquire);
        match self.mode {
            Mode::Static => state,
            Mode::Gcra {
                interval_ns,
                tolerance_ns,
            } => {
                let backlog = state.saturating_sub(now);
                (tolerance_ns.saturating_sub(backlog) / interval_ns).min(self.capacity)
            }
        }
    }

    /// Take `n` tokens, or fail without taking any.
    ///
    /// `n == 0` always succeeds.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::WouldBlock`] when fewer than `n` tokens are
    /// available (always the case when `n` exceeds the capacity).
    #[inline]
    pub fn try_acquire(&self, n: u64) -> Result<()> {
        self.try_acquire_at(n, self.now_ns())
    }

    fn try_acquire_at(&self, n: u64, now: u64) -> Result<()> {
        if n == 0 {
            return Ok(());
        }
        match self.mode {
            Mode::Static => self
                .state
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |left| {
                    left.checked_sub(n)
                })
                .map(|_| ())
                .map_err(|_| MetricsError::WouldBlock),
            Mode::Gcra {
                interval_ns,
                tolerance_ns,
            } => {
                let cost = u128::from(n) * u128::from(interval_ns);
                if cost > u128::from(tolerance_ns) {
                    return Err(MetricsError::WouldBlock);
                }
                self.state
                    .fetch_update(Ordering::AcqRel, Ordering::Acquire, |tat| {
                        let next = u128::from(tat.max(now)) + cost;
                        let ahead = next - u128::from(now);
                        if ahead > u128::from(tolerance_ns) {
                            return None;
                        }
                        u64::try_from(next).ok()
                    })
                    .map(|_| ())
                    .map_err(|_| MetricsError::WouldBlock)
            }
        }
    }

    /// Take `n` tokens if available; returns whether they were taken.
    #[inline]
    #[must_use = "the return value says whether the tokens were granted"]
    pub fn acquire(&self, n: u64) -> bool {
        self.try_acquire(n).is_ok()
    }

    /// How long until `n` tokens could be acquired, assuming no other
    /// consumer takes any. `Some(Duration::ZERO)` means "now"; `None` means
    /// "never" (`n` exceeds the capacity, or a static bucket lacks the
    /// tokens).
    #[must_use]
    pub fn time_until_available(&self, n: u64) -> Option<Duration> {
        self.time_until_available_at(n, self.now_ns())
    }

    fn time_until_available_at(&self, n: u64, now: u64) -> Option<Duration> {
        match self.mode {
            Mode::Static => (self.state.load(Ordering::Acquire) >= n).then_some(Duration::ZERO),
            Mode::Gcra {
                interval_ns,
                tolerance_ns,
            } => {
                let cost = u128::from(n) * u128::from(interval_ns);
                if cost > u128::from(tolerance_ns) {
                    return None;
                }
                let tat = self.state.load(Ordering::Acquire);
                let ready_at =
                    (u128::from(tat.max(now)) + cost).saturating_sub(u128::from(tolerance_ns));
                let wait = ready_at.saturating_sub(u128::from(now));
                Some(Duration::from_nanos(
                    u64::try_from(wait).unwrap_or(u64::MAX),
                ))
            }
        }
    }

    /// Refill the bucket to capacity.
    pub fn reset(&self) {
        let value = match self.mode {
            Mode::Static => self.capacity,
            Mode::Gcra { .. } => 0,
        };
        self.state.store(value, Ordering::Release);
    }
}

impl std::fmt::Debug for TokenBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenBucket")
            .field("capacity", &self.capacity())
            .field("available", &self.available())
            .field("refill_per_second", &self.refill_per_second())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    const SEC: u64 = 1_000_000_000;
    const DAY: u64 = 86_400 * SEC;

    #[test]
    fn test_new_bucket_starts_full() {
        let b = TokenBucket::new(10, 5.0);
        assert_eq!(b.capacity(), 10);
        assert_eq!(b.available(), 10);
        assert!((b.refill_per_second() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_zero_acquire_is_noop() {
        let b = TokenBucket::new(5, 1.0);
        b.try_acquire(0).unwrap();
        assert_eq!(b.available(), 5);
    }

    #[test]
    fn test_static_bucket_drains_then_refuses() {
        let b = TokenBucket::new(3, 0.0);
        assert!(b.acquire(1));
        assert!(b.acquire(2));
        assert!(!b.acquire(1));
        assert_eq!(b.try_acquire(1), Err(MetricsError::WouldBlock));
        assert_eq!(b.time_until_available(1), None);
        b.reset();
        assert_eq!(b.available(), 3);
        assert_eq!(b.time_until_available(3), Some(Duration::ZERO));
        assert_eq!(b.refill_per_second(), 0.0);
    }

    #[test]
    fn test_refill_with_injected_clock() {
        let b = TokenBucket::new(10, 2.0); // one token every 500 ms
        b.try_acquire_at(10, 0).unwrap();
        assert_eq!(b.available_at(0), 0);
        assert!(b.try_acquire_at(1, SEC / 4).is_err());
        assert_eq!(b.available_at(SEC / 2), 1);
        b.try_acquire_at(1, SEC / 2).unwrap();
        assert_eq!(b.available_at(10 * SEC), 10);
        assert_eq!(b.available_at(100 * SEC), 10, "capped at capacity");
    }

    #[test]
    fn test_keeps_refilling_after_fifty_days_regression_ml004() {
        // ML-004: the 1.x bucket stopped refilling after u32::MAX ms (49.7 days).
        let b = TokenBucket::new(5, 1.0);
        for day in [49, 50, 51, 100, 365 * 10] {
            let now = day * DAY;
            assert_eq!(b.available_at(now), 5, "day {day}");
            b.try_acquire_at(5, now).unwrap();
            assert_eq!(b.available_at(now), 0);
            assert_eq!(b.available_at(now + 2 * SEC), 2);
        }
        // reset() also works at any uptime.
        let later = 3651 * DAY;
        b.try_acquire_at(5, later).unwrap();
        b.reset();
        assert_eq!(b.available_at(later), 5);
    }

    #[test]
    fn test_large_capacity_and_tiny_rate_regression_ml026() {
        // ML-026: capacity above 4.29 M tokens was silently clamped and rates
        // below 0.0005/s rounded to zero.
        let big = TokenBucket::new(10_000_000, 1000.0);
        assert_eq!(big.capacity(), 10_000_000);
        assert_eq!(big.available(), 10_000_000);
        assert!(big.acquire(5_000_000));
        let slow = TokenBucket::new(1, 0.0004);
        assert!(slow.refill_per_second() > 0.0);
        slow.try_acquire_at(1, 0).unwrap();
        // One token every 2500 s.
        assert_eq!(slow.available_at(2_499 * SEC), 0);
        assert_eq!(slow.available_at(2_500 * SEC), 1);
    }

    #[test]
    fn test_effective_capacity_and_rate_are_reported() {
        let huge = TokenBucket::new(u64::MAX, 1.0);
        assert_eq!(huge.capacity(), u64::MAX / SEC);
        assert!(TokenBucket::try_new(u64::MAX, 1.0).is_err());
        let fast = TokenBucket::new(10, 1e12);
        assert!((fast.refill_per_second() - MAX_REFILL_PER_SECOND).abs() < 1.0);
        assert!(TokenBucket::try_new(10, 1e12).is_err());
        for bad in [f64::NAN, -1.0, f64::INFINITY] {
            assert_eq!(TokenBucket::new(5, bad).refill_per_second(), 0.0);
            assert!(TokenBucket::try_new(5, bad).is_err());
        }
    }

    #[test]
    fn test_acquire_more_than_capacity_never_succeeds() {
        let b = TokenBucket::new(5, 1000.0);
        assert!(!b.acquire(6));
        assert_eq!(b.time_until_available(6), None);
        assert_eq!(b.available(), 5);
    }

    #[test]
    fn test_time_until_available() {
        let b = TokenBucket::new(4, 2.0);
        assert_eq!(b.time_until_available_at(4, 0), Some(Duration::ZERO));
        // Fewer tokens than the tolerance on a full bucket (found by proptest).
        assert_eq!(b.time_until_available_at(1, 0), Some(Duration::ZERO));
        b.try_acquire_at(4, 0).unwrap();
        assert_eq!(
            b.time_until_available_at(1, 0),
            Some(Duration::from_millis(500))
        );
        assert_eq!(
            b.time_until_available_at(2, SEC / 2),
            Some(Duration::from_millis(500))
        );
    }

    #[test]
    fn test_clock_going_backwards_does_not_overgrant() {
        let b = TokenBucket::new(2, 1.0);
        b.try_acquire_at(2, 10 * SEC).unwrap();
        assert!(b.try_acquire_at(1, 5 * SEC).is_err());
        assert_eq!(b.available_at(5 * SEC), 0);
    }

    #[test]
    fn test_concurrent_acquire_never_overshoots_capacity() {
        let b = Arc::new(TokenBucket::new(100, 0.0));
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let b = Arc::clone(&b);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    (0..30).filter(|_| b.acquire(1)).count()
                })
            })
            .collect();
        let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert_eq!(total, 100);
        assert_eq!(b.available(), 0);
    }

    #[test]
    fn test_concurrent_refilling_bucket_respects_rate_with_real_clock() {
        // 1000 tokens/s, burst 10, 8 threads hammering for ~200 ms. Grants
        // must not exceed burst + rate * elapsed (plus one interval of slack).
        let b = Arc::new(TokenBucket::new(10, 1000.0));
        let barrier = Arc::new(Barrier::new(8));
        let start = Instant::now();
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let b = Arc::clone(&b);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    let mut granted = 0u64;
                    let until = Instant::now() + Duration::from_millis(200);
                    while Instant::now() < until {
                        if b.acquire(1) {
                            granted += 1;
                        }
                    }
                    granted
                })
            })
            .collect();
        let granted: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        let elapsed_ms = u64::try_from(start.elapsed().as_millis()).unwrap();
        assert!(
            granted <= 10 + elapsed_ms + 1,
            "granted {granted} in {elapsed_ms} ms"
        );
    }

    #[test]
    fn test_debug_impl() {
        let s = format!("{:?}", TokenBucket::new(7, 2.5));
        assert!(s.contains("capacity: 7"));
    }

    proptest! {
        #[test]
        fn prop_grants_never_exceed_capacity_plus_refill(
            capacity in 1u64..1000,
            rate in 0.001f64..10_000.0,
            requests in proptest::collection::vec((0u64..5 * SEC, 1u64..50), 1..100)
        ) {
            let b = TokenBucket::new(capacity, rate);
            let mut sorted = requests.clone();
            sorted.sort_unstable();
            let mut granted = 0u64;
            for (t, n) in &sorted {
                if b.try_acquire_at(*n, *t).is_ok() {
                    granted += n;
                }
            }
            let last = sorted.last().map_or(0, |(t, _)| *t);
            #[allow(clippy::cast_precision_loss)]
            let refill = b.refill_per_second() * (last as f64 / 1e9);
            #[allow(clippy::cast_precision_loss)]
            let bound = capacity as f64 + refill + 1.0;
            #[allow(clippy::cast_precision_loss)]
            let granted_f = granted as f64;
            prop_assert!(granted_f <= bound, "granted {} bound {}", granted, bound);
        }

        #[test]
        fn prop_available_never_exceeds_capacity(
            capacity in 0u64..1_000_000,
            rate in 0.0f64..1e6,
            now in 0u64..u64::MAX / 2
        ) {
            let b = TokenBucket::new(capacity, rate);
            prop_assert!(b.available_at(now) <= b.capacity());
            prop_assert!(b.capacity() <= capacity);
        }
    }
}
