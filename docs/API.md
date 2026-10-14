<h1 align="center">
        <img width="99" alt="Rust logo" src="https://raw.githubusercontent.com/jamesgober/rust-collection/72baabd71f00e14aa9184efcb16fa3deddda3a0a/assets/rust-logo.svg">
    <br><b>metrics-lib</b><br>
    <sub><sup>API REFERENCE</sup></sub>
</h1>
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./README.md" title="Documentation"><b>DOCS</b></a>
        <span>&nbsp;│&nbsp;</span>
        <span>API</span>
        <span>&nbsp;│&nbsp;</span>
        <a href="./GUIDELINES.md" title="Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

A task-oriented tour of the 2.x API. The rustdoc on
[docs.rs](https://docs.rs/metrics-lib) is the exhaustive reference; every
example here is compiled and run as a doctest with all features enabled.

- [Setup](#setup)
- [Metric types](#metric-types)
- [Labels and validation](#labels-and-validation)
- [The registry](#the-registry)
- [Exporters](#exporters)
- [Process and host metrics](#process-and-host-metrics)
- [Load shedding](#load-shedding)
- [Async helpers](#async-helpers)
- [Integrations](#integrations)
- [Errors](#errors)
- [Examples in the repository](#examples-in-the-repository)

## Setup

```toml
[dependencies]
metrics-lib = "2"
# or pick features:
# metrics-lib = { version = "2", default-features = false, features = ["count", "histogram"] }
```

Two ways to hold metrics:

- **An explicit [`MetricsCore`]** (or bare `Registry`) passed to the code
  that records. Preferred in libraries and tests.
- **The global instance** from `metrics()`, which initialises itself with the
  default configuration on first use. Configure it once at start-up with
  `init_with_config`.

```rust
use metrics_lib::{MetricsConfig, MetricsCore};
use std::time::Duration;

let core = MetricsCore::with_config(
    MetricsConfig::default()
        .with_max_series(20_000)
        .with_system_interval(Duration::from_secs(5)),
)?;
core.counter("jobs").inc();

// The global instance:
metrics_lib::metrics().counter("jobs").inc();
# Ok::<(), metrics_lib::MetricsError>(())
```

`MetricsCore::new()` is cheap: no thread is started and no system call is
made until `system()` or a `render_core` exporter is first used.

## Metric types

All metric types are lock-free, `Send + Sync`, and cache-line aligned.
Lookups return `Arc<T>`; keep the `Arc` on hot paths.

### Counter (`count`)

Monotonic `u64`. `inc`/`add` are one relaxed atomic add and wrap at
`u64::MAX`; the `try_*` methods report overflow instead.

```rust
use metrics_lib::{Counter, MetricsError};

let c = Counter::new();
c.inc();
c.add(9);
assert_eq!(c.get(), 10);
c.set_max(15); // only raises
assert_eq!(c.get(), 15);
let full = Counter::with_value(u64::MAX);
assert_eq!(full.try_inc(), Err(MetricsError::Overflow));
```

### ShardedCounter (`count`)

A counter striped over cache-line-padded slots, one per CPU, for counters
incremented from many threads at once. `get()` sums the stripes.

```rust
use metrics_lib::ShardedCounter;
use std::sync::Arc;

let hits = Arc::new(ShardedCounter::new());
std::thread::scope(|s| {
    for _ in 0..4 {
        s.spawn(|| for _ in 0..1000 { hits.inc() });
    }
});
assert_eq!(hits.get(), 4000);
```

### UpDownCounter (`count`)

Signed `i64` that goes up and down: queue lengths, open connections.

```rust
let inflight = metrics_lib::UpDownCounter::new();
inflight.inc();
inflight.add(5);
inflight.dec();
assert_eq!(inflight.get(), 5);
```

### Gauge (`gauge`)

Lock-free `f64`. `set` stores any value (NaN is a legitimate "no data");
every other unchecked method ignores non-finite input.

```rust
let g = metrics_lib::Gauge::new();
g.set(10.0);
g.add(2.5);
g.set_max(20.0);
g.set_max(f64::NAN); // ignored
g.clamp(0.0, 15.0);
assert_eq!(g.get(), 15.0);
assert!(g.try_set(f64::INFINITY).is_err());
```

`gauge_specialized::PercentageGauge` (clamped to 0–100) and
`gauge_specialized::MemoryGauge` (bytes) are thin wrappers.

### Timer (`timer`)

Count, sum, minimum and maximum of durations.

```rust
use metrics_lib::Timer;
use std::sync::Arc;
use std::time::Duration;

let t = Arc::new(Timer::new());
t.record(Duration::from_millis(3));
{
    let _guard = t.start(); // records on drop
}
let elapsed = t.start().stop(); // records now, returns the duration
t.start().discard(); // records nothing
let _owned = t.start_owned(); // guard that owns the Arc
assert!(t.count() >= 3);
assert!(elapsed < Duration::from_secs(1));
```

`MetricsCore::time(name, f)` times a closure; `MetricsCore::start_timer(name)`
returns an owned guard; the `time_block!` and `time_fn!` macros wrap blocks.

### RateMeter (`meter`)

Counts events and estimates rates over sliding windows (last second,
minute, hour, and a configurable primary window used by `rate()`).

```rust
use metrics_lib::RateMeter;
use std::time::Duration;

let m = RateMeter::with_window(Duration::from_secs(10));
m.tick_n(50);
assert_eq!(m.total(), 50);
assert!(m.rate() > 0.0); // events per second over the 10 s window
let admitted = m.tick_if_under_limit(1_000_000.0);
assert!(admitted);
```

`tick_if_under_limit` may overshoot by `threads − 1` under contention; use
`TokenBucket` for a hard limit. `rate_meter_specialized::ApiRateLimiter` and
`ThroughputMeter` build on it.

### Histogram (`histogram`)

Explicit buckets, cumulative on export, with exemplars.

```rust
use metrics_lib::{Histogram, LabelSet};
use std::time::Duration;

let h = Histogram::with_buckets([0.01, 0.1, 1.0]);
h.observe(0.05);
h.observe_duration(Duration::from_millis(250));
h.observe_with_exemplar(0.07, &LabelSet::from([("trace_id", "4bf92f35")]));
let snap = h.snapshot();
assert_eq!(snap.count, 3);
assert_eq!(snap.buckets.last().map(|b| b.count), Some(3));
assert!(h.quantile(0.5) > 0.0);
assert!(Histogram::try_with_buckets([]).is_err());
```

Constructors: `default_seconds()` (Prometheus default buckets),
`linear(start, width, n)`, `exponential(start, factor, n)`.

### Summary (`histogram`)

Streaming quantiles with a relative-error bound (2 % by default).

```rust
use metrics_lib::{Summary, SummaryConfig};

let s = Summary::with_config(
    SummaryConfig::default()
        .with_relative_accuracy(0.01)
        .with_quantiles([0.5, 0.99]),
)?;
for v in 1..=1000 {
    s.observe(f64::from(v));
}
let p99 = s.quantile(0.99);
assert!((p99 - 990.0).abs() / 990.0 <= 0.01);
# Ok::<(), metrics_lib::MetricsError>(())
```

### TokenBucket (always available)

Strict admission (GCRA): never over-admits, even under contention.

```rust
use metrics_lib::TokenBucket;

let bucket = TokenBucket::new(10, 100.0); // burst 10, 100 tokens/s
assert!(bucket.acquire(10));
assert!(!bucket.acquire(1));
assert!(bucket.time_until_available(1).is_some());
```

## Labels and validation

```rust
use metrics_lib::{labels, LabelSet, MetricsCore};

let core = MetricsCore::new();
let l = LabelSet::from([("method", "GET"), ("status", "200")]);
core.counter_with("http_requests", &l).inc();

// The `labels!` macro accepts mixed `&'static str` and `String` values.
let tenant = String::from("acme");
core.counter_with("logins", &labels!("tenant" => tenant)).inc();

// Validation up front:
let mut checked = LabelSet::new();
assert!(checked.try_add("route", "/users/{id}").is_ok());
assert!(checked.try_add("bad key", "x").is_err());
```

Rules: names `[A-Za-z][A-Za-z0-9_.]*` up to `MAX_NAME_LEN` bytes, label keys
`[A-Za-z][A-Za-z0-9_]*` up to `MAX_LABEL_KEY_LEN`, values up to
`MAX_LABEL_VALUE_LEN` bytes, at most `MAX_LABELS` pairs. The `try_*`
lookups reject invalid input; the others repair it with
`sanitize_metric_name` / `sanitize_label_key`. Exporters escape or replace
every character their format reserves, so no input can inject lines.

## The registry

```rust
use metrics_lib::{LabelSet, MetricKind, MetricsError, Registry, Unit};
use std::time::Duration;

let r = Registry::with_cardinality_cap(1_000);
r.describe_counter("jobs", "Jobs processed", Unit::None)?;
r.configure_histogram("job_seconds", [0.1, 1.0, 10.0])?;
r.counter("jobs").inc();
r.histogram("job_seconds").observe(0.4);

// One name, one kind:
assert!(matches!(r.try_gauge("jobs"), Err(MetricsError::KindMismatch { .. })));
assert_eq!(r.kind_of("jobs"), Some(MetricKind::Counter));

// Scopes prefix names:
let api = r.scoped("api.");
api.counter("requests").inc();
assert_eq!(r.counter("api.requests").get(), 1);

// Removal and expiry:
r.counter_with("per_user", &LabelSet::from([("user", "u1")])).inc();
assert!(r.remove_series("per_user", &LabelSet::from([("user", "u1")])));
let evicted = r.evict_idle(Duration::from_secs(3600));
assert_eq!(evicted, 0);
# Ok::<(), MetricsError>(())
```

Lookups of existing series take one shard read lock and do not allocate.
Diagnostics: `cardinality_count`, `cardinality_overflows`,
`invalid_inputs`, `kind_conflicts`, `exposition_conflicts`, `metric_count`.
`*_entries()` and `*_names()` list series for custom exporters.

## Exporters

### Prometheus and OpenMetrics (always available)

```rust
use metrics_lib::exporters::{openmetrics, prometheus};
use metrics_lib::MetricsCore;

let core = MetricsCore::new();
core.counter("jobs_total").add(3);
let prom = prometheus::render(core.registry());
assert!(prom.contains("# TYPE jobs_total counter\njobs_total 3\n"));
let om = openmetrics::render(core.registry());
assert!(om.contains("# TYPE jobs counter\njobs_total 3\n"));
assert!(om.ends_with("# EOF\n"));
// Including process_* families (feature `system`):
let full = prometheus::render_core(&core);
assert!(full.contains("process_resident_memory_bytes"));
```

Content types: `prometheus::CONTENT_TYPE`, `openmetrics::CONTENT_TYPE`.
`render_into` appends to a reusable buffer.

### JSON (`serde`)

```rust
use metrics_lib::exporters::json;

let core = metrics_lib::MetricsCore::new();
core.gauge("temp").set(21.5);
let body = json::render(core.registry())?;
assert!(body.contains(r#""type":"gauge","value":21.5"#));
# Ok::<(), serde_json::Error>(())
```

### StatsD (`statsd`)

```rust,no_run
use metrics_lib::exporters::statsd::StatsdSink;

let core = metrics_lib::MetricsCore::new();
core.counter("hits").add(5);
let sink = StatsdSink::new("127.0.0.1:8125")?.with_prefix("app.");
sink.send(core.registry())?; // "app.hits:5|c"
# Ok::<(), std::io::Error>(())
```

### OTLP (`otlp`)

```rust
use metrics_lib::exporters::otlp::{OtlpExporter, Temporality};

let core = metrics_lib::MetricsCore::new();
core.counter("orders").add(2);
let exporter = OtlpExporter::new("shop").with_temporality(Temporality::Cumulative);
let json = exporter.render_json(core.registry())?;
assert!(json.contains("\"asInt\":\"2\""));
let protobuf = exporter.render_protobuf(core.registry());
assert!(!protobuf.is_empty());
# Ok::<(), serde_json::Error>(())
```

### HTTP server and Pushgateway (`http`)

```rust,no_run
use metrics_lib::exporters::http::{MetricsServer, Pushgateway};
use metrics_lib::{LabelSet, MetricsCore};
use std::sync::Arc;

let core = Arc::new(MetricsCore::new());
let server = MetricsServer::spawn("0.0.0.0:9898", Arc::clone(&core))?;

let gateway = Pushgateway::new("http://pushgateway:9091")?;
gateway.push("batch_job", &LabelSet::from([("instance", "a")]), core.registry())?;
server.shutdown();
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Process and host metrics

Feature `system` (default).

```rust
use metrics_lib::{MetricsCore, ProcessCollector};

let core = MetricsCore::new();
let sys = core.system(); // starts the sampler on first use
assert!(sys.mem_used_mb() <= sys.mem_total_mb());
println!("health {:.0} ({})", sys.health_score(), sys.quick_check());

let p = ProcessCollector::new().collect()?;
assert!(p.resident_memory_bytes > 0);
# Ok::<(), std::io::Error>(())
```

Values a platform cannot provide (`thread_count` and `load_avg` on
Windows) are `None`. Linux, Windows and macOS are read directly (a process
sample takes microseconds); other targets use `sysinfo`. Thresholds live in `HealthConfig`.

## Load shedding

Feature `sample`.

```rust
use metrics_lib::{
    AdaptiveSampler, BackpressureController, CircuitBreakerConfig, MetricCircuitBreaker,
    SamplingStrategy,
};
use std::time::Duration;

let sampler = AdaptiveSampler::new(SamplingStrategy::Fixed { rate: 10 });
let _keep = sampler.should_sample();

let breaker = MetricCircuitBreaker::new(
    CircuitBreakerConfig::default()
        .with_failure_threshold(3)
        .with_timeout(Duration::from_secs(10)),
);
if breaker.check().is_ok() {
    breaker.record_success();
}

let gate = BackpressureController::new(64);
if let Some(_slot) = gate.try_acquire() {
    // do the work; the slot is released on drop
}
```

## Async helpers

Feature `async`.

```rust
use metrics_lib::async_support::{AsyncMetricsBatcher, AsyncTimerExt};
use metrics_lib::MetricsCore;
use std::sync::Arc;
use std::time::Duration;

# let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
# rt.block_on(async {
let core = Arc::new(MetricsCore::new());
let timer = core.timer("fetch_seconds");
let body = timer.time_async(|| async { "ok" }).await;
assert_eq!(body, "ok");

let batcher = Arc::new(AsyncMetricsBatcher::new(Arc::clone(&core), Duration::from_secs(1), 128));
let flusher = Arc::clone(&batcher).start_flusher();
batcher.record(|b| b.counter_inc("jobs", 1)).await;
flusher.shutdown().await;
assert_eq!(core.counter("jobs").get(), 1);
# });
```

`record_runtime_metrics` copies Tokio's runtime metrics into gauges.

## Integrations

### `metrics` facade (`metrics-rs`)

```rust
use metrics_lib::metrics_rs::MetricsLibRecorder;
use metrics_lib::MetricsCore;
use std::sync::Arc;

let core = Arc::new(MetricsCore::new());
let recorder = MetricsLibRecorder::new(Arc::clone(&core));
metrics::with_local_recorder(&recorder, || {
    metrics::counter!("requests").increment(1);
});
assert_eq!(core.counter("requests").get(), 1);
```

Install it globally with `metrics_lib::metrics_rs::install(core)`.

### Tower middleware (`tower`)

```rust
use metrics_lib::tower::RedLayer;
use metrics_lib::MetricsCore;
use std::sync::Arc;
use tower::Layer;

let core = Arc::new(MetricsCore::new());
let layer = RedLayer::new(Arc::clone(&core)).with_prefix("api_");
let _svc = layer.layer(tower::service_fn(|_req: http::Request<()>| async {
    Ok::<_, std::convert::Infallible>(http::Response::new(()))
}));
```

Records `<prefix>http_requests_total{method,status}`,
`<prefix>http_request_duration_seconds` and
`<prefix>http_requests_in_flight`.

### `tracing` (`tracing`)

```rust
use metrics_lib::tracing_ext::{observe_with_span_exemplar, time_in_span};

let core = metrics_lib::MetricsCore::new();
let rows = time_in_span("db.query", &core.timer("db_seconds"), || 3);
assert_eq!(rows, 3);
observe_with_span_exemplar(&core.histogram("latency_seconds"), 0.02);
```

### Shorthand macros

```rust
use metrics_lib::{counter, gauge, histogram};

counter!("events", "kind" => "click").inc();
gauge!("queue_depth").set(4.0);
histogram!("payload_bytes").observe(512.0);
```

## Errors

All fallible operations return `metrics_lib::Result<T>` with
[`MetricsError`]:

| Variant | Returned by |
|---|---|
| `InvalidName` | `try_*` lookups, `describe*`, `configure_*`, `validate_metric_name` |
| `InvalidLabel` | labeled `try_*` lookups, `LabelSet::try_add`, exemplars |
| `InvalidValue` | `Gauge::try_*`, `Histogram::try_observe`, `Summary::try_observe` |
| `Overflow` / `Underflow` | checked counter, timer and up/down counter methods |
| `WouldBlock` | `TokenBucket::try_acquire` |
| `CircuitOpen` | `MetricCircuitBreaker::check` |
| `CardinalityExceeded` | `try_*` lookups and `describe*` past the series cap |
| `KindMismatch` | `try_*` lookups and `describe*` for a name of another kind |
| `AlreadyInitialized` | `init_with_config` after the global instance exists |
| `Config` | invalid `MetricsConfig`, `SummaryConfig`, `TokenBucket::try_new`, … |

The enum is `#[non_exhaustive]`.

## Examples in the repository

| Example | Shows |
|---|---|
| `quick_start` | counters, gauges, timers and a Prometheus exposition |
| `quick_tour` | every metric type, kind checks, OpenMetrics |
| `labels_demo` | labels, validation, the series cap |
| `histogram_latency` | histograms versus summaries |
| `cache_hit_miss` | held handles in a loop |
| `streaming_rate_window` | sliding rate windows |
| `contention_admission` | `RateMeter` versus `TokenBucket` under contention |
| `token_bucket_limiter` | strict admission with waiting |
| `broker_throughput` | producer/consumer rates |
| `benchmark_comparison` | rough ns/op on your machine |
| `custom_exporter_openmetrics` | a custom exporter over `*_entries()` |
| `prometheus_endpoint` | the built-in `/metrics` server |
| `axum_registry_integration` | `/metrics` in an axum app |
| `axum_middleware_metrics` | `RedLayer` with route templates |
| `statsd_push`, `otlp_push`, `snapshot_serde` | the push/snapshot exporters |
| `metrics_facade` | the `metrics` crate bridge |
| `async_batch_timing` | async timing and batching |
| `cpu_stats`, `memory_stats`, `health_dashboard` | system metrics |

Run one with `cargo run --example <name> --all-features`.

[`MetricsCore`]: https://docs.rs/metrics-lib/latest/metrics_lib/struct.MetricsCore.html
[`MetricsError`]: https://docs.rs/metrics-lib/latest/metrics_lib/enum.MetricsError.html
