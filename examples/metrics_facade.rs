//! Record through the `metrics` facade into a metrics-lib registry.
//!
//! Libraries instrumented with the `metrics` crate (`metrics::counter!`, …)
//! report into whatever recorder the application installs. Installing the
//! metrics-lib bridge routes them into a `MetricsCore`, next to your own
//! metrics, and out through any metrics-lib exporter.
//!
//! Run with: `cargo run --example metrics_facade --features metrics-rs`

use metrics_lib::MetricsCore;
use metrics_lib::exporters::prometheus;
use metrics_lib::metrics_rs::MetricsLibRecorder;
use std::sync::Arc;

fn main() {
    let core = Arc::new(MetricsCore::new());
    let recorder = MetricsLibRecorder::new(Arc::clone(&core));

    // `with_local_recorder` scopes the recorder to this closure; a real
    // application calls `metrics_lib::metrics_rs::install` once at start-up.
    metrics::with_local_recorder(&recorder, || {
        metrics::describe_counter!("jobs_processed", "Jobs taken off the queue");
        metrics::counter!("jobs_processed", "queue" => "email").increment(3);
        metrics::gauge!("queue_depth", "queue" => "email").set(12.0);
        metrics::histogram!("job_duration_seconds").record(0.042);
    });

    // Native metrics-lib metrics live in the same registry.
    core.counter("native_counter").inc();

    print!("{}", prometheus::render(core.registry()));
}
