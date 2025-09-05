#![allow(dead_code)]
// Chaos Testing Suite: concurrent access and system pressure
// Gated to avoid CI flakiness and long runtimes by default.
// Run with: cargo test --features bench-tests -- --ignored --test chaos_tests
#![cfg(all(test, feature = "bench-tests", not(tarpaulin)))]

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use metrics_lib::{init, metrics};

#[ignore]
#[test]
fn chaos_concurrent_counter_and_gauge_updates() {
    init();
    let m = metrics();
    let counter = m.counter("chaos.counter");
    let gauge = m.gauge("chaos.gauge");

    let threads = 64usize;
    let iters = 250_000usize; // ~16M ops total

    let mut handles = Vec::with_capacity(threads);
    for tid in 0..threads {
        let c = Arc::clone(&counter);
        let g = Arc::clone(&gauge);
        handles.push(thread::spawn(move || {
            let mut local = 0.0f64;
            for i in 0..iters {
                // Mixed operations to exercise CAS paths
                c.inc();
                if i % 4 == 0 {
                    c.add(3);
                }
                local += (tid as f64) * 0.000_001 + (i as f64 % 7.0);
                if i % 8 == 0 {
                    g.add(0.5);
                } else if i % 8 == 4 {
                    g.sub(0.25);
                }
            }
            // Light memory pressure
            let mut v = Vec::with_capacity(8192);
            for i in 0..8192 {
                v.push(i as u64);
            }
            v.iter().fold(0u64, |acc, x| acc.wrapping_add(*x))
        }));
    }

    let mut checksum = 0u64;
    for h in handles {
        checksum = checksum.wrapping_add(h.join().expect("thread panicked"));
    }

    // Basic sanity assertions
    let cval = counter.get();
    assert!(cval >= (threads * iters) as u64);

    let gval = gauge.get();
    assert!(gval.is_finite());

    // Prevent optimizer dropping values
    eprintln!(
        "chaos checksum={}, counter={}, gauge={}",
        checksum, cval, gval
    );
}

#[ignore]
#[test]
fn chaos_ratemeter_under_load() {
    init();
    let r = metrics().rate("chaos.rps");

    let threads = 32;
    let duration = Duration::from_millis(750);
    let start = Instant::now();

    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                while start.elapsed() < duration {
                    r.tick();
                }
            });
        }
    });

    let rate = r.rate();
    assert!(rate >= 0.0);
}
