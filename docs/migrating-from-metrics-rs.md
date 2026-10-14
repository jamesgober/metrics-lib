# Migrating from the `metrics` crate

There are two ways to adopt metrics-lib in a code base instrumented with
the [`metrics`](https://docs.rs/metrics) facade (0.22+). They combine well.

## Option 1: keep the facade, swap the recorder

Enable the `metrics-rs` feature and install metrics-lib as the recorder.
Every `metrics::counter!` / `gauge!` / `histogram!` call in your code *and in
your dependencies* then records into a metrics-lib registry, and leaves
through any metrics-lib exporter. No call site changes.

```rust
use metrics_lib::metrics_rs::MetricsLibRecorder;
use metrics_lib::exporters::prometheus;
use metrics_lib::MetricsCore;
use std::sync::Arc;

let core = Arc::new(MetricsCore::new());
let recorder = MetricsLibRecorder::new(Arc::clone(&core));
// In an application: `metrics_lib::metrics_rs::install(Arc::clone(&core))?` once.
metrics::with_local_recorder(&recorder, || {
    metrics::describe_counter!("requests_total", "Requests served");
    metrics::counter!("requests_total", "method" => "GET").increment(1);
    metrics::gauge!("queue_depth").set(42.0);
    metrics::histogram!("db_seconds").record(0.012);
});
let body = prometheus::render(core.registry());
assert!(body.contains(r#"requests_total{method="GET"} 1"#));
```

Facade histograms become metrics-lib `Histogram`s with the default
latency buckets; configure others per name with
`core.registry().configure_histogram(...)` before first use. Facade names and
label keys are repaired to the metrics-lib rules (`http.server.requests`
stays, `my-metric` becomes `my_metric`).

## Option 2: native handles

Native handles skip the facade's indirection and give access to every
metric type (`ShardedCounter`, `UpDownCounter`, `Timer`, `RateMeter`,
`Summary`, `TokenBucket`).

| `metrics` | metrics-lib |
|---|---|
| `counter!("x").increment(n)` | `core.counter("x").add(n)` |
| `counter!("x", "k" => "v")` | `core.counter_with("x", &labels!("k" => "v"))` |
| `counter!("x").absolute(n)` | `core.counter("x").set_max(n)` |
| `gauge!("x").set(v)` / `increment(v)` / `decrement(v)` | `core.gauge("x").set(v)` / `add(v)` / `sub(v)` |
| `histogram!("x").record(v)` | `core.histogram("x").observe(v)` |
| `describe_counter!("x", "help")` | `core.registry().describe_counter("x", "help", Unit::None)?` |

```rust
use metrics_lib::{labels, MetricsCore, Unit};

let core = MetricsCore::new();
core.registry()
    .describe_counter("jobs_total", "Jobs processed", Unit::None)?;

// Resolve handles once; the hot path is then a few atomic operations.
let jobs = core.counter_with("jobs_total", &labels!("queue" => "email"));
let latency = core.histogram("job_seconds");
for _ in 0..3 {
    let _t = core.timer("job_wall_seconds").start_owned();
    jobs.inc();
    latency.observe(0.01);
}
assert_eq!(jobs.get(), 3);
# Ok::<(), metrics_lib::MetricsError>(())
```

The shorthand macros `metrics_lib::counter!`, `gauge!`, `histogram!`, … use
the global instance in the same `name, key => value` style.

## Differences to keep in mind

- **One name, one kind.** metrics-lib refuses to register a name as two
  kinds, because the result could not be exported validly.
- **Bounded cardinality.** The registry caps the number of series (default
  10 000); see `Registry::set_cardinality_cap` and `Registry::evict_idle`.
- **Exporters are in the crate.** Prometheus and OpenMetrics are always
  available; StatsD, OTLP, JSON and an HTTP server are features.
- **Rates.** `RateMeter` keeps in-process sliding windows; with Prometheus
  you can still compute rates from the exported `_total` counter.
