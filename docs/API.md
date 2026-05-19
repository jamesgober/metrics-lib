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
        <a href="./GUIDELINES.md" title="Developer Guidelines"><b>GUIDELINES</b></a>
    </sup>
</div>
<br>

<h4 id="example-pointers">Example Pointers</h4>

- Quick Tour: `examples/quick_tour.rs` — counter/gauge/timer/ratemeter/system health in one file.
- Async Batch + Timing: `examples/async_batch_timing.rs` — `AsyncTimerExt::time_async` and `AsyncMetricBatch`.
- Token Bucket Limiter: `examples/token_bucket_limiter.rs` — admission control with `RateMeter::tick_if_under_limit`.
- Custom Exporter (OpenMetrics-like): `examples/custom_exporter_openmetrics.rs` — text snapshot.
- Axum Middleware (minimal): `examples/axum_middleware_metrics.rs` — per-request metrics + lightweight endpoint.
- Contention & Admission: `examples/contention_admission.rs` — multi-threaded admission under target rate.
- Health Dashboard: `examples/health_dashboard.rs` — periodic snapshot of CPU/mem/load/threads/FDS/health.
- Cache Hit/Miss: `examples/cache_hit_miss.rs` — counters for hits/misses, ratio, and lookup latency.
- Broker Throughput: `examples/broker_throughput.rs` — producer/consumer RPS via `RateMeter`.
- CPU Stats Overview: `examples/cpu_stats.rs` — system CPU/load and process CPU sampling windows.
- Memory Stats Overview: `examples/memory_stats.rs` — total/used/free MB/GB and percentages (unit auto-detect).
- Axum Registry Integration: `examples/axum_registry_integration.rs` — minimal web service wiring.
- Streaming Rate Window: `examples/streaming_rate_window.rs` — periodic rate sampling demo.
- Benchmark Comparison: `examples/benchmark_comparison.rs` — microbench comparison runner.
- Quick Start: `examples/quick_start.rs` — shortest end-to-end usage.

<br>

Note: To run many non-blocking examples quickly in sequence, use the helper script:

```bash
bash tools/run_examples.sh
```
You can pass a custom comma-separated list via `EXAMPLES`, e.g.:

```bash
EXAMPLES="quick_start,quick_tour,cpu_stats" bash tools/run_examples.sh
```

## Table of Contents
- **[Installation](#installation)**
- **[Examples](#examples)**
- **[Quick Start](#quick-start)**
- **[Public APIs](#public-apis)**
 - **[API Safety](#api-safety)**
  - [Global initialization](#global-initialization)
  - [`MetricsCore`](#metricscore)
  - [`Registry`](#registry)
  - [`Counter`](#counter)
  - [`Gauge`](#gauge)
  - [`Timer`](#timer)
  - [`RateMeter`](#ratemeter)
  - [`Histogram` (v0.9.3)](#histogram)
  - [`LabelSet` & Labels (v0.9.3)](#labels)
  - [Metric metadata (v0.9.3)](#metric-metadata)
  - [`SystemHealth`](#systemhealth)
  - [`HealthConfig` & `Step` (v0.9.5)](#health-config)
  - [`ScopedRegistry` (v0.9.5)](#scoped-registry)
  - [`TokenBucket` (v0.9.5)](#token-bucket)
  - [Exporters (v0.9.3)](#exporters)
  - [Async support](#async-support)
  - [Adaptive controls](#adaptive-controls)
  - [`tracing` integration (v0.9.5)](#tracing-ext)
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
  - [Building a Custom Exporter](#real-world-custom-exporter)
  - [Memory Stats: total/used/free + percentages](#real-world-memory-stats)
  - [Memory % used for an operation (estimate)](#real-world-memory-percent-operation)
  - [CPU Stats: total/used/free + percentages](#real-world-cpu-stats)
  - [CPU % used for an operation (estimate)](#real-world-cpu-percent-operation)
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
  - [Example Pointers](#example-pointers)
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
metrics-lib = "1.0.0"
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

<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>


## Examples

Run these self-contained examples to see the library in action:

- Quick Start
  - File: `examples/quick_start.rs`
  - Run:
    ```bash
    cargo run --example quick_start --release
    ```

- Streaming Rate Window
  - File: `examples/streaming_rate_window.rs`
  - Run:
    ```bash
    cargo run --example streaming_rate_window --release
    ```

- Axum Registry Integration (minimal web service)
  - File: `examples/axum_registry_integration.rs`
  - Run:
    ```bash
    cargo run --example axum_registry_integration --release
    ```
  - Endpoints:
    - `GET /health` — liveness probe
    - `GET /metrics-demo` — updates metrics (counter/gauge/timer/rate)
    - `GET /export` — returns a JSON snapshot of selected metrics


<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>


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
- `counter(name: &str) -> Arc<Counter>`
- `gauge(name: &str) -> Arc<Gauge>`
- `timer(name: &str) -> Arc<Timer>`
- `rate(name: &str) -> Arc<RateMeter>`
- `time<T>(name: &str, f: impl FnOnce() -> T) -> T`
- `system() -> &SystemHealth`
- `registry() -> &Registry`

**v0.9.2 note:** `name` is now `&str` (was `&'static str`). String literals
still work unchanged; runtime-derived names (per-route, per-tenant, etc.) work
without `Box::leak`. Repeated lookups of the same name return the same `Arc`
and perform no allocation on the hot path; the first registration allocates
a `String` key inside the registry.

Patterns:
```rust
use metrics_lib::{init, metrics};
init();

// Static name (compile-time string literal).
let c = metrics().counter("requests");
c.inc();
c.add(5);

let g = metrics().gauge("temp_c");
g.set(21.5);

// Runtime-derived name (was previously `Box::leak`'d).
let tenant_id = "acme";
let key = format!("requests.tenant.{tenant_id}");
metrics().counter(&key).inc();

// Measure work
metrics().time("render", || { /* render frame */ });
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
  - **v0.9.2:** `add_and_get` and `inc_and_get` now use `wrapping_add` and
    will not panic on overflow in debug builds. The returned value wraps
    modulo `2^64`, matching `AtomicU64::fetch_add` semantics. Use the
    checked variants (`try_inc_and_get`, `try_fetch_add`) when an explicit
    `MetricsError::Overflow` is required.
- `saturating_add(amount)`
  - **v0.9.2:** internally uses `Relaxed compare_exchange_weak` (no more
    `SeqCst`); observable behaviour unchanged.
- `batch_inc(count: usize)`, `inc_if(condition: bool)`, `inc_max(max_value: u64) -> bool`
  - **v0.9.2:** `inc_max` switched to `Relaxed` CAS for the same
    performance improvement.
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
  - **v0.9.2:** batch totals are summed with `saturating_add` instead of
    `+=`. Adversarial inputs that would have panicked in debug builds now
    saturate at `u64::MAX` nanoseconds without panicking. The `try_record_batch`
    checked variant continues to return `MetricsError::Overflow` instead.
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
- Tumbling-window rate calculations (events/sec, minute, hour)
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

<h3 id="histogram"><code>Histogram</code></h3>

Source: `src/histogram.rs` — requires the `histogram` Cargo feature.

A bucketed observation type compatible with Prometheus / OpenMetrics histogram
semantics. Each bucket counts observations with value `<= upper_bound`;
exports render the buckets in cumulative form. The implicit `+Inf` bucket
always equals the total observation count. `sum` and `count` are tracked
separately for `_sum` / `_count` companion series.

Construction:
- `Histogram::with_buckets(bounds: impl IntoIterator<Item = f64>)` — explicit upper bounds.
- `Histogram::linear(start, width, count)` — `start, start+width, …, start+(count-1)*width`.
- `Histogram::exponential(start, factor, count)` — `start, start*factor, …`.
- `Histogram::default_seconds()` — the Prometheus default latency-seconds buckets
  (`[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]`).
- `DEFAULT_SECONDS_BUCKETS: &[f64]` — re-exported constant for the same buckets.

Observation:
- `observe(value: f64)` — non-finite values are silently dropped.
- `try_observe(value: f64) -> Result<()>` — returns
  `Err(MetricsError::InvalidValue)` for NaN / ±∞.

Read accessors:
- `count() -> u64`, `sum() -> f64`, `mean() -> f64`, `age() -> Duration`.
- `quantile(q: f64) -> f64` — clamped to `0.0..=1.0`; bucket-interpolated
  estimate, returns `0.0` on empty.
- `snapshot() -> HistogramSnapshot { buckets: Vec<HistogramBucket>, sum, count, age }` —
  buckets rendered cumulatively, trailing `+Inf` bucket appended.
- `reset()` — clears all bucket counters and sum/count.

Example:
```rust
# #[cfg(feature = "histogram")]
# {
use metrics_lib::Histogram;

let h = Histogram::with_buckets([0.01, 0.05, 0.1, 0.5, 1.0]);
h.observe(0.005);
h.observe(0.08);
h.observe(0.42);
h.observe(2.0); // +Inf bucket

assert_eq!(h.count(), 4);
assert!(h.quantile(0.5) > 0.0);
let snap = h.snapshot();
assert_eq!(snap.buckets.last().unwrap().upper_bound, f64::INFINITY);
# }
```

Registry integration:
```rust
# #[cfg(feature = "histogram")]
# {
use metrics_lib::{init, metrics, LabelSet};

init();

// Optional: pre-configure buckets for a metric name.
metrics().registry().configure_histogram(
    "rpc_duration_seconds",
    [0.005, 0.01, 0.025, 0.05, 0.1, 0.5, 1.0],
);

// Labeled histogram. First registration with a given `(name, labels)` tuple
// allocates a new `Arc<Histogram>` using the configured buckets (or the
// Prometheus default seconds layout if none configured).
let labels = LabelSet::from([("route", "/search")]);
let h = metrics().histogram_with("rpc_duration_seconds", &labels);
h.observe(0.087);
# }
```

<br>

<h3 id="labels"><code>LabelSet</code> & labeled metrics</h3>

Source: `src/labels.rs`.

`LabelSet` is a sorted, deduplicated `(key, value)` collection that
distinguishes one metric *instance* from another sharing the same name. The
inner storage is sorted by key so two label sets with the same contents but
different insertion orders hash and compare equal.

Construction:
- `LabelSet::EMPTY` / `LabelSet::new()` — empty set, allocation-free.
- `LabelSet::from([(k, v), ...])` / `FromIterator<(K, V)>` — accepts both
  string literals (`&'static str`) and owned `String`s.
- `let mut l = LabelSet::new(); l.add("k", "v");` — incremental build,
  builder-style.
- `.with("k", "v")` — consuming variant for chained construction.

Read accessors:
- `len() -> usize`, `is_empty() -> bool`.
- `iter() -> impl Iterator<Item = (&str, &str)>` — sorted by key.
- `get(key) -> Option<&str>`, `remove(key) -> bool`.
- `to_prometheus() -> String` — `{k="v",k="v"}` (used by exporters).
- `to_statsd() -> String` — `|#k:v,k:v` (DogStatsD format).

Cardinality control (on the registry):
- `Registry::set_cardinality_cap(usize)` — default `DEFAULT_CARDINALITY_CAP = 10_000`.
- `Registry::cardinality_cap() -> usize`,
  `Registry::cardinality_count() -> usize`,
  `Registry::cardinality_overflows() -> u64`.

When a fresh `(name, labels)` registration would exceed the cap:
- `try_*_with` returns `Err(MetricsError::CardinalityExceeded)`.
- `*_with` (non-`try`) routes to a process-global per-type overflow sink
  (never exported; observable via `cardinality_overflows`).

Labeled lookup methods on `MetricsCore` (each gated on its metric-type
feature):

- `counter_with(name, &LabelSet) -> Arc<Counter>` /
  `try_counter_with(...) -> Result<Arc<Counter>>`
- `gauge_with(name, &LabelSet) -> Arc<Gauge>` / `try_gauge_with`
- `timer_with(name, &LabelSet) -> Arc<Timer>` / `try_timer_with`
- `rate_with(name, &LabelSet) -> Arc<RateMeter>` / `try_rate_with`
- `histogram(name) -> Arc<Histogram>` (unlabeled),
  `histogram_with(name, &LabelSet) -> Arc<Histogram>` / `try_histogram_with`

Example:
```rust
# #[cfg(feature = "count")]
# {
use metrics_lib::{init, metrics, LabelSet, MetricsError};

init();

let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
metrics().counter_with("http_requests", &labels).inc();

// Tight cap + explicit overflow handling.
metrics().registry().set_cardinality_cap(4);
let bad = LabelSet::from([("trace_id", "deadbeef")]);
match metrics().try_counter_with("http_requests", &bad) {
    Ok(c) => c.inc(),
    Err(MetricsError::CardinalityExceeded) => { /* drop or downsample */ }
    Err(e) => panic!("unexpected error: {e}"),
}
# }
```

<br>

<h3 id="metric-metadata">Metric metadata</h3>

Source: `src/metadata.rs`.

Per-metric metadata (help text, unit, kind) feeds the `# HELP` / `# TYPE` /
`# UNIT` lines in Prometheus / OpenMetrics output, the `description` field
in OTLP, and unit suffixes in StatsD. Metadata is optional — every metric
exports successfully without it.

Types:
- `MetricKind` — `Counter | Gauge | Timer | Rate | Histogram`.
- `Unit` — enumerated standard units (`Seconds`, `Milliseconds`, `Bytes`,
  `Percent`, …) plus `Unit::Custom(&'static str)` for free-form unit names.
- `MetricMetadata { help: Cow<'static, str>, unit: Unit, kind: MetricKind }`.

Registry methods:
- `Registry::describe(name, MetricMetadata)` — store/replace metadata for a name.
- `Registry::describe_counter(name, help, unit)`
- `Registry::describe_gauge(name, help, unit)`
- `Registry::describe_timer(name, help, unit)`
- `Registry::describe_rate(name, help, unit)`
- `Registry::describe_histogram(name, help, unit)`
- `Registry::metadata(name) -> Option<MetricMetadata>`.

Example:
```rust
use metrics_lib::{init, metrics, Unit};

init();
metrics().registry().describe_counter(
    "http_requests",
    "Total HTTP requests handled",
    Unit::Custom("1"),
);
metrics().registry().describe_histogram(
    "http_request_duration_seconds",
    "Request handler latency",
    Unit::Seconds,
);
```

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

**v0.9.2 — refresh fixes:**
- `maybe_update()` now stores and compares the last-refresh timestamp in a
  single time unit (milliseconds). Earlier revisions stored nanoseconds and
  compared milliseconds, freezing the throttle so all values were pinned to
  their initial reads. After upgrade, `cpu_used()` / `mem_used_mb()` /
  `load_avg()` / process metrics refresh on the configured interval as
  documented.
- `SystemSnapshot::last_update` now reports **time since last refresh**
  (e.g., `Duration::from_millis(0..=interval_ms)`), not "monotonic time at
  last refresh" as it incorrectly did before.
- Linux process CPU is now delta-sampled: `((utime+stime) - prev) /
  (CLK_TCK * elapsed_s * cores) * 100`, normalized per-core and clamped to
  `0..=100`. First sample returns `0.0` and seeds the baseline.
- The non-Linux sysinfo refresh now uses `parking_lot::Mutex` instead of
  `std::sync::Mutex`. The redundant manual `unsafe impl Send/Sync` for
  `SystemHealth` was removed (the compiler derives both automatically).

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

<h4 id="systemhealth-platform-notes">Platform Notes</h4>

- Linux: Uses `/proc` for system and process sampling (CPU, memory, load, threads, FDs) for maximum performance and fidelity.
- Non‑Linux (macOS/Windows): Uses the `sysinfo` crate for cross‑platform values.
  - System CPU, memory, and load are reported via `sysinfo`.
  - Process CPU and memory are reported via `sysinfo`.
  - Thread count and file descriptor/handle count return defaults (1 and 0 respectively) where not exposed portably.
- Future enhancement: native macOS (sysctl/mach) and Windows (PDH/WMI/WinAPI) backends can be added for per‑platform fidelity (e.g., accurate thread/FD counts) without adding dependencies.

Examples:

- CPU overview (system/process): `examples/cpu_stats.rs`
- Memory overview (system/process): `examples/memory_stats.rs`

<br>
<h5 id="systemhealth-memory-units-note">Memory Units Note</h5>

- Depending on platform and sysinfo version, raw memory values may be reported in KiB or bytes. The provided `examples/memory_stats.rs` auto‑detects units for display (MB/GB) while keeping percentage calculations consistent.
- For production use, prefer using percentages for alerts and apply consistent conversion for display. If you need exact byte precision on macOS or Windows, consider platform APIs (e.g., `sysctl` on macOS, WinAPI on Windows) in a background task, or contribute native backends to `SystemHealth`.
- The example includes a small documented helper `normalize_sysinfo_memory_to_mb(...)` explaining invariants and edge cases; see `examples/memory_stats.rs` (comment block above the function) for details.

<br>

<h3 id="health-config"><code>HealthConfig</code> &amp; <code>Step</code> (v0.9.5)</h3>

Source: `src/system_health.rs`.

`SystemHealth::health_score()` is composed of step-wise penalties applied
to six metrics (system CPU, normalised load average, process CPU, memory
GB, threads, file descriptors). Prior to v0.9.5 the threshold ladder was
hardcoded; v0.9.5 exposes the full ladder as a tunable `HealthConfig`
value so deployments with different operating envelopes can pick their
own boundaries without forking the crate.

**Types:**

- `Step { threshold: f64, penalty: f64 }` — one `(threshold, penalty)`
  pair. When the metric exceeds `threshold`, `penalty` is subtracted from
  the running 0..=100 score. `Step::new(threshold, penalty)` is `const`.
- `HealthConfig { system_cpu, load_avg, process_cpu, memory_gb, threads,
  fds: Vec<Step> }` — one penalty ladder per metric. Within each `Vec`,
  steps **must be ordered descending by threshold** (first match wins).
- `HealthConfig::default()` — the v0.9.x defaults, preserved exactly so
  existing dashboards do not shift on upgrade.

**Methods:**

- `SystemHealth::with_config(interval: Duration, config: HealthConfig) -> Self`
  — new in v0.9.5. Combines a custom refresh interval with a custom
  score config. `SystemHealth::with_interval(d)` continues to use the
  default config.
- The `load_avg` ladder is interpreted as **multipliers of `num_cpus::get()`**
  (e.g. `Step::new(2.0, 25.0)` ⇒ trips when 1-minute load > 2× core
  count). Other ladders use the metric's natural unit.

**Examples:**

Tighter CPU thresholds for a CPU-bound service:

```rust
use metrics_lib::{HealthConfig, Step, SystemHealth};
use std::time::Duration;

let cfg = HealthConfig {
    system_cpu: vec![
        Step::new(70.0, 30.0),
        Step::new(50.0, 15.0),
        Step::new(30.0, 5.0),
    ],
    ..HealthConfig::default()
};
let health = SystemHealth::with_config(Duration::from_millis(500), cfg);
let score = health.health_score(); // 0..=100, lower under load
```

Relax FD thresholds for a connection-heavy service:

```rust
use metrics_lib::{HealthConfig, Step, SystemHealth};
use std::time::Duration;

let cfg = HealthConfig {
    fds: vec![
        Step::new(100_000.0, 15.0),
        Step::new(50_000.0, 8.0),
        Step::new(10_000.0, 3.0),
    ],
    ..HealthConfig::default()
};
let _ = SystemHealth::with_config(Duration::from_secs(1), cfg);
```

`HealthConfig` derives `serde::Serialize` behind the `serde` feature so
configurations can be loaded from JSON / TOML / YAML if desired.

<br>

<h3 id="scoped-registry"><code>ScopedRegistry</code> (v0.9.5)</h3>

Source: `src/registry.rs`.

A `ScopedRegistry` is a thin view over a `Registry` that **prepends a
fixed prefix** to every metric name on lookup / describe / configure.
Useful for tying a metrics namespace to a subsystem (`"http."`,
`"db."`, …) without rewriting every call site.

There is **no separate storage**: a scoped lookup lands in the same
underlying map as the unscoped equivalent, so
`scoped("http.").counter("requests")` and `counter("http.requests")`
return the **same** `Arc<Counter>`.

**Constructors:**

- `Registry::scoped(prefix: impl Into<String>) -> ScopedRegistry<'_>`
- `MetricsCore::scoped(prefix: impl Into<String>) -> ScopedRegistry<'_>`
  (shorthand for `self.registry().scoped(prefix)`)
- `ScopedRegistry::scoped(sub_prefix) -> ScopedRegistry<'_>` — nested
  scopes compose prefixes: `scoped("a.").scoped("b.")` ≡ `scoped("a.b.")`.

**Methods** (every method delegates to the underlying `Registry` with the
joined name):

- `counter(name)` / `gauge(name)` / `timer(name)` / `rate(name)` /
  `histogram(name)` — unlabeled lookups.
- `counter_with(name, &labels)` / `gauge_with` / `timer_with` /
  `rate_with` / `histogram_with` — labeled lookups (subject to the
  registry's cardinality cap).
- `describe_counter` / `describe_gauge` / `describe_timer` /
  `describe_rate` / `describe_histogram` — under the scoped name.
- `configure_histogram(name, buckets)` — bucket-layout pre-configuration
  under the scoped name (requires `histogram`).
- `prefix() -> &str`, `registry() -> &Registry` — accessors.

**Examples:**

Subsystem namespacing:

```rust
use metrics_lib::{init, metrics, Unit};

init();
let http = metrics().scoped("http.");
http.describe_counter("requests", "Total HTTP requests", Unit::Custom("1"));
http.counter("requests").inc();
http.gauge("inflight").set(1.0);
// Equivalent unscoped name lookups return the same `Arc<Counter>`:
assert_eq!(metrics().counter("http.requests").get(), 1);
```

Nested scopes compose:

```rust
use metrics_lib::{init, metrics};

init();
let svc = metrics().scoped("svc.");
let db = svc.scoped("db.");
db.counter("queries").inc(); // → "svc.db.queries"
```

<br>

<h3 id="token-bucket"><code>TokenBucket</code> (v0.9.5)</h3>

Source: `src/token_bucket.rs`.

Strict-admission counterpart to `RateMeter::tick_if_under_limit`. Where
the rate-meter trades correctness for hot-path speed (it has known TOCTOU
overshoot of up to `num_threads − 1` events per window), `TokenBucket`
guarantees that the **capacity is never exceeded** — every `acquire`
goes through a single `compare_exchange_weak` on a packed `(tokens,
last_refill_ms)` `u64`.

Use for: billing, hard-limit admission control, downstream service
protection. For pure observability throttling, `RateMeter` is faster.

**Constructor:**

- `TokenBucket::new(capacity: u32, refill_per_second: f64) -> Self`
  - `capacity` — burst size in whole tokens (max tokens the bucket
    holds).
  - `refill_per_second` — sustained refill rate. `0.0` produces a
    static-capacity bucket (no refill). Non-finite or negative inputs
    are coerced to `0.0`.

**Methods:**

- `try_acquire(n: u32) -> Result<()>` — atomically remove `n` tokens.
  Returns `Ok(())` on success or `Err(MetricsError::WouldBlock)` when
  fewer than `n` tokens are available after refill. `n == 0` is a no-op.
- `acquire(n: u32) -> bool` — convenience wrapper returning `true`/`false`.
- `available() -> u32` — approximate current token count (advisory; no
  retry semantics).
- `capacity() -> u32`, `refill_per_second() -> f64` — configuration
  accessors.
- `reset()` — refills the bucket to full `capacity`.

**Examples:**

Rate-limited request admission:

```rust
use metrics_lib::TokenBucket;
use std::time::Duration;

// 50 requests per second sustained, burst up to 100.
let limiter = TokenBucket::new(100, 50.0);

fn handle_request(limiter: &TokenBucket) {
    if limiter.acquire(1) {
        // … serve request …
    } else {
        // … return 429 Too Many Requests …
    }
}

handle_request(&limiter);
// Reset for the next benchmark run.
limiter.reset();
```

Burst acquire (multi-token transactions):

```rust
use metrics_lib::{TokenBucket, MetricsError};

let limiter = TokenBucket::new(50, 10.0);
match limiter.try_acquire(5) {
    Ok(()) => { /* admit the batch */ }
    Err(MetricsError::WouldBlock) => { /* retry later */ }
    Err(_) => unreachable!(),
}
```

Multiple concurrent threads racing for tokens: with capacity 100 and
8 threads each requesting 30, exactly 100 tokens are issued — no
overshoot:

```rust
use metrics_lib::TokenBucket;
use std::sync::Arc;
use std::thread;

let bucket = Arc::new(TokenBucket::new(100, 0.0));
let handles: Vec<_> = (0..8)
    .map(|_| {
        let b = Arc::clone(&bucket);
        thread::spawn(move || {
            let mut taken = 0u32;
            for _ in 0..30 {
                if b.acquire(1) { taken += 1; }
            }
            taken
        })
    })
    .collect();
let total: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
assert_eq!(total, 100);
```

<br>

<h3 id="exporters">Exporters (v0.9.3)</h3>

Five built-in exporters render the registry into popular telemetry formats.
Each is a stateless function (or thin sink for push transports) that
accepts a `&Registry` and produces a backend-specific output.

| Backend | Module | Feature flag | Output |
|---|---|---|---|
| Prometheus text | `metrics_lib::exporters::prometheus` | (always on) | `String` |
| OpenMetrics text | `metrics_lib::exporters::openmetrics` | (always on) | `String` (with trailing `# EOF\n`) |
| JSON snapshot | `metrics_lib::exporters::json` | `serde` | `String` / `RegistrySnapshot` |
| StatsD UDP | `metrics_lib::exporters::statsd` | `statsd` | UDP datagrams via `StatsdSink` |
| OTLP/HTTP+JSON | `metrics_lib::exporters::otlp` | `otlp` (→ `serde`) | `String` POST body |

All exporters honour [`LabelSet`](#labels) and [metric metadata](#metric-metadata):

- Prometheus / OpenMetrics: labels appear as `{k="v",k="v"}`; help/unit
  metadata becomes `# HELP` / `# TYPE` / `# UNIT` lines. OpenMetrics adds
  the `_total` suffix on counter samples and ends with `# EOF\n`.
- JSON snapshot: labels serialise as nested JSON objects; metadata appears
  per-series.
- StatsD: labels become DogStatsD tags (`|#k:v,k:v`); the wire format uses
  cumulative gauge mode (`|g`) since StatsD counters are deltas and the
  registry stores totals.
- OTLP: labels become `attributes`; mapped to OTLP `Sum` (counters,
  monotonic + cumulative), `Gauge`, or `Histogram` (timers + histograms).

Example — Prometheus `/metrics` body:

```rust
# #[cfg(feature = "count")]
# {
use metrics_lib::{init, metrics, LabelSet, Unit};
use metrics_lib::exporters::prometheus;

init();

metrics().registry().describe_counter(
    "http_requests",
    "Total HTTP requests",
    Unit::Custom("1"),
);
let labels = LabelSet::from([("status", "200")]);
metrics().counter_with("http_requests", &labels).add(7);

let body = prometheus::render(metrics().registry());
assert!(body.contains("# HELP http_requests Total HTTP requests"));
assert!(body.contains(r#"http_requests{status="200"} 7"#));
# }
```

Example — JSON snapshot (feature = "serde"):

```rust
# #[cfg(all(feature = "serde", feature = "count"))]
# {
use metrics_lib::{init, metrics};
use metrics_lib::exporters::json;

init();
metrics().counter("hits").inc();

let snap = json::snapshot(metrics().registry());
assert_eq!(snap.schema_version, 1);
assert!(!snap.counters.is_empty());

// Or render directly to a JSON string.
let body = json::render(metrics().registry());
let _v: serde_json::Value = serde_json::from_str(&body).unwrap();
# }
```

Example — StatsD push (feature = "statsd"):

```no_run
# #[cfg(all(feature = "statsd", feature = "count"))]
# {
use metrics_lib::{init, metrics};
use metrics_lib::exporters::statsd::StatsdSink;

init();
metrics().counter("requests").inc();

let sink = StatsdSink::new("127.0.0.1:8125")
    .expect("bind UDP")
    .with_prefix("svc.");
sink.send(metrics().registry()).expect("statsd push");
# }
```

Example — OTLP/HTTP+JSON (feature = "otlp"):

```rust
# #[cfg(all(feature = "otlp", feature = "count"))]
# {
use metrics_lib::{init, metrics};
use metrics_lib::exporters::otlp;

init();
metrics().counter("requests").inc();

let payload: String = otlp::render(metrics().registry(), "my-service");
// POST `payload` to <collector>/v1/metrics with Content-Type: application/json
let _ = payload;
# }
```

End-to-end runnable examples live in `examples/`:
`labels_demo`, `histogram_latency`, `prometheus_endpoint`, `statsd_push`,
`otlp_push`, `snapshot_serde`.

<br>

### Async support

Source: `src/async_support.rs`

- `AsyncTimerGuard` — RAII timing for async blocks
- `AsyncTimerExt` — extension trait providing `start_async()` and `time_async()`
- `TimedFuture` — `Future` wrapper returned by `time_async()`
- `AsyncMetricBatch` — batch metric updates with `counter_inc`, `gauge_set`,
  `timer_record`, `rate_tick`, `flush(&MetricsCore)`.
  - **v0.9.2 note:** name arguments are now `impl Into<Cow<'static, str>>`
    (was `&'static str`). Both string literals and owned `String`s are
    accepted; the enum stores `Cow<'static, str>` internally so static names
    cost nothing extra.

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

<h3 id="tracing-ext"><code>tracing</code> integration (v0.9.5)</h3>

Source: `src/tracing_ext.rs` — requires the `tracing` Cargo feature.

Opt-in adapters that wrap existing `Timer` operations with a
[`tracing`](https://docs.rs/tracing) span, so a single call site
populates both the metric histogram and the user's tracing subscriber.
Hot paths in the metric types themselves are **unchanged**; enabling the
`tracing` feature does not slow `Counter::inc` / `Gauge::set` /
`Timer::record` / `Histogram::observe`.

**Functions:**

- `time_in_span<T>(name: &'static str, timer: &Timer, f: impl FnOnce() -> T) -> T`
  — runs `f` inside both the supplied `Timer` and a
  `tracing::info_span!("metric.time", name = name)`. Returns whatever
  `f` returns.
- `time_global<T>(name: &'static str, f: impl FnOnce() -> T) -> T`
  — shorthand that resolves the timer from the global registry by
  `name` and forwards to `time_in_span`.

**Examples:**

Wrap a closure with both a metric timer and a tracing span:

```rust
# #[cfg(all(feature = "timer", feature = "tracing"))]
# {
use metrics_lib::{init, metrics};
use metrics_lib::tracing_ext::time_in_span;

init();
let timer = metrics().timer("db.query");
let result = time_in_span("db.query", &timer, || {
    // … work runs inside both the Timer and an info_span! …
    42
});
assert_eq!(result, 42);
assert_eq!(timer.count(), 1);
# }
```

Use the global shorthand:

```rust
# #[cfg(all(feature = "timer", feature = "tracing"))]
# {
use metrics_lib::init;
use metrics_lib::tracing_ext::time_global;

init();
let _ = time_global("rpc.call", || 11);
# }
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

## API Safety

The library prioritizes performance while preventing common misuse. Several read/return-value APIs are annotated with `#[must_use]`. This means the compiler warns if the return value is ignored. Ignoring these values usually indicates a logic bug or a lost control decision.

Key `#[must_use]` examples:

- `Counter`: `get()`, `stats()`, `age()`, `is_zero()`, `rate_per_second()`
- `Gauge`: `get()`, `stats()`, `age()`, `is_zero()`, `is_positive()`, `is_negative()`, `is_finite()`
- `Timer`: `count()`, `total()`, `average()`, `min()`, `max()`, `stats()`, `age()`, `is_empty()`, `rate_per_second()`, `RunningTimer::elapsed()`
- `RateMeter`: `rate()`, `rate_per_second()`, `rate_per_minute()`, `rate_per_hour()`, `total()`, `exceeds_rate()`, `can_allow()`, `tick_if_under_limit()`, `tick_burst_if_under_limit()`, `stats()`, `age()`, `is_empty()`

Misuse patterns to avoid:

- Dropping results without checking:
  ```rust
  // BAD: discards the admission decision
  let _ = metrics().rate("api").tick_if_under_limit(1000.0);
  ```

- Computing values and not using them:
  ```rust
  // BAD: computes but ignores the current rate
  metrics().rate("api").rate();
  ```

Prefer explicit handling:

```rust
let r = metrics().rate("api");
if r.tick_if_under_limit(1000.0) {
    // admitted
} else {
    // throttled
}

let s = r.stats();
log::debug!("rate: {:.1}/s total: {} age: {:?}", s.per_second, s.total_events, s.age);
```

Notes:

- `Result<…>`-returning APIs are not additionally marked with `#[must_use]` since `Result` already carries it.
- Methods that mutate state (e.g., `Counter::inc()`, `Gauge::set()`) intentionally do not return values.

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
count       = []                                  # Counter metric type
gauge       = []                                  # Gauge metric type
timer       = []                                  # Timer metric type
meter       = []                                  # Rate meter metric type
sample      = []                                  # Statistical sampling
histogram   = ["sample"]                          # Histogram (requires sample)
async       = ["dep:tokio"]                       # Async support (requires Tokio)
serde       = ["dep:serde"]                       # Serde serialization
all         = ["count","gauge","timer","meter","sample","histogram"]
full        = ["count","gauge","timer","meter","sample","histogram","async","serde"]
minimal     = ["count"]                           # Smallest useful build
default     = ["count","gauge","timer"]
bench-tests = []                                  # Benchmark-style CI tests
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


<br>
<h3 id="real-world-custom-exporter">Building a Custom Exporter</h3>

Example skeleton to snapshot internal metrics and ship to a custom sink (file, TCP, UDP, HTTP, etc.) without perturbing hot paths:

```rust
use metrics_lib::metrics;
use std::fmt::Write;

/// Periodically called by a background task
pub fn snapshot_metrics() -> String {
    let reg = metrics().registry();
    let mut out = String::new();

    // Example format: simple key=value lines (adapt to your collector)
    for name in reg.counter_names() {
        let v = metrics().counter(Box::leak(name.into_boxed_str())).get();
        let _ = writeln!(out, "{} {}", name, v);
    }
    for name in reg.gauge_names() {
        let v = metrics().gauge(Box::leak(name.into_boxed_str())).get();
        let _ = writeln!(out, "{} {}", name, v);
    }
    for name in reg.timer_names() {
        let s = metrics().timer(Box::leak(name.into_boxed_str())).stats();
        let _ = writeln!(out, "{}.count {}", name, s.count);
        let _ = writeln!(out, "{}.avg_ns {}", name, s.average.as_nanos());
    }
    for name in reg.rate_meter_names() {
        let r = metrics().rate(Box::leak(name.into_boxed_str()));
        let _ = writeln!(out, "{}.per_sec {:.3}", name, r.rate());
    }
    out
}
```

Guidelines:

- Run exporters on a timer or off a channel queue, not inline with critical work.
- Bound buffers and drop data on overload to protect application throughput.
- Prefer binary formats for high-throughput ingestion.


<br>
<h3 id="real-world-memory-stats">Memory Stats: total/used/free + percentages</h3>

The `SystemHealth` API provides convenient accessors for commonly used memory stats. Convert units as needed.

```rust
use metrics_lib::metrics;

fn fmt_size_mb(mb: f64) -> (f64, &'static str) {
    // convert MB → GB/TB simplistically for display
    if mb >= 1024.0 * 1024.0 { (mb / (1024.0 * 1024.0), "TB") }
    else if mb >= 1024.0 { (mb / 1024.0, "GB") } else { (mb, "MB") }
}

pub fn memory_overview() {
    let sys = metrics().system();

    let used_mb = sys.mem_used_mb();
    // If you need total/free, compute via platform helpers or your own sysinfo; here we display used directly.
    let (v, unit) = fmt_size_mb(used_mb);

    println!("mem.used: {:.2} {}", v, unit);
    println!("mem.used.pct (process): {:.2}%", sys.process_mem_used_mb() / used_mb.max(1.0) * 100.0);
}
```

Notes:

- `mem_used_mb()` and `mem_used_gb()` report current system memory usage; `process_mem_used_mb()` reports this process’s memory.
- If you require precise total/free memory, integrate your platform’s system APIs alongside `SystemHealth` and compute `free = total - used` and percentages accordingly.


<br>
<h3 id="real-world-memory-percent-operation">Memory % used for an operation (estimate)</h3>

Estimate memory consumed by a single operation by sampling process memory before and after. Express as MB/GB and as a percentage of the pre-op process memory.

```rust
use metrics_lib::metrics;

pub fn measure_op_memory<T>(f: impl FnOnce() -> T) -> (T, f64 /* delta_mb */, f64 /* pct of process */) {
    let sys = metrics().system();
    let before_mb = sys.process_mem_used_mb();
    let result = f();
    let after_mb = sys.process_mem_used_mb();
    let delta_mb = (after_mb - before_mb).max(0.0);
    let pct = if before_mb > 0.0 { (delta_mb / before_mb) * 100.0 } else { 0.0 };
    (result, delta_mb, pct)
}
```

Notes:

- This is a coarse estimate; allocator behavior and async tasks can skew instantaneous samples. For better accuracy, repeat and average.


<br>
<h3 id="real-world-cpu-stats">CPU Stats: total/used/free + percentages</h3>

`SystemHealth` exposes CPU usage percentages. Display them and convert as needed.

```rust
use metrics_lib::metrics;

pub fn cpu_overview() {
    let sys = metrics().system();
    let used = sys.cpu_used();      // e.g., 23.5 (percent)
    let free = sys.cpu_free();      // e.g., 76.5 (percent)

    println!("cpu.used: {:.1}%", used);
    println!("cpu.free: {:.1}%", free);
}
```

Notes:

- For per-core or process-specific stats, use `process_cpu_used()` and, if needed, supplement with platform APIs for core counts/affinity.


<br>
<h3 id="real-world-cpu-percent-operation">CPU % used for an operation (estimate)</h3>

Estimate CPU for an operation by sampling process CPU usage and wall time before/after. This yields a coarse percentage useful for relative comparisons.

```rust
use metrics_lib::metrics;
use std::time::Instant;

pub fn measure_op_cpu<T>(f: impl FnOnce() -> T) -> (T, f64 /* cpu_used_delta_pct */, f64 /* wall_ms */) {
    let sys = metrics().system();
    let start = Instant::now();
    let cpu_before = sys.process_cpu_used();
    let result = f();
    let wall = start.elapsed().as_millis() as f64;
    let cpu_after = sys.process_cpu_used();
    let cpu_delta = (cpu_after - cpu_before).max(0.0);
    (result, cpu_delta, wall)
}
```

Notes:

- Short operations can under-report due to sampling granularity; repeat and average for stability.
- For rigorous accounting, sample over longer windows or use OS-level per-thread CPU accounting.
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


<hr>
<br>
<a href="#top">&uarr; <b>TOP</b></a>
<br>


<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
