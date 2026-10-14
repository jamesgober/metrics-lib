//! A `/metrics` endpoint with the dependency-free built-in server.
//!
//! Run with: `cargo run --example prometheus_endpoint --features http,histogram`
//! then `curl http://127.0.0.1:9898/metrics`. Set `SERVE_FOR_SECS` to keep it
//! running longer (default: 2 s, so the example exits by itself).

use metrics_lib::exporters::http::MetricsServer;
use metrics_lib::{LabelSet, MetricsCore, Unit};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core = Arc::new(MetricsCore::new());
    core.registry().describe_counter(
        "http_requests_total",
        "Requests handled by the demo",
        Unit::None,
    )?;
    let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
    core.counter_with("http_requests_total", &labels).add(7);
    core.histogram("http_request_duration_seconds")
        .observe(0.042);

    let server = MetricsServer::spawn("127.0.0.1:0", Arc::clone(&core))?;
    println!("serving on http://{}/metrics", server.local_addr());

    // Scrape it once, like Prometheus would.
    let mut stream = TcpStream::connect(server.local_addr())?;
    stream.write_all(b"GET /metrics HTTP/1.1\r\nHost: localhost\r\n\r\n")?;
    let mut response = String::new();
    let _ = stream.read_to_string(&mut response)?;
    for line in response.lines().take(12) {
        println!("{line}");
    }

    let secs = std::env::var("SERVE_FOR_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    std::thread::sleep(Duration::from_secs(secs));
    server.shutdown();
    Ok(())
}
