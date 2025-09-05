//! Quick Start example for metrics-lib
//!
//! Run with:
//!   cargo run --example quick_start --release

use metrics_lib::{init, metrics};
use std::time::Duration;

fn main() {
    // Initialize global registry once
    init();

    // Counter usage
    let c = metrics().counter("qs.requests");
    c.inc();
    c.add(41);

    // Gauge usage
    let g = metrics().gauge("qs.cpu");
    g.set(42.5);
    g.add(7.5);

    // Timer usage (RAII)
    let t = metrics().timer("qs.db");
    {
        let _guard = t.start();
        // simulate some work
        std::thread::sleep(Duration::from_millis(1));
    }

    // RateMeter usage
    let r = metrics().rate("qs.qps");
    r.tick_n(5);

    // Read back values (sanity)
    println!(
        "quick_start: counter={}, gauge={:.2}, rate={:.2}",
        c.get(),
        g.get(),
        r.rate()
    );
}
