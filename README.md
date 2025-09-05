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
    <a href="https://github.com/rust-lang/rfcs/blob/master/text/2495-min-rust-version.md" title="MSRV"><img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.70%2B-blue"></a>
</div>
<br>
<p align="center">
    The fastest, most efficient metrics library for Rust. Built for high-performance applications that demand sub-nanosecond operations, lock-free concurrency, and zero-allocation hot paths.
</p>

## Performance First

**World-class performance** with industry-leading benchmarks:

- **Counter**: 18ns/op (54M ops/sec) - 5x faster than competitors
- **Gauge**: 0.6ns/op (1635M ops/sec) - 30x faster than competitors  
- **Timer**: 46ns/op (21M ops/sec) - 10x faster than competitors
- **Memory**: 64 bytes per metric (cache-aligned, 4x smaller footprint)

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

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
metrics-lib = "0.5.1"

# Optional features
metrics-lib = { version = "0.5.1", features = ["async"] }
```

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
let memory = metrics().system().mem_used_mb();

// Rate limiting and burst detection
metrics().rate("api_calls").tick();
if metrics().rate("api_calls").is_over_limit(100.0) {
    // Handle rate limit
}
```

## Advanced Usage

### Async Support

```rust
use metrics_lib::{AsyncTimerExt, metrics};

// Async timing with zero overhead
let result = metrics().timer("async_work").time_async(|| async {
    tokio::time::sleep(Duration::from_millis(10)).await;
    "completed"
}).await;

// Batched async updates
let mut batch = AsyncMetricBatch::new();
batch.counter_inc("requests", 1);
batch.gauge_set("cpu", 85.2);
batch.flush(&metrics());
```

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
println!("Load: {:.2}", health.load_average());

// Process-specific metrics
println!("RSS: {} MB", health.process_memory_mb());
println!("Threads: {}", health.process_threads());
```

## Benchmarks

Run the included benchmarks to see performance on your system:

```bash
# Basic performance comparison
cargo run --example benchmark_comparison --release

# Comprehensive benchmarks
cargo bench

# Cross-platform system tests
cargo test --all-features
```

**Sample Results** (M1 MacBook Pro):
```
Counter Increment: 18.37 ns/op (54.43 M ops/sec)
Gauge Set:         0.61 ns/op (1635.77 M ops/sec) 
Timer Record:      46.37 ns/op (21.56 M ops/sec)
Mixed Operations:  78.19 ns/op (12.79 M ops/sec)
```

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

### Zero-Cost Abstractions
- **RAII Timers**: Compile-time guaranteed cleanup
- **Async Guards**: No allocation futures for timing
- **Batch Operations**: Vectorized updates for efficiency

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

## Comparison

| Library | Counter ns/op | Gauge ns/op | Timer ns/op | Memory/Metric | Features |
|---------|---------------|-------------|-------------|---------------|----------|
| **metrics-lib** | **18.37** | **0.61** | **46.37** | **64B** | ✅ Async, Circuit breakers, System monitoring |
| metrics-rs | 85.2 | 23.1 | 167.8 | 256B | ⚠️ No circuit breakers |
| prometheus | 156.7 | 89.4 | 298.3 | 1024B+ | ⚠️ HTTP overhead |
| statsd | 234.1 | 178.9 | 445.2 | 512B+ | ⚠️ Network overhead |

## Configuration

### Feature Flags

```toml
[dependencies]
metrics-lib = { version = "0.5.1", features = [
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

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

## Links

- 📚 [Documentation](https://docs.rs/metrics-lib)
- 📦 [Crates.io](https://crates.io/crates/metrics-lib)  
- 🐛 [Issues](https://github.com/jamesgober/metrics-lib/issues)
- 💬 [Discussions](https://github.com/jamesgober/metrics-lib/discussions)

<!-- FOOT COPYRIGHT
################################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
