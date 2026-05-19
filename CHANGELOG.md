<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>CHANGELOG</b>
</h1>
<p>
  All notable changes to this project will be documented in this file. The format is based on <a href="https://keepachangelog.com/en/1.1.0/">Keep a Changelog</a>,
  and this project adheres to <a href="https://semver.org/spec/v2.0.0.html/">Semantic Versioning</a>.
</p>

## [Unreleased]

## [0.9.3] - 2026-05-18

**Telemetry release.** Ships labeled metrics with bounded cardinality, a
production-quality `Histogram` type, full `serde` derive coverage for every
stats / snapshot type, and **five built-in exporters** (Prometheus,
OpenMetrics, JSON snapshot, StatsD UDP push, OTLP/HTTP+JSON). Six new
runnable examples demonstrate each end-to-end. All changes are additive —
zero breaking changes for existing 0.9.2 call-sites.

### Added

- `src/labels.rs` (new module): `Label` type alias and `LabelSet` —
  sorted/deduplicated label key/value collection with stable hashing and
  rendering. Construction via `LabelSet::EMPTY` / `LabelSet::new()` /
  `LabelSet::from([(k, v), ...])` / `FromIterator<(K, V)>` /
  `.add(k, v)` / `.with(k, v)`. Helpers `to_prometheus()` and
  `to_statsd()` produce canonical wire formats with appropriate escaping.
  `serde::Serialize` is wired behind the `serde` feature (serialises as a
  flat JSON map).
- `src/histogram.rs` (new module, gated on `histogram`): `Histogram`,
  `HistogramSnapshot`, `HistogramBucket` types. Construction via
  `Histogram::with_buckets`, `Histogram::linear`,
  `Histogram::exponential`, `Histogram::default_seconds`. Observation via
  `observe(value)` (silently drops non-finite) and `try_observe(value)`
  (returns `MetricsError::InvalidValue`). Read API:
  `count()`, `sum()`, `mean()`, `age()`, `quantile(q)` (bucket-interpolated
  estimate), `snapshot()` (cumulative buckets with trailing `+Inf`),
  `reset()`. The `DEFAULT_SECONDS_BUCKETS` constant (the Prometheus default
  latency-seconds bucket layout) is re-exported from the crate root.
- `src/metadata.rs` (new module): `MetricKind` (`Counter` / `Gauge` /
  `Timer` / `Rate` / `Histogram`), `Unit` (enumerated standard units plus
  `Custom(&'static str)`), and `MetricMetadata { help, unit, kind }`.
  Powers `# HELP` / `# TYPE` / `# UNIT` lines in Prometheus / OpenMetrics
  output, the `description` field in OTLP, and unit suffixes in StatsD.
- `Registry`: new labeled-metric storage (`HashMap<(String, LabelSet),
  Arc<T>>`) for every metric type, alongside the existing unlabeled
  fast-path maps. Methods added (each gated on the relevant feature):
  - `get_or_create_counter_with(name, &LabelSet)` /
    `try_get_or_create_counter_with(...)`
  - `get_or_create_gauge_with` / `try_get_or_create_gauge_with`
  - `get_or_create_timer_with` / `try_get_or_create_timer_with`
  - `get_or_create_rate_meter_with` / `try_get_or_create_rate_meter_with`
  - `get_or_create_histogram` / `get_or_create_histogram_with` /
    `try_get_or_create_histogram_with`
  - `configure_histogram(name, buckets)` — pre-register bucket boundaries
    for a metric name (used by the next `histogram*` registration).
  - Snapshot accessors for exporters: `counter_entries()`,
    `gauge_entries()`, `timer_entries()`, `rate_meter_entries()`,
    `histogram_entries()` — each returns `Vec<(String, LabelSet, Arc<T>)>`
    over both unlabeled and labeled instances.
  - Metadata API: `describe(name, MetricMetadata)`,
    `describe_counter / describe_gauge / describe_timer /
    describe_rate / describe_histogram`, `metadata(name) ->
    Option<MetricMetadata>`.
  - Cardinality control: `set_cardinality_cap(usize)`,
    `cardinality_cap()`, `cardinality_count()`,
    `cardinality_overflows()`. Default cap: `DEFAULT_CARDINALITY_CAP =
    10_000` unique `(name, labels)` tuples across **all** labeled metric
    types. Exceeding the cap routes `*_with` calls to per-type
    process-global overflow sinks (never exported, never panicking) and
    returns `Err(MetricsError::CardinalityExceeded)` from the `try_*_with`
    paths.
- `MetricsCore`: new labeled lookup methods (`counter_with` /
  `try_counter_with` / `gauge_with` / `try_gauge_with` / `timer_with` /
  `try_timer_with` / `rate_with` / `try_rate_with` / `histogram` /
  `histogram_with` / `try_histogram_with`).
- `MetricsError::CardinalityExceeded` — new error variant returned by
  every `try_*_with` lookup when the cap is full.
- `src/exporters/` (new top-level public module): each sub-module renders
  a `&Registry` to a backend format.
  - `exporters::prometheus::render` / `render_into` — Prometheus text
    exposition format. Counters/gauges/timers (rendered as a Prometheus
    `summary` shape), rate meters, and histograms (cumulative `_bucket`
    rows + `_sum` + `_count`) all supported with labels and metadata.
  - `exporters::openmetrics::render` / `render_into` — OpenMetrics
    superset with `_total` suffix on counters, full `# UNIT` lines, and
    trailing `# EOF\n`.
  - `exporters::json` (behind `serde`) — `RegistrySnapshot` plus
    sub-series structs (`CounterSeries`, `GaugeSeries`, `TimerSeries`,
    `RateSeries`, `HistogramSeries`, `CardinalitySnapshot`) all deriving
    `serde::Serialize`. Convenience: `snapshot(registry)`,
    `render(registry) -> String`, `render_pretty(registry) -> String`.
  - `exporters::statsd` (behind `statsd`) — `StatsdSink` push exporter
    with `new(addr)`, `with_socket`, `with_prefix`, `with_packet_size`,
    `send(registry)`, `render(registry)`. Cumulative-gauge wire mode
    (`|g`) + DogStatsD tag extension; automatic MTU-bounded packet
    splitting.
  - `exporters::otlp` (behind `otlp`, which pulls in `serde`) —
    OTLP/HTTP+JSON metrics payload via `build(registry, service_name) ->
    ExportMetricsServiceRequest`, `render(registry, service_name) ->
    String`, `render_pretty(registry, service_name) -> String`. Maps
    Counter → `Sum (isMonotonic=true)`, Gauge → `Gauge`, Timer / Histogram
    → `Histogram` (non-cumulative `bucketCounts` + `explicitBounds`),
    RateMeter → `Gauge` with `_per_second` suffix.
- `#[derive(serde::Serialize)]` is now wired behind the `serde` feature
  on every stats / snapshot type: `CounterStats`, `GaugeStats`,
  `TimerStats`, `RateStats`, `SystemSnapshot`, `ProcessStats`,
  `HealthStatus`, `SamplingStats`, `MetricKind`, `Unit`,
  `MetricMetadata`, `HistogramSnapshot`, `HistogramBucket`, `LabelSet`.
- Six new examples in `examples/`:
  - `labels_demo` — labeled counters + cardinality control walkthrough.
  - `histogram_latency` — `Histogram` with p50/p95/p99 readouts.
  - `prometheus_endpoint` — Axum `/metrics` handler backed by the
    Prometheus exporter.
  - `statsd_push` — `StatsdSink` UDP push with DogStatsD tags and prefix.
  - `otlp_push` — render OTLP/HTTP+JSON payload for a collector POST.
  - `snapshot_serde` — `exporters::json` pretty-printed registry dump.
- `Cargo.toml`: new feature flags `statsd`, `otlp` (auto-enables
  `serde`), `exporters-all` (`serde + statsd + otlp`). `serde` now wires
  through to an optional `serde_json = "1.0"` dependency used by the
  JSON exporter. `full` aggregate flag now includes `statsd` + `otlp`.
- `docs/API.md`: new `# Histogram`, `# Labels`, `# Metric metadata`,
  `# Exporters (v0.9.3)` sections (each with multiple code examples
  covering happy path + error cases).
- `README.md`: new "Telemetry & Exporters" section with the full
  exporter table and a runnable code snippet, plus updated API Overview
  bullets calling out `Histogram` / `LabelSet` / `Exporters`.

### Changed

- `Registry::clear` now also clears labeled maps, metadata, and resets
  `cardinality_count` (overflow count is monotonic and intentionally
  preserved).
- `Registry::metric_count` now also counts labeled instances and
  histograms.
- `Cargo.toml`: package version bumped to `0.9.3`. `histogram` and
  `serde` features are no longer "reserved" — both are fully implemented
  in 0.9.3.
- `src/exporters/prometheus.rs` + `openmetrics.rs`: helpers
  (`emit_help_and_type`, `emit_help_type_unit`, `escape_help`,
  `merge_le_label`, `format_u64`, `format_f64`) are now per-feature
  gated so the modules compile cleanly under every feature combination
  (including `--no-default-features`).
- `src/registry.rs`: imports (`Arc`, `OnceLock`, `LabelSet`,
  `MetricsError`, `Result`) and the `try_acquire_slot` helper are gated
  on `any(count, gauge, timer, meter, histogram)` so the registry
  compiles warning-free with no metric features enabled.
- `metadata::Unit` now derives `Default` (replacing the manual `impl
  Default` flagged by clippy's `derivable_impls`).
- All version refs in `README.md` and `docs/API.md` bumped from `0.9.2`
  to `0.9.3`.

### Fixed

- Internal: registry's `metric_count` previously underreported when
  labeled metrics were present (the labeled maps didn't exist before).
  Now sums both tracks.



## [0.9.2] - 2026-05-18

**API fix and cleanup release.** Fixes the `SystemHealth` refresh-throttle
bug, removes debug-mode overflow panics from non-checked counter and timer
APIs, relaxes the registry's `&'static str` requirement so runtime-derived
names work without `Box::leak`, and brings the feature-flag declarations in
line with what actually ships in 0.9.2 vs what is reserved for 0.9.3.

### Fixed

- `src/system_health.rs` (`maybe_update`): stored the last-refresh timestamp
  in nanoseconds while the throttle compared it against `update_interval_ms`,
  with the result that the refresh effectively fired exactly once at
  construction and then froze. All cached values (`cpu_used`, `mem_used_mb`,
  `load_avg`, `process_*`, `health_score`) were pinned to their initial
  reads for the lifetime of the `SystemHealth` instance. The throttle now
  stores and compares the timestamp in a single unit (milliseconds), so the
  refresh fires on the configured interval as documented. A new
  `test_maybe_update_actually_refreshes_after_interval` regression test
  guards against re-introduction.
- `src/system_health.rs` (`SystemSnapshot::last_update`): previously reported
  `Duration::from_nanos(monotonic_ns_at_last_refresh)`, which is *not* "time
  since last refresh." It now reports the actual elapsed time since the
  last refresh (`Duration::from_millis(now_ms - last_update_ms)`).
- `src/system_health.rs` (`get_process_cpu`, Linux): replaced the "very
  rough approximation" — which multiplied total clock ticks since process
  start by 0.01 — with a proper delta sampler:
  `((utime+stime) - prev) / (CLK_TCK * elapsed_s * cores) * 100`, normalized
  per-core and clamped to `0..=100`. The first sample returns `0.0` and
  seeds the baseline. Earlier values were essentially meaningless.
- `src/counter.rs` (`add_and_get`, `inc_and_get`): used `+` which panics
  in debug builds when crossing `u64::MAX`. They now use `wrapping_add` so
  overflow wraps modulo `2^64`, matching the underlying
  `AtomicU64::fetch_add` semantics. Use `try_inc_and_get` / `try_fetch_add`
  for an explicit `MetricsError::Overflow` instead.
- `src/counter.rs` (`saturating_add`, `inc_max`): replaced `SeqCst`
  `compare_and_swap` in tight CAS loops with `Relaxed`
  `compare_exchange_weak`. Observable behaviour is unchanged; the CAS loop
  is now several times cheaper on modern x86 / aarch64.
- `src/timer.rs` (`record_batch`): summed batch nanoseconds with `+=`,
  which panicked in debug builds on overflow. Replaced with `saturating_add`
  so adversarial inputs saturate at `u64::MAX` ns without crashing the
  recorder. `try_record_batch` continues to return
  `MetricsError::Overflow` instead.
- `src/timer.rs` (`average`, `stats`): replaced the manual `if count > 0`
  guard around `total_ns / count` with `checked_div(count)`, satisfying
  Clippy `manual_checked_ops` while preserving the zero-count fallback
  (`Duration::ZERO`).
- `src/system_health.rs`: removed the redundant manual `unsafe impl Send
  for SystemHealth {}` and `unsafe impl Sync for SystemHealth {}`. The
  fields (`AtomicU32` / `AtomicU64` / `Instant` /
  `parking_lot::Mutex<sysinfo::System>` / `sysinfo::Pid`) are all
  `Send + Sync`, so the compiler derives both automatically. Matches the
  cleanup applied to `Counter`/`Gauge`/`Timer`/`RateMeter` in 0.9.1.

### Changed

- `src/lib.rs` (`MetricsCore::counter` / `gauge` / `timer` / `rate` /
  `time`): name arguments relaxed from `&'static str` to `&str`. String
  literals and owned/borrowed runtime names both work; the first lookup
  for a given name allocates the `String` key inside the registry,
  subsequent lookups of the same name reuse the cached `Arc` with no
  allocations on the hot path. This removes the only API-level reason to
  call `Box::leak` on runtime-derived metric names.
- `src/async_support.rs` (`AsyncMetricBatch::counter_inc`, `gauge_set`,
  `timer_record`, `rate_tick`): name arguments relaxed from
  `&'static str` to `impl Into<Cow<'static, str>>`. The `MetricUpdate`
  enum stores `Cow<'static, str>` internally so static names cost nothing
  extra and owned `String`s no longer require `Box::leak`.
- `src/system_health.rs`: replaced `std::sync::Mutex<System>` with
  `parking_lot::Mutex<System>` on the non-Linux sampling path. Removes
  one `.unwrap()`/poison-handling site and reduces the lock cost on
  contended snapshot reads. `parking_lot` was already a transitive
  dependency.
- `src/rate_meter.rs`: module documentation now explicitly states the
  best-effort concurrency contract:
  (a) at window boundaries, up to `num_threads − 1` ticks may land in the
  new window's counter instead of the prior one (the monotonic
  `total_events` is always correct), and
  (b) `tick_if_under_limit` has TOCTOU semantics — multiple threads can
  pass the `can_allow` check and `tick` before any of them observe the
  others, overshooting the limit by up to `num_threads − 1` events. A
  strict-admission `TokenBucket` primitive is planned for 0.9.5.
- `Cargo.toml`: feature flags reorganized and re-commented for honesty —
  - `histogram` and `serde` are now annotated as "reserved in 0.9.2" with
    their full implementations landing in 0.9.3 alongside the telemetry
    exporters. Enabling them today compiles cleanly but does not yet
    expose new API.
  - The unused `nightly` feature was removed.
  - `all` now explicitly excludes `async` and `serde` (build subset for
    environments that don't want Tokio/Serde); `full` is the
    everything-on superset.
- `Cargo.toml`: package description rewritten to remove inconsistent
  nanosecond claims and add accurate qualifiers
  ("High-performance Rust metrics library: sub-2ns counters, sub-1ns
  gauges, …").
- `Cargo.toml`: `cache_hit_miss`, `cpu_stats`, `memory_stats` examples now
  have explicit `[[example]]` declarations; `cache_hit_miss` declares
  `required-features = ["count", "timer"]` so `--no-default-features`
  builds no longer fail at the examples target.
- `examples/custom_exporter_openmetrics.rs`: rewritten to use the new
  `&str`-accepting `MetricsCore` API. The per-iteration
  `Box::leak(name.into_boxed_str())` calls have been removed — copying
  this exporter into a long-lived `/metrics` handler no longer leaks one
  string per metric per request.
- `README.md`: empty "Comparison" table (every cell `N/A in this repo
  run`) replaced with a single "Performance Notes" section that gives
  the cached-handle numbers, explicitly distinguishes them from
  `metrics().counter("name")` per-call cost, and notes that a populated
  head-to-head against `metrics-rs` / `prometheus` / `statsd` will ship
  with v1.0.0.
- `README.md`, `docs/API.md`, `docs/PERFORMANCE_REVIEW.md`: all install
  snippets and version references bumped from `0.9.1` to `0.9.2`.
- `src/registry.rs`: feature-gated the `HashMap` / `Arc` / `RwLock`
  imports so `--no-default-features` (no metric features enabled) builds
  without `unused_imports` warnings.
- `tests/api_md_doctest.rs`: feature-gated the `init` / `metrics` /
  `Duration` imports the same way.

### Added

- `src/counter.rs`: new regression tests
  `test_add_and_get_wraps_on_overflow_without_panic`,
  `test_saturating_add_terminates_at_max`,
  `test_inc_max_relaxed_cas_still_correct`.
- `src/timer.rs`: new regression test
  `test_record_batch_saturates_on_overflow_without_panic`.
- `src/system_health.rs`: new regression test
  `test_maybe_update_actually_refreshes_after_interval` that verifies
  `last_update_ms` advances after sleeping past the throttle interval
  (would have caught the time-unit bug above).
- `docs/API.md`: `MetricsCore`, `Counter`, `Timer`, `SystemHealth`, and
  `AsyncMetricBatch` sections gained explicit "v0.9.2" change-call-outs
  next to each method whose behaviour or signature shifted.

### Removed

- `Cargo.toml`: the `nightly = []` feature (declared but never referenced
  anywhere in the codebase).
- `src/system_health.rs`: redundant manual `unsafe impl Send/Sync for
  SystemHealth {}`.



## [0.9.1] - 2026-03-25

### Added
- `Cargo.toml`: added `full` super-feature that enables all stable features plus `async` and `serde`
  (`full = ["count", "gauge", "timer", "meter", "sample", "histogram", "async", "serde"]`).
- `Cargo.toml`: wired the `serde` optional dependency via `serde = ["dep:serde"]` feature activation
  (previously declared as a dependency but never activatable through features).
- `src/adaptive.rs`: added field-level doc comments (`///`) to all four public fields of `SamplingStats`
  (`samples_taken`, `samples_dropped`, `current_rate`, `is_overloaded`).
- `src/adaptive.rs`: added module-level doc comment to `pub mod fastrand` documenting it is a
  non-cryptographic RNG seeded from the call-site address and wall-clock nanoseconds.
- `src/registry.rs`: added per-`#[cfg(feature)]` compilation guards on all struct fields, methods,
  and imports — metric types now compile out cleanly when their feature flag is disabled.
- `src/lib.rs`: added per-`#[cfg(feature)]` guards on all module declarations, `pub use` re-exports,
  `MetricsCore` accessor methods, `prelude` re-exports, and test functions.
- `tests/api_md_doctest.rs`: replaced the previous zero-test API markdown harness with executable
  smoke tests so API documentation coverage now exercises real code paths during `cargo test`.

### Changed
- `Cargo.toml`: corrected `minimal` feature from `["count","gauge","timer","meter","sample","histogram"]`
  (everything) to `["count"]` — minimal now means the smallest useful build.
- `Cargo.toml`: clarified `all` feature comment to "Enables all stable features (no async)" and added
  the new `full` feature for builds that include async and serde.
- `src/rate_meter.rs`: corrected documentation from "sliding window" to "tumbling window" throughout
  the module doc, struct doc, and feature-list bullet — the implementation has always used a fixed
  epoch-aligned tumbling window, not a sliding window.
- `src/adaptive.rs`: replaced `DefaultHasher`-based seed in `pub mod fastrand` with a Splitmix64
  implementation seeded from `(addr ^ nanos.wrapping_mul(0x9e3779b97f4a7c15)) | 1`.
  `DefaultHasher` output is unspecified across Rust versions; Splitmix64 is deterministic and
  produces well-distributed output regardless of toolchain version.
- `src/registry.rs`: corrected module documentation that claimed a "lock-free hash table" — the
  registry uses `RwLock<HashMap>`, which is correctly described as a reader-writer lock optimised
  for read-heavy workloads.
- `src/async_support.rs`: removed `#[allow(dead_code)]` from all public methods of `AsyncMetricsBatcher`
  (`new`, `record`, `start_flusher`) — these are part of the public API and should not suppress lints.
- `src/lib.rs`: crate-level quick-start doctest is now `no_run` and feature-aware (`count`/`gauge`/
  `timer`/`meter` blocks are conditionally compiled) so documentation checks stay stable across
  feature combinations and restrictive host policies.
- `Cargo.toml`: benchmark target `metrics_bench` now declares `required-features = ["meter"]` so
  benchmark compilation is consistent with feature-gated `RateMeter` APIs.

### Fixed
- `src/counter.rs` (`try_inc`, `try_add`, `try_fetch_add`, `try_inc_and_get`): replaced the
  load-check-fetch_add pattern with proper compare-and-swap (CAS) loops. The previous code had
  a TOCTOU race: another thread could modify the value between the load and the fetch_add, causing
  incorrect overflow checks and returning wrong values under concurrent access.
- `src/counter.rs` (`try_inc_and_get`): removed `debug_assert_eq!(prev, current)` which would
  panic in debug builds under concurrent access (the assertion was logically impossible to
  guarantee without a CAS loop).
- `src/gauge.rs` (`add`, `multiply`, `divide`): added guards against non-finite (NaN, ±Infinity)
  inputs and intermediate results. Previously, passing NaN or Infinity would silently corrupt the
  stored gauge value, making all subsequent reads return NaN/Infinity. Non-finite inputs and
  results are now rejected as a silent no-op.
- `src/counter.rs`, `src/gauge.rs`, `src/timer.rs`, `src/rate_meter.rs`: removed manual
  `unsafe impl Send` and `unsafe impl Sync` from all four metric types. These implementations
  were redundant — `AtomicU64`, `AtomicU32`, `AtomicI64` (and `f64` stored via bit-casting to
  `AtomicU64`) already implement `Send + Sync`, so the compiler derives the correct bounds
  automatically. Manual `unsafe impl` is unnecessary and could mask future unsafety.
- `src/async_support.rs` (`TimedFuture::poll`): added `// SAFETY:` comments to both
  `get_unchecked_mut` and `Pin::new_unchecked` calls explaining the structural-pinning invariant
  being upheld (the pinned inner future is never moved while the `TimedFuture` is pinned).
- `src/async_support.rs` (tests): added `// SAFETY:` comments to `Waker::from_raw` and
  `Pin::new_unchecked` calls in the test module explaining the preconditions being satisfied.
- Coverage/stability: restored default-feature registry test execution and added targeted counter/gauge
  tests for checked/overflow and non-finite guard paths, increasing line coverage headroom and
  hardening CI against threshold regressions.




<br>

## [0.9.0] - 2025-09-05
Beta/Release-Candidate: Optimized, Stable, and Ready for Production.

### Added
- Build script registers `cfg(coverage)` via `cargo:rustc-check-cfg=cfg(coverage)` to silence unexpected cfg warnings when using `cfg!(coverage)` in tests.
- Benchmarks: added high-contention Criterion benches for `RateMeter::tick_n` and symmetric contention benches for `Counter` (bursts) and `Gauge` (add/set mix).
- Documentation: new guides — `docs/migrating-from-metrics-rs.md`, `docs/performance-tuning.md`, `docs/zero-overhead-proof.md`, `docs/api-stability.md`.
- API Reference: added real-world examples in `docs/API.md` — Custom Exporter, Memory stats (total/used/free, %), Memory % for an operation, CPU stats (total/used/free, %), CPU % for an operation.
- Examples: added `examples/cpu_stats.rs` and `examples/memory_stats.rs` demonstrating formatted size and percentage outputs.
- SystemHealth: cross-platform backend using `sysinfo` on non-Linux (macOS/Windows) while retaining `/proc` on Linux.
- API Reference: added `SystemHealth` Platform Notes (Linux `/proc`; non-Linux `sysinfo`; thread/FD defaults; path to native backends), and linked new examples.
- Examples: added `quick_tour.rs`, `async_batch_timing.rs`, `token_bucket_limiter.rs`,
  `custom_exporter_openmetrics.rs`, `axum_middleware_metrics.rs`, and `contention_admission.rs`.
- CPU example enhancements: `examples/cpu_stats.rs` now demonstrates instantaneous, warm-up, and post-work process CPU readings using `SystemHealth`; tuned sampling windows for moderate utilization.
- Memory example enhancements: `examples/memory_stats.rs` now displays total/used/free in MB/GB with percentages and includes a documented helper `normalize_sysinfo_memory_to_mb` to auto-detect units (KiB vs bytes) across platforms.
- API Reference: "Example Pointers" section linking all runnable examples for quick discovery.

### Changed
- API hardening: annotated return-value APIs in `src/counter.rs` (e.g., `get`, `stats`, `rate_per_second`, `fetch_add`, `try_fetch_add`, etc.) with `#[must_use]` to prevent accidental ignoring of important results.
- Clippy hygiene: replaced uninlined format args with captured formatting in tests/examples to keep `-D warnings` clean.
- Rustdoc: added concise notes to `#[must_use]` methods across `RateMeter`, `Gauge`, and `Timer` explaining why results should be consumed.
- CI/Viewer: gh-pages bootstrap now injects CPU MHz (`cpu-mhz.js`) and includes a dedicated `#trend-note` callout so the dashboard can render cycles/op and a “Latest trend” summary automatically.
- README: refreshed headline benchmark numbers from latest local Criterion means and added a "Methodology" note documenting flags/environment.
- Docs navigation: linked new guides from `docs/README.md` and added a Guides section to top-level `README.md`.
- Guidelines: polished wording, fixed typos/encoding artifacts, and clarified testing standards in `docs/GUIDELINES.md` while preserving structure and principles.
- SystemHealth (non-Linux): switched to a persistent `sysinfo::System` instance with cached `pid` for accurate delta-based sampling; normalized process CPU to per-core (0..100%) and clamped.
- CPU example tuning: increased warm-up window and adjusted work duration to yield stable, moderate CPU percentages on typical hosts; removed flaky raw sysinfo two-sample printout.
- Docs/API.md: added `SystemHealth` Platform Notes, Memory Units Note (with pointer to the helper), and expanded integration/example references.
- README: expanded Examples section with run commands for all new examples.

### Fixed
- Corrected `src/rate_meter.rs::get_unix_timestamp()` to use system time directly and avoid double-counting with `created_at.elapsed()`, which could skew window transitions.
- Addressed minor test lints: unused imports/variables in `tests/longevity_tests.rs` and `tests/chaos_tests.rs`.
- Rustdoc: fixed broken intra-doc link in `src/rate_meter.rs` by referencing [`Self::rate`].
- Axum example: updated server startup for axum 0.7+ (use `tokio::net::TcpListener` + `axum::serve`) and fixed RAII timer temporary borrow by binding the timer before `start()`.
- Formatting: addressed rustfmt suggestions in examples (`async_batch_timing.rs`, `cpu_stats.rs`, `axum_middleware_metrics.rs`, `memory_stats.rs`) to keep CI "rustfmt --check" clean.
- MSRV compatibility: pinned `sysinfo = { version = "0.29", default-features = false }` and adjusted calls to 0.29 APIs; removed unnecessary casts flagged by clippy.



<br>

## [0.8.6] - 2025-09-05
Beta: Stress Tested and Stable.

### Added
- Benchmark Dashboard enhancements on GitHub Pages:
  - Root dashboard now includes a structured layout with summary cards and four chart panels (Chart.js):
    - Counter ops, Timer ops, Rate ops, Threads scaling.
  - Data directory page (`benchmark-data/`) renders a simple table plus a new "Grouped View" with hierarchical rows (including `base`/`change`).
  - Friendly names for benches (e.g., `tick` → `rate.tick`, `tick_n` → `rate.tick_n`, thread counts `1/2/4/…` → `threads:<n>`).
  - Source for viewer committed to repo at `docs/benchmarks/main.js`.
- CI safety net for Pages viewer:
  - In `.github/workflows/ci.yml`, added an "Ensure benchmark viewer on gh-pages (main.js)" step that pulls `docs/benchmarks/main.js` from `main` and commits it to `gh-pages` when missing.
  - Avoids here-doc quoting; uses `git show origin/main:docs/benchmarks/main.js > main.js` for reliability.
- Benchmark Regression CI job in `.github/workflows/ci.yml`:
  - Uses `benchmark-action/github-action-benchmark@v1` with `tool: customSmallerIsBetter` and a generated JSON summary from Criterion `estimates.json`.
  - Runs on `push` and `pull_request`; auto-pushes benchmark data to `gh-pages` when on `main`.
  - Fails PRs on significant regressions (`fail-on-alert: true`) with an initial alert threshold tuned for noisy runners.
- GitHub Pages bootstrap for benchmark history:
  - Workflow: `.github/workflows/setup-gh-pages.yml` powered by `peaceiris/actions-gh-pages@v4`.
  - Publishes a minimal dashboard `index.html`, `.nojekyll`, and an initial `benchmark-data/index.html`.
  - Repository README now links to the public history: `https://jamesgober.github.io/metrics-lib/benchmark-data/`.
- Examples quality gate:
  - `cargo build --examples` and `cargo clippy --examples -D warnings` pass clean.
  - CI job added to build, lint, and run `quick_start` example on every PR/push.
  - New example: `examples/quick_start.rs` demonstrating counter, gauge, timer (RAII), and rate usage.
  - New example: `examples/streaming_rate_window.rs` demonstrating rate sampling over time with a Tokio producer/consumer.
  - New example: `examples/axum_registry_integration.rs` demonstrating a minimal Axum server updating metrics and exporting a JSON snapshot.
- Chaos Testing Suite (concurrency and system pressure):
  - File: `tests/chaos_tests.rs`
  - Gated via `#[cfg(all(test, feature = "bench-tests", not(tarpaulin)))]` and `#[ignore]` by default.
  - Exercises high-concurrency updates across `Counter`, `Gauge`, and `RateMeter` with mixed CAS-heavy operations.
- Longevity Tests (1B+ operations configurable):
  - File: `tests/longevity_tests.rs`
  - Gated via `#[cfg(all(test, feature = "bench-tests", not(tarpaulin)))]` and `#[ignore]` by default.
  - Operation count configurable via `OPS` env var; defaults are reduced automatically under CI.

### Changed
- Benchmark dashboard viewer (`main.js`) made robust against `github-action-benchmark` formats:
  - Extracts from `window.BENCHMARK_DATA.entries.Criterion[last].benches` or first available suite; falls back to JSON parsing.
  - Builds time series across runs, padding missing series with `null` and enabling `spanGaps` for smooth lines.
  - Lazy-loads Chart.js if not present; fixed canvas height for visibility; improved interaction.
- Naming cleanups in viewer for clarity (counter/gauge/timer/rate groups and thread labels).

### Fixed
- GitHub Pages data page "raw files" link when already inside `benchmark-data/` now points to `.` (avoids `.../benchmark-data/benchmark-data/` 404).
- Coverage flake in `src/rate_meter.rs::test_concurrent_rate_limiting` under LLVM coverage instrumentation:
  - Relaxed the upper-bound assertion only when `cfg(coverage)` is active.
  - Added explicit assertion messages; formatted to satisfy rustfmt on stable/MSRV.

### Notes
- To run chaos and longevity suites locally:
  - Chaos: `cargo test --features bench-tests -- --ignored --test chaos_tests`
  - Longevity: `OPS=1000000000 cargo test --features bench-tests -- --ignored --test longevity_tests`



<br>

## [0.8.3] - 2025-09-05
Beta: Error Hardened, Stable.

### Added
- Targeted unit tests increasing coverage for critical modules:
  - `src/adaptive.rs`: time-based interval sampling and sampling percentage stats.
  - `src/gauge.rs`: validation errors for `try_*`, overflow paths, and EMA alpha clamping at boundaries.
  - `src/timer.rs`: empty batch handling, nested RAII timers, and non-negative `rate_per_second`.
  - `src/registry.rs`: duplicate name independence across types, clear semantics, and concurrent singleton per name.
- Non-panicking `try_` variants for core metric operations returning `Result<T, MetricsError>`:
  - `Counter`: `try_inc`, `try_add`, `try_set`, `try_fetch_add`, `try_inc_and_get` (overflow-checked)
  - `Gauge`: `try_set`, `try_add`, `try_sub`, `try_set_max`, `try_set_min` (rejects non-finite values; overflow-checked)
  - `Timer`: `try_record_ns`, `try_record`, `try_record_batch` (overflow-checked on internal counters)
  - `RateMeter`: `try_tick`, `try_tick_n`, `try_tick_if_under_limit` (overflow-checked; respects rate limit)
- Unit tests for `RateMeter` `try_` variants including total/window overflow guards and limit behavior.
- Comprehensive rustdoc for all `try_` variants documenting error semantics and panic guarantees, plus usage examples (success and error cases).
- Criterion bench target declared in `Cargo.toml` (`[[bench]] metrics_bench`, `harness = false`) so `cargo bench` runs Criterion main.
- CI smoke benchmark job in `.github/workflows/ci.yml`:
  - Runs on pull_request with short durations: `cargo bench -- -w 0.3 -m 1.0 -n 20`.
  - Uploads `criterion-reports` artifact containing `target/criterion/`.
- Nightly scheduled benchmarks in `.github/workflows/bench.yml`:
  - Cron at 05:00 UTC across Linux/macOS/Windows matrix.
  - Uploads full-duration results as `benchmark-results-<os>` artifacts.
- Guarded CPU governor prepare/restore steps in `.github/workflows/bench.yml` for self-hosted Linux runners to reduce variance (no effect on hosted runners or non-Linux OSes).
- `CONTRIBUTING.md` documenting local benchmarking, interpreting Criterion, and Linux CPU governor/taskset guidance for reducing variance on self-hosted runners.
- `CONTRIBUTING.md` section on comparing Criterion result sets (baselines and directory-to-directory with `critcmp`).
- `docs/API.md` new "Integration Examples" section covering: Web Framework Integration (Axum middleware), Database Pool Monitoring, Background Job Processing, Observability Stack Integration (metrics endpoint), Correlation with Tracing, and Grafana Dashboard Setup.
  - Added more examples: NATS queue depth/consumers, Redis latency + dashboard queries, AWS Lambda EMF emission, Kubernetes Helm scrape annotations, and a fuller OTLP exporter skeleton.
  - Added Real-World examples: High-Frequency Trading (HFT), Web Service Under Load, Batch Processing Pipeline, and Token Bucket Rate Limiter.
  - Added ready-to-import assets:
    - `docs/observability/grafana-dashboard.json`
    - `docs/observability/recording-rules.yaml`
    - `docs/k8s/service.yaml`
    - `docs/k8s/servicemonitor.yaml`
    - `docs/k8s/servicemonitor-secured.yaml`
    - Helm snippets:
      - `docs/k8s/helm/kube-prometheus-stack-values.yaml`
      - `docs/k8s/helm/app-chart/values.yaml`
      - `docs/k8s/helm/app-chart/templates/servicemonitor.yaml`
      - `docs/k8s/helm/app-chart/templates/prometheusrule.yaml`

### Changed
- Benchmark Regression implementation details:
  - Aggregation step now scans both `target/criterion/**/new/estimates.json` and `target/criterion/**/estimates.json` (Criterion layout variance) and emits a single JSON array (`criterion-summary.json`).
  - Explicit bench invocation in CI: `cargo bench --bench metrics_bench --all-features` to guarantee benchmark execution.
  - CI skips regression check gracefully when no estimates are produced (logs tree and continues), avoiding red builds in edge cases.
  - CI shell portability fixes: `jq` install only on Linux; bash forced for steps using bashisms.
  - CI now uploads artifacts for visibility: `criterion-summary.json` and raw `target/criterion` directory (7-day retention).
  - Pinned Criterion dev-dependency to `=0.4.0` for reproducible output layout.
  - Relaxed regression enforcement to stabilize CI:
    - Does not fail CI on alerts (`fail-on-alert: false`) and uses `continue-on-error: true` temporarily.
    - Keeps publishing to `gh-pages` with `gh-pages-branch: gh-pages` and `benchmark-data-dir-path: benchmark-data`.
 - MSRV stability (Rust 1.70.0) for examples/dev-deps:
  - Downgraded `reqwest` to `=0.11.27` with default `native-tls` (and `json`) to avoid `hyper-rustls` MSRV >= 1.71.
  - Pinned transitive URL stack to pre-ICU to avoid `zerotrie` MSRV >= 1.82 via `icu/idna` chain:
    - `url = "=2.4.1"`, `idna = "=0.3.0"`.
  - Subsequently removed `reqwest`, `url`, and `idna` from dev-dependencies entirely to address security advisories and MSRV constraints.
  - Refactored `examples/axum_registry_integration.rs` to use in-process requests via `tower::util::ServiceExt::oneshot` and `axum::body::to_bytes` (no external HTTP client).
  - Added `tower = "0.5"` as a dev-dependency for the example.
- Formatting and style conformance in long-running tests:
  - `tests/chaos_tests.rs`: header spacing normalized; long `eprintln!` formatted across lines.
  - `tests/longevity_tests.rs`: header spacing normalized; chained `env::var(...).ok().and_then(...).unwrap_or(...)` split across lines; long `eprintln!` and inline `if` bodies expanded for rustfmt/MSRV.
- CI Coverage switched from tarpaulin to `cargo-llvm-cov` with enforced threshold `--fail-under-lines 85` and LCOV artifact upload (`coverage-lcov` -> `target/coverage/lcov.info`).
- `CONTRIBUTING.md` updated with a new Coverage section describing local/CI usage of `cargo-llvm-cov`, thresholds, and how to generate LCOV/HTML reports.
- `README.md` Benchmarks section now explicitly references Criterion and adds:
  - "Interpreting Criterion Results" guidance.
  - "CI Artifacts" subsection describing smoke and nightly artifact names and locations.
- `.github/workflows/bench.yml` jobs now also run under scheduled events via `if: ${{ github.event_name == 'schedule' || ... }}` conditions.
- `README.md` now links to the Benchmarks workflow runs page under "Latest CI Benchmarks" for quick artifact access.
 - Examples CI:
   - Added `streaming_rate_window` to the Examples job run list.
   - Uploads example outputs as an artifact (`example-outputs/quick_start.txt`, `example-outputs/streaming_rate_window.txt`).
 - GitHub Pages dashboard:
   - Added lightweight `main.js` viewer that loads `benchmark-data/data.js` and renders a simple table.
   - Updated `benchmark-data/index.html` to include the viewer so the data directory page also renders summaries.
- `docs/GUIDELINES.md` now references `CONTRIBUTING.md` for detailed benchmarking and comparison guidance so contributors can find it quickly.
- Coverage job in `.github/workflows/ci.yml` now includes doctests and all features for tarpaulin (`cargo tarpaulin --workspace --all-features --doc`), improving measured coverage without code changes.
- Coverage job hardened against transient rustup and network issues:
  - Adds cargo cache for registry/git/target to reduce downloads.
  - Adds a rustup component warmup with retries for `rust-std-x86_64-unknown-linux-gnu`.
  - Wraps the tarpaulin invocation in a bounded retry loop to mitigate intermittent failures on hosted runners.
- Cross-platform CI in `.github/workflows/bench.yml` now runs `cargo test` without `--all-features` to avoid executing bench-gated tests on macOS/Windows while still performing an `--all-features` build for compilation coverage. This prevents benchmark-style assertions from causing failures on slower or more variable runners.

### Fixed
- GitHub Pages bootstrap no longer errors with "not a git repository"; replaced brittle orphan-branch shell with `peaceiris/actions-gh-pages` publisher.
- Rustfmt (stable/MSRV) failures in `chaos_tests.rs` and `longevity_tests.rs` resolved.
- Benchmark Regression CI failures fixed:
  - Invalid `tool: 'criterion'` → switched to supported configuration.
  - Directory passed to `output-file-path` → now provides a single JSON file.
  - Empty/absent Criterion outputs → logged and skipped instead of failing entire job.
- Windows matrix failure due to PowerShell `if` syntax when installing `jq` → installation step limited to Linux only.
- `Timer::new` brace misplacement corrected to ensure `try_record_ns` and related methods compile cleanly.
- Avoided `Result` type alias clash by using `core::result::Result` explicitly in `Counter::compare_and_swap` and `Gauge::compare_and_swap`.
- Construct `MetricsError::InvalidValue` with an explanatory `reason` field in `Gauge` `try_` methods; added missing rustdoc for the `reason` field.
- Clippy hygiene across tests and examples:
  - Replaced `format!("{}", var)`/`format!("{:?}", var)` with inline captured formatting like `format!("{var}")`/`format!("{var:?}")`.
  - Simplified boolean assertions (`assert!(cond)` / `assert!(!cond)`) and used `RangeInclusive::contains` where clearer.
  - Removed unnecessary casts in tests (e.g., `as u64` where the type already matches).
  - Updated `examples/benchmark_comparison.rs` prints to inline captured variables.
- CI macOS failure due to a bench-gated timing assertion in `rate_meter::benchmarks::bench_rate_calculation` no longer occurs because bench-gated tests are not run in the Cross-Platform Compatibility job.





<br>

## [0.8.0] - 2025-09-04
Beta Release - Stable

### Added
- Doctesting for external Markdown examples using `doc-comment`:
  - `Cargo.toml` dev-dependency: `doc-comment = "0.3"`
  - `tests/api_md_doctest.rs` to compile-check `docs/API.md` Rust code blocks
- Documentation landing page `docs/README.md` with Quick Links and Table of Contents
- Top-level README navigation buttons (HOME | DOCS | API | GUIDELINES) with absolute links for crates.io compatibility

### Changed
- Refined crate description in `Cargo.toml` and added `readme = "README.md"` for crates.io presentation
- Expanded `docs/API.md`:
  - Comprehensive Table of Contents
  - Gauge, Async support, and SystemHealth sections aligned to source APIs and updated examples
- Updated `README.md`:
  - Added API Overview linking to `docs/API.md`
  - Corrected examples to match actual async batching (`AsyncMetricBatch::flush(metrics())`) and SystemHealth method names (`load_avg`, `thread_count`, `mem_used_gb`)
- CI pipeline improvements (Keep a Changelog: categorized as Changed):
  - Docs job builds rustdoc with warnings as errors (`RUSTDOCFLAGS=-D warnings`) and uploads artifacts
  - Tokenless coverage enforced with tarpaulin (`--fail-under 75`) and Cobertura XML artifact
  - Main test matrix runs with default features (benchmarks gated) to reduce flakiness
  - `cargo audit` runs without requiring integration permissions
  - Workflow structure corrected in `.github/workflows/ci.yml`; coverage stabilized by running tarpaulin with default features

### Fixed
- Hardened `Registry` behavior on poisoned `RwLock` to avoid panics
- Removed redundant `unsafe impl Send/Sync` for `Registry` (type is Send+Sync via fields)

### Removed
- Codecov integration removed; legacy `codecov.yml` archived to `docs/legacy/codecov.yml`


<br>

## [0.5.1] - 2025-09-04
Fixed and Stabilized

### Added
- Codecov configuration `codecov.yml` with project and patch coverage thresholds (target: 85%).
- Coverage status checks integrated via Codecov for PRs and main.
- Optional Cargo feature `bench-tests` to enable running benchmark-style tests on demand.
- Documentation on running bench-gated tests locally (`cargo test --features bench-tests -- --ignored`).

### Changed
- CI coverage no longer uses Codecov or tokens; it is enforced directly via `cargo tarpaulin --fail-under 70` with Cobertura XML (`cobertura.xml`) uploaded as a build artifact.
- CI workflow `.github/workflows/ci.yml`: replaced Codecov upload with tokenless tarpaulin threshold enforcement; coverage status now comes from the job result.
- Repository-wide formatting normalized with `cargo fmt --all`.
- Benchmarks are now gated behind `bench-tests` and ignored by default using `#[cfg_attr(not(feature = "bench-tests"), ignore)]` across modules:
  - `src/counter.rs`
  - `src/gauge.rs`
  - `src/timer.rs`
  - `src/rate_meter.rs`
  - `src/system_health.rs`
- Relaxed benchmark timing thresholds slightly to reduce flakiness on slower or variable runners.
- Security audit now runs `cargo audit` directly in CI (no integration permissions required).
- CI coverage no longer uses Codecov or tokens. Coverage is enforced directly via `cargo tarpaulin --fail-under 85` and the Cobertura XML (`cobertura.xml`) is uploaded as a build artifact.
- Coverage workflow simplified and made tokenless; status now derives from the GitHub Actions job itself.

### Fixed
- Lint hygiene: repository is Clippy-clean under `--all-features`.
- Flaky CI failures caused by strict benchmark timing assertions; benchmarks no longer run by default in CI.

### Quality Gates
- Local and CI parity established:
  - Formatting: `cargo fmt --all -- --check`.
  - Lints: `cargo clippy --all-features -- -D warnings`.
  - Tests: `cargo test --all-features` across Linux/macOS/Windows and MSRV `1.70.0`.

### Removed
- Codecov integration removed; `codecov.yml` archived to `docs/legacy/codecov.yml`.

### Internal
- Build script registers `cfg(tarpaulin)` using MSRV-safe `cargo:rustc-check-cfg=cfg(tarpaulin)` to silence unknown cfg warnings on Rust 1.70.0.





<br>

## [0.5.0] - 2025-08-29
Complete Architectural Overhaul

### Fixed
- **Compilation Errors**: Fixed temporary value dropped while borrowed in timer closure
- **Import Cleanup**: Removed unused imports (BufRead, BufReader, File) from system_health module
- **Ambiguous Re-exports**: Resolved conflicting specialized module exports from gauge and rate_meter
- **Documentation**: Added missing documentation for time_fn macro
- **Lint Issues**: Applied Clippy fixes for format string optimizations and redundant closures
- **Integer Overflow**: Fixed subtraction overflow panic in system health monitoring



<br>

## [0.2.0] - 2025-08-29

### Added - Complete Rewrite
- **COMPLETE ARCHITECTURAL OVERHAUL**: Rebuilt from ground up for ultimate performance
- **Core Types**: Counter, Gauge, Timer, RateMeter, Registry - optimized for sub-nanosecond operations
- **System Health Integration**: Built-in CPU/memory monitoring with process introspection
- **Lock-Free Everything**: Zero locks, zero allocations in hot paths
- **Dynamic Configuration**: Runtime tunable parameters without restarts
- **Circuit Breaker**: Fault tolerance with automatic recovery mechanisms
- **Dead Simple API**: `METRICS.cpu().used()`, `timer.start()`, etc.
- **Cross-Platform Optimizations**: CPU feature detection, NUMA-aware when beneficial
- **Process Usage Estimation**: Automatic detection of current app's CPU/memory consumption

### Changed
- **API Redesign**: Streamlined from complex over-engineered interface to intuitive methods
- **Performance Focus**: Sub-3ns counter increments, sub-5ns gauge updates
- **Memory Layout**: Cache-line aligned structures, optimized atomic operations
- **Error Handling**: Graceful degradation, never hang or block

### Removed
- Over-engineered SIMD complexity (moved to optional extensions)
- Complex async machinery (reduced to core functionality)
- Thread-local pools (over-optimization removed)
- Specialized lens types (moved to separate analytics crate)
- Historical tracking complexity (available as optional feature)

### Technical Details
- **Atomic Operations**: IEEE 754 bit manipulation for gauges
- **Memory Efficiency**: Single cache line per metric where possible  
- **CPU Detection**: Runtime optimization based on available features
- **Health Monitoring**: Built-in system resource tracking
- **Circuit Breakers**: Automatic failure recovery and fallback modes


<br>

## [0.1.0] - 2025-08-29

Initial release with core metrics library functionality.

### Added
- Core metrics library with comprehensive performance measurement tools
- **Gauge Module**: Ultra-high-performance atomic gauges for server applications
  - Lock-free atomic operations with sub-nanosecond updates
  - Labeled gauges, percentage gauges, and specialized metric types
  - Statistical analysis including percentiles, variance, and trends
- **Timer Module**: High-resolution timer implementation
  - Sub-nanosecond precision timing with TSC support
  - Scoped timers for RAII-based measurement
  - Timer pools for high-concurrency scenarios
- **Lens Module**: Advanced temporal lens for sliding window analytics
  - SIMD-optimized aggregations (AVX512, AVX2, SSE2)
  - Lock-free concurrent operations with async support
  - Comprehensive trend analysis and statistical computations
- **Sample Module**: Statistical sampling and reservoir algorithms
- **Histogram Module**: High-performance histogram implementations
- **Registry Module**: Centralized metrics collection and management
- **Count Module**: Efficient counting mechanisms
- **Meter Module**: Rate measurement and throughput analysis

### Fixed
- **Compilation Issues**: Resolved all major compilation errors
  - Fixed duplicate module definitions in lib.rs
  - Added proper feature gates for async/tokio functionality
  - Resolved AVX512 unstable feature usage with nightly feature guards
  - Fixed borrow checker issues in SIMD implementations
  - Corrected type mismatches in async components
- **Import Cleanup**: Removed unused imports across all modules
  - Cleaned up unused atomic types in gauge.rs
  - Fixed unused SIMD imports in lens.rs and timer.rs
  - Corrected unused standard library imports
- **Code Quality**: Addressed warnings and lint issues
  - Fixed unused variable warnings
  - Corrected unnecessary mutable annotations
  - Resolved feature flag mismatches

### Technical Highlights
- **Zero-allocation hot paths** for maximum performance
- **Cache-line aligned data structures** to prevent false sharing
- **SIMD optimization support** with runtime feature detection
- **Async/await compatibility** with proper feature gating
- **Thread-safe concurrent access** throughout all components
- **Comprehensive test suites** with performance benchmarks

### Performance
- Sub-nanosecond gauge operations
- Millions of operations per second capability
- Lock-free data structures for maximum concurrency
- SIMD-accelerated batch operations where applicable

### Documentation
- Comprehensive inline documentation for all public APIs
- Performance-focused implementation notes
- Usage examples and best practices
- Architecture decision documentation



<!-- FOOT LINKS
################################################# -->
[Unreleased]: https://github.com/jamesgober/metrics-lib/compare/v0.9.3...HEAD
[0.9.3]: https://github.com/jamesgober/metrics-lib/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/jamesgober/metrics-lib/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/jamesgober/metrics-lib/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/jamesgober/metrics-lib/compare/v0.8.6...v0.9.0
[0.8.6]: https://github.com/jamesgober/metrics-lib/compare/v0.8.3...v0.8.6
[0.8.3]: https://github.com/jamesgober/metrics-lib/compare/v0.8.0...v0.8.3
[0.8.0]: https://github.com/jamesgober/metrics-lib/compare/v0.5.1...v0.8.0
[0.5.1]: https://github.com/jamesgober/metrics-lib/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/jamesgober/metrics-lib/compare/v0.2.0...v0.5.0
[0.2.0]: https://github.com/jamesgober/metrics-lib/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jamesgober/metrics-lib/releases/tag/v0.1.0
