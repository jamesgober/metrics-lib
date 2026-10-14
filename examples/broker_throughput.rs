//! Measure producer and consumer throughput of a bounded channel.
//!
//! Run with: `cargo run --example broker_throughput --features meter --release`

use metrics_lib::MetricsCore;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let core = MetricsCore::new();
    let produced = core.rate("broker_produced");
    let consumed = core.rate("broker_consumed");
    let run_for = Duration::from_millis(500);

    let (sender, receiver) = mpsc::sync_channel::<u64>(10_000);
    let producer_thread = {
        let produced = std::sync::Arc::clone(&produced);
        thread::spawn(move || {
            let start = Instant::now();
            let mut n = 0u64;
            while start.elapsed() < run_for {
                if sender.send(n).is_err() {
                    break;
                }
                produced.tick();
                n += 1;
            }
        })
    };
    let consumer_thread = {
        let consumed = std::sync::Arc::clone(&consumed);
        thread::spawn(move || {
            while receiver.recv_timeout(Duration::from_millis(50)).is_ok() {
                consumed.tick();
            }
        })
    };
    producer_thread.join().expect("producer panicked");
    consumer_thread.join().expect("consumer panicked");

    println!(
        "produced={} ({:.0}/s now) consumed={} ({:.0}/s now)",
        produced.total(),
        produced.rate(),
        consumed.total(),
        consumed.rate()
    );
}
