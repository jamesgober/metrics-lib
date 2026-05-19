//! Demonstrates rendering an OTLP/HTTP+JSON payload for the current
//! registry. The payload is suitable for POSTing to a collector endpoint
//! such as `http://localhost:4318/v1/metrics`.
//!
//! Run with:
//!   cargo run --example otlp_push --features otlp --release
//!
//! The example prints the JSON body to stdout; integrating an HTTP client
//! (`reqwest` / `ureq` / `hyper`) is left to the consumer to keep this
//! crate dependency-light.

use metrics_lib::exporters::otlp;
use metrics_lib::{init, metrics, LabelSet, Unit};

fn main() {
    init();
    let m = metrics();

    m.registry()
        .describe_counter("requests_total", "Total HTTP requests", Unit::Custom("1"));
    m.registry()
        .describe_gauge("inflight_requests", "Requests in flight", Unit::Custom("1"));
    m.registry()
        .describe_histogram("request_duration_seconds", "Handler latency", Unit::Seconds);

    m.counter("requests_total").add(7);
    m.counter_with(
        "requests_total",
        &LabelSet::from([("method", "GET"), ("status", "200")]),
    )
    .add(15);
    m.gauge("inflight_requests").set(3.0);

    m.registry()
        .configure_histogram("request_duration_seconds", [0.01, 0.05, 0.1, 0.5, 1.0]);
    let h = m.histogram_with(
        "request_duration_seconds",
        &LabelSet::from([("route", "/search")]),
    );
    for v in &[0.005, 0.03, 0.07, 0.2, 0.45, 0.6, 2.0] {
        h.observe(*v);
    }

    let body = otlp::render_pretty(m.registry(), "demo-service");
    println!("===== OTLP/HTTP+JSON payload =====");
    println!("{body}");
    println!();
    println!("Send via:");
    println!("  curl -X POST -H 'Content-Type: application/json' \\");
    println!("       --data-binary @payload.json \\");
    println!("       http://localhost:4318/v1/metrics");
}
