//! Async support for metrics
//!
//! Provides async-aware metric recording with zero-cost abstractions

use crate::Timer;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

/// Async timer guard that records on drop
pub struct AsyncTimerGuard<'a> {
    timer: &'a Timer,
    start: Instant,
    recorded: bool,
}

impl<'a> AsyncTimerGuard<'a> {
    /// Creates a new `AsyncTimerGuard` for the given timer, starting timing immediately.
    #[inline]
    pub fn new(timer: &'a Timer) -> Self {
        Self {
            timer,
            start: Instant::now(),
            recorded: false,
        }
    }

    /// Returns the elapsed time since the guard was created.
    #[inline]
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    /// Stops the timer and records the elapsed time if not already recorded.
    #[inline]
    pub fn stop(mut self) {
        if !self.recorded {
            self.timer.record(self.start.elapsed());
            self.recorded = true;
        }
    }
}

impl<'a> Drop for AsyncTimerGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        if !self.recorded {
            self.timer.record(self.start.elapsed());
        }
    }
}

/// Extension trait for async timer operations
pub trait AsyncTimerExt {
    /// Start an async-aware timer
    fn start_async(&self) -> AsyncTimerGuard<'_>;

    /// Time an async function
    fn time_async<F, Fut, T>(&self, f: F) -> TimedFuture<'_, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>;
}

impl AsyncTimerExt for Timer {
    #[inline]
    fn start_async(&self) -> AsyncTimerGuard<'_> {
        AsyncTimerGuard::new(self)
    }

    #[inline]
    fn time_async<F, Fut, T>(&self, f: F) -> TimedFuture<'_, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        TimedFuture {
            timer: self,
            future: f(),
            start: Some(Instant::now()),
        }
    }
}

/// Future wrapper that times execution
pub struct TimedFuture<'a, F> {
    timer: &'a Timer,
    future: F,
    start: Option<Instant>,
}

impl<'a, F, T> Future for TimedFuture<'a, F>
where
    F: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let future = unsafe { Pin::new_unchecked(&mut this.future) };

        match future.poll(cx) {
            Poll::Ready(result) => {
                if let Some(start) = this.start.take() {
                    this.timer.record(start.elapsed());
                }
                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Batched metric updates for async contexts
pub struct AsyncMetricBatch {
    updates: Vec<MetricUpdate>,
}

enum MetricUpdate {
    CounterInc { name: &'static str, value: u64 },
    GaugeSet { name: &'static str, value: f64 },
    TimerRecord { name: &'static str, nanos: u64 },
    RateTick { name: &'static str },
}

impl AsyncMetricBatch {
    /// Create new batch
    pub fn new() -> Self {
        Self {
            updates: Vec::with_capacity(64),
        }
    }
}

impl Default for AsyncMetricBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncMetricBatch {
    /// Add counter increment
    #[inline]
    pub fn counter_inc(&mut self, name: &'static str, value: u64) {
        self.updates.push(MetricUpdate::CounterInc { name, value });
    }

    /// Add gauge set
    #[inline]
    pub fn gauge_set(&mut self, name: &'static str, value: f64) {
        self.updates.push(MetricUpdate::GaugeSet { name, value });
    }

    /// Add timer recording
    #[inline]
    pub fn timer_record(&mut self, name: &'static str, nanos: u64) {
        self.updates.push(MetricUpdate::TimerRecord { name, nanos });
    }

    /// Add rate tick
    #[inline]
    pub fn rate_tick(&mut self, name: &'static str) {
        self.updates.push(MetricUpdate::RateTick { name });
    }

    /// Flush all updates to metrics
    pub fn flush(self, metrics: &crate::MetricsCore) {
        for update in self.updates {
            match update {
                MetricUpdate::CounterInc { name, value } => {
                    metrics.counter(name).add(value);
                }
                MetricUpdate::GaugeSet { name, value } => {
                    metrics.gauge(name).set(value);
                }
                MetricUpdate::TimerRecord { name, nanos } => {
                    metrics.timer(name).record_ns(nanos);
                }
                MetricUpdate::RateTick { name } => {
                    metrics.rate(name).tick();
                }
            }
        }
    }

    /// Check if batch is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.updates.is_empty()
    }

    /// Get number of pending updates
    #[inline]
    pub fn len(&self) -> usize {
        self.updates.len()
    }
}

/// Async-aware metrics batcher with automatic flushing
#[cfg(feature = "async")]
#[allow(dead_code)]
pub struct AsyncMetricsBatcher {
    batch: tokio::sync::Mutex<AsyncMetricBatch>,
    flush_interval: std::time::Duration,
    max_batch_size: usize,
}

#[cfg(feature = "async")]
impl AsyncMetricsBatcher {
    /// Create new batcher
    #[allow(dead_code)]
    pub fn new(flush_interval: std::time::Duration, max_batch_size: usize) -> Self {
        Self {
            batch: tokio::sync::Mutex::new(AsyncMetricBatch::new()),
            flush_interval,
            max_batch_size,
        }
    }

    /// Record metric update
    #[allow(dead_code)]
    pub async fn record(&self, update: impl FnOnce(&mut AsyncMetricBatch)) {
        let mut batch = self.batch.lock().await;
        update(&mut batch);

        if batch.len() >= self.max_batch_size {
            let batch = std::mem::take(&mut *batch);

            // Flush in background
            tokio::spawn(async move {
                batch.flush(crate::metrics());
            });
        }
    }

    /// Start background flusher
    #[allow(dead_code)]
    pub fn start_flusher(self: std::sync::Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.flush_interval);

            loop {
                interval.tick().await;

                let batch = {
                    let mut guard = self.batch.lock().await;
                    if guard.is_empty() {
                        continue;
                    }
                    std::mem::take(&mut *guard)
                };

                batch.flush(crate::metrics());
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_timer_guard() {
        let timer = Timer::new();

        {
            let _guard = timer.start_async();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        assert_eq!(timer.count(), 1);
        assert!(timer.average() >= std::time::Duration::from_millis(9));
    }

    #[test]
    fn test_metric_batch() {
        let mut batch = AsyncMetricBatch::new();

        batch.counter_inc("test", 5);
        batch.gauge_set("test", 42.5);
        batch.timer_record("test", 1000);
        batch.rate_tick("test");

        assert_eq!(batch.len(), 4);
        assert!(!batch.is_empty());

        let metrics = crate::MetricsCore::new();
        batch.flush(&metrics);

        assert_eq!(metrics.counter("test").get(), 5);
        assert_eq!(metrics.gauge("test").get(), 42.5);
        assert_eq!(metrics.timer("test").count(), 1);
        metrics.rate("test").tick_n(1); // Simulate a tick
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_timed_future() {
        let timer = Timer::new();

        let result = timer
            .time_async(|| async {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                42
            })
            .await;

        assert_eq!(result, 42);
        assert_eq!(timer.count(), 1);
        assert!(timer.average() >= std::time::Duration::from_millis(9));
    }
}
