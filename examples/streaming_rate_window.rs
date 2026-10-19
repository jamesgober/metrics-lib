//! Watch a rate meter's sliding windows while the event rate changes.
//!
//! Run with: `cargo run --example streaming_rate_window --features meter`

use metrics_lib::RateMeter;
use std::time::{Duration, Instant};

fn main() {
    // Primary window of 500 ms: `rate()` reports events/s over it.
    let meter = RateMeter::with_window(Duration::from_millis(500));
    let start = Instant::now();
    for phase in [200u64, 1000, 50] {
        let phase_start = Instant::now();
        while phase_start.elapsed() < Duration::from_millis(600) {
            meter.tick_n(phase / 50);
            std::thread::sleep(Duration::from_millis(20));
        }
        let stats = meter.stats();
        println!(
            "t={:>4}ms target~{phase:>4}/s  rate={:>7.1}/s  last_second={:>6.0}  total={}",
            start.elapsed().as_millis(),
            stats.rate,
            stats.per_second,
            stats.total_events
        );
    }
}
