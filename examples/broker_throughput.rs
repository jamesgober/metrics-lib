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

    let (tx, rx) = mpsc::sync_channel::<u64>(10_000);
    let producer = {
        let produced = std::sync::Arc::clone(&produced);
        thread::spawn(move || {
            let start = Instant::now();
            let mut n = 0u64;
            while start.elapsed() < run_for {
                if tx.send(n).is_err() {
                    break;
                }
                produced.tick();
                n += 1;
            }
        })
    };
    let consumer = {
        let consumed = std::sync::Arc::clone(&consumed);
        thread::spawn(move || {
            while rx.recv_timeout(Duration::from_millis(50)).is_ok() {
                consumed.tick();
            }
        })
    };
    producer.join().expect("producer panicked");
    consumer.join().expect("consumer panicked");

    println!(
        "produced={} ({:.0}/s now) consumed={} ({:.0}/s now)",
        produced.total(),
        produced.rate(),
        consumed.total(),
        consumed.rate()
    );
}
