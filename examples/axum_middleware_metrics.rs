//! RED metrics (rate, errors, duration) for an axum app with `RedLayer`.
//!
//! The route label comes from axum's `MatchedPath`, a template such as
//! `/users/{id}`, never the raw path. The example drives the router
//! in-process and prints the resulting exposition.
//!
//! Run with: `cargo run --example axum_middleware_metrics --features tower`

use axum::Router;
use axum::body::Body;
use axum::extract::MatchedPath;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use metrics_lib::MetricsCore;
use metrics_lib::exporters::prometheus;
use metrics_lib::tower::RedLayer;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core = Arc::new(MetricsCore::new());
    let red = RedLayer::new(Arc::clone(&core))
        .with_route_label(|_, ext| ext.get::<MatchedPath>().map(|p| p.as_str().to_owned()));

    let app = Router::new()
        .route("/users/{id}", get(|| async { "user" }))
        .route("/fail", get(|| async { StatusCode::INTERNAL_SERVER_ERROR }))
        .route_layer(red);

    for path in ["/users/1", "/users/2", "/fail", "/missing"] {
        let response = app
            .clone()
            .oneshot(Request::get(path).body(Body::empty())?)
            .await?;
        println!("GET {path} -> {}", response.status());
    }

    for line in prometheus::render(core.registry())
        .lines()
        .filter(|l| l.starts_with("http_requests"))
    {
        println!("{line}");
    }
    Ok(())
}
