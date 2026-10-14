//! Event-rate meter with sliding-window estimates.
//!
//! A [`RateMeter`] counts events and estimates how many happened over the
//! last second, minute and hour, plus over one configurable *primary*
//! window used by [`RateMeter::rate`] and the limit helpers.
//!
//! # How the estimate works
//!
//! Each window keeps two counters: events in the current fixed slot and in
//! the previous one. The sliding estimate is
//!
//! ```text
//! previous × (1 − elapsed_fraction_of_current_slot) + current
//! ```
//!
//! the "sliding window counter" approximation. It does not drop to zero at
//! every slot boundary (so `tick_if_under_limit` cannot admit twice the limit
//! across a boundary), and it assumes events were spread evenly over the
//! previous slot.
//!
//! Time comes from the monotonic [`Instant`] clock, so wall-clock steps (NTP,
//! manual changes) do not disturb the windows. All counters are `u64`.
//!
//! # Accuracy under concurrency
//!
//! - [`RateMeter::total`] is always exact.
//! - When threads race across a slot boundary, events recorded between the
//!   boundary being claimed and the old slot being archived are attributed
//!   to the previous slot. The error is bounded by the events recorded in
//!   that window of a few nanoseconds and self-corrects one slot later.
//! - [`RateMeter::tick_if_under_limit`] reads, then ticks: concurrent callers
//!   can overshoot the limit by up to `threads − 1` events. For strict
//!   admission use [`TokenBucket`](crate::TokenBucket).

use crate::util::CasUpdate;
use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

const NANOS_PER_SEC: u64 = 1_000_000_000;
/// Smallest accepted primary window.
pub const MIN_RATE_WINDOW: Duration = Duration::from_millis(1);

/// One sliding-window counter.
struct Window {
    width_ns: u64,
    /// Index (`now / width`) of the slot `current` belongs to.
    slot: AtomicU64,
    current: AtomicU64,
    previous: AtomicU64,
}

impl Window {
    const fn new(width_ns: u64) -> Self {
        Self {
            width_ns,
            slot: AtomicU64::new(0),
            current: AtomicU64::new(0),
            previous: AtomicU64::new(0),
        }
    }

    #[inline]
    fn record(&self, now_ns: u64, n: u64) {
        let target = now_ns / self.width_ns;
        let mut seen = self.slot.load(Ordering::Acquire);
        while target > seen {
            match self
                .slot
                .compare_exchange(seen, target, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => {
                    let finished = self.current.swap(0, Ordering::AcqRel);
                    let carried = if target == seen + 1 { finished } else { 0 };
                    self.previous.store(carried, Ordering::Release);
                    break;
                }
                Err(actual) => seen = actual,
            }
        }
        let _ = self.current.fetch_add(n, Ordering::Relaxed);
    }

    /// Sliding estimate of events in the last `width` at `now_ns`.
    #[allow(clippy::cast_precision_loss)] // statistics: f64 precision is the contract
    fn estimate(&self, now_ns: u64) -> f64 {
        let target = now_ns / self.width_ns;
        let slot = self.slot.load(Ordering::Acquire);
        let current = self.current.load(Ordering::Relaxed) as f64;
        let previous = self.previous.load(Ordering::Relaxed) as f64;
        let into_slot = (now_ns % self.width_ns) as f64 / self.width_ns as f64;
        if target == slot {
            previous * (1.0 - into_slot) + current
        } else if target == slot + 1 {
            current * (1.0 - into_slot)
        } else if target < slot {
            // A reader with an older clock reading than the last writer.
            current
        } else {
            0.0
        }
    }

    fn reset(&self, now_ns: u64) {
        self.slot.store(now_ns / self.width_ns, Ordering::Release);
        self.current.store(0, Ordering::Relaxed);
        self.previous.store(0, Ordering::Relaxed);
    }

    #[allow(clippy::cast_precision_loss)]
    fn width_secs(&self) -> f64 {
        self.width_ns as f64 / NANOS_PER_SEC as f64
    }
}

/// Lock-free event counter with sliding-window rate estimates.
///
/// # Examples
///
/// ```
/// use metrics_lib::RateMeter;
///
/// let requests = RateMeter::new();
/// requests.tick();
/// requests.tick_n(9);
/// assert_eq!(requests.total(), 10);
/// assert!(requests.rate() > 0.0);
/// ```
#[repr(align(64))]
pub struct RateMeter {
    total: AtomicU64,
    /// Primary window; `None` when it equals the one-second window.
    primary: Option<Window>,
    second: Window,
    minute: Window,
    hour: Window,
    created_at: Instant,
}

/// Point-in-time statistics for a [`RateMeter`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct RateStats {
    /// Total events recorded.
    pub total_events: u64,
    /// Events per second over the primary window.
    pub rate: f64,
    /// Estimated events in the last second.
    pub per_second: f64,
    /// Estimated events in the last minute.
    pub per_minute: f64,
    /// Estimated events in the last hour.
    pub per_hour: f64,
    /// Average events per second since creation.
    pub average_rate: f64,
    /// Time since the meter was created.
    pub age: Duration,
    /// Width of the primary window.
    pub window: Duration,
}

impl RateMeter {
    /// Create a meter whose primary window is one second.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::with_window(Duration::from_secs(1))
    }

    /// Create a meter whose primary window ([`RateMeter::rate`] and the limit
    /// helpers) is `window`, clamped to at least [`MIN_RATE_WINDOW`].
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::RateMeter;
    /// use std::time::Duration;
    ///
    /// let m = RateMeter::with_window(Duration::from_secs(10));
    /// assert_eq!(m.window(), Duration::from_secs(10));
    /// ```
    #[must_use]
    pub fn with_window(window: Duration) -> Self {
        let width = crate::util::duration_to_nanos(window.max(MIN_RATE_WINDOW));
        Self {
            total: AtomicU64::new(0),
            primary: (width != NANOS_PER_SEC).then(|| Window::new(width)),
            second: Window::new(NANOS_PER_SEC),
            minute: Window::new(60 * NANOS_PER_SEC),
            hour: Window::new(3600 * NANOS_PER_SEC),
            created_at: Instant::now(),
        }
    }

    /// Width of the primary window.
    #[must_use]
    pub fn window(&self) -> Duration {
        Duration::from_nanos(self.primary_window().width_ns)
    }

    fn primary_window(&self) -> &Window {
        self.primary.as_ref().unwrap_or(&self.second)
    }

    #[inline]
    fn now_ns(&self) -> u64 {
        crate::util::duration_to_nanos(self.created_at.elapsed())
    }

    /// Record one event.
    #[inline]
    pub fn tick(&self) {
        self.tick_n(1);
    }

    /// Record `n` events.
    #[inline]
    pub fn tick_n(&self, n: u64) {
        if n > 0 {
            let _ = self.total.fetch_add(n, Ordering::Relaxed);
            self.record_windows(self.now_ns(), n);
        }
    }

    #[inline]
    fn record_windows(&self, now_ns: u64, n: u64) {
        if let Some(primary) = &self.primary {
            primary.record(now_ns, n);
        }
        self.second.record(now_ns, n);
        self.minute.record(now_ns, n);
        self.hour.record(now_ns, n);
    }

    /// Record one event unless the total would overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when the total is `u64::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::RateMeter;
    ///
    /// let m = RateMeter::new();
    /// m.try_tick().unwrap();
    /// assert_eq!(m.total(), 1);
    /// ```
    #[inline]
    pub fn try_tick(&self) -> Result<()> {
        self.try_tick_n(1)
    }

    /// Record `n` events unless the total would overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when `total + n` exceeds
    /// `u64::MAX`; nothing is recorded.
    pub fn try_tick_n(&self, n: u64) -> Result<()> {
        if n == 0 {
            return Ok(());
        }
        let _ = self
            .total
            .cas_update(Ordering::Relaxed, Ordering::Relaxed, |t| t.checked_add(n))
            .map_err(|_| MetricsError::Overflow)?;
        self.record_windows(self.now_ns(), n);
        Ok(())
    }

    /// Events per second, estimated over the primary window.
    #[inline]
    #[must_use]
    pub fn rate(&self) -> f64 {
        self.rate_at(self.now_ns())
    }

    fn rate_at(&self, now_ns: u64) -> f64 {
        let w = self.primary_window();
        w.estimate(now_ns) / w.width_secs()
    }

    /// Estimated events in the last second.
    #[inline]
    #[must_use]
    pub fn rate_per_second(&self) -> f64 {
        self.second.estimate(self.now_ns())
    }

    /// Estimated events in the last minute.
    #[inline]
    #[must_use]
    pub fn rate_per_minute(&self) -> f64 {
        self.minute.estimate(self.now_ns())
    }

    /// Estimated events in the last hour.
    #[inline]
    #[must_use]
    pub fn rate_per_hour(&self) -> f64 {
        self.hour.estimate(self.now_ns())
    }

    /// Total events since creation (or the last reset). Always exact.
    #[inline]
    #[must_use]
    pub fn total(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }

    /// `true` when [`RateMeter::rate`] is above `limit` events per second.
    #[inline]
    #[must_use]
    pub fn exceeds_rate(&self, limit: f64) -> bool {
        self.rate() > limit
    }

    /// `true` when recording `n` more events now would keep the primary
    /// window at or below `limit` events per second.
    #[inline]
    #[must_use]
    pub fn can_allow(&self, n: u64, limit: f64) -> bool {
        self.can_allow_at(self.now_ns(), n, limit)
    }

    #[allow(clippy::cast_precision_loss)]
    fn can_allow_at(&self, now_ns: u64, n: u64, limit: f64) -> bool {
        let w = self.primary_window();
        (w.estimate(now_ns) + n as f64) / w.width_secs() <= limit
    }

    /// Record one event if [`RateMeter::can_allow`] says it fits under
    /// `limit`. Returns whether it was recorded. See the module docs for the
    /// concurrent overshoot bound.
    #[inline]
    #[must_use = "the return value says whether the event was admitted"]
    pub fn tick_if_under_limit(&self, limit: f64) -> bool {
        self.tick_burst_if_under_limit(1, limit)
    }

    /// Checked form of [`RateMeter::tick_if_under_limit`].
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Overflow`] when the event fits under the limit
    /// but the total would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::RateMeter;
    ///
    /// let m = RateMeter::new();
    /// assert_eq!(m.try_tick_if_under_limit(10.0), Ok(true));
    /// ```
    pub fn try_tick_if_under_limit(&self, limit: f64) -> Result<bool> {
        let now = self.now_ns();
        if self.can_allow_at(now, 1, limit) {
            self.try_tick()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Record `n` events if they all fit under `limit`. Returns whether they
    /// were recorded.
    #[inline]
    #[must_use = "the return value says whether the events were admitted"]
    pub fn tick_burst_if_under_limit(&self, n: u64, limit: f64) -> bool {
        let now = self.now_ns();
        if self.can_allow_at(now, n, limit) {
            let _ = self.total.fetch_add(n, Ordering::Relaxed);
            self.record_windows(now, n);
            true
        } else {
            false
        }
    }

    /// Clear every counter and window.
    pub fn reset(&self) {
        let now = self.now_ns();
        self.total.store(0, Ordering::Relaxed);
        if let Some(primary) = &self.primary {
            primary.reset(now);
        }
        self.second.reset(now);
        self.minute.reset(now);
        self.hour.reset(now);
    }

    /// Point-in-time statistics.
    #[must_use]
    pub fn stats(&self) -> RateStats {
        let now = self.now_ns();
        let total_events = self.total();
        let age = Duration::from_nanos(now);
        RateStats {
            total_events,
            rate: self.rate_at(now),
            per_second: self.second.estimate(now),
            per_minute: self.minute.estimate(now),
            per_hour: self.hour.estimate(now),
            average_rate: crate::util::per_second(total_events, age),
            age,
            window: self.window(),
        }
    }

    /// Time since the meter was created.
    #[inline]
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// `true` when no event was recorded.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
}

impl Default for RateMeter {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RateMeter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RateMeter({:.1}/s, {} total)", self.rate(), self.total())
    }
}

impl std::fmt::Debug for RateMeter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.stats();
        f.debug_struct("RateMeter")
            .field("total_events", &stats.total_events)
            .field("rate", &stats.rate)
            .field("window", &stats.window)
            .finish()
    }
}

/// Rate meters specialised for request limiting and byte throughput.
pub mod specialized {
    use super::{AtomicU32, Ordering, RateMeter};

    /// Requests-per-second limiter built on [`RateMeter`].
    ///
    /// Best-effort: concurrent callers can overshoot the limit by up to
    /// `threads − 1` requests. Use [`TokenBucket`](crate::TokenBucket) for a
    /// hard limit.
    #[repr(align(64))]
    pub struct ApiRateLimiter {
        meter: RateMeter,
        limit: AtomicU32,
    }

    impl ApiRateLimiter {
        /// Create a limiter allowing `requests_per_second`.
        #[must_use]
        pub fn new(requests_per_second: u32) -> Self {
            Self {
                meter: RateMeter::new(),
                limit: AtomicU32::new(requests_per_second),
            }
        }

        /// Admit one request if it fits under the limit.
        #[inline]
        #[must_use = "the return value says whether the request was admitted"]
        pub fn try_request(&self) -> bool {
            self.meter
                .tick_if_under_limit(f64::from(self.limit.load(Ordering::Relaxed)))
        }

        /// Admit `n` requests if they all fit under the limit.
        #[inline]
        #[must_use = "the return value says whether the requests were admitted"]
        pub fn try_requests(&self, n: u32) -> bool {
            self.meter.tick_burst_if_under_limit(
                u64::from(n),
                f64::from(self.limit.load(Ordering::Relaxed)),
            )
        }

        /// Change the limit.
        #[inline]
        pub fn set_limit(&self, requests_per_second: u32) {
            self.limit.store(requests_per_second, Ordering::Relaxed);
        }

        /// Current limit.
        #[inline]
        #[must_use]
        pub fn get_limit(&self) -> u32 {
            self.limit.load(Ordering::Relaxed)
        }

        /// Current request rate (events per second).
        #[inline]
        #[must_use]
        pub fn current_rate(&self) -> f64 {
            self.meter.rate()
        }

        /// Total admitted requests.
        #[inline]
        #[must_use]
        pub fn total_requests(&self) -> u64 {
            self.meter.total()
        }

        /// `true` when the current rate is above the limit.
        #[inline]
        #[must_use]
        pub fn is_over_limit(&self) -> bool {
            self.meter
                .exceeds_rate(f64::from(self.limit.load(Ordering::Relaxed)))
        }

        /// Clear the meter.
        #[inline]
        pub fn reset(&self) {
            self.meter.reset();
        }
    }

    impl Default for ApiRateLimiter {
        /// 1000 requests per second.
        fn default() -> Self {
            Self::new(1000)
        }
    }

    /// Byte-throughput meter built on [`RateMeter`].
    #[repr(align(64))]
    #[derive(Default)]
    pub struct ThroughputMeter {
        meter: RateMeter,
    }

    impl ThroughputMeter {
        /// Create an empty meter.
        #[must_use]
        pub fn new() -> Self {
            Self {
                meter: RateMeter::new(),
            }
        }

        /// Record `bytes` transferred (any `u64`, no truncation).
        #[inline]
        pub fn record_bytes(&self, bytes: u64) {
            self.meter.tick_n(bytes);
        }

        /// Bytes per second.
        #[inline]
        #[must_use]
        pub fn bytes_per_second(&self) -> f64 {
            self.meter.rate()
        }

        /// KiB per second.
        #[inline]
        #[must_use]
        pub fn kb_per_second(&self) -> f64 {
            self.meter.rate() / 1024.0
        }

        /// MiB per second.
        #[inline]
        #[must_use]
        pub fn mb_per_second(&self) -> f64 {
            self.meter.rate() / (1024.0 * 1024.0)
        }

        /// GiB per second.
        #[inline]
        #[must_use]
        pub fn gb_per_second(&self) -> f64 {
            self.meter.rate() / (1024.0 * 1024.0 * 1024.0)
        }

        /// Total bytes recorded.
        #[inline]
        #[must_use]
        pub fn total_bytes(&self) -> u64 {
            self.meter.total()
        }

        /// Clear the meter.
        #[inline]
        pub fn reset(&self) {
            self.meter.reset();
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact comparisons of exactly representable values
mod tests {
    use super::specialized::{ApiRateLimiter, ThroughputMeter};
    use super::*;
    use proptest::prelude::*;
    use std::sync::{Arc, Barrier};

    const SEC: u64 = NANOS_PER_SEC;

    #[test]
    fn test_rate_meter_basic_operations() {
        let meter = RateMeter::new();
        assert!(meter.is_empty());
        meter.tick();
        meter.tick_n(5);
        meter.tick_n(0);
        assert_eq!(meter.total(), 6);
        assert!(!meter.is_empty());
        assert!(meter.rate() > 0.0);
        assert!(meter.rate_per_second() > 0.0);
        assert!(meter.rate_per_minute() >= 6.0 - 1e-9);
        assert!(meter.rate_per_hour() >= 6.0 - 1e-9);
        assert!(format!("{meter}").contains("6 total"));
        assert!(format!("{meter:?}").contains("RateMeter"));
        assert!(meter.age() < Duration::from_secs(60));
    }

    #[test]
    fn test_window_sliding_estimate_with_injected_clock() {
        let w = Window::new(SEC);
        w.record(0, 100);
        assert_eq!(w.estimate(0), 100.0);
        // Halfway into the next slot, half of the previous slot still counts.
        w.record(SEC, 10);
        assert_eq!(w.estimate(SEC + SEC / 2), 50.0 + 10.0);
        // A reader one slot ahead sees the decaying tail of `current`.
        assert_eq!(w.estimate(2 * SEC + SEC / 2), 5.0);
        // Two or more empty slots later: nothing.
        assert_eq!(w.estimate(5 * SEC), 0.0);
        // A skipped slot drops the previous count.
        w.record(5 * SEC, 1);
        assert_eq!(w.estimate(5 * SEC), 1.0);
        // A reader with an older clock sees the current slot.
        assert_eq!(w.estimate(4 * SEC), 1.0);
    }

    #[test]
    fn test_rate_does_not_reset_at_boundary_regression_ml014() {
        // ML-014: rate() read ~0 right after each wall-clock second, letting
        // tick_if_under_limit admit up to 2x the limit across a boundary.
        let meter = RateMeter::new();
        for i in 0..10 {
            meter.record_windows(SEC - 1_000 + i, 1);
        }
        let just_after = SEC + 1;
        assert!(meter.rate_at(just_after) > 9.9);
        assert!(!meter.can_allow_at(just_after, 1, 10.0));
    }

    #[test]
    fn test_custom_window_is_honoured_regression_ml014() {
        let meter = RateMeter::with_window(Duration::from_secs(10));
        assert_eq!(meter.window(), Duration::from_secs(10));
        meter.record_windows(0, 50);
        // 50 events in a 10 s window = 5 events/s.
        assert_eq!(meter.rate_at(0), 5.0);
        assert!(meter.can_allow_at(0, 50, 10.0));
        assert!(!meter.can_allow_at(0, 51, 10.0));
        assert_eq!(meter.stats().window, Duration::from_secs(10));
        assert_eq!(
            RateMeter::with_window(Duration::ZERO).window(),
            MIN_RATE_WINDOW
        );
    }

    #[test]
    fn test_counters_do_not_wrap_at_u32_regression_ml014() {
        let meter = RateMeter::new();
        let big = u64::from(u32::MAX) + 10;
        meter.tick_n(big);
        assert_eq!(meter.total(), big);
        assert!(meter.rate_per_hour() >= big as f64 * 0.99);
    }

    #[test]
    fn test_rate_limiting_helpers() {
        let meter = RateMeter::new();
        assert!(meter.tick_if_under_limit(10.0));
        assert!(meter.tick_if_under_limit(10.0));
        meter.tick_n(8);
        assert!(!meter.tick_if_under_limit(10.0));
        assert!(meter.exceeds_rate(9.0));
        assert!(!meter.exceeds_rate(11.0));
        assert!(!meter.can_allow(1, 10.0));
        assert_eq!(meter.try_tick_if_under_limit(10.0), Ok(false));
        assert_eq!(meter.try_tick_if_under_limit(1000.0), Ok(true));
    }

    #[test]
    fn test_burst_rate_limiting() {
        let meter = RateMeter::new();
        assert!(meter.tick_burst_if_under_limit(5, 10.0));
        assert!(!meter.tick_burst_if_under_limit(10, 10.0));
        assert!(meter.tick_burst_if_under_limit(3, 10.0));
        assert_eq!(meter.total(), 8);
    }

    #[test]
    fn test_try_tick_overflow() {
        let meter = RateMeter::new();
        meter.try_tick().unwrap();
        meter.try_tick_n(0).unwrap();
        meter.total.store(u64::MAX, Ordering::Relaxed);
        assert_eq!(meter.try_tick(), Err(MetricsError::Overflow));
        assert_eq!(
            meter.try_tick_if_under_limit(f64::MAX),
            Err(MetricsError::Overflow)
        );
    }

    #[test]
    fn test_reset_and_stats() {
        let meter = RateMeter::with_window(Duration::from_secs(2));
        meter.tick_n(100);
        let stats = meter.stats();
        assert_eq!(stats.total_events, 100);
        assert!(stats.per_second > 0.0);
        assert!(stats.rate > 0.0);
        assert!(stats.average_rate > 0.0);
        meter.reset();
        assert_eq!(meter.total(), 0);
        assert_eq!(meter.rate(), 0.0);
        assert_eq!(meter.rate_per_minute(), 0.0);
        assert!(meter.is_empty());
    }

    #[test]
    fn test_api_rate_limiter() {
        let limiter = ApiRateLimiter::new(10);
        for _ in 0..10 {
            assert!(limiter.try_request());
        }
        assert!(!limiter.try_request());
        assert_eq!(limiter.total_requests(), 10);
        assert!(limiter.current_rate() > 0.0);
        limiter.set_limit(20);
        assert_eq!(limiter.get_limit(), 20);
        assert!(!limiter.is_over_limit());
        limiter.reset();
        assert!(limiter.try_requests(5));
        assert!(!limiter.try_requests(20));
        assert_eq!(limiter.total_requests(), 5);
        assert_eq!(ApiRateLimiter::default().get_limit(), 1000);
    }

    #[test]
    fn test_throughput_meter_keeps_full_u64_regression_ml006() {
        // ML-006: record_bytes truncated to u32, so 5 GiB counted as 1 GiB.
        let meter = ThroughputMeter::new();
        let five_gib = 5 * 1024 * 1024 * 1024u64;
        meter.record_bytes(five_gib);
        meter.record_bytes(4 * 1024 * 1024 * 1024);
        assert_eq!(meter.total_bytes(), five_gib + 4 * 1024 * 1024 * 1024);
        assert!(meter.bytes_per_second() > 0.0);
        assert!(meter.kb_per_second() > 0.0);
        assert!(meter.mb_per_second() > 0.0);
        assert!(meter.gb_per_second() > 0.0);
        meter.reset();
        assert_eq!(meter.total_bytes(), 0);
        assert_eq!(ThroughputMeter::default().total_bytes(), 0);
    }

    #[test]
    fn test_rate_meter_concurrent_total_is_exact() {
        let meter = Arc::new(RateMeter::new());
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let meter = Arc::clone(&meter);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    for _ in 0..20_000 {
                        meter.tick();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(meter.total(), 160_000);
        // The hour window holds every event (the test runs well within it).
        assert!((meter.rate_per_hour() - 160_000.0).abs() < 1.0);
    }

    #[test]
    fn test_window_roll_under_contention_stays_consistent() {
        // Threads race across many slot boundaries with an injected clock.
        // The slot index must end on the last slot, and the two visible slots
        // can never hold more than was recorded in total.
        let w = Arc::new(Window::new(1_000));
        let barrier = Arc::new(Barrier::new(4));
        let handles: Vec<_> = (0..4u64)
            .map(|_| {
                let w = Arc::clone(&w);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    for t in 0..10_000u64 {
                        w.record(t * 10, 1);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let last_slot = w.slot.load(Ordering::Relaxed);
        assert_eq!(last_slot, 99);
        let visible = w.current.load(Ordering::Relaxed) + w.previous.load(Ordering::Relaxed);
        assert!(visible >= 1, "the last recorded event must be visible");
        assert!(visible <= 4 * 10_000, "{visible}");
        assert!(w.estimate(99_990) <= 40_000.0);
    }

    proptest! {
        #[test]
        fn prop_estimate_never_exceeds_events_in_two_slots(
            events in proptest::collection::vec((0u64..10 * SEC, 1u64..1000), 1..100)
        ) {
            let mut sorted = events.clone();
            sorted.sort_unstable();
            let w = Window::new(SEC);
            for (t, n) in &sorted {
                w.record(*t, *n);
            }
            let now = sorted.last().map_or(0, |(t, _)| *t);
            let slot = now / SEC;
            let recent: u64 = sorted
                .iter()
                .filter(|(t, _)| *t / SEC + 1 >= slot)
                .map(|(_, n)| n)
                .sum();
            let est = w.estimate(now);
            prop_assert!(est >= 0.0);
            prop_assert!(est <= recent as f64 + 1e-6, "est {} recent {}", est, recent);
        }
    }
}
