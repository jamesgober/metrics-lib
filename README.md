<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>metrics-lib</b>
    <br>
    <sub><sup>Ultra-fast lock-free metrics for high-performance applications</sup></sub>
</h1>
<div align="center">
    <a href="https://crates.io/crates/metrics-lib"><img alt="Crates.io" src="https://img.shields.io/crates/v/metrics-lib"></a>
    <a href="https://crates.io/crates/metrics-lib" alt="Download metrics-lib"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/metrics-lib?color=%230099ff"></a>
    <a href="https://docs.rs/metrics-lib" title="metrics-lib Documentation"><img alt="docs.rs" src="https://img.shields.io/docsrs/metrics-lib"></a>
    <a href="https://github.com/jamesgober/metrics-lib/actions"><img alt="GitHub CI" src="https://github.com/jamesgober/metrics-lib/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://github.com/jamesgober/metrics-lib/actions/workflows/bench.yml" title="metrics-lib Benchmarks"><img alt="Benchmarks" src="https://github.com/jamesgober/metrics-lib/actions/workflows/bench.yml/badge.svg"></a>
    <a href="https://jamesgober.github.io/metrics-lib/benchmark-data/" title="Benchmark Regression">
        <img alt="Benchmark Regression" src="https://img.shields.io/github/actions/workflow/status/jamesgober/metrics-lib/ci.yml?branch=main&label=Benchmark%20Regression&logo=github">
    </a>
    <a href="https://github.com/rust-lang/rfcs/blob/master/text/2495-min-rust-version.md" title="MSRV"><img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.70%2B-blue"></a>
</div>
<br>
<p align="center">
    A high‑performance, in‑process metrics library for Rust. It provides ultra‑low‑overhead counters, gauges (atomic f64), timers (nanosecond precision), and sliding‑window rate meters designed for production hot paths. The core is lock‑free on the hot path, allocation‑free during steady state, and cache‑aligned for minimal contention. Optional async helpers, adaptive controls, and system health snapshots are available without imposing overhead when unused. MSRV is 1.70+. CI enforces formatting, lints, coverage (85% threshold), rustdoc warnings, and publish dry‑runs for reliability.
</p>

<div align="center">
    <sup>
        <a href="https://github.com/jamesgober/metrics-lib/blob/main/README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="https://github.com/jamesgober/metrics-lib/blob/main/docs/README.md" title="Documentation"><b>DOCS</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="https://github.com/jamesgober/metrics-lib/blob/main/docs/API.md" title="API Reference"><b>API</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="https://github.com/jamesgober/metrics-lib/blob/main/docs/GUIDELINES.md" title="Developer Guidelines"><b>GUIDELINES</b></a>
    </sup>
 </div>

<hr>

## Performance First

**World-class performance** with industry-leading benchmarks:

- **Counter**: 4.93ns/op (202.84M ops/sec)
- **Gauge**: 0.53ns/op (1886.79M ops/sec)  
- **Timer**: 10.87ns/op (91.99M ops/sec)
- **Memory**: 64 bytes per metric (cache-aligned)

<hr>

## Features

### Core Metrics
- **🔢 Counters** - Atomic increment/decrement with overflow protection
- **📊 Gauges** - IEEE 754 atomic floating-point with mathematical operations
- **⏱️ Timers** - Nanosecond precision with RAII guards and batch recording
- **📈 Rate Meters** - Sliding window rates with burst detection and API limiting
- **💾 System Health** - Built-in CPU, memory, and process monitoring

### Advanced Features
- **Lock-Free** - Zero locks in hot paths, pure atomic operations
- **Async Native** - First-class async/await support with zero-cost abstractions
- **Resilience** - Circuit breakers, adaptive sampling, and backpressure control
- **Cross-Platform** - Linux, macOS, Windows with optimized system integrations
- **Cache-Aligned** - 64-byte alignment prevents false sharing

<hr>

## API Overview

For a complete reference with examples, see `docs/API.md`.

- [`Counter`](./docs/API.md#counter) — ultra-fast atomic counters with batch and conditional ops
- [`Gauge`](./docs/API.md#gauge) — atomic f64 gauges with math ops, EMA, and min/max helpers
- [`Timer`](./docs/API.md#timer) — nanosecond timers, RAII guards, and closure/async timing
- [`RateMeter`](./docs/API.md#ratemeter) — sliding-window rate tracking and bursts
- [`SystemHealth`](./docs/API.md#systemhealth) — CPU, memory, load, threads, FDs, health score
- [Async support](./docs/API.md#async-support) — `AsyncTimerExt`, `AsyncMetricBatch`
- [Adaptive controls](./docs/API.md#adaptive-controls) — sampling, circuit breaker, backpressure
- [Prelude](./docs/API.md#prelude) — convenient re-exports

### Error handling: try_ variants

All core metrics expose non-panicking `try_` methods that validate inputs and return `Result<_, MetricsError>` instead of panicking:

- `Counter`: `try_inc`, `try_add`, `try_set`, `try_fetch_add`, `try_inc_and_get`
- `Gauge`: `try_set`, `try_add`, `try_sub`, `try_set_max`, `try_set_min`
- `Timer`: `try_record_ns`, `try_record`, `try_record_batch`
- `RateMeter`: `try_tick`, `try_tick_n`, `try_tick_if_under_limit`

Error semantics:
- `MetricsError::Overflow` — arithmetic would overflow/underflow an internal counter.
- `MetricsError::InvalidValue { reason }` — non-finite or otherwise invalid input (e.g., NaN for `Gauge`).
- `MetricsError::OverLimit` — operation would exceed a configured limit (e.g., rate limiting helpers).

Example:

```rust
use metrics_lib::{init, metrics, MetricsError};

init();
let c = metrics().counter("jobs");
c.try_add(10)?;      // Result<(), MetricsError>
let r = metrics().rate("qps");
let allowed = r.try_tick_if_under_limit(1000.0)?; // Result<bool, MetricsError>
```

Panic guarantees: the plain methods (`inc`, `add`, `set`, `tick`, etc.) prioritize speed and may saturate or assume valid inputs. Prefer `try_` variants when you need explicit error handling.

<hr>
<br>

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
metrics-lib = "0.8.6"

# Optional features
metrics-lib = { version = "0.8.6", features = ["async"] }
```

<hr>
<br>

## Quick Start

```rust
use metrics_lib::{init, metrics};

// Initialize once at startup
init();

// Counters - fastest operations (18ns)
metrics().counter("requests").inc();
metrics().counter("errors").add(5);

// Gauges - sub-nanosecond operations (0.6ns)
metrics().gauge("cpu_usage").set(87.3);
metrics().gauge("memory_gb").add(1.5);

// Timers - automatic RAII timing
{
    let _timer = metrics().timer("api_call").start();
    // Your code here - automatically timed on drop
}

// Or time a closure
let result = metrics().time("db_query", || {
    // Database operation
    "user_data"
});

// System health monitoring
let cpu = metrics().system().cpu_used();
let memory_gb = metrics().system().mem_used_gb();

// Rate metering
metrics().rate("api_calls").tick();
```

## Observability Quick Start

- Integration Examples: see `docs/API.md#integration-examples`
- Grafana dashboard (ready to import): `docs/observability/grafana-dashboard.json`
- Prometheus recording rules: `docs/observability/recording-rules.yaml`
- Kubernetes Service: `docs/k8s/service.yaml`
- Prometheus Operator ServiceMonitor: `docs/k8s/servicemonitor.yaml`
- Secured ServiceMonitor (TLS/Bearer): `docs/k8s/servicemonitor-secured.yaml`

Commands

```bash
# Import Grafana dashboard via API
curl -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <GRAFANA_API_TOKEN>" \
  http://<grafana-host>/api/dashboards/db \
  -d @docs/observability/grafana-dashboard.json

# Validate Prometheus recording rules
promtool check rules docs/observability/recording-rules.yaml

# Apply Kubernetes manifests
kubectl apply -f docs/k8s/service.yaml
kubectl apply -f docs/k8s/servicemonitor.yaml
# For secured endpoints
kubectl apply -f docs/k8s/servicemonitor-secured.yaml
```

## Advanced Usage

### Async Support

```rust
use std::time::Duration;
use metrics_lib::{metrics, AsyncMetricBatch, AsyncTimerExt};

// Async timing with zero overhead and typed result
let result: &str = metrics()
    .timer("async_work")
    .time_async(|| async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "completed"
    })
    .await;

// Batched async updates (flush takes &MetricsCore)
let mut batch = AsyncMetricBatch::new();
batch.counter_inc("requests", 1);
batch.gauge_set("cpu", 85.2);
batch.flush(metrics());
```

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

- Quick Tour
  - File: `examples/quick_tour.rs`
  - Run:
    ```bash
    cargo run --example quick_tour --release
    ```

- Async Batch + Timing
  - File: `examples/async_batch_timing.rs`
  - Run:
    ```bash
    cargo run --example async_batch_timing --release
    ```

- Token Bucket Rate Limiter
  - File: `examples/token_bucket_limiter.rs`
  - Run:
    ```bash
    cargo run --example token_bucket_limiter --release
    ```

- Custom Exporter (OpenMetrics-like)
  - File: `examples/custom_exporter_openmetrics.rs`
  - Run:
    ```bash
    cargo run --example custom_exporter_openmetrics --release
    ```

- Axum Middleware Metrics (minimal)
  - File: `examples/axum_middleware_metrics.rs`
  - Run:
    ```bash
    cargo run --example axum_middleware_metrics --release
    ```

- Contention & Admission Demo
  - File: `examples/contention_admission.rs`
  - Run:
    ```bash
    cargo run --example contention_admission --release
    ```

- CPU Stats Overview
  - File: `examples/cpu_stats.rs`
  - Run:
    ```bash
    cargo run --example cpu_stats --release
    ```

- Memory Stats Overview
  - File: `examples/memory_stats.rs`
  - Run:
    ```bash
    cargo run --example memory_stats --release
    ```

- Health Dashboard
  - File: `examples/health_dashboard.rs`
  - Run:
    ```bash
    cargo run --example health_dashboard --release
    ```

- Cache Hit/Miss
  - File: `examples/cache_hit_miss.rs`
  - Run:
    ```bash
    cargo run --example cache_hit_miss --release
    ```

- Broker Throughput
  - File: `examples/broker_throughput.rs`
  - Run:
    ```bash
    cargo run --example broker_throughput --release
    ```

### More Real-World Examples (API Reference)

- Building a Custom Exporter — see `docs/API.md` → [Building a Custom Exporter](./docs/API.md#real-world-custom-exporter)
- Memory Stats: total/used/free + percentages — see `docs/API.md` → [Memory Stats](./docs/API.md#real-world-memory-stats)
- Memory % used for an operation (estimate) — see `docs/API.md` → [Memory % for an operation](./docs/API.md#real-world-memory-percent-operation)
- CPU Stats: total/used/free + percentages — see `docs/API.md` → [CPU Stats](./docs/API.md#real-world-cpu-stats)
- CPU % used for an operation (estimate) — see `docs/API.md` → [CPU % for an operation](./docs/API.md#real-world-cpu-percent-operation)

### Resilience Features

```rust
use metrics_lib::{AdaptiveSampler, SamplingStrategy, MetricCircuitBreaker};

// Adaptive sampling under load
let sampler = AdaptiveSampler::new(SamplingStrategy::Dynamic {
    min_rate: 1,
    max_rate: 100,
    target_throughput: 10000,
});

if sampler.should_sample() {
    metrics().timer("expensive_op").record(duration);
}

// Circuit breaker protection
let breaker = MetricCircuitBreaker::new(Default::default());
if breaker.is_allowed() {
    // Perform operation
    breaker.record_success();
} else {
    // Circuit is open, skip operation
}
```

### System Monitoring

```rust
let health = metrics().system();

println!("CPU: {:.1}%", health.cpu_used());
println!("Memory: {:.1} GB", health.mem_used_gb());
println!("Load: {:.2}", health.load_avg());
println!("Threads: {}", health.thread_count());
```

## Benchmarks

Run the included benchmarks to see performance on your system:

```bash
# Basic performance comparison
cargo run --example benchmark_comparison --release

# Comprehensive benchmarks (Criterion)
cargo bench

# Cross-platform system tests
cargo test --all-features
```

### Interpreting Criterion Results

- Criterion writes reports to `target/criterion/` with per-benchmark statistics and comparisons.
- Key numbers to watch: `time: [low … mean … high]` and outlier percentages.
- Compare runs over time to detect regressions. Store artifacts from CI for historical comparison.
- Benchmarks are microbenchmarks; validate with end-to-end measurements as needed.

#### CI Artifacts

- Pull Requests: CI runs a fast smoke bench and uploads `criterion-reports` with `target/criterion`.
- Nightly: The `Benchmarks` workflow runs full-duration benches on Linux/macOS/Windows and uploads artifacts as `benchmark-results-<os>`.
- You can download these artifacts from the GitHub Actions run page to compare results across commits.

#### Latest CI Benchmarks

View the latest nightly results and artifacts here:

[Latest CI Benchmarks (Benchmarks workflow)](https://github.com/jamesgober/metrics-lib/actions/workflows/bench.yml)

Benchmark history (GitHub Pages):

[Benchmark History (gh-pages)](https://jamesgober.github.io/metrics-lib/benchmark-data/)

**Sample Results** (M1 MacBook Pro):
```
Counter Increment: 4.93 ns/op (202.84 M ops/sec)
Gauge Set:         0.53 ns/op (1886.79 M ops/sec)
Timer Record:      10.87 ns/op (91.99 M ops/sec)
Mixed Operations:  106.39 ns/op (9.40 M ops/sec)
```

<sub>Notes: Latest numbers taken from local Criterion means under `target/criterion/**/new/estimates.json`. Actual throughput varies by CPU and environment; use the GitHub Pages benchmark history for trends.</sub>

### Methodology

- Tooling: Criterion with release builds.
- Flags for stability on local runs: `cargo bench -- -w 3.0 -m 5.0 -n 100` (increase on dedicated runners).
- Environment disclosure (example):
  - CPU: Apple M1 Pro (performance cores)
  - Rust: stable toolchain
  - Target: aarch64-apple-darwin
  - Governor: default (for CI use a performance governor where applicable)

See also: `docs/zero-overhead-proof.md` for assembly inspection and binary size analysis, and `docs/performance-tuning.md` for environment hardening.

## Architecture

### Lock-Free Design
- **Atomic Operations**: All metrics use `Relaxed` ordering for maximum performance
- **Cache-Line Alignment**: 64-byte alignment eliminates false sharing
- **Compare-and-Swap**: Lock-free min/max tracking in timers
- **Thread-Local Storage**: Fast random number generation for sampling

### Memory Layout
```rust
#[repr(align(64))]
pub struct Counter {
    value: AtomicU64,           // 8 bytes
    // 56 bytes padding to cache line boundary
}
```
<br>

### Zero-Cost Abstractions
- **RAII Timers**: Compile-time guaranteed cleanup
- **Async Guards**: No allocation futures for timing
- **Batch Operations**: Vectorized updates for efficiency

<hr>
<br>

## Testing

Comprehensive test suite with **87 unit tests** and **2 documentation tests**:

```bash
# Run all tests
cargo test

# Test with all features
cargo test --all-features

# Run only bench-gated tests (feature-flagged and ignored by default)
cargo test --features bench-tests -- --ignored

# Run benchmarks (Criterion)
cargo bench

# Check for memory leaks (with valgrind)
cargo test --target x86_64-unknown-linux-gnu
```

<hr>
<br>

## Cross-Platform Support

**Tier 1 Support:**
- ✅ Linux (x86_64, aarch64)
- ✅ macOS (x86_64, Apple Silicon)  
- ✅ Windows (x86_64)

**System Integration:**
- Linux: `/proc` filesystem, `sysinfo` APIs
- macOS: `mach` system calls, `sysctl` APIs
- Windows: Performance counters, WMI integration

**Graceful Fallbacks:**
- Unsupported platforms default to portable implementations
- Feature detection at runtime
- No panics on missing system features

<hr>
<br>

## Comparison

| Library | Counter ns/op | Gauge ns/op | Timer ns/op | Memory/Metric | Features |
|---------|---------------|-------------|-------------|---------------|----------|
| **metrics-lib** | **4.93** | **0.53** | **10.87** | **64B** | ✅ Async, Circuit breakers, System monitoring |
| metrics-rs | 85.2 | 23.1 | 167.8 | 256B | ⚠️ No circuit breakers |
| prometheus | 156.7 | 89.4 | 298.3 | 1024B+ | ⚠️ HTTP overhead |
| statsd | 234.1 | 178.9 | 445.2 | 512B+ | ⚠️ Network overhead |

<hr>
<br>

## Configuration

### Feature Flags

```toml
[dependencies]
metrics-lib = { version = "0.8.6", features = [
    "async",     # Async/await support (requires tokio)
    "histogram", # Advanced histogram support
    "all"        # Enable all features
]}
```

### Runtime Configuration

```rust
use metrics_lib::{init_with_config, Config};

let config = Config {
    max_metrics: 10000,
    update_interval_ms: 1000,
    enable_system_metrics: true,
};

init_with_config(config);
```

<hr>
<br>

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md).

### Development Setup

```bash
# Clone repository
git clone https://github.com/jamesgober/metrics-lib.git
cd metrics-lib

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench

# Check formatting and lints
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
```

<hr>
<br>

## Links

- 📚 [Documentation](https://docs.rs/metrics-lib)
- 📦 [Crates.io](https://crates.io/crates/metrics-lib)  
- 🐛 [Issues](https://github.com/jamesgober/metrics-lib/issues)
- 💬 [Discussions](https://github.com/jamesgober/metrics-lib/discussions)

### Guides

- Migrating from metrics-rs: [`docs/migrating-from-metrics-rs.md`](./docs/migrating-from-metrics-rs.md)
- Performance Tuning: [`docs/performance-tuning.md`](./docs/performance-tuning.md)
- Zero-Overhead Proof: [`docs/zero-overhead-proof.md`](./docs/zero-overhead-proof.md)
- API Stability Guarantees: [`docs/api-stability.md`](./docs/api-stability.md)

<br>
<hr>
<br>

<!-- LICENSE
############################################# -->
<div id="license">
    <h2>⚖️ License</h2>
    <p>Licensed under the <b>Apache License</b>, version 2.0 (the <b>"License"</b>); you may not use this software, including, but not limited to the source code, media files, ideas, techniques, or any other associated property or concept belonging to, associated with, or otherwise packaged with this software except in compliance with the <b>License</b>.</p>
    <p>You may obtain a copy of the <b>License</b> at: <a href="http://www.apache.org/licenses/LICENSE-2.0" title="Apache-2.0 License" target="_blank">http://www.apache.org/licenses/LICENSE-2.0</a>.</p>
    <p>Unless required by applicable law or agreed to in writing, software distributed under the <b>License</b> is distributed on an "<b>AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND</b>, either express or implied.</p>
    <p>See the <a href="./LICENSE" title="Software License file">LICENSE</a> file included with this project for the specific language governing permissions and limitations under the <b>License</b>.</p>
</div>

<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
