//! The basics: counters, gauges, timers and a Prometheus exposition.
//!
//! Run with: `cargo run --example quick_start`

use metrics_lib::exporters::prometheus;
use metrics_lib::{LabelSet, metrics};
use std::time::Duration;

fn main() {
    // The global instance initialises itself on first use.
    let m = metrics();

    let requests = m.counter("requests");
    requests.inc();
    requests.add(41);
    m.counter_with("requests", &LabelSet::from([("status", "500")]))
        .inc();

    m.gauge("queue_depth").set(7.0);

    {
        // `start_owned` keeps the timer alive for the guard's lifetime.
        let _guard = m.timer("db_query_seconds").start_owned();
        std::thread::sleep(Duration::from_millis(2));
    }
    let rows = m.time("load_rows_seconds", || 3);

    println!("requests={} rows={rows}", requests.get());
    print!("{}", prometheus::render(m.registry()));
}
