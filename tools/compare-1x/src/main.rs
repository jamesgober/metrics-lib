//! Interleaved timing of metrics-lib 1.0.0 (`ml1`) against the working tree
//! (`ml2`). Each operation is timed in 60 rounds per version, alternating,
//! three times over, and the minimum per-operation time is reported, which
//! filters out preemption on a loaded machine.

use std::hint::black_box;
use std::time::Instant;

const ROUNDS: usize = 60;

fn min_ns(iters: u32, mut f: impl FnMut()) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        for _ in 0..iters {
            f();
        }
        let ns = start.elapsed().as_secs_f64() * 1e9 / f64::from(iters);
        best = best.min(ns);
    }
    best
}

fn main() {
    let filter = std::env::args().nth(1).unwrap_or_default();

    let r1 = ml1::Registry::new();
    let r2 = ml2::Registry::new();
    for i in 0..200 {
        let name = format!("metric_{i}");
        let _ = r1.get_or_create_counter(&name);
        let _ = r2.counter(&name);
    }
    let l1 = ml1::LabelSet::from([("method", "GET"), ("status", "200")]);
    let l2 = ml2::LabelSet::from([("method", "GET"), ("status", "200")]);
    let h1 = ml1::Histogram::default_seconds();
    let h2 = ml2::Histogram::default_seconds();
    for i in 0..1_000u32 {
        h1.observe(f64::from(i % 100) * 0.005);
        h2.observe(f64::from(i % 100) * 0.005);
    }
    let (m1, m2) = (ml1::RateMeter::new(), ml2::RateMeter::new());
    let (c1, c2) = (ml1::Counter::new(), ml2::Counter::new());
    let (t1, t2) = (ml1::Timer::new(), ml2::Timer::new());

    type Case<'a> = (&'static str, Box<dyn FnMut() -> (f64, f64) + 'a>);
    let mut cases: Vec<Case<'_>> = vec![
        (
            "counter_inc",
            Box::new(|| (min_ns(200_000, || c1.inc()), min_ns(200_000, || c2.inc()))),
        ),
        (
            "timer_record_ns",
            Box::new(|| {
                (
                    min_ns(200_000, || t1.record_ns(black_box(1234))),
                    min_ns(200_000, || t2.record_ns(black_box(1234))),
                )
            }),
        ),
        (
            "histogram_observe",
            Box::new(|| {
                (
                    min_ns(100_000, || h1.observe(black_box(0.042))),
                    min_ns(100_000, || h2.observe(black_box(0.042))),
                )
            }),
        ),
        (
            "histogram_snapshot",
            Box::new(|| {
                (
                    min_ns(20_000, || drop(black_box(h1.snapshot()))),
                    min_ns(20_000, || drop(black_box(h2.snapshot()))),
                )
            }),
        ),
        (
            "unlabeled_lookup_inc",
            Box::new(|| {
                (
                    min_ns(20_000, || {
                        r1.get_or_create_counter(black_box("metric_42")).inc()
                    }),
                    min_ns(20_000, || r2.counter(black_box("metric_42")).inc()),
                )
            }),
        ),
        (
            "labeled_lookup_inc",
            Box::new(|| {
                (
                    min_ns(20_000, || {
                        r1.get_or_create_counter_with(black_box("req"), &l1).inc();
                    }),
                    min_ns(20_000, || r2.counter_with(black_box("req"), &l2).inc()),
                )
            }),
        ),
        (
            "rate_tick",
            Box::new(|| (min_ns(100_000, || m1.tick()), min_ns(100_000, || m2.tick()))),
        ),
        (
            "rate_tick_n",
            Box::new(|| {
                (
                    min_ns(100_000, || m1.tick_n(black_box(8))),
                    min_ns(100_000, || m2.tick_n(black_box(8))),
                )
            }),
        ),
        (
            "rate_rate",
            Box::new(|| {
                (
                    min_ns(100_000, || {
                        black_box(m1.rate());
                    }),
                    min_ns(100_000, || {
                        black_box(m2.rate());
                    }),
                )
            }),
        ),
    ];

    println!(
        "{:22} {:>10} {:>10} {:>8}",
        "operation", "1.0.0", "2.x", "change"
    );
    for (name, case) in &mut cases {
        if !name.contains(filter.as_str()) {
            continue;
        }
        let (mut old, mut new) = (f64::MAX, f64::MAX);
        for _ in 0..3 {
            let (a, b) = case();
            old = old.min(a);
            new = new.min(b);
        }
        println!(
            "{name:22} {old:>7.2} ns {new:>7.2} ns {:>+7.1}%",
            (new / old - 1.0) * 100.0
        );
    }
}
