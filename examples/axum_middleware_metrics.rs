use axum::{routing::get, Router};
use metrics_lib::{init, metrics};
use std::time::Duration;

#[tokio::main]
async fn main() {
    init();

    // Simple middleware-like handler wrappers using closures
    async fn handle_index() -> &'static str {
        // Per-request metrics
        let m = metrics();
        m.counter("http_requests_total").inc();
        m.gauge("inflight_requests").add(1.0);
        {
            let t = m.timer("handler_latency");
            let _guard = t.start();
            // Simulate work
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        m.gauge("inflight_requests").add(-1.0);
        "ok"
    }

    async fn handle_metrics() -> String {
        // Minimal text snapshot (not full OpenMetrics; see custom_exporter_openmetrics.rs)
        let m = metrics();
        let mut out = String::new();
        use std::fmt::Write;
        let _ = writeln!(
            out,
            "http_requests_total {}",
            m.counter("http_requests_total").get()
        );
        let _ = writeln!(
            out,
            "inflight_requests {}",
            m.gauge("inflight_requests").get()
        );
        let t = m.timer("handler_latency");
        let _ = writeln!(out, "handler_latency_count {}", t.count());
        let _ = writeln!(out, "handler_latency_avg_ns {}", t.average().as_nanos());
        out
    }

    let app = Router::new()
        .route("/", get(handle_index))
        .route("/metrics-lite", get(handle_metrics));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Serving on http://{}  →  GET /, GET /metrics-lite", addr);
    axum::serve(listener, app).await.unwrap();
}
