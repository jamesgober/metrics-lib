#![allow(dead_code)]

// Longevity Tests: sustained 1B+ operations (configurable)
// Gated and ignored by default to avoid CI timeouts.
// Run with: OPS=1000000000 cargo test --features bench-tests -- --ignored --test longevity_tests

#![cfg(all(test, feature = "bench-tests", not(tarpaulin)))]

use std::env;
use std::time::{Duration, Instant};

use metrics_lib::{init, metrics};

fn parse_ops(default_ops: u64) -> u64 {
    env::var("OPS").ok().and_then(|s| s.parse().ok()).unwrap_or(default_ops)
}

#[ignore]
#[test]
fn longevity_counter_throughput() {
    init();
    let m = metrics();
    let c = m.counter("longevity.counter");

    let default_ops = if std::env::var("CI").is_ok() {
        10_000_000
    } else {
        100_000_000
    };
    let ops = parse_ops(default_ops);

    let start = Instant::now();
    for _ in 0..ops {
        c.inc();
    }
    let dur = start.elapsed();

    let rps = (ops as f64) / dur.as_secs_f64();
    eprintln!(
        "longevity ops={} duration={:?} ops/sec={:.0}",
        ops, dur, rps
    );

    // Sanity
    assert_eq!(c.get(), ops);
}

#[ignore]
#[test]
fn longevity_mixed_operations() {
    init();
    let m = metrics();
    let c = m.counter("longevity.mixed.counter");
    let g = m.gauge("longevity.mixed.gauge");

    let default_ops = if std::env::var("CI").is_ok() {
        5_000_000
    } else {
        50_000_000
    };
    let ops = parse_ops(default_ops);

    let start = Instant::now();
    for i in 0..ops {
        c.inc();
        if i % 2 == 0 {
            c.add(1);
        }
        if i % 4 == 0 {
            g.add(0.001);
        } else if i % 4 == 2 {
            g.sub(0.001);
        }
    }
    let dur = start.elapsed();

    eprintln!("longevity.mixed ops={} duration={:?}", ops, dur);
    assert!(g.get().is_finite());
}
