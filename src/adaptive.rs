//! Load-shedding helpers: adaptive sampling, a circuit breaker and a
//! backpressure controller.
//!
//! Everything here is lock-free. The sampler's dynamic-rate adjustment is
//! claimed by one thread with a compare-and-swap on a timestamp, so callers
//! never wait on each other.

use crate::util::CasUpdate;
use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// How an [`AdaptiveSampler`] decides which events to keep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SamplingStrategy {
    /// Keep one event in `rate`. `0` and `1` both keep every event.
    Fixed {
        /// Keep one in `rate` events.
        rate: u32,
    },
    /// Adjust the 1-in-N rate once per second to keep roughly
    /// `target_throughput` sampled events per second. The rate doubles when
    /// more than `target_throughput` events were sampled in the last second
    /// and halves when fewer than half were, staying within
    /// `min_rate..=max_rate`.
    Dynamic {
        /// Smallest 1-in-N rate (`0` is treated as `1`).
        min_rate: u32,
        /// Largest 1-in-N rate (raised to `min_rate` when smaller).
        max_rate: u32,
        /// Sampled events per second to aim for.
        target_throughput: u64,
    },
    /// Keep at most one event per `min_interval` nanoseconds.
    TimeBased {
        /// Minimum spacing between kept events, in nanoseconds.
        min_interval: u64,
    },
}

/// Sampler for shedding metric-recording load.
///
/// Each sampler has its own state (rate, counters, last-sample time and
/// random stream), so samplers never influence one another.
///
/// # Examples
///
/// ```
/// use metrics_lib::{AdaptiveSampler, SamplingStrategy};
///
/// let sampler = AdaptiveSampler::with_seed(SamplingStrategy::Fixed { rate: 4 }, 42);
/// let kept = (0..10_000).filter(|_| sampler.should_sample()).count();
/// assert!((2_000..3_000).contains(&kept));
/// ```
pub struct AdaptiveSampler {
    strategy: SamplingStrategy,
    current_rate: AtomicU32,
    samples_taken: AtomicU64,
    samples_dropped: AtomicU64,
    /// Nanoseconds since `created_at` of the last dynamic adjustment.
    last_adjustment_ns: AtomicU64,
    /// Nanoseconds since `created_at` of the last kept event (`TimeBased`);
    /// `u64::MAX` before the first one.
    last_sample_ns: AtomicU64,
    overloaded: AtomicBool,
    rng: AtomicU64,
    created_at: Instant,
}

const SPLITMIX_GAMMA: u64 = 0x9e37_79b9_7f4a_7c15;

impl AdaptiveSampler {
    /// Create a sampler seeded from the clock.
    #[must_use]
    pub fn new(strategy: SamplingStrategy) -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(SPLITMIX_GAMMA, |d| {
                crate::util::duration_to_nanos(d) ^ SPLITMIX_GAMMA
            });
        Self::with_seed(strategy, seed)
    }

    /// Create a sampler with a fixed random seed, for reproducible decisions
    /// (tests, simulations).
    #[must_use]
    pub fn with_seed(strategy: SamplingStrategy, seed: u64) -> Self {
        let strategy = normalize(strategy);
        let initial_rate = match strategy {
            SamplingStrategy::Fixed { rate } => rate,
            SamplingStrategy::Dynamic { min_rate, .. } => min_rate,
            SamplingStrategy::TimeBased { .. } => 1,
        };
        Self {
            strategy,
            current_rate: AtomicU32::new(initial_rate),
            samples_taken: AtomicU64::new(0),
            samples_dropped: AtomicU64::new(0),
            last_adjustment_ns: AtomicU64::new(0),
            last_sample_ns: AtomicU64::new(u64::MAX),
            overloaded: AtomicBool::new(false),
            rng: AtomicU64::new(seed),
            created_at: Instant::now(),
        }
    }

    /// The strategy in effect (after normalisation of out-of-range rates).
    #[must_use]
    pub fn strategy(&self) -> SamplingStrategy {
        self.strategy
    }

    #[inline]
    fn now_ns(&self) -> u64 {
        crate::util::duration_to_nanos(self.created_at.elapsed())
    }

    /// Decide whether to keep the current event.
    #[inline]
    #[must_use = "the return value is the sampling decision"]
    pub fn should_sample(&self) -> bool {
        match self.strategy {
            SamplingStrategy::Fixed { .. } => self.sample_one_in_rate(),
            SamplingStrategy::Dynamic { .. } => {
                self.maybe_adjust(self.now_ns());
                self.sample_one_in_rate()
            }
            SamplingStrategy::TimeBased { min_interval } => {
                self.sample_time_based(self.now_ns(), min_interval)
            }
        }
    }

    #[inline]
    fn next_random(&self) -> u64 {
        // SplitMix64 over a shared Weyl sequence: each call gets a distinct
        // input, so concurrent callers never reuse a value.
        let mut z = self
            .rng
            .fetch_add(SPLITMIX_GAMMA, Ordering::Relaxed)
            .wrapping_add(SPLITMIX_GAMMA);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    #[inline]
    fn sample_one_in_rate(&self) -> bool {
        let rate = self.current_rate.load(Ordering::Relaxed).max(1);
        // Multiply-shift maps the 64-bit value onto 0..rate without modulo.
        let keep = rate == 1 || ((u128::from(self.next_random()) * u128::from(rate)) >> 64) == 0;
        self.count_decision(keep)
    }

    #[inline]
    fn count_decision(&self, keep: bool) -> bool {
        let counter = if keep {
            &self.samples_taken
        } else {
            &self.samples_dropped
        };
        let _ = counter.fetch_add(1, Ordering::Relaxed);
        keep
    }

    fn sample_time_based(&self, now: u64, min_interval: u64) -> bool {
        let keep = self
            .last_sample_ns
            .cas_update(Ordering::AcqRel, Ordering::Acquire, |last| {
                (last == u64::MAX || now.saturating_sub(last) >= min_interval).then_some(now)
            })
            .is_ok();
        self.count_decision(keep)
    }

    fn maybe_adjust(&self, now: u64) {
        const PERIOD: u64 = 1_000_000_000;
        let last = self.last_adjustment_ns.load(Ordering::Relaxed);
        if now.saturating_sub(last) < PERIOD {
            return;
        }
        // Exactly one caller wins the period and adjusts.
        if self
            .last_adjustment_ns
            .compare_exchange(last, now, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return;
        }
        let SamplingStrategy::Dynamic {
            min_rate,
            max_rate,
            target_throughput,
        } = self.strategy
        else {
            return;
        };
        let taken = self.samples_taken.swap(0, Ordering::Relaxed);
        let _ = self.samples_dropped.swap(0, Ordering::Relaxed);
        let current = self.current_rate.load(Ordering::Relaxed);
        let next = if taken > target_throughput {
            current.saturating_mul(2).min(max_rate)
        } else if taken < target_throughput / 2 {
            (current / 2).max(min_rate)
        } else {
            current
        };
        self.current_rate.store(next, Ordering::Relaxed);
        self.overloaded
            .store(next > min_rate.saturating_mul(2), Ordering::Relaxed);
    }

    /// Current 1-in-N rate.
    #[inline]
    #[must_use]
    pub fn current_rate(&self) -> u32 {
        self.current_rate.load(Ordering::Relaxed)
    }

    /// `true` when a dynamic sampler has raised its rate above twice its
    /// minimum.
    #[inline]
    #[must_use]
    pub fn is_overloaded(&self) -> bool {
        self.overloaded.load(Ordering::Relaxed)
    }

    /// Counters for the current measurement period.
    #[must_use]
    pub fn stats(&self) -> SamplingStats {
        SamplingStats {
            samples_taken: self.samples_taken.load(Ordering::Relaxed),
            samples_dropped: self.samples_dropped.load(Ordering::Relaxed),
            current_rate: self.current_rate(),
            is_overloaded: self.is_overloaded(),
        }
    }
}

/// Clamp rates into a range the sampler can use (ML-011).
fn normalize(strategy: SamplingStrategy) -> SamplingStrategy {
    match strategy {
        SamplingStrategy::Fixed { rate } => SamplingStrategy::Fixed { rate: rate.max(1) },
        SamplingStrategy::Dynamic {
            min_rate,
            max_rate,
            target_throughput,
        } => {
            let min_rate = min_rate.max(1);
            SamplingStrategy::Dynamic {
                min_rate,
                max_rate: max_rate.max(min_rate),
                target_throughput,
            }
        }
        other @ SamplingStrategy::TimeBased { .. } => other,
    }
}

impl std::fmt::Debug for AdaptiveSampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdaptiveSampler")
            .field("strategy", &self.strategy)
            .field("stats", &self.stats())
            .finish_non_exhaustive()
    }
}

/// Counters snapshot from an [`AdaptiveSampler`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub struct SamplingStats {
    /// Events kept in the current period.
    pub samples_taken: u64,
    /// Events dropped in the current period.
    pub samples_dropped: u64,
    /// Current 1-in-N rate (`1` keeps everything).
    pub current_rate: u32,
    /// `true` when the dynamic rate is above twice its minimum.
    pub is_overloaded: bool,
}

impl SamplingStats {
    /// Percentage of events kept (`100.0` when nothing was seen).
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // statistics: f64 precision is the contract
    pub fn sampling_percentage(&self) -> f64 {
        let total = self.samples_taken + self.samples_dropped;
        if total == 0 {
            100.0
        } else {
            self.samples_taken as f64 / total as f64 * 100.0
        }
    }
}

/// State of a [`MetricCircuitBreaker`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CircuitState {
    /// Calls flow normally; failures are counted.
    Closed,
    /// Calls are rejected until the timeout elapses.
    Open,
    /// A limited number of trial calls is let through.
    HalfOpen,
}

impl CircuitState {
    const fn as_u32(self) -> u32 {
        match self {
            CircuitState::Closed => 0,
            CircuitState::Open => 1,
            CircuitState::HalfOpen => 2,
        }
    }

    const fn from_u32(value: u32) -> Self {
        match value {
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => CircuitState::Closed,
        }
    }
}

/// Thresholds for a [`MetricCircuitBreaker`].
///
/// # Examples
///
/// ```
/// use metrics_lib::CircuitBreakerConfig;
/// use std::time::Duration;
///
/// let config = CircuitBreakerConfig::default()
///     .with_failure_threshold(3)
///     .with_timeout(Duration::from_secs(5));
/// assert_eq!(config.failure_threshold, 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CircuitBreakerConfig {
    /// Consecutive failures (while closed) that open the circuit.
    pub failure_threshold: u64,
    /// Successes (while half-open) that close the circuit again.
    pub success_threshold: u64,
    /// How long the circuit stays open before trial calls are allowed.
    pub timeout: Duration,
    /// Trial calls admitted while half-open.
    pub half_open_max_calls: u64,
}

impl Default for CircuitBreakerConfig {
    /// Five failures open the circuit for 30 s; three of up to ten trial
    /// calls must succeed to close it.
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(30),
            half_open_max_calls: 10,
        }
    }
}

impl CircuitBreakerConfig {
    /// Set [`CircuitBreakerConfig::failure_threshold`].
    #[must_use]
    pub fn with_failure_threshold(mut self, failures: u64) -> Self {
        self.failure_threshold = failures;
        self
    }

    /// Set [`CircuitBreakerConfig::success_threshold`].
    #[must_use]
    pub fn with_success_threshold(mut self, successes: u64) -> Self {
        self.success_threshold = successes;
        self
    }

    /// Set [`CircuitBreakerConfig::timeout`].
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set [`CircuitBreakerConfig::half_open_max_calls`].
    #[must_use]
    pub fn with_half_open_max_calls(mut self, calls: u64) -> Self {
        self.half_open_max_calls = calls;
        self
    }
}

/// Circuit breaker guarding an unreliable dependency (for example a metrics
/// backend you push to).
///
/// # Examples
///
/// ```
/// use metrics_lib::{CircuitBreakerConfig, CircuitState, MetricCircuitBreaker};
///
/// let breaker = MetricCircuitBreaker::new(CircuitBreakerConfig::default().with_failure_threshold(2));
/// breaker.record_failure();
/// breaker.record_failure();
/// assert_eq!(breaker.state(), CircuitState::Open);
/// assert!(breaker.check().is_err());
/// ```
pub struct MetricCircuitBreaker {
    state: AtomicU32,
    failures: AtomicU64,
    successes: AtomicU64,
    half_open_calls: AtomicU64,
    /// Nanoseconds since `created_at` of the last state change.
    last_change_ns: AtomicU64,
    config: CircuitBreakerConfig,
    created_at: Instant,
}

impl MetricCircuitBreaker {
    /// Create a closed breaker.
    #[must_use]
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: AtomicU32::new(CircuitState::Closed.as_u32()),
            failures: AtomicU64::new(0),
            successes: AtomicU64::new(0),
            half_open_calls: AtomicU64::new(0),
            last_change_ns: AtomicU64::new(0),
            config,
            created_at: Instant::now(),
        }
    }

    /// The configuration in use.
    #[must_use]
    pub fn config(&self) -> &CircuitBreakerConfig {
        &self.config
    }

    #[inline]
    fn now_ns(&self) -> u64 {
        crate::util::duration_to_nanos(self.created_at.elapsed())
    }

    /// Current state.
    #[must_use]
    pub fn state(&self) -> CircuitState {
        CircuitState::from_u32(self.state.load(Ordering::Acquire))
    }

    /// `true` when a call may proceed. Moves an expired open circuit to
    /// half-open, and counts the call against the half-open trial budget.
    #[must_use = "the return value says whether the call may proceed"]
    pub fn is_allowed(&self) -> bool {
        self.is_allowed_at(self.now_ns())
    }

    fn is_allowed_at(&self, now: u64) -> bool {
        match self.state() {
            CircuitState::Closed => true,
            CircuitState::Open => {
                let opened = self.last_change_ns.load(Ordering::Acquire);
                let timeout = crate::util::duration_to_nanos(self.config.timeout);
                if now.saturating_sub(opened) < timeout {
                    return false;
                }
                // The counters were zeroed when the circuit opened and nothing
                // touches them while it is open, so the transition only flips
                // the state; every caller then follows the half-open rules.
                let _ = self.state.compare_exchange(
                    CircuitState::Open.as_u32(),
                    CircuitState::HalfOpen.as_u32(),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
                self.admit_half_open()
            }
            CircuitState::HalfOpen => self.admit_half_open(),
        }
    }

    fn admit_half_open(&self) -> bool {
        let max = self.config.half_open_max_calls;
        self.half_open_calls
            .cas_update(Ordering::AcqRel, Ordering::Acquire, |c| {
                (c < max).then(|| c + 1)
            })
            .is_ok()
    }

    /// [`MetricCircuitBreaker::is_allowed`] as a `Result`.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::CircuitOpen`] when the call must not proceed.
    pub fn check(&self) -> Result<()> {
        if self.is_allowed() {
            Ok(())
        } else {
            Err(MetricsError::CircuitOpen)
        }
    }

    /// Record a successful call.
    pub fn record_success(&self) {
        match self.state() {
            CircuitState::Closed => self.failures.store(0, Ordering::Relaxed),
            CircuitState::HalfOpen => {
                let successes = self.successes.fetch_add(1, Ordering::AcqRel) + 1;
                if successes >= self.config.success_threshold {
                    self.transition(CircuitState::HalfOpen, CircuitState::Closed);
                }
            }
            CircuitState::Open => {}
        }
    }

    /// Record a failed call.
    pub fn record_failure(&self) {
        match self.state() {
            CircuitState::Closed => {
                let failures = self.failures.fetch_add(1, Ordering::AcqRel) + 1;
                if failures >= self.config.failure_threshold {
                    self.transition(CircuitState::Closed, CircuitState::Open);
                }
            }
            CircuitState::HalfOpen => self.transition(CircuitState::HalfOpen, CircuitState::Open),
            CircuitState::Open => {}
        }
    }

    fn transition(&self, from: CircuitState, to: CircuitState) {
        if self
            .state
            .compare_exchange(
                from.as_u32(),
                to.as_u32(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
        {
            self.reset_counters(self.now_ns());
        }
    }

    fn reset_counters(&self, now: u64) {
        self.failures.store(0, Ordering::Relaxed);
        self.successes.store(0, Ordering::Relaxed);
        self.half_open_calls.store(0, Ordering::Relaxed);
        self.last_change_ns.store(now, Ordering::Release);
    }
}

impl std::fmt::Debug for MetricCircuitBreaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricCircuitBreaker")
            .field("state", &self.state())
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

/// Bounds the number of in-flight operations.
///
/// # Examples
///
/// ```
/// use metrics_lib::BackpressureController;
///
/// let gate = BackpressureController::new(2);
/// let a = gate.try_acquire().unwrap();
/// let _b = gate.try_acquire().unwrap();
/// assert!(gate.try_acquire().is_none());
/// drop(a);
/// assert!(gate.try_acquire().is_some());
/// ```
pub struct BackpressureController {
    max_pending: u64,
    pending: AtomicU64,
    rejected: AtomicU64,
}

impl BackpressureController {
    /// Allow at most `max_pending` concurrent guards.
    #[must_use]
    pub fn new(max_pending: usize) -> Self {
        Self {
            max_pending: u64::try_from(max_pending).unwrap_or(u64::MAX),
            pending: AtomicU64::new(0),
            rejected: AtomicU64::new(0),
        }
    }

    /// Reserve a slot. The slot is released when the guard is dropped.
    /// Returns `None` (and counts a rejection) when all slots are taken.
    #[must_use = "dropping the guard releases the slot immediately"]
    pub fn try_acquire(&self) -> Option<BackpressureGuard<'_>> {
        let max = self.max_pending;
        if self
            .pending
            .cas_update(Ordering::AcqRel, Ordering::Acquire, |p| {
                (p < max).then(|| p + 1)
            })
            .is_ok()
        {
            Some(BackpressureGuard { controller: self })
        } else {
            let _ = self.rejected.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Slots in use.
    #[must_use]
    pub fn pending_count(&self) -> u64 {
        self.pending.load(Ordering::Acquire)
    }

    /// Rejected acquisitions since creation.
    #[must_use]
    pub fn rejected_count(&self) -> u64 {
        self.rejected.load(Ordering::Relaxed)
    }

    /// Configured slot count.
    #[must_use]
    pub fn max_pending(&self) -> u64 {
        self.max_pending
    }
}

impl std::fmt::Debug for BackpressureController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackpressureController")
            .field("max_pending", &self.max_pending)
            .field("pending", &self.pending_count())
            .field("rejected", &self.rejected_count())
            .finish()
    }
}

/// Slot held in a [`BackpressureController`]; released on drop.
#[must_use = "dropping the guard releases the slot immediately"]
pub struct BackpressureGuard<'a> {
    controller: &'a BackpressureController,
}

impl Drop for BackpressureGuard<'_> {
    fn drop(&mut self) {
        let _ = self.controller.pending.fetch_sub(1, Ordering::AcqRel);
    }
}

impl std::fmt::Debug for BackpressureGuard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackpressureGuard").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    const SEC: u64 = 1_000_000_000;

    #[test]
    fn test_fixed_sampling_rate_is_close_to_one_in_n() {
        let sampler = AdaptiveSampler::with_seed(SamplingStrategy::Fixed { rate: 10 }, 7);
        let kept = (0..100_000).filter(|_| sampler.should_sample()).count();
        assert!((9_000..11_000).contains(&kept), "{kept}");
        let stats = sampler.stats();
        assert_eq!(stats.samples_taken + stats.samples_dropped, 100_000);
    }

    #[test]
    fn test_seeded_samplers_are_reproducible() {
        let a = AdaptiveSampler::with_seed(SamplingStrategy::Fixed { rate: 3 }, 99);
        let b = AdaptiveSampler::with_seed(SamplingStrategy::Fixed { rate: 3 }, 99);
        let da: Vec<bool> = (0..1000).map(|_| a.should_sample()).collect();
        let db: Vec<bool> = (0..1000).map(|_| b.should_sample()).collect();
        assert_eq!(da, db);
    }

    #[test]
    fn test_zero_rates_do_not_panic_regression_ml011() {
        // ML-011: `Fixed { rate: 0 }` panicked (range 1..=0).
        let fixed = AdaptiveSampler::new(SamplingStrategy::Fixed { rate: 0 });
        assert!(fixed.should_sample());
        assert_eq!(fixed.current_rate(), 1);
        let dynamic = AdaptiveSampler::new(SamplingStrategy::Dynamic {
            min_rate: 0,
            max_rate: 0,
            target_throughput: 10,
        });
        assert!(dynamic.should_sample());
        assert_eq!(
            dynamic.strategy(),
            SamplingStrategy::Dynamic {
                min_rate: 1,
                max_rate: 1,
                target_throughput: 10
            }
        );
    }

    #[test]
    fn test_dynamic_adjustment_saturates_regression_ml011() {
        let s = AdaptiveSampler::with_seed(
            SamplingStrategy::Dynamic {
                min_rate: u32::MAX - 1,
                max_rate: u32::MAX,
                target_throughput: 0,
            },
            1,
        );
        let _ = s.count_decision(true);
        s.maybe_adjust(2 * SEC);
        assert_eq!(s.current_rate(), u32::MAX);
    }

    #[test]
    fn test_dynamic_rate_rises_and_falls() {
        let s = AdaptiveSampler::with_seed(
            SamplingStrategy::Dynamic {
                min_rate: 1,
                max_rate: 64,
                target_throughput: 10,
            },
            3,
        );
        for _ in 0..100 {
            let _ = s.sample_one_in_rate();
        }
        s.maybe_adjust(SEC + 1);
        assert_eq!(s.current_rate(), 2);
        // Within the same period nothing changes.
        s.maybe_adjust(SEC + 2);
        assert_eq!(s.current_rate(), 2);
        for _ in 0..1000 {
            let _ = s.sample_one_in_rate();
        }
        s.maybe_adjust(3 * SEC);
        assert_eq!(s.current_rate(), 4);
        assert!(s.is_overloaded());
        // A quiet period halves the rate.
        s.maybe_adjust(5 * SEC);
        assert_eq!(s.current_rate(), 2);
        assert!(!s.is_overloaded());
        assert!(format!("{s:?}").contains("AdaptiveSampler"));
    }

    #[test]
    fn test_time_based_samplers_are_independent_regression_ml016() {
        // ML-016: TimeBased state was a thread-local shared by every sampler.
        let a = AdaptiveSampler::new(SamplingStrategy::TimeBased {
            min_interval: 60 * SEC,
        });
        let b = AdaptiveSampler::new(SamplingStrategy::TimeBased {
            min_interval: 60 * SEC,
        });
        assert!(a.should_sample());
        assert!(b.should_sample());
        assert!(!a.should_sample());
        assert!(!b.should_sample());
    }

    #[test]
    fn test_time_based_interval_with_injected_clock() {
        let s = AdaptiveSampler::new(SamplingStrategy::TimeBased { min_interval: 100 });
        assert!(s.sample_time_based(1_000, 100));
        assert!(!s.sample_time_based(1_050, 100));
        assert!(s.sample_time_based(1_100, 100));
        let stats = s.stats();
        assert_eq!((stats.samples_taken, stats.samples_dropped), (2, 1));
        assert!((stats.sampling_percentage() - 200.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_sampling_percentage_defaults_to_100() {
        let s = AdaptiveSampler::new(SamplingStrategy::Fixed { rate: 10 });
        assert!((s.stats().sampling_percentage() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_circuit_breaker_lifecycle_with_injected_clock() {
        let breaker = MetricCircuitBreaker::new(
            CircuitBreakerConfig::default()
                .with_failure_threshold(3)
                .with_success_threshold(2)
                .with_timeout(Duration::from_millis(100))
                .with_half_open_max_calls(2),
        );
        assert!(breaker.is_allowed_at(0));
        breaker.record_success();
        for _ in 0..3 {
            breaker.record_failure();
        }
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(breaker.check().is_err());
        let opened = breaker.last_change_ns.load(Ordering::Relaxed);
        assert!(!breaker.is_allowed_at(opened + 50_000_000));
        // After the timeout: two trial calls only.
        let later = opened + 200_000_000;
        assert!(breaker.is_allowed_at(later));
        assert_eq!(breaker.state(), CircuitState::HalfOpen);
        assert!(breaker.is_allowed_at(later));
        assert!(!breaker.is_allowed_at(later));
        breaker.record_success();
        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.check().is_ok());
        assert_eq!(breaker.config().failure_threshold, 3);
        assert!(format!("{breaker:?}").contains("Closed"));
    }

    #[test]
    fn test_circuit_breaker_half_open_failure_reopens() {
        let breaker = MetricCircuitBreaker::new(
            CircuitBreakerConfig::default()
                .with_failure_threshold(1)
                .with_timeout(Duration::ZERO),
        );
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        breaker.record_success(); // ignored while open
        breaker.record_failure(); // ignored while open
        assert!(breaker.is_allowed());
        assert_eq!(breaker.state(), CircuitState::HalfOpen);
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        assert_eq!(CircuitState::from_u32(99), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_half_open_budget_under_contention() {
        let breaker = Arc::new(MetricCircuitBreaker::new(
            CircuitBreakerConfig::default()
                .with_failure_threshold(1)
                .with_timeout(Duration::ZERO)
                .with_half_open_max_calls(5),
        ));
        breaker.record_failure();
        let barrier = Arc::new(Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let breaker = Arc::clone(&breaker);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    (0..10).filter(|_| breaker.is_allowed()).count()
                })
            })
            .collect();
        let admitted: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert_eq!(admitted, 5);
    }

    #[test]
    fn test_backpressure_is_exact_under_contention() {
        let gate = Arc::new(BackpressureController::new(4));
        let barrier = Arc::new(Barrier::new(16));
        let peak = Arc::new(AtomicU64::new(0));
        let handles: Vec<_> = (0..16)
            .map(|_| {
                let gate = Arc::clone(&gate);
                let barrier = Arc::clone(&barrier);
                let peak = Arc::clone(&peak);
                std::thread::spawn(move || {
                    let _ = barrier.wait();
                    for _ in 0..1000 {
                        if let Some(guard) = gate.try_acquire() {
                            let _ = peak.fetch_max(gate.pending_count(), Ordering::Relaxed);
                            drop(guard);
                        }
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert!(peak.load(Ordering::Relaxed) <= 4);
        assert_eq!(gate.pending_count(), 0);
        assert_eq!(gate.max_pending(), 4);
        assert!(format!("{gate:?}").contains("BackpressureController"));
    }

    #[test]
    fn test_backpressure_basic() {
        let gate = BackpressureController::new(1);
        let guard = gate.try_acquire().unwrap();
        assert!(format!("{guard:?}").contains("BackpressureGuard"));
        assert!(gate.try_acquire().is_none());
        assert_eq!(gate.rejected_count(), 1);
        drop(guard);
        assert!(gate.try_acquire().is_some());
    }
}
