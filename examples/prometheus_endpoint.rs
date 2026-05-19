//! Minimal Axum `/metrics` endpoint backed by the built-in Prometheus
//! exporter. The handler is allocation-light (one render per request) and
//! returns the exposition body with the canonical `Content-Type`.
//!
//! Run with:
//!   cargo run --example prometheus_endpoint --features async --release
//! Then:
//!   curl http://127.0.0.1:9898/metrics

use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use metrics_lib::{init, metrics, LabelSet, Unit};
use std::time::Duration;

#[tokio::main]
async fn main() {
    init();
    let m = metrics();

    // Describe metrics so the exporter emits `# HELP` / `# TYPE` / `# UNIT`.
    m.registry().describe_counter(
        "http_requests_total",
        "Total HTTP requests handled by the demo server",
        Unit::Custom("1"),
    );
    m.registry().describe_histogram(
        "http_request_duration_seconds",
        "Per-request handler latency",
        Unit::Seconds,
    );

    // Spawn a background task that pretends to handle traffic, so the
    // endpoint has data to expose immediately.
    tokio::spawn(async {
        loop {
            let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
            metrics().counter_with("http_requests_total", &labels).inc();
            let h = metrics().histogram_with("http_request_duration_seconds", &labels);
            h.observe(0.005 + 0.01 * (rand_f64()));
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/healthz", get(|| async { "OK" }));

    let addr = "127.0.0.1:9898";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Serving Prometheus metrics on http://{addr}/metrics");
    axum::serve(listener, app).await.unwrap();
}

async fn metrics_handler() -> impl IntoResponse {
    let body = metrics_lib::exporters::prometheus::render(metrics().registry());
    (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        body,
    )
}

fn rand_f64() -> f64 {
    use std::cell::Cell;
    thread_local! { static S: Cell<u64> = const { Cell::new(0xABCDEF0123456789) }; }
    S.with(|s| {
        let mut x = s.get();
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        s.set(x);
        (x >> 32) as f64 / (u32::MAX as f64)
    })
}
