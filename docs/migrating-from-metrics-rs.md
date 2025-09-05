# Migrating from metrics-rs

This guide helps you port codebases using the `metrics` ecosystem (metrics-rs) to `metrics-lib` with minimal friction. The focus is on in-process hot-path metrics with ultra-low overhead.

## Key differences

- API surface
  - metrics-rs: macro-first (`counter!`, `gauge!`, `histogram!`) routed through a global recorder/recorder backend.
  - metrics-lib: direct typed handles (`Counter`, `Gauge`, `Timer`, `RateMeter`) via a thin global `MetricsCore` or explicit instances.
- Overhead model
  - metrics-rs: depends on the installed recorder, label handling, and chosen exporter.
  - metrics-lib: zero-allocation hot paths, lock-free atomics, cache-line alignment; exporting is intentionally kept out of the hot path.

## Common migrations

### Counters

metrics-rs:
```rust
metrics::counter!("requests_total", 1);
```
metrics-lib:
```rust
use metrics_lib::{init, metrics};
init();
metrics().counter("requests_total").inc();
```

Batching:
```rust
// metrics-rs often relies on exporter batching.
// metrics-lib exposes explicit batch ops or manual aggregation.
metrics().counter("requests_total").add(10);
```

### Gauges

metrics-rs:
```rust
metrics::gauge!("cpu_pct", 87.3);
```
metrics-lib:
```rust
metrics().gauge("cpu_pct").set(87.3);
```

### Timers / Histograms

metrics-rs:
```rust
let start = std::time::Instant::now();
// ... work ...
metrics::histogram!("latency", start.elapsed());
```
metrics-lib:
```rust
let t = metrics().timer("latency").start();
// ... work ...
t.stop(); // recorded on drop
```

Or record directly:
```rust
metrics().timer("latency").record(std::time::Duration::from_micros(123));
```

### Rates / Throughput

metrics-rs: derived from counters via observer queries (PromQL, etc.).

metrics-lib: in-process sliding window via `RateMeter` with `tick()`, `rate()`, and burst admission helpers:
```rust
let r = metrics().rate("api_calls");
r.tick();
if r.tick_if_under_limit(1000.0) { /* allowed */ }
```

## Labels and cardinality

- metrics-rs encourages labels; high cardinality can be expensive.
- metrics-lib keeps the hot path label-free. Prefer out-of-band export formatting or background enrichment.

Migration tip: keep a small, stable set of metric names; push optional labels to your exporter/collector stage.

## Exporting

- metrics-rs: exporter-integrated.
- metrics-lib: export via background snapshots (JSON, Prometheus/OpenMetrics bridge, etc.). See `docs/API.md#observability-stack-integration`.

## Error handling and safety

- Prefer `try_` variants (`try_add`, `try_set`, `try_record`) when inputs can be invalid.
- Result-bearing methods use `#[must_use]` so you don’t accidentally drop critical decisions (e.g., rate-limiter admissions).

## Migration patterns

- Initialize once at startup: `init()` / `init_with_config()`.
- Pre-register handles to avoid lookup overhead in hot paths.
- Replace macros with explicit typed handles and method calls.
- Move label logic to exporters or logging layers.

## Validation checklist

- Counters, Gauges, Timers produce the same values as before under a test harness.
- Rate-limiting behavior matches expectations with `RateMeter`.
- Benchmarks on representative hardware show expected improvements.

## Example diff

Before:
```rust
metrics::counter!("jobs", 1);
metrics::gauge!("queue_depth", 42.0);
metrics::histogram!("db_ms", 12.0);
```
After:
```rust
let m = metrics_lib::metrics();
m.counter("jobs").inc();
m.gauge("queue_depth").set(42.0);
m.timer("db_ms").record(std::time::Duration::from_millis(12));
```
