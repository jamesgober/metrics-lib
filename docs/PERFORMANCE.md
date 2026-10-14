# Performance

Measured costs of the hot paths, the registry and the exporters, how they
were measured, and how they compare with 1.0.0 on the same machine. Every
number here comes from the Criterion benches in `benches/`; how to reproduce
them is at the end.

## Conditions

| | |
|---|---|
| CPU | AMD Ryzen 9 9950X3D (16 cores, 32 threads) |
| OS | Windows 11 Pro (x86_64-pc-windows-msvc) |
| Compiler | rustc 1.85.0, bench profile with `lto = "fat"`, `codegen-units = 1` |
| Criterion | 0.7, `--warm-up-time 2 --measurement-time 4`, 100 samples |
| Date | 2026-10-13 |
| Load | The machine was also running other builds. Figures are medians and should be read as upper bounds; repeated runs varied by up to about 15 %. |

Times are per operation unless stated otherwise. "1.0.0" figures come from
the 1.0.0 benches run on the same machine on the same day; a dash means 1.0
had no comparable benchmark or feature.

## Recording into a held handle

These are the paths to put in hot loops: one atomic operation (or a short
compare-and-swap loop) on a handle you already hold.

| Operation | 2.0.0 | 1.0.0 |
|---|---|---|
| `Counter::inc` | 4.5 ns | 4.5 ns |
| `Counter::add` | 4.5 ns | 4.5 ns |
| `Counter::get` | 0.45 ns | — |
| `Counter::try_add` (overflow-checked) | 5.4 ns | — |
| `ShardedCounter::inc` (uncontended) | 5.2 ns | — |
| `UpDownCounter::add` | 4.5 ns | — |
| `Gauge::set` | 0.30 ns | 0.25 ns |
| `Gauge::get` | 0.36 ns | — |
| `Gauge::add` | 4.8 ns | 4.8 ns |
| `Gauge::set_max` (no change) | 0.94 ns | — |
| `Timer::record_ns` | 9.3 ns | 8.8 ns |
| `Timer::record(Duration)` | 9.2 ns | 8.9 ns |
| `Timer::start` + `stop` (two clock reads) | 64 ns | 63 ns |
| `Timer::start_owned` + drop | 70 ns | — |
| `RateMeter::tick` | 48 ns | 49 ns |
| `RateMeter::tick_n` | 52 ns | 46 ns |
| `RateMeter::rate` | 39 ns | 35 ns |
| `Histogram::observe` (11 bounds + `+Inf`) | 15 ns | 19 ns |
| `Histogram::observe_with_exemplar` | 99 ns | — |
| `Summary::observe` | 20 ns | — |
| `TokenBucket::try_acquire` (granted) | 34 ns | — |
| `TokenBucket::try_acquire` (refused) | 29 ns | — |

Sub-nanosecond results (`get`, `Gauge::set`) are a plain load or store that
the benchmark loop pipelines; treat them as "free", not as a latency.
`Timer::start`/`stop`, `RateMeter` and `TokenBucket` read the monotonic
clock, which costs about 25–30 ns per read on Windows.

The 1.x README claimed about 1.5 ns for `Counter::inc` and about 3 ns for
`Timer::record`. Neither reproduced on this machine with the 1.0.0 code
(4.5 ns and 8.9 ns above); a `lock xadd` costs a few nanoseconds on current
x86 cores even uncontended. The claims are corrected everywhere.

## Contention

`contended_increments`: N threads each add 100 000 to one shared counter.
Total wall time per run (lower is better).

| Threads | `Counter` | `ShardedCounter` | Ratio |
|---|---|---|---|
| 1 | 0.53 ms | 0.53 ms | 1.0× |
| 4 | 3.2 ms | 1.0 ms | 3.1× |
| 8 | 7.8 ms | 0.89 ms | 8.7× |
| 16 | 13.1 ms | 1.15 ms | 11.4× |

A second run measured 6.7 ms against 0.89 ms at eight threads (7.5×). One
`Counter` is a single cache line that every core fights over; a
`ShardedCounter` gives each thread its own 128-byte stripe and sums the
stripes on read (`get` takes 5.5 ns with the default stripe count).

## Registry lookups

Looking a metric up by name takes a shard read lock and hashes the name (and
the labels). The figures include the `inc` on the returned handle.

| Lookup | 2.0.0 | 1.0.0 |
|---|---|---|
| Unlabeled counter, `MetricsCore` | 38 ns | — |
| Unlabeled counter, global `metrics()` | 40 ns | 36 ns |
| Unlabeled gauge | 35 ns | — |
| Unlabeled timer | 44 ns | — |
| Labeled counter (2.0: two labels; 1.0: one label) | 84 ns | 136 ns |
| Labeled histogram, two labels | 100 ns | — |
| Held handle (no lookup) | 4.4 ns | — |
| `LabelSet::from` two pairs | 43 ns | 62 ns |
| First registration of a labeled series | 0.88 µs | — |
| `evict_idle` scan over 10 000 idle series | 50 µs | — |

Labeled lookups in 2.0 hash the `LabelSet` in place instead of building an
owned `(String, LabelSet)` key, so a hit does not allocate. Unlabeled
lookups are 2–4 ns slower than in 1.0 (within the run-to-run spread, but
consistent): 2.0 routes them through the same family map as labeled series
and checks the metric kind, which is what makes the series cap and the
one-name-one-kind rule possible.

Concurrent labeled lookups of one hot series (`registry_concurrent`) reach
about 16 M lookups/s with one thread and about 17 M/s with four or eight:
the shard's read lock is a shared cache line, so lookups of one series do
not scale with cores. Hold the handle instead (previous section).

## Exposition

`exposition`: 100 label sets × (counter, gauge, histogram with the 11
default bounds, timer) = 400 series and about 2 000 lines, rendered into a
reused buffer.

| Format | 2.0.0 |
|---|---|
| Prometheus text | 0.24 ms |
| OpenMetrics text | 0.29 ms |

That is roughly 0.6 µs per series or 120 ns per line, including the
snapshot of every series, sorting, escaping and the collision checks. 1.0
had no benchmark of comparable size (its 28-series render took 17 µs).

`Histogram::snapshot` takes 133 ns for the 11 default bounds (1.0: 83 ns). The
difference is the exemplar slot each bucket now carries.

## Reproducing

```bash
cargo bench --all-features --bench metrics_bench -- --warm-up-time 2 --measurement-time 4
cargo bench --all-features --bench registry_bench -- --warm-up-time 2 --measurement-time 4
```

For 1.0.0, check out `v1.0.0` and run
`cargo bench --features meter --bench metrics_bench`.

Report the CPU, OS, `rustc -V` and Criterion flags with any figure you
publish, and prefer a quiet machine with a fixed CPU frequency (see
[`performance-tuning.md`](./performance-tuning.md)).
