//! Registry lookup, registration and exposition benchmarks.
//!
//! Run with: `cargo bench --bench registry_bench --all-features`

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use metrics_lib::exporters::{openmetrics, prometheus};
use metrics_lib::{LabelSet, MetricsCore, Registry};
use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

fn lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_lookup");
    let core = MetricsCore::new();
    core.counter("requests").inc();
    let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
    core.counter_with("requests", &labels).inc();
    for i in 0..1000 {
        core.counter(&format!("filler_{i}")).inc();
    }

    group.bench_function("counter_unlabeled_hit", |b| {
        b.iter(|| core.counter(black_box("requests")).inc());
    });
    group.bench_function("counter_labeled_hit", |b| {
        b.iter(|| {
            core.counter_with(black_box("requests"), black_box(&labels))
                .inc()
        });
    });
    let held = core.counter("requests");
    group.bench_function("counter_held_handle", |b| b.iter(|| held.inc()));
    group.bench_function("gauge_unlabeled_hit", |b| {
        b.iter(|| core.gauge(black_box("gauge_hit")).set(1.0));
    });
    group.bench_function("timer_unlabeled_hit", |b| {
        b.iter(|| core.timer(black_box("timer_hit")).record_ns(100));
    });
    group.bench_function("histogram_labeled_hit", |b| {
        b.iter(|| {
            core.histogram_with(black_box("lat"), black_box(&labels))
                .observe(0.01)
        });
    });
    group.finish();
}

fn concurrent_lookups(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_concurrent");
    group.measurement_time(Duration::from_secs(4));
    for threads in [1usize, 4, 8] {
        group.throughput(Throughput::Elements(threads as u64 * 10_000));
        group.bench_with_input(
            BenchmarkId::new("labeled_lookup_inc", threads),
            &threads,
            |b, &threads| {
                let core = Arc::new(MetricsCore::new());
                let labels: Vec<LabelSet> = (0..threads)
                    .map(|t| LabelSet::from([("worker", t.to_string())]))
                    .collect();
                b.iter(|| {
                    std::thread::scope(|s| {
                        for l in &labels {
                            let core = &core;
                            s.spawn(move || {
                                for _ in 0..10_000 {
                                    core.counter_with("jobs", l).inc();
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

fn registration(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_registration");
    group.bench_function("new_labeled_series", |b| {
        let registry = Registry::with_cardinality_cap(usize::MAX);
        let mut i = 0u64;
        b.iter(|| {
            i += 1;
            registry
                .counter_with("series", &LabelSet::from([("id", i.to_string())]))
                .inc();
        });
    });
    group.bench_function("evict_idle_10k_series", |b| {
        let registry = Registry::with_cardinality_cap(usize::MAX);
        for i in 0..10_000 {
            registry
                .counter_with("series", &LabelSet::from([("id", i.to_string())]))
                .inc();
        }
        b.iter(|| black_box(registry.evict_idle(Duration::from_secs(3600))));
    });
    group.finish();
}

fn exposition(c: &mut Criterion) {
    let mut group = c.benchmark_group("exposition");
    let core = MetricsCore::new();
    for i in 0..100 {
        let labels = LabelSet::from([("route", format!("/r{i}")), ("method", "GET".into())]);
        core.counter_with("http_requests", &labels).add(i);
        core.gauge_with("inflight", &labels)
            .set(f64::from(u32::try_from(i).unwrap_or(0)));
        core.histogram_with("latency_seconds", &labels)
            .observe(0.01);
        core.timer_with("handler", &labels).record_ns(1_000);
    }
    group.throughput(Throughput::Elements(400));
    let mut buf = String::with_capacity(256 * 1024);
    group.bench_function("prometheus_400_series", |b| {
        b.iter(|| {
            buf.clear();
            prometheus::render_into(&mut buf, core.registry());
            black_box(buf.len())
        });
    });
    group.bench_function("openmetrics_400_series", |b| {
        b.iter(|| {
            buf.clear();
            openmetrics::render_into(&mut buf, core.registry());
            black_box(buf.len())
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    lookups,
    concurrent_lookups,
    registration,
    exposition
);
criterion_main!(benches);
