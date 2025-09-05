//! Axum Registry Integration Example
//!
//! Demonstrates integrating metrics-lib with an Axum web service.
//! Starts a server, exercises a route that updates metrics, then queries an export endpoint
//! and shuts down. This keeps the example self-contained and CI-friendly.
//!
//! Run with:
//!   cargo run --example axum_registry_integration --release

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::{routing::get, Json, Router};
use serde_json::json;
use std::time::Duration;
use tower::util::ServiceExt; // for `oneshot`

use metrics_lib::{init, metrics};

async fn health() -> &'static str {
    "OK"
}

async fn metrics_demo() -> &'static str {
    let m = metrics();
    m.counter("web.requests").inc();
    m.gauge("web.cpu").add(0.5);
    let t = m.timer("web.latency");
    let _g = t.start();
    // simulate tiny work
    tokio::time::sleep(Duration::from_millis(5)).await;
    m.rate("web.qps").tick();
    "demo-updated"
}

async fn export_metrics() -> Json<serde_json::Value> {
    let m = metrics();
    let c = m.counter("web.requests").get();
    let g = m.gauge("web.cpu").get();
    let q = m.rate("web.qps").rate();
    Json(json!({
        "web.requests": c,
        "web.cpu": g,
        "web.qps": q
    }))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/metrics-demo", get(metrics_demo))
        .route("/export", get(export_metrics));

    // Exercise routes in-process using tower::ServiceExt::oneshot
    // Health
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/health")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // Update metrics
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics-demo")
                .body(Body::empty())?,
        )
        .await?;
    assert!(res.status().is_success());

    // Export JSON snapshot
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/export")
                .body(Body::empty())?,
        )
        .await?;
    assert!(res.status().is_success());
    // For demonstration, print the body; convert Bytes to JSON (cap at 64KiB)
    let bytes = to_bytes(res.into_body(), 64 * 1024).await?;
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    println!("export snapshot: {v}");

    Ok(())
}
