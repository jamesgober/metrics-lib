//! Hot-path benchmarks for the metric primitives. Every performance number
//! in the README and `docs/PERFORMANCE.md` comes from this file or from
//! `registry_bench.rs`.
//!
//! Run with: `cargo bench --bench metrics_bench --all-features`

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use metrics_lib::{
    Counter, Gauge, Histogram, LabelSet, RateMeter, ShardedCounter, Summary, Timer, TokenBucket,
    UpDownCounter, metrics,
};
use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

fn counter(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter");
    let counter = Counter::new();
    group.bench_function("inc", |b| b.iter(|| counter.inc()));
    group.bench_function("add", |b| b.iter(|| counter.add(black_box(5))));
    group.bench_function("get", |b| b.iter(|| black_box(counter.get())));
    group.bench_function("try_add", |b| b.iter(|| counter.try_add(black_box(1))));
    let sharded = ShardedCounter::new();
    group.bench_function("sharded_inc", |b| b.iter(|| sharded.inc()));
    group.bench_function("sharded_get", |b| b.iter(|| black_box(sharded.get())));
    let updown = UpDownCounter::new();
    group.bench_function("up_down_add", |b| b.iter(|| updown.add(black_box(-1))));
    group.finish();
}

/// Total time for `threads` threads to perform 100 000 increments each on
/// one shared counter: the contention case `ShardedCounter` exists for.
fn contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("contended_increments");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(5));
    const PER_THREAD: u64 = 100_000;
    for threads in [1usize, 4, 8, 16] {
        group.throughput(Throughput::Elements(threads as u64 * PER_THREAD));
        group.bench_with_input(
            BenchmarkId::new("counter", threads),
            &threads,
            |b, &threads| {
                let counter = Arc::new(Counter::new());
                b.iter(|| {
                    std::thread::scope(|s| {
                        for _ in 0..threads {
                            s.spawn(|| {
                                for _ in 0..PER_THREAD {
                                    counter.inc();
                                }
                            });
                        }
                    });
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("sharded_counter", threads),
            &threads,
            |b, &threads| {
                let counter = Arc::new(ShardedCounter::new());
                b.iter(|| {
                    std::thread::scope(|s| {
                        for _ in 0..threads {
                            s.spawn(|| {
                                for _ in 0..PER_THREAD {
                                    counter.inc();
                                }
                            });
                        }
                    });
                });
            },
        );
    }
    group.finish();
}

fn gauge(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge");
    let gauge = Gauge::new();
    group.bench_function("set", |b| b.iter(|| gauge.set(black_box(42.5))));
    group.bench_function("add", |b| b.iter(|| gauge.add(black_box(1.5))));
    group.bench_function("get", |b| b.iter(|| black_box(gauge.get())));
    group.bench_function("set_max", |b| b.iter(|| gauge.set_max(black_box(10.0))));
    group.finish();
}

fn timer(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer");
    let timer = Arc::new(Timer::new());
    group.bench_function("record_ns", |b| {
        b.iter(|| timer.record_ns(black_box(1_000)))
    });
    group.bench_function("record", |b| {
        b.iter(|| timer.record(black_box(Duration::from_micros(3))));
    });
    group.bench_function("start_stop", |b| b.iter(|| timer.start().stop()));
    group.bench_function("start_owned_drop", |b| b.iter(|| drop(timer.start_owned())));
    group.finish();
}

fn rate_meter(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate_meter");
    let meter = RateMeter::new();
    group.bench_function("tick", |b| b.iter(|| meter.tick()));
    group.bench_function("tick_n", |b| b.iter(|| meter.tick_n(black_box(8))));
    group.bench_function("rate", |b| b.iter(|| black_box(meter.rate())));
    group.finish();
}

fn distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("distribution");
    let h = Histogram::default_seconds();
    let mut v = 0.0f64;
    group.bench_function("histogram_observe", |b| {
        b.iter(|| {
            v = (v + 0.0137) % 12.0;
            h.observe(black_box(v));
        });
    });
    let trace = LabelSet::from([("trace_id", "4bf92f3577b34da6")]);
    group.bench_function("histogram_observe_with_exemplar", |b| {
        b.iter(|| h.observe_with_exemplar(black_box(0.07), &trace));
    });
    let plain = Histogram::default_seconds();
    for i in 0..1_000u32 {
        plain.observe(f64::from(i % 100) * 0.005);
    }
    group.bench_function("histogram_snapshot", |b| {
        b.iter(|| black_box(plain.snapshot()));
    });
    group.bench_function("histogram_snapshot_with_exemplars", |b| {
        b.iter(|| black_box(h.snapshot()));
    });
    let s = Summary::new();
    group.bench_function("summary_observe", |b| {
        b.iter(|| {
            v = (v + 0.0137) % 12.0;
            s.observe(black_box(v));
        });
    });
    for _ in 0..10_000 {
        s.observe(0.05);
    }
    group.bench_function("summary_quantile", |b| {
        b.iter(|| black_box(s.quantile(0.99)))
    });
    group.finish();
}

fn admission(c: &mut Criterion) {
    let mut group = c.benchmark_group("admission");
    let bucket = TokenBucket::new(u64::MAX / 2_000_000, 1e6);
    group.bench_function("token_bucket_acquire", |b| {
        b.iter(|| bucket.acquire(black_box(1)))
    });
    let empty = TokenBucket::new(1, 0.0);
    let _ = empty.acquire(1);
    group.bench_function("token_bucket_reject", |b| {
        b.iter(|| empty.acquire(black_box(1)))
    });
    group.finish();
}

fn global(c: &mut Criterion) {
    let mut group = c.benchmark_group("global");
    let core = metrics();
    core.counter("bench_global").inc();
    group.bench_function("counter_lookup_inc", |b| {
        b.iter(|| core.counter(black_box("bench_global")).inc());
    });
    group.bench_function("labels_from_array_2", |b| {
        b.iter(|| black_box(LabelSet::from([("method", "GET"), ("status", "200")])));
    });
    group.finish();
}

criterion_group!(
    benches,
    counter,
    contention,
    gauge,
    timer,
    rate_meter,
    distributions,
    admission,
    global
);
criterion_main!(benches);
