//! Push the registry to a StatsD / DogStatsD agent over UDP.
//!
//! Run with: `cargo run --example statsd_push --features statsd`
//! The target defaults to `127.0.0.1:8125`; override with `STATSD_ADDR`. UDP
//! needs no listener, so the example works without an agent.

use metrics_lib::exporters::statsd::{CounterMode, StatsdSink};
use metrics_lib::{LabelSet, MetricsCore};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let core = MetricsCore::new();
    let requests = core.counter_with("requests", &LabelSet::from([("status", "200")]));
    requests.add(36);
    core.gauge("cpu_used").set(73.2);
    core.gauge("temperature_delta").set(-4.0);
    core.timer("rpc").record(Duration::from_micros(850));

    let addr = std::env::var("STATSD_ADDR").unwrap_or_else(|_| "127.0.0.1:8125".to_owned());
    let sink = StatsdSink::new(addr.as_str())?
        .with_prefix("myapp.")
        .with_counter_mode(CounterMode::Delta);

    println!("--- first push ---\n{}", sink.render(core.registry()));
    requests.add(4);
    println!("--- second push: counters send only the increase ---");
    print!("{}", sink.render(core.registry()));
    let bytes = sink.send(core.registry())?;
    println!("sent {bytes} bytes to {addr}");
    Ok(())
}
