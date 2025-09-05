//! Axum Registry Integration Example
//!
//! Demonstrates integrating metrics-lib with an Axum web service.
//! Starts a server, exercises a route that updates metrics, then queries an export endpoint
//! and shuts down. This keeps the example self-contained and CI-friendly.
//!
//! Run with:
//!   cargo run --example axum_registry_integration --release

use axum::{routing::get, Json, Router};
use serde_json::json;
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;

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

    // Bind to an ephemeral port
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr: SocketAddr = listener.local_addr()?;

    // Serve in background
    let handle = tokio::spawn(async move {
        // Run the server until aborted by the example
        let _ = axum::serve(listener, app).await;
    });

    // Exercise routes with reqwest then stop
    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    // Health
    let r = client.get(format!("{}/health", base)).send().await?;
    assert!(r.status().is_success());

    // Update metrics
    let r = client.get(format!("{}/metrics-demo", base)).send().await?;
    assert!(r.status().is_success());

    // Export JSON snapshot
    let r = client.get(format!("{}/export", base)).send().await?;
    let v: serde_json::Value = r.json().await?;
    println!("export snapshot: {}", v);

    // Done — cancel server task
    handle.abort();
    Ok(())
}
