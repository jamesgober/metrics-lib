//! Serve `/metrics` from an existing axum application.
//!
//! The route renders the core with `prometheus::render_core`, which includes
//! the `process_*` families when the `system` feature is on. The example
//! drives the router in-process, so it exits on its own.
//!
//! Run with: `cargo run --example axum_registry_integration`

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use axum::routing::get;
use axum::{Router, extract::State};
use metrics_lib::exporters::prometheus;
use metrics_lib::{LabelSet, MetricsCore, Unit};
use std::sync::Arc;
use tower::ServiceExt;

async fn work(State(core): State<Arc<MetricsCore>>) -> &'static str {
    let _timer = core.timer("work_seconds").start_owned();
    core.counter_with("work_total", &LabelSet::from([("kind", "demo")]))
        .inc();
    core.gauge("last_work_size").set(42.0);
    core.histogram("work_payload_bytes").observe(512.0);
    "done"
}

async fn metrics(State(core): State<Arc<MetricsCore>>) -> impl axum::response::IntoResponse {
    (
        [(header::CONTENT_TYPE, prometheus::CONTENT_TYPE)],
        prometheus::render_core(&core),
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core = Arc::new(MetricsCore::new());
    core.registry()
        .describe_counter("work_total", "Units of work done", Unit::None)?;
    core.registry()
        .configure_histogram("work_payload_bytes", [128.0, 1024.0, 8192.0])?;

    let app = Router::new()
        .route("/work", get(work))
        .route("/metrics", get(metrics))
        .with_state(Arc::clone(&core));

    for _ in 0..3 {
        let res = app
            .clone()
            .oneshot(Request::get("/work").body(Body::empty())?)
            .await?;
        assert_eq!(res.status(), StatusCode::OK);
    }

    let res = app
        .oneshot(Request::get("/metrics").body(Body::empty())?)
        .await?;
    let body = to_bytes(res.into_body(), 1 << 20).await?;
    let text = String::from_utf8(body.to_vec())?;
    for line in text.lines().filter(|l| l.starts_with("work")) {
        println!("{line}");
    }
    Ok(())
}
