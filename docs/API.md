<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
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
        <a href="./GUIDELINES.md" title="Developer Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

## Table of Contents
- **[Installation](#installation)**
- **[Quick Start](#quick-start)**
- **[Public APIs](#public-apis)**
  - [Global initialization](#global-initialization)
  - [`MetricsCore`](#metricscore)
  - [`Registry`](#registry)
  - [`Counter`](#counter)
  - [`Gauge`](#gauge)
  - [`Timer`](#timer)
  - [`RateMeter`](#ratemeter)
  - [`SystemHealth`](#systemhealth)
  - [Async support](#async-support)
  - [Adaptive controls](#adaptive-controls)
  - [Prelude](#prelude)
- **[Deployment Patterns](#deployment-patterns)**
  - [Initialization Patterns](#1-initialization-patterns)
  - [High-Volume Strategies](#2-high-volume-strategies)
  - [Memory Management](#3-memory-management)
  - [Multi-Service Patterns](#4-multi-service-patterns)
  - [Export and Ingestion](#5-export-and-ingestion)
  - [On-Call Diagnostics](#6-on-call-diagnostics)
  - [Feature Gating Strategies](#7-feature-gating-strategies)
- **[Real-World Examples](#real-world-examples)**
  - [High-Frequency Trading (HFT)](#real-world-high-frequency-trading)
  - [Web Service Under Load](#real-world-web-service-under-load)
  - [Batch Processing Pipeline](#real-world-batch-processing-pipeline)
  - [Token Bucket Rate Limiter](#real-world-token-bucket-rate-limiter)
- **[Integration Examples](#integration-examples)**
  - [1. Web Framework Integration](#web-framework-integration)
  - [2. Database Pool Monitoring](#database-pool-monitoring)
  - [3. Background Job Processing](#background-job-processing)
  - [4. Observability Stack Integration](#observability-stack-integration)
  - [5. Correlation with Tracing](#correlation-with-tracing)
  - [6. Grafana Dashboard Setup](#grafana-dashboard-setup)
  - [7. Message Brokers (Kafka/NATS) Throughput and Lag](#message-brokers-throughput)
  - [8. Caches (Redis) Hit/Miss, Pool Metrics, TTL Health](#caches-hit-miss-pool-metrics)
  - [9. Serverless (AWS Lambda) Cold-Start and Duration](#serverless-cold-start-and-duration)
  - [10.  Kubernetes Scraping & Pod-level Dashboards](#kubernetes-scraping)
  - [11. OpenTelemetry Export Bridge (example skeleton)](#open-telemetry-export)
  - [12. NATS-Specific Queue Depth and Consumers](#nats-specific-queue)
  - [13. Redis Latency Histogram and Dashboard Queries](#redis-latency-histogram)
  - [14. AWS Lambda EMF (Embedded Metric Format) Emission](#aws-lambda-emf)
  - [15. Kubernetes Helm Values (Prometheus Scrape Annotations)](#kubernetes-helm-values)
  - [16. Full OTLP Exporter Skeleton (tonic)](#otlp-exporter)
  - [17. Grafana Panels (Ready-to-Copy JSON)](#grafana-panels)
  - [18. Prometheus Operator ServiceMonitor](#prometheus-operator-servicemonitor)
  - [19. Full Grafana Dashboard (Ready-to-Import JSON)](#full-grafana-dashboard)
  - [20. Prometheus Recording Rules (Latency and Rates)](#prometheus-recording-rules)
  - [21. Prometheus Operator ServiceMonitor (Secured Endpoint)](#prometheus-operator-servicemonitor)
  - [22. Helm Snippets (kube-prometheus-stack and App Chart)](#helm-snippets)
- **[Notes](#notes)**


<br><br>

## Installation

### Default Installation

#### Install Manually

Add this to your `Cargo.toml`:
```toml
[dependencies]
metrics-lib = "0.8.0"
```

<br>

#### Install via Terminal
```bash
# Basic installation
cargo add metrics-lib
```

<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

## Error handling and panic guarantees

All core metric types provide non-panicking `try_` variants that return `Result<_, MetricsError>` with explicit validation and overflow checks. Prefer these when inputs may be untrusted or when you want to handle errors explicitly.

- `Counter`: `try_inc`, `try_add`, `try_set`, `try_fetch_add`, `try_inc_and_get` — return `MetricsError::Overflow` on arithmetic overflow.
- `Gauge`: `try_set`, `try_add`, `try_sub`, `try_set_max`, `try_set_min` — return `MetricsError::InvalidValue { reason }` for non-finite values and `MetricsError::Overflow` if math overflows.
- `Timer`: `try_record_ns`, `try_record`, `try_record_batch` — overflow-checked on internal counters.
- `RateMeter`: `try_tick`, `try_tick_n`, `try_tick_if_under_limit` — overflow-checked; `try_tick_if_under_limit` returns `Ok(bool)` indicating admission; may return `MetricsError::OverLimit` for strict policies where applicable.

Panic guidelines:
- The non-`try_` methods prioritize ultra-low latency and assume valid inputs. They generally do not panic but may saturate or accept values without validation.
- Use `try_` methods for correctness-critical paths, external inputs, or when building safety-critical systems.

Example:

```rust
use metrics_lib::{init, metrics, MetricsError};
init();

let g = metrics().gauge("cpu_pct");
g.try_set(87.3)?; // Result<(), MetricsError>

let r = metrics().rate("api");
let ok = r.try_tick_if_under_limit(1000.0)?; // Result<bool, MetricsError>
if ok { /* proceed */ }
```


## Quick Start

```rust
use metrics_lib::{init, metrics};
 
fn main() {
    // Initialize once at startup
    init();
 
    // Counter (ultra-fast)
    metrics().counter("requests").inc();
 
    // Gauge (atomic f64)
    metrics().gauge("cpu_usage_pct").set(87.3);
 
    // Timer (nanosecond precision)
    let t = metrics().timer("db_query").start();
    // ... do work ...
    t.stop();
 
    // Or time a closure and return its result
    let user = metrics().time("fetch_user", || {
        // ... expensive work ...
        42
    });
    assert_eq!(user, 42);
}
```

<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>


## Public APIs

### Global initialization

- `init() -> &'static MetricsCore`
  - Initializes the global metrics singleton (`METRICS`). Safe to call multiple times; first call wins.
- `metrics() -> &'static MetricsCore`
  - Returns the global `MetricsCore`. Panics if `init()` has not been called.
- `static METRICS: OnceLock<MetricsCore>`
  - Exposed for advanced embeddings. Prefer `init()`/`metrics()` for normal use.

Example:
```rust
use metrics_lib::{init, metrics};
 
fn startup() {
    init();
    metrics().counter("boot").inc();
}
```

<br>

### `MetricsCore`

Source: `src/lib.rs` (`MetricsCore`)

- `MetricsCore::new() -> Self`
- `counter(name: &'static str) -> Arc<Counter>`
- `gauge(name: &'static str) -> Arc<Gauge>`
- `timer(name: &'static str) -> Arc<Timer>`
- `rate(name: &'static str) -> Arc<RateMeter>`
- `time<T>(name: &'static str, f: impl FnOnce() -> T) -> T`
- `system() -> &SystemHealth`
- `registry() -> &Registry`

Patterns:
```rust
let c = metrics().counter("requests");
c.inc();
c.add(5);
 
let g = metrics().gauge("temp_c");
g.set(21.5);
 
// Measure work
metrics().time("render", || render_frame());
```

<br>

### `Registry`

Source: `src/registry.rs`

- `Registry::new() -> Self`
- `get_or_create_counter(name: &str) -> Arc<Counter>`
- `get_or_create_gauge(name: &str) -> Arc<Gauge>`
- `get_or_create_timer(name: &str) -> Arc<Timer>`
- `get_or_create_rate_meter(name: &str) -> Arc<RateMeter>`
- `counter_names() -> Vec<String>`
- `gauge_names() -> Vec<String>`
- `timer_names() -> Vec<String>`
- `rate_meter_names() -> Vec<String>`
- `metric_count() -> usize`
- `clear()`

Example:
```rust
use metrics_lib::{init, metrics};
 
init();
let reg = metrics().registry();
let qps = reg.get_or_create_rate_meter("qps");
qps.tick();
 
assert!(metrics().registry().metric_count() >= 1);
```

<br>

### `Counter`

Source: `src/counter.rs`

Structs:
- `Counter` (cache-line aligned)
- `CounterStats { value: u64, age: Duration, rate_per_second: f64, total: u64 }`

Core methods (ultra-fast, lock-free):
- `Counter::new()`, `Counter::with_value(initial: u64)`
- `inc()`, `add(amount: u64)`
- `get() -> u64`, `is_zero() -> bool`, `age() -> Duration`, `rate_per_second() -> f64`
- `reset()`, `set(value: u64)`, `compare_and_swap(expected, new) -> Result<u64,u64>`
- `fetch_add(amount) -> u64`, `add_and_get(amount) -> u64`, `inc_and_get() -> u64`
- `saturating_add(amount)`
- `batch_inc(count: usize)`, `inc_if(condition: bool)`, `inc_max(max_value: u64) -> bool`
- `stats() -> CounterStats`

Example:
```rust
use metrics_lib::{init, metrics};
init();
let c = metrics().counter("jobs_processed");
c.inc();
c.add(10);
 
// Rate since start
let rps = c.rate_per_second();
let s = c.stats();
println!("jobs={}, rps={:.1}", s.value, s.rate_per_second);
```

<br>

### `Gauge`

Source: `src/gauge.rs`

Structs:
- `Gauge` (atomic f64)
- `GaugeStats { value: f64, age: Duration, updates: Option<u64> }`

Common methods:
- `Gauge::new()`, `Gauge::with_value(initial: f64)`
- `set(v: f64)`, `get() -> f64`
- Arithmetic updates: `add(v)`, `sub(v)`
- Min/Max: `set_max(v)`, `set_min(v)`
- Math utilities: `multiply(factor)`, `divide(divisor)`, `abs()`, `clamp(min, max)`
- EMA: `update_ema(sample, alpha)`
- Introspection: `is_zero()`, `is_positive()`, `is_negative()`, `is_finite()`, `age()`
- CAS: `compare_and_swap(expected, new) -> Result<f64, f64>`
- Stats: `stats() -> GaugeStats`

Example:
```rust
use metrics_lib::{init, metrics};
init();
let cpu = metrics().gauge("cpu_pct");
cpu.set(12.0);
cpu.add(2.5);
println!("cpu now: {}%", cpu.get());
```

Specialized gauges (re-exported as `gauge_specialized`):
- `PercentageGauge`, `MemoryGauge`, etc. See `gauge::specialized` for details.

<br>

### `Timer`

Source: `src/timer.rs`

Concepts:
- `Timer`: records durations with nanosecond precision.
- `RunningTimer`: RAII guard from `start()`; call `stop()` to record.

Common methods:
- `Timer::new()`
- `start() -> RunningTimer`
- `record(duration: Duration)`
- `record_ns(ns: u64)` — fastest manual record path
- `record_batch(durations: &[Duration])`
- `count() -> u64`, `total() -> Duration`, `min() -> Duration`, `max() -> Duration`, `average() -> Duration`
- `stats() -> TimerStats { count, total, average, min, max, age, rate_per_second }`
- Helpers: macro/utility functions for timing blocks and functions (see source).

Example:
```rust
use metrics_lib::{init, metrics};
use std::time::Duration;
 
init();
let t = metrics().timer("encode");
{
    let run = t.start();
    // ... do work ...
    run.stop();
}
// Manual recording
t.record(Duration::from_millis(3));
let s = t.stats();
println!("samples: {} avg: {:?}", s.count, s.average);
```

<br>

### `RateMeter`

Source: `src/rate_meter.rs`

Concepts:
- Sliding-window rate calculations (events/sec, minute, hour)
- Optional lightweight rate-limiting helpers

Common methods:
- `RateMeter::new()`
- `tick()` — record an event
- `tick_n(n: u32)` — record multiple events
- `rate() -> f64` — recent events/second (alias: `rate_per_second()`)
- `rate_per_minute() -> f64`, `rate_per_hour() -> f64`
- `total() -> u64`, `reset()`
- `stats() -> RateStats { total_events, per_second, per_minute, per_hour, average_rate, age, window_fill }`

Example:
```rust
use metrics_lib::{init, metrics};
init();
let r = metrics().rate("api_calls");
for _ in 0..100 { r.tick(); }
println!("rate/sec: {:.1}", r.rate());
```

Specialized meters (re-exported as `rate_meter_specialized`):
- `ApiRateLimiter`, `ThroughputMeter`, etc. See `rate_meter::specialized`.

<br>

### `SystemHealth`

Source: `src/system_health.rs`

Highlights:
- CPU and memory usage (process/system)
- Load average, threads, file descriptors, health score

Key methods (see `src/system_health.rs` for full details):
- `cpu_used() -> f64`, `cpu_free() -> f64`
- `mem_used_mb() -> f64`, `mem_used_gb() -> f64`
- `process_cpu_used() -> f64`, `process_mem_used_mb() -> f64`
- `load_avg() -> f64`
- `thread_count() -> u32`, `fd_count() -> u32`
- `health_score() -> f64`, `quick_check() -> HealthStatus`
- `update()` (force refresh), `snapshot() -> SystemSnapshot`, `process() -> ProcessStats`

Example:
```rust
use metrics_lib::{init, metrics};
init();
let sys = metrics().system();
println!(
    "cpu={:.1}% mem_mb={:.1}",
    sys.cpu_used(),
    sys.mem_used_mb()
);
```

<br>

### Async support

Source: `src/async_support.rs`

- `AsyncTimerGuard` — RAII timing for async blocks
- `AsyncTimerExt` — extension trait providing `start_async()` and `time_async()`
- `TimedFuture` — `Future` wrapper returned by `time_async()`
- `AsyncMetricBatch` — batch metric updates with `counter_inc`, `gauge_set`, `timer_record`, `rate_tick`, `flush(&MetricsCore)`

Example (Tokio):
```rust
use metrics_lib::{init, metrics, AsyncTimerExt, AsyncMetricBatch};

#[tokio::main]
async fn main() {
    init();

    // Time an async operation and get its result
    let timer = metrics().timer("async_task");
    let result: i32 = timer
        .time_async(|| async {
            // ... async work ...
            42
        })
        .await;
    assert_eq!(result, 42);

    // RAII guard form
    {
        let _guard = timer.start_async();
        // ... async work interleaved ...
        // recorded on drop
    }

    // Batch updates (flush is synchronous and takes &MetricsCore)
    let mut batch = AsyncMetricBatch::new();
    batch.counter_inc("jobs_done", 1);
    batch.gauge_set("queue_depth", 3.0);
    batch.timer_record("async_task", 500_000); // ns
    batch.rate_tick("qps");
    batch.flush(metrics());
}
```

<br>

### Adaptive controls

Source: `src/adaptive.rs`

- `SamplingStrategy`
  - `Fixed { rate: u32 }`
  - `Dynamic { min_rate, max_rate, target_throughput }`
  - `TimeBased { min_interval: u64 /* ns */ }`
- `AdaptiveSampler::new(strategy)`; `should_sample() -> bool`; `current_rate() -> u32`; `stats()`
- `MetricCircuitBreaker` with `CircuitBreakerConfig { failure_threshold, success_threshold, timeout, half_open_max_calls }`
  - `is_allowed() -> bool`, `record_success()`, `record_failure()`
- `BackpressureController` (re-exported): utilities to reduce work under load

Example (sampling):
```rust
use metrics_lib::{AdaptiveSampler, SamplingStrategy};
 
let sampler = AdaptiveSampler::new(SamplingStrategy::Dynamic {
    min_rate: 1,
    max_rate: 1024,
    target_throughput: 10_000,
});
if sampler.should_sample() {
    // record detailed metrics/logging
}
```

Example (circuit breaker):
```rust
use metrics_lib::{AdaptiveSampler, MetricCircuitBreaker};
use metrics_lib::adaptive::CircuitBreakerConfig;
 
let cb = MetricCircuitBreaker::new(CircuitBreakerConfig { ..Default::default() });
if cb.is_allowed() {
    // perform work and then report result
    cb.record_success();
} else {
    // shed load
}
```

<br>

### Prelude

Import the most common items ergonomically:

```rust
use metrics_lib::prelude::*;
 
fn main() {
    init();
    metrics().counter("ready").inc();
}
```

<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

## Deployment Patterns

This section documents proven deployment approaches for using `metrics-lib` in production systems at scale.

### 1. Initialization Patterns

```rust
// Where to initialize in different app types (Tokio web service example)
use metrics_lib::{init_with_config, Config};

#[tokio::main]
async fn main() {
    // Initialize BEFORE spawning workers or background tasks
    init_with_config(Config {
        max_metrics: 10_000,
        enable_system_metrics: true,
        ..Default::default()
    });

    // Now safe to use across all threads/tasks
    // build_server().await;
}
```

Other patterns:
- CLI/tools: call `init()`/`init_with_config()` at the very start of `main()`.
- Libraries: accept `&MetricsCore` explicitly or rely on the global via `metrics()` when appropriate.
- Tests/benches: initialize once per process; subsequent calls are no-ops.

### 2. High-Volume Strategies

```rust
// Strategy 1: Adaptive Sampling (reduce overhead on hot paths)
use metrics_lib::{metrics, AdaptiveSampler, SamplingStrategy};

let sampler = AdaptiveSampler::new(SamplingStrategy::Dynamic {
    min_rate: 1,
    max_rate: 1024,
    target_throughput: 1_000_000, // target ~1M ops/sec
});

if sampler.should_sample() {
    metrics().timer("hot_path").record_ns(250); // fast-path manual ns record
}
```

```rust
// Strategy 2: Batch Collection (amortize costs under bursty load)
use metrics_lib::{metrics, AsyncMetricBatch};

let mut batch = AsyncMetricBatch::new();
batch.counter_inc("requests", 1);
batch.gauge_set("cpu", 82.4);
batch.timer_record("db", 120_000); // ns
batch.rate_tick("qps");
batch.flush(metrics()); // single synchronized flush
```

```rust
// Strategy 3: Thread-Local Aggregation (application-level)
// Aggregate counts locally and flush periodically to reduce contention
thread_local! {
    static LOCAL_COUNT: std::cell::Cell<u64> = std::cell::Cell::new(0);
}

fn on_event() {
    LOCAL_COUNT.with(|c| c.set(c.get() + 1));
}

fn flush_local() {
    let count = LOCAL_COUNT.with(|c| { let v = c.get(); c.set(0); v });
    if count > 0 {
        metrics_lib::metrics().counter("events").add(count);
    }
}
```

Guidelines:
- Prefer `record_ns`/`batch_inc`/`flush` in the hottest paths.
- Sample or downsample high-cardinality metrics.
- Avoid per-op string formatting or allocation; use `&'static str` names.

### 3. Memory Management

- Bounded vs. unbounded: limit `max_metrics` via `Config` for controlled memory use.
- Name cardinality: avoid embedding unbounded values (IDs, UUIDs) in metric names.
- Recycling: reuse metric instances via the `Registry`; avoid creating/dropping in tight loops.
- Cleanup: if dynamic names are required, provide explicit cleanup points (e.g., `Registry::clear()` in test lifecycles).
- Alignment: metrics are 64-byte cache-line aligned; avoid creating excessive distinct metrics to keep cache footprint small.

### 4. Multi-Service Patterns

- Naming: use service prefixes like `"auth.requests"`, `"billing.latency"`, `"api.v2.error_rate"`.
- Correlation: align metric names/labels with tracing spans or request IDs (in structured logs), not in the metric name itself.
- Boundaries: maintain separate registries per service when embedding `metrics-lib` inside multi-tenant binaries.
- Aggregation: push metrics to a single exporter/collector at service boundaries; keep in-process metrics lock-free and fast.

### 5. Export and Ingestion

`metrics-lib` focuses on ultra-fast in-process metrics. For exporting, consider bridging to your observability stack:

- Push gateway: periodically snapshot internal counters/gauges and send to an external collector.
- File/pipe sink: write snapshots to a file or stdout for sidecar ingestion.
- Structured logs: emit key metrics in JSON logs for log-based analytics.

Example (periodic snapshot skeleton):
```rust
use std::time::Duration;
use tokio::time::interval;
use metrics_lib::metrics;

#[tokio::main]
async fn main() {
    metrics_lib::init();

    let mut tick = interval(Duration::from_secs(10));
    loop {
        tick.tick().await;
        // Example: read values atomically and ship to a gateway
        let requests = metrics().counter("requests").get();
        let error_rate = metrics().rate("errors").rate();
        // send_to_gateway(requests, error_rate).await?;
    }
}
```

Guidelines:
- Keep export paths off the hot path; use async tasks and backpressure-aware queues.
- Bound queue sizes; drop or sample on overload to protect the application.
- Prefer binary formats for high throughput (CBOR, protobuf) when applicable.

### 6. On-Call Diagnostics

Enable targeted, temporary metrics during incidents without long-term overhead:

- Compile-time flags: feature-gate diagnostic code.
- Runtime toggles: environment variables or admin endpoints enable additional metrics.

Examples:
```rust
// Compile-time gate (Cargo feature)
#[cfg(feature = "diagnostics")]
pub fn diag_tick() {
    metrics_lib::metrics().counter("diag.slow_path").inc();
}
```

```rust
// Runtime gate via env var
if std::env::var("METRICS_DIAG").as_deref() == Ok("1") {
    metrics_lib::metrics().gauge("diag.queue_depth").set(42.0);
}
```

Guidelines:
- Ensure diagnostic code is zero-overhead when disabled (compile-time or fast runtime checks).
- Use stable, prefixed names (e.g., `diag.*`) and document cleanup/removal plans.

### 7. Feature Gating Strategies

Use Cargo features to tailor performance and binary size to environments:

- `default` minimal footprint; enable heavier components only where needed.
- `async`: include async helpers only when an async runtime is used.
- `bench-tests`: keep benchmark-style tests out of default CI runs to avoid flakiness.

Cargo.toml example:
```toml
[features]
# Keep defaults minimal for production
default = []
async = []
bench-tests = []
all = ["async"]
```

CI best practices:
- Run unit tests with default features for consumer parity.
- Run all-features in a separate job when validating optional integrations.
- Keep benchmark-style tests gated behind `--features bench-tests -- --ignored`.



<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

<h2 id="real-world-examples">Real-World Examples</h2>

<br>
<h3 id="real-world-high-frequency-trading">High-Frequency Trading (HFT)</h3>

Constraints: sub-microsecond hot paths, no allocations, no locks, bounded cardinality.

Key patterns:
- Pre-register metric handles at startup.
- Use counters/gauges inline; export asynchronously off the hot path.
- Avoid per-symbol labels in names; sample or aggregate in fixed windows.

```rust
use metrics_lib::{metrics, Timer};

// Pre-register at init
pub fn init_metrics() {
    let m = metrics();
    m.counter("orders_submitted");
    m.counter("orders_rejected");
    m.timer("match_latency_ns");
    m.gauge("orderbook_depth");
}

#[inline(always)]
pub fn on_match(orderbook_depth: u64) {
    // Minimal work: record, no allocations
    let _t = metrics().timer("match_latency_ns").start();
    // ... matching logic ...
    metrics().gauge("orderbook_depth").set(orderbook_depth as f64);
}

#[inline(always)]
pub fn submit_ok() { metrics().counter("orders_submitted").inc(); }
#[inline(always)]
pub fn submit_reject() { metrics().counter("orders_rejected").inc(); }
```

Guidance:
- Keep metrics names stable; do not embed symbol/account IDs.
- If symbol-level insight is required, sample 1/N events and export summaries via background task.
- Prefer histogram buckets sized for nanosecond ranges if using histograms.


<br>
<h3 id="real-world-web-service-under-load">Web Service Under Load</h3>

Track throughput, error rate, and tail latency. Use recording rules to reduce dashboard cost.

```rust
use metrics_lib::metrics;

pub async fn handle_request() -> Result<&'static str, anyhow::Error> {
    let _t = metrics().timer("http_request_duration_s").start();
    metrics().counter("http_requests_total").inc();
    // ... work ...
    Ok("ok")
}

pub fn on_error() {
    metrics().counter("http_errors_total").inc();
}
```

Prometheus queries:
- Rate: `sum(rate(http_requests_total[5m]))` per job/route (avoid high-cardinality routes; use normalized labels or grouping).
- Error ratio: `sum(rate(http_errors_total[5m])) / sum(rate(http_requests_total[5m]))`.
- p95: `histogram_quantile(0.95, sum(rate(http_request_duration_s_bucket[5m])) by (le))` if using histogram form.


<br>
<h3 id="real-world-batch-processing-pipeline">Batch Processing Pipeline</h3>

Measure per-batch latency, items processed, and failures. Emit gauges for backlogs.

```rust
use metrics_lib::metrics;

pub fn process_batch(batch_size: usize) {
    let _t = metrics().timer("batch_duration_s").start();
    // ... process ...
    metrics().counter("batch_processed_items_total").add(batch_size as u64);
}

pub fn record_failure() { metrics().counter("batch_failures_total").inc(); }
pub fn backlog_set(count: usize) { metrics().gauge("queue_backlog").set(count as f64); }
```

Grafana tips:
- Use dual-axis panel for `rate(batch_processed_items_total[5m])` and backlog gauge.
- Alert if backlog grows while throughput drops.



<br>
<h3 id="real-world-token-bucket-rate-limiter">Token Bucket Rate Limiter</h3>

Use `RateMeter` for observed rate and gauges for bucket level; timers for wait time.

```rust
use metrics_lib::{metrics, RateMeter};

pub struct Limiter {
    meter: RateMeter,
    capacity: u64,
    tokens: u64,
}

impl Limiter {
    pub fn allow(&mut self) -> bool {
        self.meter.tick();
        if self.tokens > 0 { self.tokens -= 1; true } else { false }
    }
    pub fn report(&self) {
        metrics().gauge("ratelimit_tokens").set(self.tokens as f64);
        metrics().gauge("ratelimit_capacity").set(self.capacity as f64);
    }
}
```


<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

## Integration Examples

This section shows how to integrate `metrics-lib` with common stacks. These examples are illustrative and may require adapting types to your application framework.

<h3 id="web-framework-integration">1. Web Framework Integration (Axum middleware)</h3>

```rust
use axum::{http::Request, middleware::Next, response::Response};
use metrics_lib::metrics;

pub async fn metrics_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let path = req.uri().path();
    let timer = metrics().timer("http.request").start();

    let response = next.run(req).await;

    // Request/Status counters
    metrics().counter("http.requests").inc();
    metrics()
        .counter(match response.status().as_u16() {
            200..=299 => "http.status.2xx",
            300..=399 => "http.status.3xx",
            400..=499 => "http.status.4xx",
            500..=599 => "http.status.5xx",
            _ => "http.status.other",
        })
        .inc();

    // Optional: per-path timer (beware cardinality)
    metrics().timer(&format!("http.request.{}", path)).record(timer.elapsed());

    response
}
```

Guidance:
- Prefer a small, bounded set of status counters over per-path status metrics.
- Use per-path timers sparingly to avoid high-cardinality names.

<br>
<h3 id="database-pool-monitoring">2. Database Pool Monitoring</h3>

```rust
use metrics_lib::metrics;

pub struct ConnectionPool {
    inner: deadpool_postgres::Pool, // example; adapt to your pool type
}

impl ConnectionPool {
    pub async fn get(&self) -> deadpool_postgres::Client {
        let _wait = metrics().timer("db.pool.wait").start();
        metrics().gauge("db.pool.active").add(1.0);

        let client = self.inner.get().await.expect("db conn");

        // Update gauges after acquiring (adjust per pool’s API)
        metrics().gauge("db.pool.idle").set(self.idle_count() as f64);
        client
    }

    fn idle_count(&self) -> usize {
        // Implement based on your pool’s introspection
        0
    }
}
```

Guidance:
- Keep `db.pool.*` names stable. Prefer gauges for current levels and timers for waits.
- Consider periodic snapshots for totals (e.g., acquired/failed).

<br>
<h3 id="background-job-processing">3. Background Job Processing</h3>

```rust
use metrics_lib::metrics;

pub struct Job { pub kind: &'static str }

pub async fn process_job(job: Job) {
    let _guard = metrics().timer(&format!("job.{}.duration", job.kind)).start();

    match execute_job(job).await {
        Ok(_) => metrics().counter("jobs.success").inc(),
        Err(_) => {
            metrics().counter("jobs.failed").inc();
            // Optional: trip a circuit breaker based on failures
            // my_breaker.record_failure();
        }
    }
}

async fn execute_job(_job: Job) -> Result<(), ()> {
    Ok(())
}
```

Guidance:
- Name metrics by job-kind for aggregate SLOs; avoid embedding unbounded IDs in metric names.
- Add a rate meter (e.g., `jobs.rate`) in the worker loop if you need throughput.

<br>
<h3 id="observability-stack-integration">4. Observability Stack Integration (metrics endpoint)</h3>

```rust
use metrics_lib::metrics;
use std::fmt::Write;

/// Expose a simple text endpoint for scraping
pub async fn metrics_endpoint() -> String {
    // Placeholder snapshot API; adapt to your registry access
    let reg = metrics().registry();

    let mut output = String::new();
    // Example formatting; adapt to Prometheus/OpenMetrics as needed
    for name in reg.counter_names() {
        let v = metrics().counter(Box::leak(name.into_boxed_str())).get();
        let _ = writeln!(output, "# TYPE {} counter", name);
        let _ = writeln!(output, "{} {}", name, v);
    }
    output
}
```

Guidance:
- For Prometheus, prefer an OpenMetrics-compliant format and stable names.
- Keep export off the hot path; run in a separate async task.

<br>
<h3 id="correlation-with-tracing">5. Correlation with Tracing</h3>

```rust
use metrics_lib::metrics;
use std::time::Instant;

async fn do_work() {}

async fn traced_operation() {
    // Example using an external tracing system; pseudocode span
    // let span = tracing::span!(Level::INFO, "op");
    // let _enter = span.enter();

    let start = Instant::now();
    do_work().await;
    let dur = start.elapsed();

    metrics().timer("operation").record(dur);
    // span.record("timer.duration_ms", dur.as_millis() as i64);
}
```

Guidance:
- Use the same operation names between metrics and spans for easy join in dashboards.
- Record high-level spans and add targeted timers for critical sections.

<br>
<h3 id="grafana-dashboard-setup">6. Grafana Dashboard Setup (via Prometheus)</h3>

High-level steps:

1. Export metrics in a Prometheus/OpenMetrics-compatible format (see "Observability Stack Integration").
2. Configure Prometheus to scrape your service:

```yaml
scrape_configs:
  - job_name: 'metrics-lib-example'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: /metrics
    scrape_interval: 15s
```

3. In Grafana, add Prometheus as a data source and create a dashboard:
   - Panel examples:
     - Rate: `rate(http_requests_total[5m])`
     - Latency: `histogram_quantile(0.95, sum(rate(operation_duration_bucket[5m])) by (le))`
     - In-flight: `db_pool_active`

Tips:
- Keep metric names compliant and low-cardinality.
- Consider per-service prefixes, e.g., `auth_*`, `api_*`.


<br>
<h3 id="message-brokers-throughput">7. Message Brokers (Kafka/NATS) Throughput and Lag</h3>

```rust
use metrics_lib::metrics;

pub struct BrokerConsumer;

impl BrokerConsumer {
    pub async fn on_batch(&self, batch_size: usize, current_lag: u64) {
        // Throughput
        metrics().rate("broker.consume").tick_n(batch_size as u32);
        metrics().counter("broker.messages").add(batch_size as u64);

        // Lag (gauge)
        metrics().gauge("broker.lag").set(current_lag as f64);

        // Batch processing time
        let _t = metrics().timer("broker.batch.duration").start();
        // ... process batch ...
    }
}
```

Guidance:
- Use `rate` for instantaneous throughput and `counter` for cumulative messages.
- For Kafka consumer lag, prefer a gauge fed by the broker/consumer metrics.

<br>
<h3 id="caches-hit-miss-pool-metrics">8. Caches (Redis) Hit/Miss, Pool Metrics, TTL Health</h3>

```rust
use metrics_lib::metrics;

pub async fn cache_get(key: &str) -> Option<Vec<u8>> {
    let _t = metrics().timer("cache.get").start();
    // let result = redis.get(key).await?;
    let result: Option<Vec<u8>> = None;

    match result {
        Some(v) => {
            metrics().counter("cache.hit").inc();
            Some(v)
        }
        None => {
            metrics().counter("cache.miss").inc();
            None
        }
    }
}

pub fn update_pool_metrics(active: usize, idle: usize) {
    metrics().gauge("cache.pool.active").set(active as f64);
    metrics().gauge("cache.pool.idle").set(idle as f64);
}

pub fn ttl_health(sampled_ttl_secs: u64) {
    metrics().gauge("cache.ttl.sample").set(sampled_ttl_secs as f64);
}
```

Guidance:
- Track `hit/miss` counters; derive hit ratio in your dashboard.
- Record pool size as gauges; avoid per-connection metrics.

<br>
<h3 id="serverless-cold-start-and-duration">9. Serverless (AWS Lambda) Cold-Start and Duration</h3>

```rust
use metrics_lib::{init, metrics};
use std::time::Instant;

static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

// Pseudocode handler
pub async fn handler() {
    // Cold start detection: first set of START indicates cold start
    let first = START.set(Instant::now()).is_ok();
    if first {
        metrics().counter("lambda.cold_start").inc();
    }

    let _t = metrics().timer("lambda.invoke.duration").start();
    // ... handle request ...
}
```

Guidance:
- Cold-start counter increments once per fresh runtime.
- Use percentiles on `lambda.invoke.duration` to track tail latency.

<br>
<h3 id="kubernetes-scraping">10. Kubernetes Scraping & Pod-level Dashboards</h3>

Annotate your Deployment/Pod to expose metrics to Prometheus:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: metrics-lib-example
spec:
  replicas: 2
  selector:
    matchLabels: { app: metrics-lib-example }
  template:
    metadata:
      labels: { app: metrics-lib-example }
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/path: "/metrics"
        prometheus.io/port: "8080"
    spec:
      containers:
        - name: app
          image: your-image:tag
          ports:
            - containerPort: 8080
```

Dashboard tips:
- Per-pod panels: select by `pod` label for debugging noisy neighbors.
- SLO panels: aggregate across pods by `deployment`/`job`.

<br>
<h3 id="open-telemetry-export">11. OpenTelemetry Export Bridge (example skeleton)</h3>

```rust
// Bridge metrics-lib snapshot into OpenTelemetry metrics (pseudocode)
use metrics_lib::metrics;

pub async fn export_to_otel() {
    // Access registry (adapt based on your API)
    let reg = metrics().registry();

    // Iterate counters
    for name in reg.counter_names() {
        let total = metrics().counter(Box::leak(name.clone().into_boxed_str())).get();
        // otel_meter.u64_counter(name).add(total, &[]);
    }

    // Gauges, timers, and rates would be mapped similarly using OTLP exporters.
}
```

Guidance:
- Prefer push from a periodic task; avoid exporting on the hot path.
- Use OTLP/gRPC exporters and batch processors for efficiency.

<br>
<h3 id="nats-specific-queue">12. NATS-Specific Queue Depth and Consumers</h3>

```rust
use metrics_lib::metrics;

pub struct NatsStats { pub consumers: u32, pub pending: u64 }

pub fn record_nats_queue(queue: &'static str, stats: NatsStats) {
    // Bounded name patterns per queue
    metrics().gauge(&format!("nats.{}.consumers", queue)).set(stats.consumers as f64);
    metrics().gauge(&format!("nats.{}.pending", queue)).set(stats.pending as f64);
}
```

Guidance:
- Prefer a fixed set of queue names; avoid dynamic/tenant IDs in metric names.
- For shard/partition details, use separate prefixed metrics rather than labels in names.


<br>
<h3 id="redis-latency-histogram">13. Redis Latency Histogram and Dashboard Queries</h3>

```rust
use metrics_lib::metrics;
use std::time::Instant;

pub async fn redis_set(key: &str, _val: &[u8]) {
    let start = Instant::now();
    // redis.set(key, val).await?;
    metrics().timer("redis.set").record(start.elapsed());
}

pub async fn redis_get(key: &str) {
    let start = Instant::now();
    // let _ = redis.get::<_, Option<Vec<u8>>>(key).await?;
    metrics().timer("redis.get").record(start.elapsed());
}
```

Grafana query tips (Prometheus examples):
- Hit ratio: `sum(rate(cache_hit[5m])) / (sum(rate(cache_hit[5m])) + sum(rate(cache_miss[5m])))`
- P95 get latency: `histogram_quantile(0.95, sum(rate(redis_get_duration_bucket[5m])) by (le))`


<br>
<h3 id="aws-lambda-emf">14. AWS Lambda EMF (Embedded Metric Format) Emission</h3>

```rust
// Emit selected metrics as EMF JSON to stdout for CloudWatch ingestion (pseudocode)
use metrics_lib::metrics;
use serde_json::json;

pub fn emit_emf() {
    let requests = metrics().counter("requests").get();
    let cold = metrics().counter("lambda.cold_start").get();

    let doc = json!({
        "_aws": {"Timestamp": chrono::Utc::now().timestamp_millis(),
                 "CloudWatchMetrics": [{
            "Namespace": "metrics_lib",
            "Dimensions": [["service"]],
            "Metrics": [
                {"Name": "requests", "Unit": "Count"},
                {"Name": "lambda_cold_start", "Unit": "Count"}
            ]
        }]},
        "service": "example",
        "requests": requests,
        "lambda_cold_start": cold
    });
    println!("{}", doc.to_string());
}
```

Guidance:
- Keep EMF payloads small; emit periodically, not on every invocation.
- Use CloudWatch Logs subscription filters to forward to other sinks if needed.

<br>
<h3 id="kubernetes-helm-values">15. Kubernetes Helm Values (Prometheus Scrape Annotations)</h3>

```yaml
# values.yaml fragment
service:
  port: 8080

podAnnotations:
  prometheus.io/scrape: "true"
  prometheus.io/path: "/metrics"
  prometheus.io/port: "{{ .Values.service.port }}"
```

```yaml
# deployment.yaml fragment
metadata:
  annotations:
    {{- toYaml .Values.podAnnotations | nindent 4 }}
```

Guidance:
- Centralize scrape annotations in `values.yaml` to keep templates clean.
- Prefer ServiceMonitors if using the Prometheus Operator.

<br>
<h3 id="otlp-exporter">16. Full OTLP Exporter Skeleton (tonic)</h3>

```rust
// Pseudocode: batch export counters/gauges to an OTLP collector via tonic
use metrics_lib::metrics;
// use opentelemetry_proto::collector::metrics::v1::metrics_service_client::MetricsServiceClient;
// use opentelemetry_proto::metrics::v1::*;

pub async fn export_otlp(_endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = MetricsServiceClient::connect(endpoint.to_string()).await?;
    let reg = metrics().registry();

    // Build ResourceMetrics/ScopeMetrics/Metric structures here from registry
    // let request = ExportMetricsServiceRequest { resource_metrics: vec![ ... ] };
    // client.export(request).await?;
    Ok(())
}
```

Guidance:
- Use a background task and a bounded channel to batch and send metrics.
- Prefer gzip compression and delta temporality where supported for efficiency.

<br>
<h3 id="grafana-dashboard-setup">17. Grafana Panels (Ready-to-Copy JSON)</h3>

These minimal panels assume Prometheus as datasource with the name `Prometheus`. Adjust `datasource` UID/name as needed.

Rate panel (requests per second):

```json
{
  "type": "timeseries",
  "title": "HTTP Requests/s",
  "datasource": { "type": "prometheus", "uid": "Prometheus" },
  "targets": [
    { "expr": "rate(http_requests_total[5m])", "legendFormat": "req/s" }
  ],
  "fieldConfig": { "defaults": { "unit": "req/s" }, "overrides": [] }
}
```

Latency panel (P95 from histogram):

```json
{
  "type": "timeseries",
  "title": "p95 Operation Duration",
  "datasource": { "type": "prometheus", "uid": "Prometheus" },
  "targets": [
    { "expr": "histogram_quantile(0.95, sum(rate(operation_duration_bucket[5m])) by (le))", "legendFormat": "p95" }
  ],
  "fieldConfig": { "defaults": { "unit": "s" }, "overrides": [] }
}
```

Gauge panel (queue depth):

```json
{
  "type": "gauge",
  "title": "Queue Depth",
  "datasource": { "type": "prometheus", "uid": "Prometheus" },
  "targets": [
    { "expr": "nats_myqueue_pending" }
  ],
  "fieldConfig": { "defaults": { "unit": "none" }, "overrides": [] }
}
```

Tip: To embed into an existing dashboard JSON, copy each object into the dashboard `panels` array and position/size them via `gridPos`.

<br>
<h3 id="prometheus-operator-servicemonitor">18. Prometheus Operator ServiceMonitor</h3>

If your cluster uses the Prometheus Operator, define a `ServiceMonitor` instead of raw scrape annotations.

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: metrics-lib-example
  labels:
    release: prometheus  # matches your Prometheus helm release selector
spec:
  selector:
    matchLabels:
      app: metrics-lib-example
  namespaceSelector:
    matchNames: ["default"]
  endpoints:
    - port: http
      path: /metrics
      interval: 15s
```

Example Service to pair with it:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: metrics-lib-example
  labels:
    app: metrics-lib-example
spec:
  selector:
    app: metrics-lib-example
  ports:
    - name: http
      port: 8080
      targetPort: 8080
```


<br>
<h3 id="full-grafana-dashboard">19. Full Grafana Dashboard (Ready-to-Import JSON)</h3>

This compact dashboard includes three panels (Requests/s, p95 latency, Queue depth). Replace the datasource `uid` as needed.

```json
{
  "title": "metrics-lib Example",
  "schemaVersion": 39,
  "panels": [
    {
      "type": "timeseries",
      "title": "HTTP Requests/s",
      "datasource": { "type": "prometheus", "uid": "Prometheus" },
      "targets": [{ "expr": "rate(http_requests_total[5m])", "legendFormat": "req/s" }],
      "fieldConfig": { "defaults": { "unit": "req/s" }, "overrides": [] },
      "gridPos": { "h": 8, "w": 12, "x": 0, "y": 0 }
    },
    {
      "type": "timeseries",
      "title": "p95 Operation Duration",
      "datasource": { "type": "prometheus", "uid": "Prometheus" },
      "targets": [{ "expr": "histogram_quantile(0.95, sum(rate(operation_duration_bucket[5m])) by (le))", "legendFormat": "p95" }],
      "fieldConfig": { "defaults": { "unit": "s" }, "overrides": [] },
      "gridPos": { "h": 8, "w": 12, "x": 12, "y": 0 }
    },
    {
      "type": "gauge",
      "title": "Queue Depth",
      "datasource": { "type": "prometheus", "uid": "Prometheus" },
      "targets": [{ "expr": "nats_myqueue_pending" }],
      "fieldConfig": { "defaults": { "unit": "none" }, "overrides": [] },
      "gridPos": { "h": 8, "w": 6, "x": 0, "y": 8 }
    }
  ],
  "time": { "from": "now-6h", "to": "now" },
  "refresh": "30s"
}
```


<br>
<h3 id="prometheus-recording-rules">20. Prometheus Recording Rules (Latency and Rates)</h3>

Reduce query cost by materializing common expressions.

```yaml
groups:
  - name: metrics-lib.rules
    interval: 30s
    rules:
      - record: job:http_requests:rate5m
        expr: sum by (job) (rate(http_requests_total[5m]))
      - record: job:operation_duration:p95_5m
        expr: |
          histogram_quantile(0.95,
            sum by (job, le) (rate(operation_duration_bucket[5m]))
          )
      - record: job:broker_consume:rate5m
        expr: sum by (job) (rate(broker_messages_total[5m]))
```

<br>
<h3 id="prometheus-operator-servicemonitor">21. Prometheus Operator ServiceMonitor (Secured Endpoint)</h3>

For TLS/bearer-protected endpoints. Assumes a secret containing `token` and a CA bundle.

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: metrics-lib-example-secured
  labels:
    release: prometheus
spec:
  selector:
    matchLabels:
      app: metrics-lib-example
  namespaceSelector:
    matchNames: ["default"]
  endpoints:
    - port: https
      path: /metrics
      interval: 15s
      scheme: https
      tlsConfig:
        ca:
          secret:
            name: metrics-ca
            key: ca.crt
        insecureSkipVerify: false
      bearerTokenSecret:
        name: metrics-bearer
        key: token
```


<br>
<h3 id="helm-snippets">22. Helm Snippets (kube-prometheus-stack and App Chart)</h3>

- kube-prometheus-stack values: `docs/k8s/helm/kube-prometheus-stack-values.yaml`
  - Includes `additionalServiceMonitors` and `additionalPrometheusRulesMap` for a quick drop-in.
  - Apply:
    - `helm repo add prometheus-community https://prometheus-community.github.io/helm-charts`
    - `helm repo update`
    - `helm upgrade --install monitoring prometheus-community/kube-prometheus-stack -f docs/k8s/helm/kube-prometheus-stack-values.yaml`

- Example application Helm chart templates:
  - Values: `docs/k8s/helm/app-chart/values.yaml`
  - Templates: `docs/k8s/helm/app-chart/templates/servicemonitor.yaml`, `prometheusrule.yaml`
  - Enable via values:
    - `.Values.metrics.serviceMonitor.enabled: true`
    - `.Values.metrics.rules.enabled: true`


<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>

## Notes

- All hot-path operations are lock-free and allocation-free where possible.
- For best latency, prefer batching (`Counter::batch_inc`, `AsyncMetricBatch`) in bursty workloads.
- Avoid calling `metrics()` before `init()`. In library code, consider taking `&MetricsCore` explicitly.
- For specialized meters/gauges, see the `specialized` submodules re-exported as `gauge_specialized` and `rate_meter_specialized`.
- Keep limiter metrics sparse; avoid per-user limiters unless cardinality is controlled.
- For multi-tenant systems, expose only tier-level or route-level aggregates.
