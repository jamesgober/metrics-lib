//! Adaptive sampling and backpressure mechanisms
//! 
//! Provides automatic load shedding and sampling to maintain performance under pressure

use std::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Adaptive sampling strategy
#[derive(Debug, Clone, Copy)]
pub enum SamplingStrategy {
    /// Fixed rate sampling (1 in N)
    /// 
    /// # Fields
    /// - `rate`: The fixed sampling rate, where 1 in `rate` samples are taken.
    Fixed {
        /// The fixed sampling rate, where 1 in `rate` samples are taken.
        rate: u32 
    },
    /// Dynamic rate based on load
    /// 
    /// # Fields
    /// - `min_rate`: The minimum sampling rate.
    /// - `max_rate`: The maximum sampling rate.
    /// - `target_throughput`: The target throughput to maintain.
    Dynamic { 
        /// The minimum sampling rate.
        min_rate: u32,
        /// The maximum sampling rate.
        max_rate: u32,
        /// The target throughput to maintain.
        target_throughput: u64,
    },
    /// Time-based sampling
    /// 
    /// # Fields
    /// - `min_interval`: The minimum interval between samples.
    TimeBased { 
        /// Duration in nanoseconds
        min_interval: u64,
    },
}

/// Adaptive sampler for load shedding
pub struct AdaptiveSampler {
    strategy: SamplingStrategy,
    current_rate: AtomicU32,
    samples_taken: AtomicU64,
    samples_dropped: AtomicU64,
    last_adjustment: parking_lot::Mutex<Instant>,
    overloaded: AtomicBool,
}

impl AdaptiveSampler {
    /// Create new sampler with strategy
    pub fn new(strategy: SamplingStrategy) -> Self {
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
            last_adjustment: parking_lot::Mutex::new(Instant::now()),
            overloaded: AtomicBool::new(false),
        }
    }
    
    /// Check if sample should be taken
    #[inline]
    pub fn should_sample(&self) -> bool {
        match self.strategy {
            SamplingStrategy::Fixed { .. } => {
                self.should_sample_fixed()
            }
            SamplingStrategy::Dynamic { .. } => {
                self.should_sample_dynamic()
            }
            SamplingStrategy::TimeBased { min_interval } => {
                self.should_sample_time_based(Duration::from_nanos(min_interval))
            }
        }
    }
    
    #[inline]
    fn should_sample_fixed(&self) -> bool {
        let rate = self.current_rate.load(Ordering::Relaxed);
        if rate == 1 {
            self.samples_taken.fetch_add(1, Ordering::Relaxed);
            return true;
        }
        
        // Fast thread-local random
        let should_sample = fastrand::u32(1..=rate) == 1;
        
        if should_sample {
            self.samples_taken.fetch_add(1, Ordering::Relaxed);
        } else {
            self.samples_dropped.fetch_add(1, Ordering::Relaxed);
        }
        
        should_sample
    }
    
    fn should_sample_dynamic(&self) -> bool {
        // Check if we need to adjust rate
        let mut last_adjustment = self.last_adjustment.lock();
        let now = Instant::now();
        
        if now.duration_since(*last_adjustment) > Duration::from_secs(1) {
            self.adjust_dynamic_rate();
            *last_adjustment = now;
        }
        drop(last_adjustment);
        
        self.should_sample_fixed()
    }
    
    fn should_sample_time_based(&self, min_interval: Duration) -> bool {
        thread_local! {
            static LAST_SAMPLE: std::cell::RefCell<Option<Instant>> = const { std::cell::RefCell::new(None) };
        }
        
        LAST_SAMPLE.with(|last| {
            let mut last = last.borrow_mut();
            let now = Instant::now();
            
            match *last {
                Some(last_time) if now.duration_since(last_time) < min_interval => {
                    self.samples_dropped.fetch_add(1, Ordering::Relaxed);
                    false
                }
                _ => {
                    *last = Some(now);
                    self.samples_taken.fetch_add(1, Ordering::Relaxed);
                    true
                }
            }
        })
    }
    
    fn adjust_dynamic_rate(&self) {
        if let SamplingStrategy::Dynamic { min_rate, max_rate, target_throughput } = self.strategy {
            let taken = self.samples_taken.load(Ordering::Relaxed);
            let current_rate = self.current_rate.load(Ordering::Relaxed);
            
            let new_rate = if taken > target_throughput {
                // Increase sampling rate (sample less)
                (current_rate * 2).min(max_rate)
            } else if taken < target_throughput / 2 {
                // Decrease sampling rate (sample more)
                (current_rate / 2).max(min_rate)
            } else {
                current_rate
            };
            
            if new_rate != current_rate {
                self.current_rate.store(new_rate, Ordering::Relaxed);
                self.overloaded.store(new_rate > min_rate * 2, Ordering::Relaxed);
            }
            
            // Reset counters
            self.samples_taken.store(0, Ordering::Relaxed);
            self.samples_dropped.store(0, Ordering::Relaxed);
        }
    }
    
    /// Get current sampling rate
    #[inline]
    pub fn current_rate(&self) -> u32 {
        self.current_rate.load(Ordering::Relaxed)
    }
    
    /// Check if system is overloaded
    #[inline]
    pub fn is_overloaded(&self) -> bool {
        self.overloaded.load(Ordering::Relaxed)
    }
    
    /// Get sampling statistics
    pub fn stats(&self) -> SamplingStats {
        SamplingStats {
            samples_taken: self.samples_taken.load(Ordering::Relaxed),
            samples_dropped: self.samples_dropped.load(Ordering::Relaxed),
            current_rate: self.current_rate.load(Ordering::Relaxed),
            is_overloaded: self.is_overloaded(),
        }
    }
}

/// Sampling statistics
#[derive(Debug, Clone)]
pub struct SamplingStats {
    pub samples_taken: u64,
    pub samples_dropped: u64,
    pub current_rate: u32,
    pub is_overloaded: bool,
}

impl SamplingStats {
    /// Calculate sampling percentage
    pub fn sampling_percentage(&self) -> f64 {
        let total = self.samples_taken + self.samples_dropped;
        if total == 0 {
            100.0
        } else {
            (self.samples_taken as f64 / total as f64) * 100.0
        }
    }
}

/// Circuit breaker for metric recording
pub struct MetricCircuitBreaker {
    state: AtomicU32,
    failures: AtomicU64,
    successes: AtomicU64,
    last_state_change: parking_lot::Mutex<Instant>,
    config: CircuitBreakerConfig,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u64,
    pub success_threshold: u64,
    pub timeout: Duration,
    pub half_open_max_calls: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(30),
            half_open_max_calls: 10,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed = 0,
    Open = 1,
    HalfOpen = 2,
}

impl MetricCircuitBreaker {
    /// Create new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: AtomicU32::new(CircuitState::Closed as u32),
            failures: AtomicU64::new(0),
            successes: AtomicU64::new(0),
            last_state_change: parking_lot::Mutex::new(Instant::now()),
            config,
        }
    }
    
    /// Check if operation is allowed
    #[inline]
    pub fn is_allowed(&self) -> bool {
        let state = self.get_state();
        
        match state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout has passed
                let last_change = *self.last_state_change.lock();
                if Instant::now().duration_since(last_change) > self.config.timeout {
                    self.transition_to(CircuitState::HalfOpen);
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => {
                // Allow limited calls
                let calls = self.successes.load(Ordering::Relaxed) + 
                           self.failures.load(Ordering::Relaxed);
                calls < self.config.half_open_max_calls
            }
        }
    }
    
    /// Record success
    #[inline]
    pub fn record_success(&self) {
        let state = self.get_state();
        
        match state {
            CircuitState::Closed => {
                self.failures.store(0, Ordering::Relaxed);
            }
            CircuitState::HalfOpen => {
                let successes = self.successes.fetch_add(1, Ordering::Relaxed) + 1;
                if successes >= self.config.success_threshold {
                    self.transition_to(CircuitState::Closed);
                }
            }
            CircuitState::Open => {} // Shouldn't happen
        }
    }
    
    /// Record failure
    #[inline]
    pub fn record_failure(&self) {
        let state = self.get_state();
        
        match state {
            CircuitState::Closed => {
                let failures = self.failures.fetch_add(1, Ordering::Relaxed) + 1;
                if failures >= self.config.failure_threshold {
                    self.transition_to(CircuitState::Open);
                }
            }
            CircuitState::HalfOpen => {
                self.transition_to(CircuitState::Open);
            }
            CircuitState::Open => {} // Already open
        }
    }
    
    #[inline]
    fn get_state(&self) -> CircuitState {
        match self.state.load(Ordering::Relaxed) {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => unreachable!(),
        }
    }
    
    fn transition_to(&self, new_state: CircuitState) {
        self.state.store(new_state as u32, Ordering::Relaxed);
        self.failures.store(0, Ordering::Relaxed);
        self.successes.store(0, Ordering::Relaxed);
        *self.last_state_change.lock() = Instant::now();
    }
}

/// Backpressure controller
pub struct BackpressureController {
    max_pending: usize,
    pending: AtomicU64,
    rejected: AtomicU64,
}

impl BackpressureController {
    /// Create new controller
    pub fn new(max_pending: usize) -> Self {
        Self {
            max_pending,
            pending: AtomicU64::new(0),
            rejected: AtomicU64::new(0),
        }
    }
    
    /// Try to acquire slot
    #[inline]
    pub fn try_acquire(&self) -> Option<BackpressureGuard<'_>> {
        let pending = self.pending.fetch_add(1, Ordering::Relaxed);
        
        if pending >= self.max_pending as u64 {
            self.pending.fetch_sub(1, Ordering::Relaxed);
            self.rejected.fetch_add(1, Ordering::Relaxed);
            None
        } else {
            Some(BackpressureGuard { controller: self })
        }
    }
    
    /// Get current pending count
    #[inline]
    pub fn pending_count(&self) -> u64 {
        self.pending.load(Ordering::Relaxed)
    }
    
    /// Get rejected count
    #[inline]
    pub fn rejected_count(&self) -> u64 {
        self.rejected.load(Ordering::Relaxed)
    }
}

/// RAII guard for backpressure
pub struct BackpressureGuard<'a> {
    controller: &'a BackpressureController,
}

impl<'a> Drop for BackpressureGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        self.controller.pending.fetch_sub(1, Ordering::Relaxed);
    }
}

// Re-export for convenience
pub mod fastrand {
    #[inline]
    pub fn u32(range: std::ops::RangeInclusive<u32>) -> u32 {
        let start = *range.start();
        let end = *range.end();
        if start == end {
            return start;
        }
        
        // Fast thread-local RNG using xorshift
        thread_local! {
            static RNG: std::cell::Cell<u32> = std::cell::Cell::new({
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&std::thread::current().id(), &mut hasher);
                std::hash::Hasher::finish(&hasher) as u32 | 1
            });
        }
        
        RNG.with(|rng| {
            let mut x = rng.get();
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            rng.set(x);
            start + (x % (end - start + 1))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fixed_sampling() {
        let sampler = AdaptiveSampler::new(SamplingStrategy::Fixed { rate: 10 });
        
        let mut sampled = 0;
        for _ in 0..1000 {
            if sampler.should_sample() {
                sampled += 1;
            }
        }
        
        // Should be approximately 100 (10%)
        assert!(sampled > 50 && sampled < 150);
    }
    
    #[test]
    fn test_circuit_breaker() {
        let breaker = MetricCircuitBreaker::new(CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            half_open_max_calls: 5,
        });
        
        // Initially closed
        assert!(breaker.is_allowed());
        
        // Record failures to open
        for _ in 0..3 {
            breaker.record_failure();
        }
        
        // Should be open
        assert!(!breaker.is_allowed());
        
        // Wait for timeout
        std::thread::sleep(Duration::from_millis(150));
        
        // Should transition to half-open
        assert!(breaker.is_allowed());
        
        // Record successes to close
        breaker.record_success();
        breaker.record_success();
        
        // Should be closed again
        assert!(breaker.is_allowed());
    }
    
    #[test]
    fn test_backpressure() {
        let controller = BackpressureController::new(5);
        
        let mut guards = Vec::new();
        
        // Acquire up to limit
        for _ in 0..5 {
            guards.push(controller.try_acquire().unwrap());
        }
        
        // Next should fail
        assert!(controller.try_acquire().is_none());
        assert_eq!(controller.rejected_count(), 1);
        
        // Drop one guard
        guards.pop();
        
        // Should be able to acquire again
        assert!(controller.try_acquire().is_some());
    }
}
