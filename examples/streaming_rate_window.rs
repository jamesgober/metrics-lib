//! Streaming Rate Window Demo
//!
//! Demonstrates a simple streaming workload updating a `RateMeter` and
//! printing the observed rate periodically over a short window.
//!
//! Run with:
//!   cargo run --example streaming_rate_window --release

use metrics_lib::{init, metrics};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    init();

    let r = metrics().rate("demo.stream.rps");
    let mut rng = StdRng::seed_from_u64(42);

    // Producer task: generate ticks with light burstiness
    let producer = tokio::spawn(async move {
        let mut t = tokio::time::interval(Duration::from_millis(10));
        for _ in 0..400u32 { // ~4 seconds
            t.tick().await;
            let n = 1 + (rng.gen::<u8>() % 5) as u64; // 1..=5 ticks
            r.tick_n(n);
        }
    });

    // Reporter: sample and print current rate
    let reporter = tokio::spawn(async move {
        let mut t = tokio::time::interval(Duration::from_millis(250));
        for _ in 0..16 { // ~4 seconds
            t.tick().await;
            let rate = metrics().rate("demo.stream.rps").rate();
            println!("stream rate ~{:.2} events/sec", rate);
        }
    });

    let _ = tokio::join!(producer, reporter);
}
