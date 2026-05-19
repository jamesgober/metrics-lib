# API Stability — metrics-lib 1.0

`metrics-lib 1.0.0` is the API freeze. This document defines exactly what
the crate commits to and what it does not, so consumers can decide
confidently how to depend on it.

## The promise

The crate follows [Semantic Versioning](https://semver.org/) with the
strict cargo interpretation:

- **`1.x.y` releases never break the documented public surface.** Adding
  new variants, methods, types, modules, or feature flags is always
  allowed and shipped as a minor (`1.X.0`) bump. Removing, renaming, or
  changing the signature of anything listed in
  [§ Frozen surface](#frozen-surface) requires a major (`2.0.0`) bump.
- **MSRV (Minimum Supported Rust Version) is `1.70`** at 1.0.0. MSRV
  bumps within `1.x` are treated as minor changes (advertised in the
  CHANGELOG) but not as breaking — consumers on older toolchains should
  pin `metrics-lib = "=1.M.P"` if they need a fixed MSRV.
- **The `[features]` set is part of the public API.** Renaming or
  removing a feature flag is a `2.0` change. Adding new feature flags
  is additive (minor).

## Frozen surface

Every item below is committed at `1.0.0`. The list is exhaustive — if
something is not listed here and is reachable from `metrics_lib::*`, it
is **not** part of the stability promise.

### Crate-root globals

- `metrics_lib::METRICS: OnceLock<MetricsCore>`
- `metrics_lib::init() -> &'static MetricsCore`
- `metrics_lib::metrics() -> &'static MetricsCore`
- `metrics_lib::Result<T>` (alias for `std::result::Result<T, MetricsError>`)
- `metrics_lib::MetricsError` (enum + every variant currently exposed)
- `metrics_lib::MetricsCore` (struct + every `pub fn` shown by `cargo
  doc --no-deps`)

### Core metric types

- `Counter`, `CounterStats` (gated on `count`)
- `Gauge`, `GaugeStats`, `gauge_specialized::PercentageGauge`,
  `gauge_specialized::CpuGauge`, `gauge_specialized::MemoryGauge` (gated
  on `gauge`)
- `Timer`, `RunningTimer<'a>`, `TimerStats`, the `time_block!` and
  `time_fn!` macros, `metrics_lib::utils::*` timing helpers (gated on
  `timer`)
- `RateMeter`, `RateStats`, `rate_meter_specialized::ApiRateLimiter`,
  `rate_meter_specialized::ThroughputMeter` (gated on `meter`)
- `Histogram`, `HistogramSnapshot`, `HistogramBucket`,
  `DEFAULT_SECONDS_BUCKETS` (gated on `histogram`)

### Labels & metadata

- `Label` (type alias), `LabelSet`
- `MetricKind`, `MetricMetadata`, `Unit` (including `Unit::Custom`)

### Registry

- `Registry`, `ScopedRegistry<'a>`, `DEFAULT_CARDINALITY_CAP`

### System health

- `SystemHealth`, `SystemSnapshot`, `ProcessStats`, `HealthStatus`,
  `HealthConfig`, `Step`

### Token bucket (strict admission)

- `TokenBucket`

### Async support (gated on `async`)

- `AsyncTimerExt`, `AsyncTimerGuard<'a>`, `AsyncMetricBatch`,
  `AsyncMetricsBatcher`

### Adaptive controls (gated on `sample`)

- `AdaptiveSampler`, `SamplingStrategy`, `MetricCircuitBreaker`,
  `BackpressureController`

### `tracing` integration (gated on `tracing`)

- `metrics_lib::tracing_ext::time_in_span`
- `metrics_lib::tracing_ext::time_global`

### Exporters

- `metrics_lib::exporters::prometheus::{render, render_into}`
  (always available)
- `metrics_lib::exporters::openmetrics::{render, render_into}`
  (always available)
- `metrics_lib::exporters::json::{snapshot, render, render_pretty}` +
  the `RegistrySnapshot` / `CounterSeries` / `GaugeSeries` /
  `TimerSeries` / `RateSeries` / `HistogramSeries` /
  `CardinalitySnapshot` value types (gated on `serde`)
- `metrics_lib::exporters::statsd::StatsdSink` (gated on `statsd`)
- `metrics_lib::exporters::otlp::{render, render_pretty, build,
  ExportMetricsServiceRequest, ResourceMetrics, ScopeMetrics, Resource,
  InstrumentationScope, Metric, MetricData, NumberData, NumberDataPoint,
  NumberValue, HistogramData, HistogramDataPoint, KeyValue, AnyValue}`
  (gated on `otlp`)

### Prelude

- `metrics_lib::prelude::*` — see
  [`docs/API.md#prelude`](./API.md#prelude) for the exact list. The
  prelude itself is part of the frozen surface; its contents are
  versioned so that re-exporting new items from it counts as a minor
  bump.

### Cargo features

The feature set is frozen at 1.0.0:

`count`, `gauge`, `timer`, `meter`, `sample`, `histogram`, `bench-tests`,
`async`, `serde`, `statsd`, `otlp`, `tracing`, `exporters-all`, `all`,
`full`, `default`, `minimal`.

## Explicitly NOT part of the promise

- Anything inside a `#[doc(hidden)]` module, item, or method.
- The `#[cfg(test)]` and `#[cfg(all(test, feature = "bench-tests", …))]`
  modules.
- Internal performance characteristics. The Criterion benchmarks track
  per-version trends, but `Counter::inc` does not commit to a specific
  ns/op number across hardware. Don't depend on relative ordering
  between, say, `metrics().counter(name)` and `metrics().counter_with(name, &labels)`
  beyond "the cached-handle path is at least as fast."
- The exact text of error display messages (`MetricsError::Display`).
  The variants and their discriminants are stable; the human-readable
  format is not.
- The dependency tree below the top-level dependency declarations in
  `Cargo.toml`. Patch / minor bumps of transitive dependencies happen
  as a normal part of `1.x` development.
- The `MSRV` for newly added feature flags. A new feature introduced in
  `1.M.0` may require a newer toolchain than the rest of the crate;
  this is announced in the CHANGELOG.

## Behavioural contracts worth quoting

These are the corner-of-the-spec behaviours that consumers may rely on:

- **`Counter`** is monotonic across `inc` / `add` / `fetch_add` /
  `add_and_get`. `reset` and `set` exist and are the only ways to
  decrease the value. `try_inc` / `try_add` / `try_fetch_add` /
  `try_inc_and_get` return `Err(MetricsError::Overflow)` iff the
  arithmetic would overflow `u64::MAX`; the counter is unchanged on
  overflow.
- **`Gauge` `try_*` methods reject non-finite inputs**
  (`f64::NAN`, `f64::INFINITY`, `f64::NEG_INFINITY`) with
  `Err(MetricsError::InvalidValue { reason })` and do not mutate the
  gauge. The non-`try` methods silently ignore non-finite inputs.
- **`Timer::record_batch`** saturates internally on overflow (no panic);
  `try_record_batch` returns `Err(MetricsError::Overflow)`.
- **`RateMeter::tick_if_under_limit`** has documented TOCTOU semantics
  — it may briefly overshoot the limit by up to `num_threads − 1`
  events per window. Use `TokenBucket::try_acquire` when strict
  admission is required.
- **`TokenBucket::try_acquire`** never exceeds the configured capacity,
  proven by an in-tree property test
  (`tests/proptests.rs::token_bucket_no_overshoot_no_refill`).
- **`Registry::scoped(prefix)`** is name-rewriting only — the same
  `Arc<Counter>` (etc.) backs `scoped("p.").counter("x")` and
  `counter("p.x")`.
- **`SystemHealth` readers are atomic-load-only** — no mutex, no
  syscall, no async-runtime stall, regardless of platform. A background
  sampler thread (or explicit `update()` calls in `manual()` mode) is
  the only writer.
- **Cardinality cap** defaults to `DEFAULT_CARDINALITY_CAP = 10_000`
  unique `(name, labels)` tuples across **all** labeled metric types.
  `try_*_with` methods return `Err(MetricsError::CardinalityExceeded)`
  when full; non-`try` `*_with` methods route to a process-global
  per-type overflow sink (never exported, never panicking).

## What changes in `2.0.0`

The roadmap that follows 1.0 is intentionally undocumented at the
SemVer level. Plausible breaking changes that would justify a `2.0`:

- Migrating `criterion` to `0.5+` and removing the
  `RUSTSEC-2024-0375` (`atty` unmaintained) ignore from `deny.toml`.
- Switching the registry's labeled-lookup composite key to a more
  efficient encoding (e.g. interned strings) — would affect public
  storage types if they're exposed.
- Generalising the histogram percentile path (HDR-histogram, t-digest)
  in a way that changes return types.

Any of these will be announced via a `2.0-pre` cycle on `main` first.
