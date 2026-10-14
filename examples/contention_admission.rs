//! Best-effort admission with `RateMeter` versus strict admission with
//! `TokenBucket`, under 8 contending threads.
//!
//! Run with: `cargo run --example contention_admission --features meter --release`

use metrics_lib::{RateMeter, TokenBucket};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const LIMIT: f64 = 10_000.0;
const THREADS: usize = 8;
const RUN: Duration = Duration::from_secs(1);

fn run(admit: impl Fn() -> bool + Sync) -> f64 {
    let admitted = AtomicU64::new(0);
    let start = Instant::now();
    std::thread::scope(|s| {
        for _ in 0..THREADS {
            s.spawn(|| {
                while start.elapsed() < RUN {
                    if admit() {
                        let _ = admitted.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });
    #[allow(clippy::cast_precision_loss)]
    let per_sec = admitted.load(Ordering::Relaxed) as f64 / start.elapsed().as_secs_f64();
    per_sec
}

fn main() {
    let meter = Arc::new(RateMeter::new());
    let best_effort = run(|| meter.tick_if_under_limit(LIMIT));
    let bucket = TokenBucket::new(100, LIMIT);
    let strict = run(|| bucket.acquire(1));
    println!("target {LIMIT:.0}/s with {THREADS} threads");
    println!("RateMeter::tick_if_under_limit admitted {best_effort:.0}/s (may overshoot)");
    println!("TokenBucket::acquire             admitted {strict:.0}/s (never above rate + burst)");
}
