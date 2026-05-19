//! Demonstrates pushing the current registry state to a StatsD agent via UDP.
//!
//! Run with:
//!   cargo run --example statsd_push --features statsd --release
//!
//! By default this points at `127.0.0.1:8125` (the canonical StatsD/DogStatsD
//! UDP port). Override with `STATSD_ADDR=host:port`. The example *renders*
//! the body and prints it to stdout even when no agent is listening, so you
//! can verify the wire format without setting up an agent.

use metrics_lib::exporters::statsd::StatsdSink;
use metrics_lib::{init, metrics, LabelSet};
use std::env;

fn main() {
    init();
    let m = metrics();

    m.counter("requests").add(42);
    m.counter_with(
        "requests",
        &LabelSet::from([("status", "200"), ("method", "GET")]),
    )
    .add(36);
    m.gauge("cpu_used").set(73.2);
    m.gauge_with("cpu_used", &LabelSet::from([("core", "0")]))
        .set(81.0);
    {
        let t = m.timer("rpc_latency");
        let _g = t.start();
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    let addr = env::var("STATSD_ADDR").unwrap_or_else(|_| "127.0.0.1:8125".to_string());
    let sink = StatsdSink::new(&*addr)
        .expect("bind UDP")
        .with_prefix("demo.");

    // Render so we can show the wire format regardless of whether an agent
    // is actually listening at `addr`.
    let body = sink.render(m.registry());
    println!("===== StatsD wire format (would be sent to {addr}) =====");
    println!("{body}");

    match sink.send(m.registry()) {
        Ok(n) => println!("Pushed {n} bytes to {addr}"),
        Err(e) => eprintln!("StatsD push to {addr} failed: {e}"),
    }
}
