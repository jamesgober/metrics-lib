//! Host and process CPU usage from `SystemHealth`.
//!
//! CPU percentages are deltas between samples, so they read 0 until the
//! second sample.
//!
//! Run with: `cargo run --example cpu_stats`

use metrics_lib::SystemHealth;
use std::time::{Duration, Instant};

fn main() {
    let health = SystemHealth::with_interval(Duration::from_millis(200));

    // Burn some CPU on this thread so process usage moves.
    let start = Instant::now();
    let mut x = 1u64;
    while start.elapsed() < Duration::from_millis(600) {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    }
    std::hint::black_box(x);

    let snap = health.snapshot();
    println!("host CPU:     {:5.1}%", snap.system_cpu_percent);
    println!(
        "process CPU:  {:5.1}% of all cores",
        snap.process_cpu_percent
    );
    match snap.load_average {
        Some(load) => println!("load (1 min): {load:.2}"),
        None => println!("load (1 min): not available on this platform"),
    }
    println!(
        "CPU seconds:  {:.3}",
        health.process_metrics().cpu_seconds_total
    );
}
