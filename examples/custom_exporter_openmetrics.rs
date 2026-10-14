//! Write a custom exporter on top of the registry's entry accessors.
//!
//! The built-in exporters cover Prometheus, OpenMetrics, StatsD, OTLP and
//! JSON; this shows the public surface for anything else (here: a CSV
//! dump). Names and labels in the registry are already validated.
//!
//! Run with: `cargo run --example custom_exporter_openmetrics`

use metrics_lib::{LabelSet, MetricsCore, Registry};
use std::fmt::Write as _;

fn labels(l: &LabelSet) -> String {
    l.iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn to_csv(registry: &Registry) -> String {
    let mut out = String::from("kind,name,labels,value\n");
    for (name, l, c) in registry.counter_entries() {
        let _ = writeln!(out, "counter,{name},{},{}", labels(&l), c.get());
    }
    for (name, l, g) in registry.gauge_entries() {
        let _ = writeln!(out, "gauge,{name},{},{}", labels(&l), g.get());
    }
    out
}

fn main() {
    let core = MetricsCore::new();
    core.counter("jobs").add(12);
    core.counter_with("jobs", &LabelSet::from([("queue", "email")]))
        .add(3);
    core.gauge("queue_depth").set(3.0);
    print!("{}", to_csv(core.registry()));

    // The built-in OpenMetrics exporter, for comparison.
    print!(
        "{}",
        metrics_lib::exporters::openmetrics::render(core.registry())
    );
}
