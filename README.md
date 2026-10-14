<h1 align="center">
    <img width="99" alt="Rust logo" src="https://raw.githubusercontent.com/jamesgober/rust-collection/72baabd71f00e14aa9184efcb16fa3deddda3a0a/assets/rust-logo.svg">
    <br>
    <b>metrics-lib</b>
    <br>
    <sub>
        <sup>RUST APPLICATION METRICS</sup>
    </sub>
</h1>

<p align="center">
    <a href="https://crates.io/crates/metrics-lib"><img src="https://img.shields.io/crates/v/metrics-lib.svg" alt="Crates.io"></a>
    <a href="https://crates.io/crates/metrics-lib" alt="Download"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/metrics-lib?color=%230099ff"></a>
    <a href="https://docs.rs/metrics-lib"><img src="https://docs.rs/metrics-lib/badge.svg" alt="Documentation"></a>
    <a href="https://github.com/jamesgober/metrics-lib/actions"><img alt="GitHub CI" src="https://github.com/jamesgober/metrics-lib/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://github.com/rust-lang/rfcs/blob/master/text/2495-min-rust-version.md" title="MSRV"><img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.85-blue"></a>
    <a href="#license"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg" alt="License"></a>
</p>

<p align="center">
    <b>Counters, gauges, timers, histograms and summaries for Rust services.</b>
    <br>
    <i>Atomic hot paths, a bounded registry, and exposition that scrapers accept.</i>
</p>

<br>

<p>
    <strong>metrics-lib</strong> is an in-process metrics library: you record into counters, gauges, timers, rate meters, histograms and summaries, and export to Prometheus, OpenMetrics, OTLP, StatsD or JSON. Recording into a held counter or gauge is a single atomic operation; timers, histograms and summaries use a few atomic operations, and only timers, rate meters and token buckets read the clock. None of them takes a lock or allocates, except <code>observe_with_exemplar</code>, which stores the exemplar's labels. The registry that hands out those handles validates names, caps the number of series and keeps one kind per name, so a busy or hostile workload cannot grow memory without bound or produce output a scraper rejects.
</p>

<p>
    It is built for services that want one dependency for the whole path from instrumentation to a <code>/metrics</code> endpoint. The core has no required dependencies; integrations sit behind features: a built-in HTTP endpoint and Pushgateway client, a <code>metrics</code>-crate recorder so existing <code>metrics::counter!</code> call sites keep working, <code>tower</code> middleware for request rate, errors and duration, and <code>tracing</code> exemplars.
</p>

<p>
    Every exporter's output is parsed by strict Prometheus and OpenMetrics validators in the test suite, including 2,048 randomly generated registries per run, and every performance number on this page comes from the Criterion benches in the repository with the machine stated next to it.
</p>

---

## Features

### **Metric types**
- **Counter, ShardedCounter, UpDownCounter** - monotonic, contention-striped and signed integer counters.
- **Gauge** - `f64` with set, add, min/max tracking and exponential moving average.
- **Timer** - count, sum, min and max of durations; RAII guards that can outlive a temporary handle.
- **Histogram** - cumulative buckets with exemplars; **Summary** - streaming quantiles with 2 % relative error.
- **RateMeter, TokenBucket** - sliding-window rates and a GCRA rate limiter.

### **Registry**
- **Labels** - `LabelSet` keys and values validated (or repaired) and escaped for every format.
- **Bounded** - a series cap (default 10 000), idle-series eviction and explicit removal.
- **One name, one kind** - conflicting registrations are refused instead of producing invalid output.
- **Sharded** - 16 independently locked shards; labeled lookups do not allocate.

### **Export**
- **Prometheus and OpenMetrics text** - always available, with exemplars in OpenMetrics.
- **OTLP** (JSON and protobuf, cumulative or delta), **StatsD** (deltas), **JSON** snapshots.
- **HTTP** - a dependency-free `/metrics` server and a Pushgateway client.
- **Process metrics** - the standard `process_*` series on Linux, macOS and Windows.

### **Reliability and safety**
- **No unsafe code** - `#![forbid(unsafe_code)]`.
- **No panics on input** - invalid numbers and names are rejected by `try_*` methods and ignored or repaired by the others.
- **Tested** - unit, integration, stress, property and doc tests; every feature is linted and tested on its own in CI.

---

## Installation

```toml
[dependencies]
metrics-lib = "2"

# Pick features:
metrics-lib = { version = "2", features = [
    "histogram",   # Histogram and Summary
    "meter",       # RateMeter
    "http",        # /metrics server and Pushgateway client
    "tower",       # request metrics middleware
] }
```

| Feature | Enables | Default |
|---|---|---|
| `count` | `Counter`, `ShardedCounter`, `UpDownCounter` | yes |
| `gauge` | `Gauge` | yes |
| `timer` | `Timer` | yes |
| `system` | `SystemHealth`, `ProcessCollector` (uses `sysinfo` off Linux) | yes |
| `meter` | `RateMeter` | |
| `histogram` | `Histogram`, `Summary` | |
| `sample` | adaptive sampler, circuit breaker, backpressure | |
| `async` | async timing, batching, Tokio runtime metrics | |
| `serde` | JSON exporter, `Serialize` snapshots | |
| `statsd` | StatsD exporter | |
| `otlp` | OTLP exporter (JSON and protobuf) | |
| `http` | `MetricsServer`, `Pushgateway` | |
| `tracing` | span timing and exemplars | |
| `metrics-rs` | `metrics` crate recorder | |
| `tower` | `RedLayer` middleware | |
| `all` / `full` | every metric type / everything | |

**Recommendations**: a library that only counts: `default-features = false, features = ["count"]`. A web service: the defaults plus `histogram`, `http` and `tower`. Code already using the `metrics` crate: add `metrics-rs`.

---

## Quick Start

```rust
use metrics_lib::{LabelSet, MetricsCore};
use metrics_lib::exporters::prometheus;

let core = MetricsCore::new();

// Look a metric up once and keep the handle on hot paths.
let requests = core.counter_with("http_requests", &LabelSet::from([("method", "GET")]));
requests.inc();

core.gauge("queue_depth").set(12.0);
core.histogram("request_seconds").observe(0.042);

// Time a block; the guard records when it is dropped.
{
    let _t = core.start_timer("db_query");
}

let text = prometheus::render(core.registry());
assert!(text.contains(r#"http_requests{method="GET"} 1"#));
```

The global instance works the same way and configures itself on first use:

```rust
use metrics_lib::{init_with_config, metrics, MetricsConfig};

init_with_config(MetricsConfig::default().with_max_series(50_000)).ok();
metrics().counter("jobs_done").inc();
```

### **Serve `/metrics`**

```rust,no_run
use metrics_lib::exporters::http::MetricsServer;
use metrics_lib::MetricsCore;
use std::sync::Arc;

let core = Arc::new(MetricsCore::new());
let server = MetricsServer::spawn("0.0.0.0:9898", Arc::clone(&core))?;
// ... run the application; Prometheus scrapes http://host:9898/metrics
server.shutdown();
# Ok::<(), std::io::Error>(())
```

### **Instrument a tower or axum service**

```rust
use metrics_lib::tower::RedLayer;
use metrics_lib::MetricsCore;
use std::sync::Arc;
use tower::Layer;

let core = Arc::new(MetricsCore::new());
let _service = RedLayer::new(Arc::clone(&core)).layer(tower::service_fn(
    |_req: http::Request<()>| async { Ok::<_, std::convert::Infallible>(http::Response::new(())) },
));
// Records http_requests_total{method,status}, http_request_duration_seconds
// and http_requests_in_flight.
```

### **Keep existing `metrics` call sites**

```rust
use metrics_lib::metrics_rs::MetricsLibRecorder;
use metrics_lib::MetricsCore;
use std::sync::Arc;

let core = Arc::new(MetricsCore::new());
let recorder = MetricsLibRecorder::new(Arc::clone(&core));
// Applications call `metrics_lib::metrics_rs::install(core)` once instead.
metrics::with_local_recorder(&recorder, || {
    metrics::counter!("logins_total", "provider" => "github").increment(1);
});
assert_eq!(core.registry().counter_names(), vec!["logins_total"]);
```

### **Bounded, validated registries**

```rust
use metrics_lib::{MetricsError, Registry};

let registry = Registry::with_cardinality_cap(2);
registry.counter("a").inc();
registry.counter("b").inc();
assert!(matches!(registry.try_counter("c"), Err(MetricsError::CardinalityExceeded)));
assert!(matches!(registry.try_gauge("a"), Err(MetricsError::KindMismatch { .. })));
assert!(matches!(registry.try_counter("bad name"), Err(MetricsError::InvalidName { .. })));
```

More in [`docs/API.md`](docs/API.md) and the [examples](examples/).

---

## Performance

Criterion medians on an AMD Ryzen 9 9950X3D, Windows 11, Rust 1.85, `lto = "fat"`, on a machine that was also running other builds (so these are upper bounds). Method and the full table: [`docs/PERFORMANCE.md`](docs/PERFORMANCE.md).

| Operation | Time |
|---|---|
| `Counter::inc` on a held handle | 4.5 ns |
| `Gauge::set` | 0.3 ns |
| `Timer::record_ns` | 9.3 ns |
| `Histogram::observe` | 15 ns |
| `RateMeter::tick` | 48 ns |
| Unlabeled registry lookup plus `inc` | 38 ns |
| Labeled registry lookup plus `inc` | 84 ns (two labels) |
| 8 threads × 100 000 increments, one `Counter` | 7.8 ms |
| 8 threads × 100 000 increments, `ShardedCounter` | 0.89 ms |

Hold handles on hot paths: a lookup costs a shard read lock and a hash of the name, about eight times the increment itself.

---

## **Status: Stable**

**Current Version:** `2.0.0`, a major release that fixes the correctness problems found in the 2026-10-13 audit. Upgrading from 1.x: [`docs/MIGRATION-2.0.md`](docs/MIGRATION-2.0.md).

**API stability**: the 2.x contract is in [`docs/STABILITY.md`](docs/STABILITY.md). Public enums and option structs are `#[non_exhaustive]`, so additions ship in minor releases.

---

## **Documentation & Resources**

- **[Documentation index](docs/README.md)**
- **[API reference](docs/API.md)** - every type and exporter with runnable examples
- **[Performance](docs/PERFORMANCE.md)** and **[tuning](docs/performance-tuning.md)**
- **[Migrating from 1.x](docs/MIGRATION-2.0.md)** and **[from the `metrics` crate](docs/migrating-from-metrics-rs.md)**
- **[docs.rs](https://docs.rs/metrics-lib)**, **[crates.io](https://crates.io/crates/metrics-lib)**, **[examples](examples/)**, **[benchmarks](benches/)**
- **Observability assets**: a Grafana dashboard, Prometheus recording rules and Kubernetes manifests in [`docs/observability/`](docs/observability/) and [`docs/k8s/`](docs/k8s/)

### **Troubleshooting**

<details>
  <summary><b>My metric shows up with an underscore instead of a dot or dash?</b></summary>
    <p>Prometheus names cannot contain <code>.</code> or <code>-</code>. Dots are kept in the registry and written as <code>_</code>; other invalid characters are repaired at registration. <code>try_*</code> lookups return <code>InvalidName</code> instead.</p>
</details>

<details>
  <summary><b>A lookup returned a metric that is not exported?</b></summary>
    <p>The name is registered as another kind, or the series cap is full. Check <code>Registry::kind_conflicts()</code> and <code>cardinality_overflows()</code>, or use the <code>try_*</code> lookups to get the error.</p>
</details>

<details>
  <summary><b><code>timer("x").start()</code> fails with E0716?</b></summary>
    <p><code>start()</code> borrows the timer. Use <code>start_owned()</code> or <code>MetricsCore::start_timer</code>, which own their handle.</p>
</details>

<details>
  <summary><b>Thread count or load average is <code>None</code>?</b></summary>
    <p>The platform does not report it (thread count outside Linux, load average on Windows). The exporters omit those series.</p>
</details>

### **Version Compatibility**
- **Rust**: 1.85+ (edition 2024). Raising the MSRV is a major change (see `docs/STABILITY.md`).
- **Platforms**: Linux, macOS and Windows are tested in CI.
- **Feature flags**: additive; every feature builds, lints and passes its unit tests on its own.

### **Development Setup**
```bash
git clone https://github.com/jamesgober/metrics-lib.git
cd metrics-lib
cargo test --all-features                                 # unit, integration and doc tests
cargo clippy --all-targets --all-features -- -D warnings  # lints
cargo bench --all-features --bench metrics_bench          # benchmarks
```

## **Contributing**

See the [Contributing Guide](CONTRIBUTING.md) and the [development guidelines](docs/GUIDELINES.md).

<hr>

<!-- LICENSE
############################################# -->
<div id="license">
    <h2>License</h2>
    <p><b>metrics-lib</b> is dual-licensed under either of:</p>
    <ul>
        <li><b>Apache License, Version 2.0</b> — see <a href="./LICENSE-APACHE" title="Apache-2.0 License">LICENSE-APACHE</a> or <a href="http://www.apache.org/licenses/LICENSE-2.0" title="Apache-2.0 License" target="_blank">apache.org/licenses/LICENSE-2.0</a></li>
        <li><b>MIT License</b> — see <a href="./LICENSE-MIT" title="MIT License">LICENSE-MIT</a> or <a href="https://opensource.org/licenses/MIT" title="MIT License" target="_blank">opensource.org/licenses/MIT</a></li>
    </ul>
    <p>at your option.</p>
    <h3>Contribution</h3>
    <p>Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in <b>metrics-lib</b> by you, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.</p>
    <p>Unless required by applicable law or agreed to in writing, software distributed under the Licenses is distributed on an <b>"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND</b>, either express or implied.</p>
</div>


<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2026 <strong>JAMES GOBER.</strong></sup>
</div>
