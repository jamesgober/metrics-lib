//! Demonstrates the `Histogram` type for latency distributions + percentile
//! readouts.
//!
//! Run with:
//!   cargo run --example histogram_latency --features histogram --release

use metrics_lib::{init, metrics, LabelSet, Unit};
use std::time::Duration;

fn main() {
    init();
    let m = metrics();

    // Pre-configure latency buckets (Prometheus default range). The standard
    // `DEFAULT_SECONDS_BUCKETS` works for most HTTP / RPC use cases.
    m.registry().configure_histogram(
        "request_duration_seconds",
        [
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ],
    );
    m.registry().describe_histogram(
        "request_duration_seconds",
        "HTTP request handler latency",
        Unit::Seconds,
    );

    // Simulate two endpoints with different latency profiles.
    let fast = LabelSet::from([("route", "/healthz")]);
    let slow = LabelSet::from([("route", "/search")]);

    for _ in 0..1000 {
        // /healthz: tight around 5–10 ms.
        let v = 0.005 + (fastrand() * 0.005);
        m.histogram_with("request_duration_seconds", &fast)
            .observe(v);
    }
    for _ in 0..200 {
        // /search: spread across 100ms–2s.
        let v = 0.1 + (fastrand() * 1.9);
        m.histogram_with("request_duration_seconds", &slow)
            .observe(v);
    }

    // Direct percentile readouts.
    let h_fast = m.histogram_with("request_duration_seconds", &fast);
    let h_slow = m.histogram_with("request_duration_seconds", &slow);
    println!("Endpoint           p50         p95         p99         count");
    println!(
        "/healthz   {:>10}  {:>10}  {:>10}  {:>10}",
        fmt_dur(h_fast.quantile(0.50)),
        fmt_dur(h_fast.quantile(0.95)),
        fmt_dur(h_fast.quantile(0.99)),
        h_fast.count()
    );
    println!(
        "/search    {:>10}  {:>10}  {:>10}  {:>10}",
        fmt_dur(h_slow.quantile(0.50)),
        fmt_dur(h_slow.quantile(0.95)),
        fmt_dur(h_slow.quantile(0.99)),
        h_slow.count()
    );

    println!();
    println!("===== Prometheus exposition =====");
    println!(
        "{}",
        metrics_lib::exporters::prometheus::render(m.registry())
    );
}

fn fmt_dur(seconds: f64) -> String {
    if seconds < 0.001 {
        format!("{:.0}μs", seconds * 1_000_000.0)
    } else if seconds < 1.0 {
        format!("{:.1}ms", seconds * 1_000.0)
    } else {
        format!("{:.2}s", seconds)
    }
}

/// Tiny LCG for deterministic example output. Returns a value in `[0.0, 1.0)`.
fn fastrand() -> f64 {
    use std::cell::Cell;
    thread_local! {
        static STATE: Cell<u64> = const { Cell::new(0x123456789ABCDEF0) };
    }
    STATE.with(|s| {
        let mut x = s.get();
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        s.set(x);
        (x >> 32) as f64 / (u32::MAX as f64)
    })
}

// Silence "unused" if no Duration use is needed.
#[allow(dead_code)]
fn _silence(_: Duration) {}
