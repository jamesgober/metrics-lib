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
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Public APIs](#public-apis)
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
  - [Notes](#notes)


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

<br>

## Notes

- All hot-path operations are lock-free and allocation-free where possible.
- For best latency, prefer batching (`Counter::batch_inc`, `AsyncMetricBatch`) in bursty workloads.
- Avoid calling `metrics()` before `init()`. In library code, consider taking `&MetricsCore` explicitly.
- For specialized meters/gauges, see the `specialized` submodules re-exported as `gauge_specialized` and `rate_meter_specialized`.
