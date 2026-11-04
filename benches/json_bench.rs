//! The two JSON writers side by side: serde_json (features `serde`, `otlp`)
//! and jsonx-lib (features `json`, `otlp-json`), on the same registry.
//!
//! Run with: `cargo bench --bench json_bench --all-features`
//!
//! The registry is a typical HTTP service: request counters by method,
//! route and status, latency histograms by route, a few gauges, timers,
//! rate meters and summaries (159 series).

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use metrics_lib::exporters::{json, otlp};
use metrics_lib::{LabelSet, Registry, Unit};
use std::hint::black_box;
use std::time::Duration;

fn service_registry() -> Registry {
    let r = Registry::new();
    r.describe_counter("http_requests", "Requests served", Unit::None)
        .unwrap();
    r.describe_histogram("http_request_duration", "Request latency", Unit::Seconds)
        .unwrap();
    let routes = [
        "/",
        "/login",
        "/logout",
        "/api/users",
        "/api/users/:id",
        "/api/orders",
        "/api/orders/:id",
        "/api/search",
        "/static/*",
        "/health",
    ];
    for (i, route) in routes.iter().enumerate() {
        for method in ["GET", "POST", "PUT", "DELETE"] {
            for status in ["200", "404", "500"] {
                let labels =
                    LabelSet::from([("method", method), ("route", route), ("status", status)]);
                r.counter_with("http_requests", &labels)
                    .add(i as u64 * 1_000 + 17);
            }
        }
        let h = r.histogram_with(
            "http_request_duration",
            &LabelSet::from([("route", *route)]),
        );
        for j in 0..200 {
            h.observe(f64::from(j) * 0.000_37 * (i as f64 + 1.0));
        }
    }
    for i in 0..20 {
        r.gauge_with(
            "pool_in_use",
            &LabelSet::from([("pool", format!("pool-{i}"))]),
        )
        .set(f64::from(i) * 1.25);
    }
    for name in ["db_query", "cache_get", "render", "auth", "queue_wait"] {
        let t = r.timer(name);
        for j in 1..100u64 {
            t.record(Duration::from_micros(j * 37));
        }
    }
    r.rate_meter("events").tick_n(12_345);
    r.rate_meter("bytes_in").tick_n(98_765_432);
    for name in ["payload_size", "batch_size"] {
        let s = r.summary(name);
        for j in 0..500 {
            s.observe(f64::from(j) * 3.5);
        }
    }
    r
}

fn snapshot_writers(c: &mut Criterion) {
    let r = service_registry();
    let snap = json::snapshot(&r);
    assert_eq!(serde_json::to_string(&snap).unwrap(), snap.to_json());
    let bytes = snap.to_json().len() as u64;

    let mut group = c.benchmark_group("json_snapshot_write");
    group.throughput(Throughput::Bytes(bytes));
    group.bench_function("serde_json", |b| {
        b.iter(|| serde_json::to_string(black_box(&snap)).unwrap());
    });
    group.bench_function("jsonx", |b| b.iter(|| black_box(&snap).to_json()));
    group.bench_function("serde_json_pretty", |b| {
        b.iter(|| serde_json::to_string_pretty(black_box(&snap)).unwrap());
    });
    group.bench_function("jsonx_pretty", |b| {
        b.iter(|| black_box(&snap).to_json_pretty())
    });
    group.finish();

    let mut group = c.benchmark_group("json_render");
    group.throughput(Throughput::Bytes(bytes));
    group.bench_function("render_serde_json", |b| {
        b.iter(|| json::render(black_box(&r)).unwrap());
    });
    group.bench_function("to_json_jsonx", |b| b.iter(|| json::to_json(black_box(&r))));
    group.finish();
}

fn otlp_writers(c: &mut Criterion) {
    let r = service_registry();
    let exporter = otlp::OtlpExporter::new("checkout")
        .with_resource_attribute("deployment.environment", "prod");
    let bytes = exporter.to_json(&r).len() as u64;

    let mut group = c.benchmark_group("otlp_json");
    group.throughput(Throughput::Bytes(bytes));
    group.bench_function("render_json_serde_json", |b| {
        b.iter(|| exporter.render_json(black_box(&r)).unwrap());
    });
    group.bench_function("to_json_jsonx", |b| {
        b.iter(|| exporter.to_json(black_box(&r)))
    });
    group.finish();
}

criterion_group!(benches, snapshot_writers, otlp_writers);
criterion_main!(benches);
