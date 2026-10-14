//! Time async work and batch metric updates from many tasks.
//!
//! Run with: `cargo run --example async_batch_timing --features async`

use metrics_lib::MetricsCore;
use metrics_lib::async_support::{AsyncMetricBatch, AsyncMetricsBatcher, AsyncTimerExt};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let core = Arc::new(MetricsCore::new());

    // Time a future; `Arc<Timer>` makes the timed future `'static`.
    let timer = core.timer("fetch_seconds");
    let body = timer
        .time_async(|| async {
            tokio::time::sleep(Duration::from_millis(25)).await;
            "payload"
        })
        .await;
    println!("fetched {body:?} in {:?}", timer.average());

    // A one-off batch applied in one go.
    let mut batch = AsyncMetricBatch::new();
    batch.counter_inc("jobs", 3);
    batch.timer_record("serialize_seconds", 123_000);
    batch.flush(&core);

    // A shared batcher for many tasks, flushed every 50 ms in the background.
    let batcher = Arc::new(AsyncMetricsBatcher::new(
        Arc::clone(&core),
        Duration::from_millis(50),
        256,
    ));
    let flusher = Arc::clone(&batcher).start_flusher();
    let tasks: Vec<_> = (0..10)
        .map(|_| {
            let batcher = Arc::clone(&batcher);
            tokio::spawn(async move { batcher.record(|b| b.counter_inc("jobs", 1)).await })
        })
        .collect();
    for task in tasks {
        task.await.expect("task panicked");
    }
    flusher.shutdown().await;

    println!(
        "jobs={} serialize_count={}",
        core.counter("jobs").get(),
        core.timer("serialize_seconds").count()
    );
}
