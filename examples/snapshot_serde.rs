//! Captures the registry as a single JSON snapshot using `exporters::json`.
//!
//! Run with:
//!   cargo run --example snapshot_serde --features serde --release

use metrics_lib::exporters::json;
use metrics_lib::{init, metrics, LabelSet, Unit};

fn main() {
    init();
    let m = metrics();

    m.registry().describe_counter(
        "jobs_total",
        "Background job invocations",
        Unit::Custom("1"),
    );
    m.registry()
        .describe_gauge("queue_depth", "Pending background work", Unit::Custom("1"));

    m.counter("jobs_total").add(120);
    m.counter_with("jobs_total", &LabelSet::from([("kind", "email")]))
        .add(34);
    m.counter_with("jobs_total", &LabelSet::from([("kind", "report")]))
        .add(86);
    m.gauge("queue_depth").set(5.0);

    // Render pretty so the output is easy to scan in the terminal.
    let body = json::render_pretty(m.registry());
    println!("{body}");
}
