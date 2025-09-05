use metrics_lib::{metrics, AsyncMetricBatch, AsyncTimerExt};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    metrics_lib::init();

    let m = metrics();

    // Zero-cost async timing of an operation returning &str
    let result: &str = m
        .timer("async_op")
        .time_async(|| async {
            tokio::time::sleep(Duration::from_millis(25)).await;
            "ok"
        })
        .await;

    println!("async_op result={result}");

    // Batch a few updates and flush once
    let mut batch = AsyncMetricBatch::new();
    batch.counter_inc("jobs", 3);
    batch.gauge_set("queue_depth", 7.0);
    batch.timer_record("serialize_ns", 123_000);
    batch.rate_tick("ingest_rps");
    batch.flush(metrics());

    println!(
        "after batch: jobs={}, depth={:.1}, timers.count={}",
        m.counter("jobs").get(),
        m.gauge("queue_depth").get(),
        m.timer("serialize_ns").count()
    );
}
