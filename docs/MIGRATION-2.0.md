# Migrating from metrics-lib 1.x to 2.0

2.0 fixes correctness bugs that 1.0 froze into its API (invalid Prometheus
output, wrong memory units, a token bucket that stopped refilling, unbounded
registries, injectable names) and raises the MSRV. Most programs need only
the first two steps below; the rest of this guide lists every breaking change
with the replacement.

Snippets labelled *1.x* show the old code and are not compiled; every Rust
block is compiled and run as a doctest.

## 1. Toolchain and dependency line

- **MSRV is now Rust 1.85** (edition 2024). 1.x supported 1.70.
- Update `Cargo.toml`:

  ```toml
  [dependencies]
  metrics-lib = "2"
  ```

- The crate is now dual-licensed `Apache-2.0 OR MIT` (1.x: Apache-2.0).

## 2. Features

| 1.x | 2.0 |
|---|---|
| system health always compiled | feature `system` (in `default`) |
| `histogram` enabled `sample` | `histogram` stands alone; add `sample` if you use the sampler |
| `async` failed to compile without `meter` | `async` works alone; each batch method needs its metric feature |
| `bench-tests` | removed; Criterion benches are authoritative |
| — | new: `http`, `metrics-rs`, `tower` |

`default` is now `count`, `gauge`, `timer`, `system`.

## 3. Initialisation

`metrics()` no longer panics: it initialises the global instance with the
default configuration on first use. Configure it once at start-up with
`init_with_config` (the function the 1.x docs promised):

```rust
use metrics_lib::{init_with_config, MetricsConfig};

let core = init_with_config(MetricsConfig::default().with_max_series(50_000))
    .expect("first initialisation");
assert_eq!(core.registry().cardinality_cap(), 50_000);
```

`MetricsCore::new()` no longer starts a sampler thread or calls into the
OS. `system()` starts the monitor on first use (feature `system`).

## 4. Names and labels are validated

Names must match `[A-Za-z][A-Za-z0-9_.]*` (at most 200 bytes) and label keys
`[A-Za-z][A-Za-z0-9_]*`. The unchecked lookups repair invalid input
(`my-metric` becomes `my_metric`, `9lives` becomes `m_9lives`) and count it
in `Registry::invalid_inputs`; the `try_*` lookups return
`MetricsError::InvalidName` / `InvalidLabel`. Dotted names keep working and
render with `_` in Prometheus/OpenMetrics.

```rust
use metrics_lib::{MetricsCore, MetricsError};

let core = MetricsCore::new();
core.counter("my-metric").inc(); // stored as `my_metric`
assert_eq!(core.counter("my_metric").get(), 1);
assert!(matches!(core.try_counter("bad name"), Err(MetricsError::InvalidName { .. })));
```

Histograms reserve the label key `le`, summaries and timers `quantile`.

## 5. One name, one kind

In 1.x `counter("x")` and `gauge("x")` were unrelated and produced an
invalid exposition. In 2.0 the second kind is refused: `try_gauge("x")`
returns `MetricsError::KindMismatch`, and `gauge("x")` returns a detached
metric (counted in `Registry::kind_conflicts`). Give the second metric its
own name.

## 6. The series cap counts every series

`set_cardinality_cap` (default 10 000) now bounds unlabeled series too, and
families that only carry metadata take one slot. Past the cap, `try_*`
lookups return `CardinalityExceeded` and the others return a detached
overflow metric. New: `Registry::remove`, `remove_series` and `evict_idle`.

## 7. Registry API

| 1.x | 2.0 |
|---|---|
| `get_or_create_counter(name)` | `counter(name)` (old name deprecated) |
| `get_or_create_counter_with(name, labels)` | `counter_with(name, labels)` (old name deprecated) |
| `try_get_or_create_counter_with` | `try_counter_with` (old name deprecated) |
| same for gauge, timer, rate meter, histogram | same pattern; rate meter is `rate_meter` |
| `describe_*(…)` returned `()` | returns `Result<()>` |
| `configure_histogram(…)` returned `()` | returns `Result<()>`, rejects empty bucket lists |
| `counter_names()` listed unlabeled names | lists every counter family, sorted |

New: `try_counter`/`try_gauge`/… for unlabeled lookups, `sharded_counter*`,
`up_down_counter*`, `summary*`, `configure_summary`, `kind_of`,
`with_cardinality_cap`.

```rust
use metrics_lib::{Registry, Unit};

let registry = Registry::new();
registry.describe_counter("jobs", "Jobs done", Unit::None)?;
registry.counter("jobs").inc();
# Ok::<(), metrics_lib::MetricsError>(())
```

## 8. Metric types

**Timer.** `start()` borrows the timer, which fails to compile on a
temporary (`metrics().timer("x").start()`, error E0716). Use
`start_owned()` or `MetricsCore::start_timer`:

```rust
let core = metrics_lib::MetricsCore::new();
{
    let _guard = core.timer("db_seconds").start_owned();
}
assert_eq!(core.timer("db_seconds").count(), 1);
```

`RunningTimer::stop` returns the elapsed `Duration`; guards are
`#[must_use]` and have `discard()`. The timing helpers moved from
`metrics_lib::utils` to `metrics_lib::timer_utils`, and
`utils::benchmark` (which printed to stdout) is removed. The sum no longer
wraps.

**Counter.** `CounterStats::total` (a copy of `value`) is removed. New
`set_max`.

**Gauge.** `GaugeStats::updates` (always `None`) is removed. `set_max`,
`set_min`, `update_ema` and `clamp` ignore NaN/infinite input instead of
storing it; `clamp` with `min > max` is a no-op instead of a panic. `set`
still stores any value.

**RateMeter.** `tick_n` takes `u64` (was `u32`). `rate()` is a sliding
estimate over the primary window instead of "events since the start of the
current wall-clock second", and `with_window` is honoured.
`rate_per_minute`/`rate_per_hour` are sliding estimates. `RateStats` lost
`window_fill` and gained `rate` and `window`.

**TokenBucket.** Capacity and token counts are `u64`
(`new(capacity: u64, rate)`, `try_acquire(u64)`, `acquire(u64)`,
`available() -> u64`). Refill keeps working for 584 years of uptime instead
of 49.7 days. New: `try_new`, `time_until_available`.

**Histogram.** `HistogramBucket` keeps its 1.x shape (`upper_bound`,
`count`, still `Copy`) but is `#[non_exhaustive]`. Exemplars are in
`HistogramSnapshot::exemplars` (parallel to `buckets`, empty when none were
recorded) and `HistogramSnapshot::exemplar(i)`.
`with_buckets([])` falls back to one `1.0` bucket instead of panicking in
debug builds; `try_with_buckets` reports it.

**System health** (feature `system`). Memory readings on Windows/macOS were
1024× too high; they are now correct. `thread_count()`, `fd_count()` and
`load_avg()` return `Option` instead of made-up constants. `HealthConfig`
replaced `memory_gb` with `memory_percent`. `update_interval_ms()` became
`update_interval() -> Duration`. `SystemSnapshot` reports bytes
(`memory_total_bytes`, `memory_used_bytes`, `process_memory_bytes`).

**Adaptive controls** (feature `sample`). `CircuitBreakerConfig`,
`SamplingStats`, `BackpressureGuard` and the new `CircuitState` are public.
`Fixed { rate: 0 }` samples everything instead of panicking.

**Async** (feature `async`). `AsyncMetricBatch::flush` takes `&mut self` and
the target `&MetricsCore`; `rate_tick` takes a count;
`AsyncMetricsBatcher::new` takes the `Arc<MetricsCore>` to flush into and
`start_flusher` returns a handle with `shutdown()`. `AsyncTimerExt` is
implemented for `Timer` and `Arc<Timer>` (the latter makes `'static`
futures).

## 9. Errors

`MetricsError` is `#[non_exhaustive]` (add a `_` arm). `InvalidName` carries
the name and reason. Removed (never produced in 1.x): `Overloaded`,
`OverLimit`, `NotInitialized`. Added: `InvalidLabel`, `KindMismatch`,
`AlreadyInitialized`. `Config` holds a `Cow<'static, str>`.

`MetricKind`, `Unit`, `MetricMetadata`, the stats/snapshot structs and the
config structs are `#[non_exhaustive]`: build them with their constructors
and `with_*` methods.

## 10. Exporter output

Dashboards may need small query changes:

| Kind | 1.x exposition | 2.0 exposition |
|---|---|---|
| timer `t` | `# TYPE t histogram`, `t_count`, `t_sum_seconds`, `t_min_seconds`, `t_max_seconds` | `# TYPE t summary`, `t_count`, `t_sum`; gauges `t_min`, `t_max` (seconds) |
| rate meter `r` | `# TYPE r gauge`, `r_total`, `r_per_second` | counter `r_total`, gauge `r_per_second` |
| dotted names | written verbatim | `.` written as `_` |
| `# UNIT` in Prometheus format | emitted | not emitted (OpenMetrics only, when the name ends in the unit) |

- `json::render` returns `serde_json::Result<String>`; schema version 2 lists
  series in one array tagged by `"type"`.
- `StatsdSink` sends counters as deltas (`|c`) by default
  (`with_counter_mode(CounterMode::Gauge)` restores totals); negative gauges
  are sent as `0` then the value.
- The per-kind JSON series structs (`CounterSeries`, `GaugeSeries`,
  `TimerSeries`, `RateSeries`, `HistogramSeries`) are replaced by
  `JsonSeries` with a tagged `JsonValue`; the OTLP `NumberData` and
  `HistogramData` structs by the `MetricData` enum.
- OTLP: use `OtlpExporter` (`render`/`render_protobuf` remain as one-shot
  helpers returning `Result`/`Vec<u8>`). Timers export as `Summary` named as
  registered (no `_seconds` suffix); start times are stable per series.

## 11. Getting help

File an issue at <https://github.com/jamesgober/metrics-lib/issues> with
the code that no longer compiles.
