//! A quick, unscientific throughput check of the hot paths on this machine.
//!
//! For real numbers use the Criterion benches (`cargo bench`), which control
//! for warm-up and noise; this example only shows the order of magnitude.
//!
//! Run with: `cargo run --example benchmark_comparison --features meter --release`

use metrics_lib::{Counter, Gauge, RateMeter, ShardedCounter, Timer};
use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

const N: u32 = 10_000_000;

fn per_op(label: &str, f: impl Fn()) {
    let start = Instant::now();
    for _ in 0..N {
        f();
    }
    let ns = start.elapsed().as_secs_f64() * 1e9 / f64::from(N);
    println!("{label:<28} {ns:>6.2} ns/op");
}

fn main() {
    let counter = Counter::new();
    per_op("Counter::inc", || counter.inc());
    let gauge = Gauge::new();
    per_op("Gauge::set", || gauge.set(black_box(1.5)));
    let timer = Timer::new();
    per_op("Timer::record_ns", || timer.record_ns(black_box(1_000)));
    let meter = RateMeter::new();
    per_op("RateMeter::tick", || meter.tick());

    for threads in [1u32, 4, 8] {
        let plain = Arc::new(Counter::new());
        let sharded = Arc::new(ShardedCounter::new());
        for (label, inc) in [
            (
                "Counter, contended",
                Box::new({
                    let c = Arc::clone(&plain);
                    move || c.inc()
                }) as Box<dyn Fn() + Send + Sync>,
            ),
            (
                "ShardedCounter, contended",
                Box::new({
                    let c = Arc::clone(&sharded);
                    move || c.inc()
                }),
            ),
        ] {
            let per_thread = N / threads;
            let start = Instant::now();
            std::thread::scope(|s| {
                for _ in 0..threads {
                    s.spawn(|| {
                        for _ in 0..per_thread {
                            inc();
                        }
                    });
                }
            });
            let ns = start.elapsed().as_secs_f64() * 1e9 / f64::from(per_thread * threads);
            println!("{label:<28} {threads} threads: {ns:>6.2} ns/op (wall time per op)");
        }
    }
}
