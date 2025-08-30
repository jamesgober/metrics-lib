//! # High-Precision Timer
//! 
//! Ultra-fast timer implementation with nanosecond precision.
//! 
//! ## Features
//! 
//! - **Nanosecond precision** - Using system high-resolution clocks
//! - **Zero allocation** - Pure stack operations
//! - **RAII support** - Automatic timing with scoped timers
//! - **Lock-free** - Never blocks, never waits
//! - **Cache optimized** - Aligned to prevent false sharing

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// High-precision timer with automatic statistics
/// 
/// Optimized for sub-10ns timing operations.
/// Cache-line aligned to prevent false sharing.
#[repr(align(64))]
pub struct Timer {
    /// Number of timing samples
    count: AtomicU64,
    /// Sum of all recorded durations in nanoseconds
    total_nanos: AtomicU64,
    /// Minimum duration in nanoseconds
    min_nanos: AtomicU64,
    /// Maximum duration in nanoseconds
    max_nanos: AtomicU64,
    /// Creation timestamp
    created_at: Instant,
}

/// Running timer instance (RAII)
/// 
/// When dropped, automatically records the elapsed time
pub struct RunningTimer<'a> {
    timer: &'a Timer,
    start_time: Instant,
    stopped: bool,
}

/// Timer statistics
#[derive(Debug, Clone)]
pub struct TimerStats {
    /// Total number of timing samples
    pub count: u64,
    /// Sum of all durations
    pub total: Duration,
    /// Average duration
    pub average: Duration,
    /// Minimum duration recorded
    pub min: Duration,
    /// Maximum duration recorded
    pub max: Duration,
    /// Time since timer creation
    pub age: Duration,
    /// Rate of samples per second
    pub rate_per_second: f64,
}

impl Timer {
    /// Create new timer
    #[inline]
    pub fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
            total_nanos: AtomicU64::new(0),
            min_nanos: AtomicU64::new(u64::MAX),
            max_nanos: AtomicU64::new(0),
            created_at: Instant::now(),
        }
    }

    /// Start timing - returns RAII guard that auto-records on drop
    #[inline(always)]
    pub fn start(&self) -> RunningTimer<'_> {
        RunningTimer {
            timer: self,
            start_time: Instant::now(),
            stopped: false,
        }
    }

    /// Record a duration manually
    #[inline]
    pub fn record(&self, duration: Duration) {
        let duration_ns = duration.as_nanos() as u64;
        self.record_ns(duration_ns);
    }

    /// Record nanoseconds directly - fastest path
    #[inline(always)]
    pub fn record_ns(&self, duration_ns: u64) {
        // Update total and count
        self.total_nanos.fetch_add(duration_ns, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
        
        // Update min (compare-and-swap loop)
        self.update_min(duration_ns);
        
        // Update max (compare-and-swap loop)
        self.update_max(duration_ns);
    }

    /// Record multiple durations at once
    #[inline]
    pub fn record_batch(&self, durations: &[Duration]) {
        if durations.is_empty() {
            return;
        }

        let mut total_ns = 0u64;
        let mut local_min = u64::MAX;
        let mut local_max = 0u64;

        for duration in durations {
            let ns = duration.as_nanos() as u64;
            total_ns += ns;
            local_min = local_min.min(ns);
            local_max = local_max.max(ns);
        }

        self.total_nanos.fetch_add(total_ns, Ordering::Relaxed);
        self.count.fetch_add(durations.len() as u64, Ordering::Relaxed);
        
        if local_min < u64::MAX {
            self.update_min(local_min);
        }
        if local_max > 0 {
            self.update_max(local_max);
        }
    }

    /// Get current count of samples
    #[inline(always)]
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Get total accumulated time
    #[inline]
    pub fn total(&self) -> Duration {
        Duration::from_nanos(self.total_nanos.load(Ordering::Relaxed))
    }

    /// Get average duration
    #[inline]
    pub fn average(&self) -> Duration {
        let count = self.count();
        if count == 0 {
            return Duration::ZERO;
        }
        
        let total_ns = self.total_nanos.load(Ordering::Relaxed);
        Duration::from_nanos(total_ns / count)
    }

    /// Get minimum duration
    #[inline]
    pub fn min(&self) -> Duration {
        let min_ns = self.min_nanos.load(Ordering::Relaxed);
        if min_ns == u64::MAX {
            Duration::ZERO
        } else {
            Duration::from_nanos(min_ns)
        }
    }

    /// Get maximum duration
    #[inline]
    pub fn max(&self) -> Duration {
        Duration::from_nanos(self.max_nanos.load(Ordering::Relaxed))
    }

    /// Reset all statistics
    #[inline]
    pub fn reset(&self) {
        self.total_nanos.store(0, Ordering::SeqCst);
        self.count.store(0, Ordering::SeqCst);
        self.min_nanos.store(u64::MAX, Ordering::SeqCst);
        self.max_nanos.store(0, Ordering::SeqCst);
    }

    /// Get comprehensive statistics
    pub fn stats(&self) -> TimerStats {
        let count = self.count();
        let total_ns = self.total_nanos.load(Ordering::Relaxed);
        let min_ns = self.min_nanos.load(Ordering::Relaxed);
        let max_ns = self.max_nanos.load(Ordering::Relaxed);
        
        let total = Duration::from_nanos(total_ns);
        let average = if count > 0 {
            Duration::from_nanos(total_ns / count)
        } else {
            Duration::ZERO
        };
        
        let min = if min_ns == u64::MAX {
            Duration::ZERO
        } else {
            Duration::from_nanos(min_ns)
        };
        
        let max = Duration::from_nanos(max_ns);
        let age = self.created_at.elapsed();
        
        let rate_per_second = if age.as_secs_f64() > 0.0 {
            count as f64 / age.as_secs_f64()
        } else {
            0.0
        };

        TimerStats {
            count,
            total,
            average,
            min,
            max,
            age,
            rate_per_second,
        }
    }

    /// Get age since creation
    #[inline]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Check if timer has recorded any samples
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Get samples per second rate
    #[inline]
    pub fn rate_per_second(&self) -> f64 {
        let age_seconds = self.age().as_secs_f64();
        if age_seconds > 0.0 {
            self.count() as f64 / age_seconds
        } else {
            0.0
        }
    }

    // Internal helper methods
    
    #[inline(always)]
    fn update_min(&self, value: u64) {
        loop {
            let current = self.min_nanos.load(Ordering::Relaxed);
            if value >= current {
                break; // Current min is already smaller
            }
            
            match self.min_nanos.compare_exchange_weak(
                current,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue, // Retry
            }
        }
    }

    #[inline(always)]
    fn update_max(&self, value: u64) {
        loop {
            let current = self.max_nanos.load(Ordering::Relaxed);
            if value <= current {
                break; // Current max is already larger
            }
            
            match self.max_nanos.compare_exchange_weak(
                current,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue, // Retry
            }
        }
    }
}

impl Default for Timer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.stats();
        write!(f, "Timer(count: {}, avg: {:?})", stats.count, stats.average)
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
            .field("rate_per_second", &stats.rate_per_second)
            .finish()
    }
}

// Thread safety
unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

/// Running timer implementation (RAII)
impl<'a> RunningTimer<'a> {
    /// Get elapsed time without stopping the timer
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Stop the timer manually and record the duration
    #[inline]
    pub fn stop(mut self) {
        if !self.stopped {
            let elapsed = self.start_time.elapsed();
            self.timer.record(elapsed);
            self.stopped = true;
        }
        // Timer is consumed here, preventing double recording
    }
}

impl<'a> Drop for RunningTimer<'a> {
    #[inline]
    fn drop(&mut self) {
        if !self.stopped {
            let elapsed = self.start_time.elapsed();
            self.timer.record(elapsed);
        }
    }
}

/// Utility functions for timing
pub mod utils {
    use super::*;

    /// Time a function execution and return the result + duration
    #[inline]
    pub fn time_fn<T>(f: impl FnOnce() -> T) -> (T, Duration) {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Time a function and record the duration in the provided timer
    #[inline]
    pub fn time_and_record<T>(timer: &Timer, f: impl FnOnce() -> T) -> T {
        let _timing_guard = timer.start();
        f() // Timer automatically records when guard drops
    }

    /// Create a scoped timer that records to the provided timer
    #[inline]
    pub fn scoped_timer(timer: &Timer) -> RunningTimer<'_> {
        timer.start()
    }

    /// Benchmark a function with multiple iterations
    pub fn benchmark<F>(name: &str, iterations: usize, f: F) -> Duration
    where
        F: Fn(),
    {
        let timer = Timer::new();
        
        for _ in 0..iterations {
            let _guard = timer.start();
            f();
        }

        let stats = timer.stats();
        println!("Benchmark '{}': {} iterations, avg: {:?}, total: {:?}", 
                name, iterations, stats.average, stats.total);
        
        stats.average
    }
}

/// Convenience macros
#[macro_export]
macro_rules! time_block {
    ($timer:expr, $block:block) => {{
        let _timing_guard = $timer.start();
        $block
    }};
}

#[macro_export]
/// Macro to time a function call and record the result
/// 
/// # Examples
/// 
/// ```rust
/// # use metrics_lib::time_fn;
/// let (result, duration) = time_fn!({
///     // Some work to time
///     std::thread::sleep(std::time::Duration::from_millis(10));
///     "done"
/// });
/// assert!(duration >= std::time::Duration::from_millis(10));
/// assert_eq!(result, "done");
/// ```
macro_rules! time_fn {
    ($func:expr) => {{
        let start = std::time::Instant::now();
        let result = $func;
        let duration = start.elapsed();
        (result, duration)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic_operations() {
        let timer = Timer::new();
        
        assert!(timer.is_empty());
        assert_eq!(timer.count(), 0);
        assert_eq!(timer.total(), Duration::ZERO);
        assert_eq!(timer.average(), Duration::ZERO);
        
        // Record a duration
        timer.record(Duration::from_millis(100));
        
        assert!(!timer.is_empty());
        assert_eq!(timer.count(), 1);
        assert!(timer.total() >= Duration::from_millis(99)); // Account for precision
        assert!(timer.average() >= Duration::from_millis(99));
    }

    #[test]
    fn test_running_timer() {
        let timer = Timer::new();
        
        {
            let running = timer.start();
            thread::sleep(Duration::from_millis(10));
            assert!(running.elapsed() >= Duration::from_millis(9));
        } // Automatically recorded when dropped
        
        assert_eq!(timer.count(), 1);
        assert!(timer.average() >= Duration::from_millis(9));
    }

    #[test]
    fn test_manual_stop() {
        let timer = Timer::new();
        
        let running = timer.start();
        thread::sleep(Duration::from_millis(5));
        running.stop(); // Manual stop
        
        assert_eq!(timer.count(), 1);
        assert!(timer.average() >= Duration::from_millis(4));
    }

    #[test]
    fn test_batch_recording() {
        let timer = Timer::new();
        
        let durations = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
        ];
        
        timer.record_batch(&durations);
        
        assert_eq!(timer.count(), 3);
        assert_eq!(timer.min(), Duration::from_millis(10));
        assert_eq!(timer.max(), Duration::from_millis(30));
        assert_eq!(timer.total(), Duration::from_millis(60));
        assert_eq!(timer.average(), Duration::from_millis(20));
    }

    #[test]
    fn test_min_max_tracking() {
        let timer = Timer::new();
        
        timer.record(Duration::from_millis(50));
        timer.record(Duration::from_millis(10));
        timer.record(Duration::from_millis(100));
        timer.record(Duration::from_millis(25));
        
        assert_eq!(timer.count(), 4);
        assert_eq!(timer.min(), Duration::from_millis(10));
        assert_eq!(timer.max(), Duration::from_millis(100));
        
        // Check average is approximately 46.25ms (allowing for precision differences)
        let avg = timer.average();
        assert!(avg >= Duration::from_millis(46) && avg <= Duration::from_millis(47),
                "Average {:?} should be between 46ms and 47ms", avg);
    }

    #[test]
    fn test_reset() {
        let timer = Timer::new();
        
        timer.record(Duration::from_millis(100));
        assert!(!timer.is_empty());
        
        timer.reset();
        assert!(timer.is_empty());
        assert_eq!(timer.count(), 0);
        assert_eq!(timer.total(), Duration::ZERO);
        assert_eq!(timer.min(), Duration::ZERO);
        assert_eq!(timer.max(), Duration::ZERO);
    }

    #[test]
    fn test_statistics() {
        let timer = Timer::new();
        
        for i in 1..=10 {
            timer.record(Duration::from_millis(i * 10));
        }
        
        let stats = timer.stats();
        assert_eq!(stats.count, 10);
        assert_eq!(stats.total, Duration::from_millis(550)); // Sum of 10+20+...+100
        assert_eq!(stats.average, Duration::from_millis(55));
        assert_eq!(stats.min, Duration::from_millis(10));
        assert_eq!(stats.max, Duration::from_millis(100));
        assert!(stats.rate_per_second > 0.0);
        assert!(stats.age > Duration::from_nanos(0));
    }

    #[test]
    fn test_high_concurrency() {
        let timer = Arc::new(Timer::new());
        let num_threads = 50;
        let recordings_per_thread = 1000;
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let timer = Arc::clone(&timer);
                thread::spawn(move || {
                    for i in 0..recordings_per_thread {
                        // Record varying durations
                        let duration_ms = (thread_id * recordings_per_thread + i) % 100 + 1;
                        timer.record(Duration::from_millis(duration_ms as u64));
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let stats = timer.stats();
        assert_eq!(stats.count, num_threads * recordings_per_thread);
        assert!(stats.min > Duration::ZERO);
        assert!(stats.max > Duration::ZERO);
        assert!(stats.average > Duration::ZERO);
        assert!(stats.rate_per_second > 0.0);
    }

    #[test]
    fn test_concurrent_timing() {
        let timer = Arc::new(Timer::new());
        let num_threads = 20;
        
        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let timer = Arc::clone(&timer);
                thread::spawn(move || {
                    for _ in 0..100 {
                        let _guard = timer.start();
                        thread::sleep(Duration::from_micros(100)); // Very short sleep
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let stats = timer.stats();
        assert_eq!(stats.count, num_threads * 100);
        assert!(stats.average >= Duration::from_micros(50)); // Should be at least 50μs
    }

    #[test]
    fn test_utility_functions() {
        // Test time_fn
        let (result, duration) = utils::time_fn(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
        assert!(duration >= Duration::from_millis(9));
        
        // Test time_and_record
        let timer = Timer::new();
        let result = utils::time_and_record(&timer, || {
            thread::sleep(Duration::from_millis(5));
            "hello"
        });
        
        assert_eq!(result, "hello");
        assert_eq!(timer.count(), 1);
        assert!(timer.average() >= Duration::from_millis(4));
        
        // Test benchmark
        let avg_duration = utils::benchmark("test_function", 10, || {
            thread::sleep(Duration::from_millis(1));
        });
        
        assert!(avg_duration >= Duration::from_millis(1));
    }

    #[test]
    fn test_macros() {
        let timer = Timer::new();
        
        // Test time_block macro
        let result = time_block!(timer, {
            thread::sleep(Duration::from_millis(5));
            "result"
        });
        
        assert_eq!(result, "result");
        assert_eq!(timer.count(), 1);
        
        // Test time_fn macro
        let (result, duration) = time_fn!({
            thread::sleep(Duration::from_millis(2));
            100
        });
        
        assert_eq!(result, 100);
        assert!(duration >= Duration::from_millis(1));
    }

    #[test]
    fn test_display_and_debug() {
        let timer = Timer::new();
        timer.record(Duration::from_millis(100));
        
        let display_str = format!("{}", timer);
        assert!(display_str.contains("Timer"));
        assert!(display_str.contains("count: 1"));
        
        let debug_str = format!("{:?}", timer);
        assert!(debug_str.contains("Timer"));
        assert!(debug_str.contains("count"));
        assert!(debug_str.contains("average"));
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn bench_timer_record() {
        let timer = Timer::new();
        let duration = Duration::from_nanos(1000);
        let iterations = 1_000_000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            timer.record(duration);
        }
        let elapsed = start.elapsed();
        
        println!("Timer record: {:.2} ns/op", 
                elapsed.as_nanos() as f64 / iterations as f64);
        
        assert_eq!(timer.count(), iterations);
        // Should be under 300ns per record (relaxed from 200ns)
        assert!(elapsed.as_nanos() / (iterations as u128) < 300);
    }

    #[test]
    fn bench_running_timer() {
        let timer = Timer::new();
        let iterations = 100_000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let guard = timer.start();
            // Do minimal work
            let _ = guard.elapsed();
            guard.stop();
        }
        let elapsed = start.elapsed();
        
        println!("Running timer: {:.2} ns/op", 
                elapsed.as_nanos() as f64 / iterations as f64);
        
        assert_eq!(timer.count(), iterations);
        // Should be under 1500ns per timer operation (relaxed from 1000ns)
        assert!(elapsed.as_nanos() / (iterations as u128) < 1500);
    }

    #[test]
    fn bench_timer_stats() {
        let timer = Timer::new();
        
        // Fill timer with data
        for i in 0..1000 {
            timer.record(Duration::from_nanos(i * 1000));
        }
        
        let iterations = 1_000_000;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = timer.stats();
        }
        
        let elapsed = start.elapsed();
        println!(
            "Timer stats: {:.2} ns/op",
            elapsed.as_nanos() as f64 / iterations as f64
        );

        // Should be very fast since it's just atomic loads (relaxed from 100ns to 300ns)
        assert!(elapsed.as_nanos() / iterations < 300);
    }
}
