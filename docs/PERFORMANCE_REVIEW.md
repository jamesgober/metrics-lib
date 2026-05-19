# Performance and Architecture Review (v0.9.4)

This document summarises the performance profile and the architectural
decisions behind `metrics-lib` as of `0.9.4`. Numbers here are reproducible
locally via the commands in [§ Reproducing the numbers](#reproducing-the-numbers).

## Snapshot

- Focus: low-overhead, in-process metrics for hot paths.
- Hot-path strategy: lock-free atomics, cache-line alignment (`#[repr(align(64))]`),
  `Relaxed` ordering on the success path.
- Registry strategy: `RwLock<HashMap<…>>` per metric type (unlabeled fast
  path) + a second `RwLock<HashMap<(String, LabelSet), …>>` per type for
  labeled instances. Read-locked hits are the common case; the write lock is
  only taken on first registration of a `(name, labels)` tuple.
- `SystemHealth`: a **background sampler thread** (v0.9.4) refreshes cached
  atomics every `update_interval_ms`; readers are pure atomic loads — no
  mutex, no syscall, no async-runtime stall.
- Histogram hot path: binary-search bucket lookup (O(log B)) + three
  `Relaxed` atomic operations (one bucket `fetch_add`, one `total`
  `fetch_add`, one `sum_bits` CAS).

## Cached-handle vs global-lookup

Two distinct latencies are worth tracking separately.

**Cached-handle path** — the hot-loop pattern: cache the `Arc<Counter>` /
`Arc<Gauge>` / `Arc<Timer>` / `Arc<RateMeter>` / `Arc<Histogram>` at startup,
then call `.inc()` / `.set()` / `.record()` / `.tick()` / `.observe()` on
the cached reference. This is what the headline microbenchmark numbers
measure.

**Global-lookup path** — `metrics().counter("name").inc()` per call. Each
call goes through:

1. `OnceLock` load of the global `MetricsCore`.
2. `RwLock::read()` on the per-type map.
3. `HashMap::get(&str)` to find the existing `Arc`.
4. `Arc::clone()` to bump the strong count.
5. The actual `.inc()` / `.set()` / etc.

This is substantially slower than the cached path. For label-bearing
lookups (`metrics().counter_with("name", &labels)`) there is an additional
`String` clone + `LabelSet` clone to build the composite key.

The Criterion `cached_vs_global` benchmark group reports both numbers
side-by-side so users can verify the trade-off on their own hardware.

## Verified Benchmark Reference

All benches are in [`benches/metrics_bench.rs`](../benches/metrics_bench.rs).
Run with:

```bash
cargo bench --bench metrics_bench --all-features
```

Bench groups (Criterion):

| Group | Coverage |
|---|---|
| `counter` | `inc`, `add`, `get`, concurrent bursts |
| `gauge` | `set`, `add`, `get`, `set_min`, `set_max`, concurrent add+set |
| `timer` | `record`, `record_ns`, `start_stop`, RAII guard, `stats` |
| `rate_meter` | `tick`, `tick_n`, `rate`, concurrent `tick_n` (4 threads) |
| `global_metrics` | `counter`/`gauge`/`timer`/`mixed_operations` via the global registry |
| `scaling` | counter scaling across 1/2/4/8/16 threads |
| `labels` (v0.9.4) | `LabelSet::from`, `to_prometheus`, hashing |
| `histogram` (v0.9.4) | `observe` (default seconds, 5-bucket, 11-bucket uniform), concurrent observe, `quantile`, `snapshot` |
| `exporters` (v0.9.4) | full-registry render: Prometheus, OpenMetrics, JSON, StatsD, OTLP/HTTP+JSON |
| `cached_vs_global` (v0.9.4) | counter increment on a cached `Arc` vs. `metrics().counter("name").inc()`; also labeled lookup |

## Architectural strengths

### 1) Hot-path execution model

- Atomic operations in core metric types avoid mutex contention.
- Cache-line alignment (`#[repr(align(64))]`) reduces false sharing for
  heavily contended metrics.
- Fast-path methods are minimal — `inc`, `set`, `record_ns`, `tick`,
  `observe` — and `#[inline(always)]` so call-sites compile down to a
  handful of instructions.

### 2) Correctness hardening (v0.9.2 / v0.9.3)

- Counter checked methods (`try_*`) use CAS loops to avoid TOCTOU races.
- Gauge math methods guard non-finite input/result paths.
- Counter wrapping/overflow paths use `wrapping_add` / `saturating_add`
  rather than `+` that would panic in debug builds.
- Timer `record_batch` saturates internally on adversarial input.
- Async pin-projection paths have explicit `SAFETY:` rationale.
- `RateMeter` documents window-boundary event-loss and TOCTOU bounds.

### 3) `SystemHealth` background sampler (v0.9.4)

- A single named OS thread (`metrics-lib-health-sampler`) refreshes the
  cached atomics. Readers (`cpu_used`, `mem_used_mb`, `health_score`, …)
  do one `Relaxed` atomic load — no mutex, no syscall.
- Drop semantics: when the owning `SystemHealth` drops, the sampler
  thread is signalled to stop and joined. The thread sleeps in
  `MAX_SLEEP_CHUNK_MS`-bounded chunks (currently 1 s) so `Drop` latency
  is bounded even on long configured intervals.
- `SystemHealth::manual()` provides a sampler-free instance for
  full-control use cases.

### 4) Feature-gated binary shaping

- Metric types compile out cleanly when feature-disabled.
- Exporters are independent feature flags (`serde` → JSON; `statsd`,
  `otlp`) so consumers only pay for the formats they emit.
- `histogram` and `sample` features cleanly add their hot paths without
  affecting unrelated builds.

### 5) Test and quality gates

- `cargo fmt --all -- --check`, `cargo clippy --all-features
  --all-targets -- -D warnings` (and the same with `--no-default-features`
  and `--features minimal`) all green.
- 200+ unit tests across all metric types and exporters.
- 23 rustdoc-tests with `RUSTDOCFLAGS="-D warnings"` clean.
- Line coverage 91%+ under `cargo llvm-cov --all-features
  --fail-under-lines 85`.
- The 16 timing-coupled `assert!(elapsed/iter < N)` checks that used to
  flake on coverage runs were removed in v0.9.4 — Criterion's
  benchmark-regression-action remains the authoritative regression
  detector.

## Trade-offs and known limits

### 1) Registry is not lock-free

Lookup uses `RwLock<HashMap<…>>`. Read-heavy workloads see almost no
contention because shared read locks are cheap. Writers are only invoked
on first registration of a `(name, labels)` tuple. For workloads that
register tens of thousands of distinct keys per second, the lock can
become a bottleneck — that scenario is better served by caching the
returned `Arc` once and reusing it.

### 2) Labeled lookup allocates the composite key

`metrics().counter_with("name", &labels)` allocates a `(String,
LabelSet)` key on every call (to look up the existing `Arc`). The hot
path is therefore the **cached `Arc`** pattern — call `counter_with` once
at startup and reuse the returned reference. The Criterion
`cached_vs_global` group makes the cost difference visible.

### 3) Bench-test smoke checks are throughput-only

The `mod benchmarks` blocks gated on `feature = "bench-tests"` print
ns/op and verify the operation count, but no longer assert against fixed
thresholds — those assertions flaked on coverage instrumentation and
were the wrong defence anyway (Criterion handles regression detection).
Run Criterion locally and review the GitHub Pages benchmark dashboard
for trend tracking.

### 4) Histogram quantile is bucket-interpolated

`Histogram::quantile(q)` returns a linearly-interpolated estimate from
the cumulative bucket counts. For accurate tail percentiles in
high-cardinality distributions you'd need an HDR-histogram or t-digest
adapter — those are out of scope for v0.9.x.

## Reproducing the numbers

For release-quality confidence, run on a dedicated machine with the
following matrix:

```bash
# Code quality
cargo fmt --all -- --check
cargo clippy --all-features --all-targets -- -D warnings
cargo clippy --no-default-features --all-targets -- -D warnings
cargo clippy --no-default-features --features minimal --all-targets -- -D warnings

# Tests
cargo test
cargo test --all-features
cargo test --no-default-features

# Bench-gated tests (slow; explicit opt-in)
cargo test --all-features --features bench-tests -- --ignored

# Documentation
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps

# Coverage (gate: 85% line)
RUSTFLAGS="--cfg coverage" cargo llvm-cov --summary-only --workspace \
  --all-features --fail-under-lines 85 \
  --ignore-filename-regex '^(dev/|docs/|benches/|examples/)'

# Microbenchmarks (Criterion)
cargo bench --bench metrics_bench --all-features
```

For stable wall-clock numbers, on Linux:

- Pin the CPU governor: `sudo cpupower frequency-set -g performance`.
- Pin the bench to a specific core: `taskset -c 2 cargo bench …`.
- Disable Turbo Boost variability if you need single-thread numbers
  reproducible across runs.

CI runs `cargo bench --bench metrics_bench --all-features` on every push
to `main` and publishes the data to the GitHub Pages dashboard at
<https://jamesgober.github.io/metrics-lib/>.

## Conclusion

`metrics-lib` `0.9.4` adds the background `SystemHealth` sampler thread
and expanded Criterion coverage on top of the v0.9.3 telemetry feature
set. Hot-path arithmetic and ordering choices have not changed since
v0.9.2; the v0.9.4 improvements are concentrated on the side paths
(snapshot reads, exporter rendering, histogram inlining) and on the
visibility infrastructure (benches + dashboard) needed to keep numbers
honest going forward.
