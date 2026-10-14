//! Capture every series as one JSON document.
//!
//! Run with: `cargo run --example snapshot_serde --features serde`

use metrics_lib::exporters::json;
use metrics_lib::{LabelSet, MetricsCore};

fn main() -> Result<(), serde_json::Error> {
    let core = MetricsCore::new();
    core.counter("hits").add(5);
    core.counter_with("hits", &LabelSet::from([("status", "404")]))
        .inc();
    core.gauge("temperature").set(21.5);
    core.gauge("unknown").set(f64::NAN); // written as "NaN"

    let snapshot = json::snapshot(core.registry());
    println!(
        "{} series, schema v{}",
        snapshot.series.len(),
        snapshot.schema_version
    );
    println!("{}", json::render_pretty(core.registry())?);
    Ok(())
}
