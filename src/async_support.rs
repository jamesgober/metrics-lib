//! Tokio helpers (feature `async`).
//!
//! - [`AsyncTimerExt`] times a future with a [`Timer`](crate::Timer):
//!   `timer.time_async(|| fetch()).await` records how long the future took
//!   to complete.
//! - [`AsyncMetricBatch`] collects updates and applies them to a
//!   [`MetricsCore`] in one go.
//! - [`AsyncMetricsBatcher`] does that for concurrent tasks, flushing when
//!   the batch is full and on a timer driven by a cancellable background task.
//! - [`record_runtime_metrics`] copies Tokio's stable runtime metrics into
//!   gauges.
//!
//! Nothing here blocks the executor: flushing a batch only performs atomic
//! updates and registry lookups.

use crate::{MetricsCore, Registry};
use std::borrow::Cow;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

#[cfg(feature = "timer")]
use crate::Timer;

/// Guard that records the elapsed time into a timer when dropped or
/// stopped. Same as [`RunningTimer`](crate::RunningTimer); kept for code
/// that wants an async-specific name.
#[cfg(feature = "timer")]
#[must_use = "dropping the guard immediately records a near-zero duration"]
pub struct AsyncTimerGuard<'a> {
    timer: &'a Timer,
    start: Instant,
    armed: bool,
}

#[cfg(feature = "timer")]
impl<'a> AsyncTimerGuard<'a> {
    /// Start timing into `timer`.
    pub fn new(timer: &'a Timer) -> Self {
        Self {
            timer,
            start: Instant::now(),
            armed: true,
        }
    }

    /// Time elapsed so far.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Record now and return the elapsed time.
    #[allow(clippy::must_use_candidate)] // the duration is optional information
    pub fn stop(mut self) -> Duration {
        let elapsed = self.start.elapsed();
        self.timer.record(elapsed);
        self.armed = false;
        elapsed
    }
}

#[cfg(feature = "timer")]
impl Drop for AsyncTimerGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.timer.record(self.start.elapsed());
        }
    }
}

#[cfg(feature = "timer")]
impl std::fmt::Debug for AsyncTimerGuard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncTimerGuard")
            .field("elapsed", &self.elapsed())
            .finish_non_exhaustive()
    }
}

/// Timing helpers for futures, implemented for `Timer` (the future borrows
/// the timer) and `Arc<Timer>` (the future owns a clone, so it can be
/// `'static` and spawned).
#[cfg(feature = "timer")]
pub trait AsyncTimerExt {
    /// The handle the returned future keeps (`&Timer` or `Arc<Timer>`).
    type Handle<'a>: std::ops::Deref<Target = Timer>
    where
        Self: 'a;

    /// Start a guard that records on drop.
    fn start_async(&self) -> AsyncTimerGuard<'_>;

    /// Wrap the future returned by `f`; its run time is recorded when it
    /// completes. A future dropped before completion records nothing.
    fn time_async<F, Fut>(&self, f: F) -> TimedFuture<Self::Handle<'_>, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future;
}

#[cfg(feature = "timer")]
impl AsyncTimerExt for Timer {
    type Handle<'a> = &'a Timer;

    fn start_async(&self) -> AsyncTimerGuard<'_> {
        AsyncTimerGuard::new(self)
    }

    fn time_async<F, Fut>(&self, f: F) -> TimedFuture<&Timer, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future,
    {
        TimedFuture::new(self, f())
    }
}

#[cfg(feature = "timer")]
impl AsyncTimerExt for Arc<Timer> {
    type Handle<'a> = Arc<Timer>;

    fn start_async(&self) -> AsyncTimerGuard<'_> {
        AsyncTimerGuard::new(self)
    }

    fn time_async<F, Fut>(&self, f: F) -> TimedFuture<Arc<Timer>, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future,
    {
        TimedFuture::new(Arc::clone(self), f())
    }
}

#[cfg(feature = "timer")]
pin_project_lite::pin_project! {
    /// Future returned by [`AsyncTimerExt::time_async`].
    ///
    /// The clock starts on the first poll and the elapsed time is recorded
    /// when the inner future completes.
    #[must_use = "futures do nothing unless awaited"]
    pub struct TimedFuture<T, F> {
        timer: T,
        start: Option<Instant>,
        #[pin]
        future: F,
    }
}

#[cfg(feature = "timer")]
impl<T, F> TimedFuture<T, F> {
    fn new(timer: T, future: F) -> Self {
        Self {
            timer,
            start: None,
            future,
        }
    }
}

#[cfg(feature = "timer")]
impl<T, F> Future for TimedFuture<T, F>
where
    T: std::ops::Deref<Target = Timer>,
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let start = *this.start.get_or_insert_with(Instant::now);
        match this.future.poll(cx) {
            Poll::Ready(output) => {
                this.timer.record(start.elapsed());
                Poll::Ready(output)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(feature = "timer")]
impl<T, F> std::fmt::Debug for TimedFuture<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimedFuture")
            .field("started", &self.start.is_some())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
enum MetricUpdate {
    #[cfg(feature = "count")]
    CounterAdd { name: Cow<'static, str>, value: u64 },
    #[cfg(feature = "gauge")]
    GaugeSet { name: Cow<'static, str>, value: f64 },
    #[cfg(feature = "timer")]
    TimerRecord { name: Cow<'static, str>, nanos: u64 },
    #[cfg(feature = "meter")]
    RateTick { name: Cow<'static, str>, count: u64 },
}

/// A list of metric updates applied together with
/// [`AsyncMetricBatch::flush`].
///
/// Each method is available when its metric feature is enabled.
#[derive(Debug, Clone, Default)]
pub struct AsyncMetricBatch {
    updates: Vec<MetricUpdate>,
}

impl AsyncMetricBatch {
    /// An empty batch.
    #[must_use]
    pub fn new() -> Self {
        Self {
            updates: Vec::with_capacity(64),
        }
    }

    /// Queue `counter(name).add(value)`.
    #[cfg(feature = "count")]
    pub fn counter_inc(&mut self, name: impl Into<Cow<'static, str>>, value: u64) {
        self.updates.push(MetricUpdate::CounterAdd {
            name: name.into(),
            value,
        });
    }

    /// Queue `gauge(name).set(value)`.
    #[cfg(feature = "gauge")]
    pub fn gauge_set(&mut self, name: impl Into<Cow<'static, str>>, value: f64) {
        self.updates.push(MetricUpdate::GaugeSet {
            name: name.into(),
            value,
        });
    }

    /// Queue `timer(name).record_ns(nanos)`.
    #[cfg(feature = "timer")]
    pub fn timer_record(&mut self, name: impl Into<Cow<'static, str>>, nanos: u64) {
        self.updates.push(MetricUpdate::TimerRecord {
            name: name.into(),
            nanos,
        });
    }

    /// Queue `rate(name).tick_n(count)`.
    #[cfg(feature = "meter")]
    pub fn rate_tick(&mut self, name: impl Into<Cow<'static, str>>, count: u64) {
        self.updates.push(MetricUpdate::RateTick {
            name: name.into(),
            count,
        });
    }

    /// Apply every queued update to `core`, in order, and empty the batch.
    #[allow(unused_variables)]
    pub fn flush(&mut self, core: &MetricsCore) {
        for update in self.updates.drain(..) {
            match update {
                #[cfg(feature = "count")]
                MetricUpdate::CounterAdd { name, value } => core.counter(&name).add(value),
                #[cfg(feature = "gauge")]
                MetricUpdate::GaugeSet { name, value } => core.gauge(&name).set(value),
                #[cfg(feature = "timer")]
                MetricUpdate::TimerRecord { name, nanos } => core.timer(&name).record_ns(nanos),
                #[cfg(feature = "meter")]
                MetricUpdate::RateTick { name, count } => core.rate(&name).tick_n(count),
            }
        }
    }

    /// `true` when nothing is queued.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.updates.is_empty()
    }

    /// Number of queued updates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.updates.len()
    }
}

/// Shared batch for many tasks, flushed into an explicit [`MetricsCore`].
///
/// [`AsyncMetricsBatcher::record`] flushes inline once the batch reaches
/// `max_batch_size`; [`AsyncMetricsBatcher::start_flusher`] adds periodic
/// flushing on a background task you can stop.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "count")]
/// # {
/// use metrics_lib::async_support::AsyncMetricsBatcher;
/// use metrics_lib::MetricsCore;
/// use std::sync::Arc;
/// use std::time::Duration;
///
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
/// # rt.block_on(async {
/// let core = Arc::new(MetricsCore::new());
/// let batcher = Arc::new(AsyncMetricsBatcher::new(Arc::clone(&core), Duration::from_millis(50), 100));
/// let flusher = Arc::clone(&batcher).start_flusher();
/// batcher.record(|b| b.counter_inc("jobs", 1)).await;
/// flusher.shutdown().await;
/// assert_eq!(core.counter("jobs").get(), 1);
/// # });
/// # }
/// ```
pub struct AsyncMetricsBatcher {
    core: Arc<MetricsCore>,
    batch: tokio::sync::Mutex<AsyncMetricBatch>,
    flush_interval: Duration,
    max_batch_size: usize,
}

impl AsyncMetricsBatcher {
    /// Batch into `core`, flushing every `flush_interval` (once the flusher
    /// runs) or when `max_batch_size` updates are queued (at least 1).
    #[must_use]
    pub fn new(core: Arc<MetricsCore>, flush_interval: Duration, max_batch_size: usize) -> Self {
        Self {
            core,
            batch: tokio::sync::Mutex::new(AsyncMetricBatch::new()),
            flush_interval,
            max_batch_size: max_batch_size.max(1),
        }
    }

    /// Queue updates with `update`; flushes inline when the batch is full.
    pub async fn record(&self, update: impl FnOnce(&mut AsyncMetricBatch)) {
        let mut batch = self.batch.lock().await;
        update(&mut batch);
        if batch.len() >= self.max_batch_size {
            batch.flush(&self.core);
        }
    }

    /// Apply everything queued now.
    pub async fn flush(&self) {
        self.batch.lock().await.flush(&self.core);
    }

    /// Start a task that flushes every `flush_interval`. Requires a Tokio
    /// runtime with the timer enabled.
    #[must_use = "dropping the handle detaches the flusher; call shutdown() to stop it"]
    pub fn start_flusher(self: Arc<Self>) -> FlusherHandle {
        let stop = Arc::new(tokio::sync::Notify::new());
        let signal = Arc::clone(&stop);
        let task = tokio::spawn(async move {
            loop {
                let stopped = tokio::time::timeout(self.flush_interval, signal.notified())
                    .await
                    .is_ok();
                self.flush().await;
                if stopped {
                    break;
                }
            }
        });
        FlusherHandle { stop, task }
    }
}

impl std::fmt::Debug for AsyncMetricsBatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncMetricsBatcher")
            .field("flush_interval", &self.flush_interval)
            .field("max_batch_size", &self.max_batch_size)
            .finish_non_exhaustive()
    }
}

/// Handle to the task started by [`AsyncMetricsBatcher::start_flusher`].
#[derive(Debug)]
pub struct FlusherHandle {
    stop: Arc<tokio::sync::Notify>,
    task: tokio::task::JoinHandle<()>,
}

impl FlusherHandle {
    /// Stop the flusher after one final flush, and wait for it.
    pub async fn shutdown(self) {
        self.stop.notify_one();
        // The task only ends by our signal; a join error means it panicked,
        // and there is nothing left to flush in that case.
        let _ = self.task.await;
    }
}

/// Copy Tokio's stable runtime metrics into gauges of `registry`:
/// `<prefix>tokio_workers`, `<prefix>tokio_alive_tasks` and
/// `<prefix>tokio_global_queue_depth`. Call it periodically (for example
/// right before rendering).
///
/// Requires the `gauge` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "gauge")]
/// # {
/// use metrics_lib::async_support::record_runtime_metrics;
/// use metrics_lib::Registry;
///
/// let registry = Registry::new();
/// let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(2).build().unwrap();
/// record_runtime_metrics(rt.handle(), &registry, "app_");
/// assert_eq!(registry.gauge("app_tokio_workers").get(), 2.0);
/// # }
/// ```
#[cfg(feature = "gauge")]
#[allow(clippy::cast_precision_loss)] // counts far below 2^53
pub fn record_runtime_metrics(handle: &tokio::runtime::Handle, registry: &Registry, prefix: &str) {
    let m = handle.metrics();
    registry
        .gauge(&format!("{prefix}tokio_workers"))
        .set(m.num_workers() as f64);
    registry
        .gauge(&format!("{prefix}tokio_alive_tasks"))
        .set(m.num_alive_tasks() as f64);
    registry
        .gauge(&format!("{prefix}tokio_global_queue_depth"))
        .set(m.global_queue_depth() as f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
    }

    #[cfg(feature = "timer")]
    #[test]
    fn test_async_timer_guard() {
        let timer = Timer::new();
        {
            let guard = timer.start_async();
            assert!(guard.elapsed() < Duration::from_secs(5));
            assert!(format!("{guard:?}").contains("AsyncTimerGuard"));
        }
        let _ = AsyncTimerGuard::new(&timer).stop();
        assert_eq!(timer.count(), 2);
    }

    #[cfg(feature = "timer")]
    #[test]
    fn test_timed_future_records_on_completion() {
        let rt = runtime();
        let timer = Arc::new(Timer::new());
        let value = rt.block_on(async {
            timer
                .time_async(|| async {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    42
                })
                .await
        });
        assert_eq!(value, 42);
        assert_eq!(timer.count(), 1);
        assert!(timer.average() >= Duration::from_millis(9));
        let _ = Arc::clone(&timer).start_async();
        assert_eq!(timer.count(), 2);
        let owned = Timer::new();
        let v = rt.block_on(owned.time_async(|| async { 7 }));
        assert_eq!(owned.count(), 1);
        assert_eq!(v, 7);
    }

    #[cfg(feature = "timer")]
    #[test]
    fn test_timed_future_is_spawnable_and_cancel_records_nothing() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_time()
            .build()
            .unwrap();
        let timer = Arc::new(Timer::new());
        let fut = timer.time_async(|| async { 1 });
        assert!(format!("{fut:?}").contains("TimedFuture"));
        assert_eq!(rt.block_on(rt.spawn(fut)).unwrap(), 1);
        assert_eq!(timer.count(), 1);
        let dropped = timer.time_async(std::future::pending::<()>);
        drop(dropped);
        assert_eq!(timer.count(), 1);
    }

    #[cfg(all(
        feature = "count",
        feature = "gauge",
        feature = "timer",
        feature = "meter"
    ))]
    #[test]
    fn test_metric_batch_flushes_every_kind() {
        let mut batch = AsyncMetricBatch::new();
        batch.counter_inc("c", 5);
        batch.gauge_set("g", 42.5);
        batch.timer_record("t", 1000);
        batch.rate_tick("r", 3);
        assert_eq!(batch.len(), 4);
        assert!(!batch.is_empty());
        let core = MetricsCore::new();
        batch.flush(&core);
        assert!(batch.is_empty());
        assert_eq!(core.counter("c").get(), 5);
        assert!((core.gauge("g").get() - 42.5).abs() < f64::EPSILON);
        assert_eq!(core.timer("t").count(), 1);
        assert_eq!(core.rate("r").total(), 3);
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_batcher_flushes_when_full_and_on_shutdown() {
        let rt = runtime();
        rt.block_on(async {
            let core = Arc::new(MetricsCore::new());
            let batcher = Arc::new(AsyncMetricsBatcher::new(
                Arc::clone(&core),
                Duration::from_secs(3600),
                2,
            ));
            assert!(format!("{batcher:?}").contains("AsyncMetricsBatcher"));
            batcher.record(|b| b.counter_inc("jobs", 1)).await;
            assert_eq!(core.counter("jobs").get(), 0);
            batcher.record(|b| b.counter_inc("jobs", 1)).await;
            assert_eq!(core.counter("jobs").get(), 2, "flushed when full");
            let flusher = Arc::clone(&batcher).start_flusher();
            batcher.record(|b| b.counter_inc("jobs", 1)).await;
            flusher.shutdown().await;
            assert_eq!(core.counter("jobs").get(), 3, "flushed on shutdown");
        });
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_batcher_flushes_on_interval() {
        let rt = runtime();
        rt.block_on(async {
            let core = Arc::new(MetricsCore::new());
            let batcher = Arc::new(AsyncMetricsBatcher::new(
                Arc::clone(&core),
                Duration::from_millis(20),
                1000,
            ));
            let flusher = Arc::clone(&batcher).start_flusher();
            batcher.record(|b| b.counter_inc("tick", 1)).await;
            let deadline = Instant::now() + Duration::from_secs(5);
            while core.counter("tick").get() == 0 && Instant::now() < deadline {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            assert_eq!(core.counter("tick").get(), 1);
            batcher.flush().await;
            flusher.shutdown().await;
        });
    }

    #[cfg(feature = "gauge")]
    #[test]
    fn test_runtime_metrics_are_recorded() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(3)
            .build()
            .unwrap();
        let registry = Registry::new();
        record_runtime_metrics(rt.handle(), &registry, "");
        assert!((registry.gauge("tokio_workers").get() - 3.0).abs() < f64::EPSILON);
        assert!(registry.gauge("tokio_alive_tasks").get() >= 0.0);
        assert!(registry.gauge("tokio_global_queue_depth").get() >= 0.0);
    }
}
