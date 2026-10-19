//! Build OTLP payloads (JSON and protobuf) for a collector.
//!
//! The exporter is transport-agnostic: POST the body to the collector's
//! `/v1/metrics` with the matching content type. This example only prints
//! the payloads.
//!
//! Run with: `cargo run --example otlp_push --features otlp,histogram`

use metrics_lib::exporters::otlp::{
    JSON_CONTENT_TYPE, OtlpExporter, PROTOBUF_CONTENT_TYPE, Temporality,
};
use metrics_lib::{LabelSet, MetricsCore, Unit};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core = MetricsCore::new();
    core.registry()
        .describe_counter("orders", "Orders placed", Unit::None)?;
    core.counter_with("orders", &LabelSet::from([("region", "eu")]))
        .add(17);
    core.gauge("queue_depth").set(4.0);
    core.timer("checkout").record(Duration::from_millis(37));
    core.histogram("payment_seconds").observe(0.12);

    // Keep one exporter per destination: in delta mode it remembers what it
    // sent last time.
    let exporter = OtlpExporter::new("checkout-service")
        .with_resource_attribute("deployment.environment", "staging")
        .with_temporality(Temporality::Delta);

    let json = exporter.render_json(core.registry())?;
    println!(
        "POST /v1/metrics ({JSON_CONTENT_TYPE}), {} bytes",
        json.len()
    );
    println!("{json}");

    core.counter_with("orders", &LabelSet::from([("region", "eu")]))
        .add(3);
    let protobuf = exporter.render_protobuf(core.registry());
    println!(
        "POST /v1/metrics ({PROTOBUF_CONTENT_TYPE}), {} bytes (orders delta = 3)",
        protobuf.len()
    );
    Ok(())
}
