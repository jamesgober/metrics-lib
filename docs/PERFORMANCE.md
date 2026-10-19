# Performance

Measured costs of the hot paths, the registry and the exporters, how they
were measured, and how they compare with 1.0.0. Every number here comes
from code in the repository; how to reproduce it is at the end.

## Conditions

| | |
|---|---|
| CPU | AMD Ryzen 9 9950X3D (16 cores, 32 threads) |
| OS | Windows 11 Pro (x86_64-pc-windows-msvc) |
| Compiler | rustc 1.85.0 for the Criterion tables, 1.95.0 for the 1.0 comparison; `lto = "fat"`, `codegen-units = 1` |
| Criterion | 0.7, `--warm-up-time 2 --measurement-time 4`, 100 samples |
| Date | 2026-10-14 |
| Load | The machine was also running other builds. Medians can be inflated by up to about 15 % between runs; the 1.0 comparison uses the minimum of many interleaved runs to filter that out. |

Times are per operation unless stated otherwise.

## Recording into a held handle

These are the paths to put in hot loops. Criterion medians:

| Operation | Time |
|---|---|
| `Counter::inc` | 4.2 ns |
| `Counter::add` | 4.5 ns |
| `Counter::get` | 0.25 ns |
| `Counter::try_add` (overflow-checked) | 5.2 ns |
| `ShardedCounter::inc` (uncontended) | 4.2 ns |
| `UpDownCounter::add` | 4.4 ns |
| `Gauge::set` | 0.28 ns |
| `Gauge::get` | 0.27 ns |
| `Gauge::add` | 4.4 ns |
| `Gauge::set_max` (no change) | 0.55 ns |
| `Timer::record_ns` | 8.2 ns |
| `Timer::record(Duration)` | 8.3 ns |
| `Timer::start` + `stop` (two clock reads) | 59 ns |
| `Timer::start_owned` + drop | 67 ns |
| `RateMeter::tick` | 44 ns |
| `RateMeter::tick_n` | 44 ns |
| `RateMeter::rate` | 28 ns |
| `Histogram::observe` (11 bounds + `+Inf`) | 13.5 ns |
| `Histogram::observe_with_exemplar` | 84 ns |
| `Summary::observe` | 16.6 ns |
| `TokenBucket::try_acquire` (granted) | 32 ns |
| `TokenBucket::try_acquire` (refused) | 27 ns |

Sub-nanosecond results (`get`, `Gauge::set`) are a plain load or store that
the benchmark loop pipelines; treat them as "free", not as a latency.
`Timer::start`/`stop`, `RateMeter` and `TokenBucket` read the monotonic
clock, which costs about 25–30 ns per read on Windows.

The 1.x README claimed about 1.5 ns for `Counter::inc` and about 3 ns for
`Timer::record`. Neither reproduces with the 1.0.0 code on this machine (see
the comparison below); a `lock xadd` costs a few nanoseconds on current x86
cores even uncontended. The claims are corrected everywhere.

## Comparison with 1.0.0

[`tools/compare-1x`](../tools/compare-1x) links 1.0.0 (from crates.io)
and the working tree into one binary and times each operation in 60
interleaved rounds, three times over; the table shows the minimum per
operation, which filters out preemption on a loaded machine (rustc 1.95,
fat LTO, `cargo run --release` in that directory). Both versions see the
same registry: 200 unlabeled counters, one counter with two labels, and a
histogram with the default bounds holding 1 000 observations. Repeated
runs agreed within about 3 percentage points.

| Operation | 1.0.0 | 2.0.0 | Change |
|---|---|---|---|
| `Counter::inc` | 3.85 ns | 3.90 ns | +1 % |
| `Timer::record_ns` | 7.67 ns | 7.63 ns | −1 % |
| `Histogram::observe` | 12.50 ns | 12.56 ns | +1 % |
| Labeled lookup + `inc` | 97.8 ns | 57.2 ns | −42 % |
| Unlabeled lookup + `inc` | 26.5 ns | 28.6 ns | +8 % |
| `Histogram::snapshot` | 48.7 ns | 51.0 ns | +5 % |
| `RateMeter::tick` | 36.5 ns | 41.2 ns | +13 % |
| `RateMeter::tick_n` | 36.5 ns | 41.4 ns | +14 % |
| `RateMeter::rate` | 22.6 ns | 31.8 ns | +41 % |

Labeled lookups hash the `LabelSet` in place instead of building an owned
`(String, LabelSet)` key, so a hit no longer allocates.

The regressions above 5 % and what they buy:

**Unlabeled lookup, +2 ns.** 1.0 kept one unbounded map per metric type.
2.0 routes the name to one of 16 shards, looks it up in a map of families
that holds every kind (so one name can only ever be one kind), and checks
the kind of the stored series. That is what makes the series cap, the
kind-conflict check and eviction possible. A held handle skips the lookup
entirely (4 ns per increment). A faster keyed hash for names was tried and
measured slower than the standard hasher for names of this length.

**`Histogram::snapshot`, +2 ns.** The snapshot carries an `exemplars`
vector (empty and unallocated when no exemplar was recorded) and checks
whether exemplar storage exists.

**`RateMeter`, +5 ns per tick and +9 ns per `rate()`.** These are the price
of the 1.x bugs it fixes:

- 1.x read the wall clock (`SystemTime`), so an NTP step or a manual clock
  change corrupted its windows. 2.0 reads the monotonic `Instant`, which on
  Windows is a performance-counter read plus a scale to nanoseconds, a few
  nanoseconds more per call.
- 1.x `rate()` returned "events since the start of the current wall-clock
  second", which drops to zero at every boundary and let
  `tick_if_under_limit` admit twice its limit across one. 2.0 computes a
  sliding estimate (previous slot weighted by the elapsed fraction plus the
  current slot) and honours `with_window`; that is three loads and a
  multiply-add instead of one load.
- 1.x counted in `u32` per window (`tick_n` truncated large batches). 2.0
  keeps `u64` second, minute and hour windows (plus the primary window when
  it is not one second), so a tick updates each.

Both paths avoid divisions: the slot test is a multiply and a compare, and
the estimate multiplies by a precomputed reciprocal of the window width.

## Contention

`contended_increments`: N threads each add 100 000 to one shared counter.
Total wall time per run (lower is better).

| Threads | `Counter` | `ShardedCounter` | Ratio |
|---|---|---|---|
| 1 | 0.53 ms | 0.53 ms | 1.0× |
| 4 | 3.0 ms | 0.67 ms | 4.5× |
| 8 | 6.1 ms | 1.18 ms | 5.2× |
| 16 | 12.9 ms | 1.47 ms | 8.8× |

Across runs the eight-thread ratio ranged from 5× to 9×. One `Counter` is a
single cache line that every core fights over; a `ShardedCounter` gives
each thread its own 128-byte stripe and sums the stripes on read (`get`
takes 4.2 ns with the default stripe count).

## Registry lookups

Looking a metric up by name takes a shard read lock and hashes the name
(and the labels). The figures include the `inc` on the returned handle.
Criterion medians, 1 000 other families in the registry:

| Lookup | Time |
|---|---|
| Unlabeled counter, `MetricsCore` | 32 ns |
| Unlabeled counter, global `metrics()` | 32 ns |
| Unlabeled gauge | 28 ns |
| Unlabeled timer | 36 ns |
| Labeled counter, two labels | 61 ns |
| Labeled histogram, two labels | 72 ns |
| Held handle (no lookup) | 4.0 ns |
| `LabelSet::from` two pairs | 35 ns |
| First registration of a labeled series | 0.71 µs |
| `evict_idle` scan over 10 000 idle series | 46 µs |

Concurrent labeled lookups of one hot series (`registry_concurrent`) reach
about 19 M lookups/s with one thread and about 18–19 M/s with four or
eight: the shard's read lock is a shared cache line, so lookups of one
series do not scale with cores. Hold the handle instead.

## Exposition

`exposition`: 100 label sets × (counter, gauge, histogram with the 11
default bounds, timer) = 400 series and about 2 000 lines, rendered into a
reused buffer.

| Render | Time |
|---|---|
| Prometheus text | 0.14 ms |
| OpenMetrics text | 0.14 ms |
| `render_core` (Prometheus text plus the `process_*` families, what `MetricsServer` serves) | 0.14 ms |

That is roughly 0.35 µs per series or 70 ns per line, including the
snapshot of every series, sorting, escaping and the collision checks.

`Histogram::snapshot` takes 58 ns for the default bounds; with exemplars
recorded it takes 213 ns, because each exemplar's labels are copied. Only
the OpenMetrics and JSON exporters ask for exemplars; Prometheus text and
OTLP take a counts-only snapshot. `Summary::snapshot` (four quantiles from
one read of the sketch) takes 0.75 µs.

### Process and host metrics by platform

Criterion medians on the reference host (Windows), `system` group of
`metrics_bench`; the `sysinfo` column is the path Windows used before
(sysinfo 0.36, timed in a scratch binary on the same host, median of 20):

| Operation | In-house backend | `sysinfo` 0.36 |
|---|---|---|
| `ProcessCollector::collect` (one `process_*` sample) | 0.66 µs | 31 ms |
| `SystemHealth::update` (host and process) | 0.14 ms (0.27 ms in the final re-audit, host-dependent) | about 32 ms |
| `render_core`, 400 series plus `process_*` | 0.15 ms | (plus one sample) |

`sysinfo` scans every process on the host to refresh one; the 2.0.0
re-audit measured 330–500 ms per sample on this host under heavier load.
The in-house backends read the current process directly:

| Platform | Backend |
|---|---|
| Linux | `/proc/self/*`, `/proc/stat`, `/proc/meminfo` (tens of microseconds) |
| Windows | `GetProcessTimes`, `K32GetProcessMemoryInfo`, `GetProcessHandleCount`, `GetSystemTimes`, `GlobalMemoryStatusEx` |
| macOS | `getrusage`, `proc_pidinfo`, `getrlimit`, `host_statistics(64)`, `sysctlbyname("hw.memsize")`, `getloadavg` (verified by CI's macOS runners; not timed here) |
| FreeBSD and other targets | `sysinfo` (process-table scan) |

On the `sysinfo` targets `render_core` and `MetricsServer` serve the latest
sample of the background system monitor instead of sampling during the
scrape. Everywhere, the monitor never spends more than a quarter of its
time sampling: when one sample takes longer than a quarter of the interval,
the next one is pushed back.

## Reproducing

```bash
cargo bench --all-features --bench metrics_bench -- --warm-up-time 2 --measurement-time 4
cargo bench --all-features --bench registry_bench -- --warm-up-time 2 --measurement-time 4
```

For 1.0.0, check out `v1.0.0` and run
`cargo bench --features meter --bench metrics_bench`, or build a binary that
depends on both versions under renamed dependencies (`ml1 = { package =
"metrics-lib", version = "1" }`, `ml2 = { package = "metrics-lib", path =
"." }`) and interleave the two, as the comparison above does.

Report the CPU, OS, `rustc -V` and Criterion flags with any figure you
publish, and prefer a quiet machine with a fixed CPU frequency (see
[`performance-tuning.md`](./performance-tuning.md)).
