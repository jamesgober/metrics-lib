//! Demonstrates labeled metrics with the `LabelSet` API + cardinality control.
//!
//! Run with:
//!   cargo run --example labels_demo --release

use metrics_lib::{init, metrics, LabelSet, Unit};

fn main() {
    init();
    let m = metrics();

    // Describe the metric once — exporters pick up the help text + unit.
    m.registry().describe_counter(
        "http_requests",
        "Total HTTP requests handled",
        Unit::Custom("1"),
    );

    // Build label sets fluently. Repeated lookups of the same `(name, labels)`
    // return the same `Arc<Counter>`, so cache them in hot paths.
    let get_200 = LabelSet::from([("method", "GET"), ("status", "200")]);
    let post_500 = LabelSet::from([("method", "POST"), ("status", "500")]);

    for _ in 0..10 {
        m.counter_with("http_requests", &get_200).inc();
    }
    for _ in 0..3 {
        m.counter_with("http_requests", &post_500).inc();
    }

    // Per-tenant runtime-derived labels (no Box::leak required).
    for tenant in ["acme", "globex", "initech"] {
        let labels = LabelSet::from([("tenant", tenant.to_string())]);
        m.counter_with("auth_failures", &labels).add(2);
    }

    // Cardinality control: configure a tight cap and watch overflows.
    m.registry().set_cardinality_cap(8);
    for i in 0..100 {
        let labels = LabelSet::from([("k", format!("v{i}"))]);
        m.counter_with("overflow_demo", &labels).inc();
    }

    println!("metric_count       = {}", m.registry().metric_count());
    println!("cardinality_count  = {}", m.registry().cardinality_count());
    println!("cardinality_cap    = {}", m.registry().cardinality_cap());
    println!(
        "cardinality_overflows = {}",
        m.registry().cardinality_overflows()
    );
    println!();
    println!("===== Prometheus snapshot =====");
    println!(
        "{}",
        metrics_lib::exporters::prometheus::render(m.registry())
    );
}
