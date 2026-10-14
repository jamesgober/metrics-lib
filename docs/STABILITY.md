# API Stability — metrics-lib 2.x

This document defines what metrics-lib 2.x commits to, so you can decide how
to depend on it. It replaces the 1.0 document, whose promise of "additive
variants in minor releases" could not hold for exhaustive enums.

## The promise

metrics-lib follows [Semantic Versioning](https://semver.org/) with Cargo's
interpretation:

- **2.x releases do not break the public API.** Anything reachable from
  `metrics_lib::*` keeps its signature and documented behaviour until 3.0.
- **Additive growth is allowed in minor releases.** The public enums and the
  structs with public fields are `#[non_exhaustive]`, so new variants and
  fields can be added in a minor release without breaking `match`
  statements (they need a `_` arm) or construction (use the constructors and
  `with_*` methods).
- **MSRV is Rust 1.85.** Raising it is a major change.
- **Feature flags are API.** Removing or renaming a feature is a major
  change; adding one is minor. Features are additive: enabling one never
  disables anything.
- **Deprecation.** Items are deprecated with `#[deprecated(since, note)]`
  for at least one minor release and removed only in the next major. The
  1.x registry names (`get_or_create_*`) are deprecated in 2.0 and will be
  removed in 3.0.
- **Yanking.** Only for a critical correctness or security bug, together
  with a same-day patch release.

## What is covered

Everything public, in particular:

- the metric types (`Counter`, `ShardedCounter`, `UpDownCounter`, `Gauge`,
  `Timer`, `RateMeter`, `Histogram`, `Summary`, `TokenBucket`) and their
  stats/snapshot types;
- `Registry`, `ScopedRegistry`, `MetricsCore`, `MetricsConfig`, the global
  functions (`init`, `init_with_config`, `metrics`, `try_metrics`) and
  `METRICS`;
- `LabelSet`, the validation functions and limits (`MAX_NAME_LEN`, …);
- `MetricsError` and `Result`;
- the exporters' functions and types, and the **shape of their output**: the
  family and sample names, types and units described in each exporter's
  module documentation (a 2.x release will not rename an exported series);
- the feature-gated modules (`async_support`, `tracing_ext`, `metrics_rs`,
  `tower`, `exporters::http`) and the `sample` and `system` types.

## What is not covered

- Exact numeric output of estimates (`RateMeter` sliding estimates,
  `Summary` quantiles within their stated error bound, histogram
  interpolation), `Debug` output, error message text, and the order of
  hash-based iteration where the docs do not promise one.
- Performance figures. They are measured and published (see
  `docs/PERFORMANCE.md`), not promised.
- The exact set of `sysinfo` versions used on targets without an in-house
  system backend (FreeBSD and the other BSDs, …), as long as
  the MSRV holds.
- Items hidden from the documentation and anything named `__*`.

## Behavioural contracts

- **Names and labels.** Every name and label that reaches an exporter is
  valid for it. Invalid input is refused by the `try_*` lookups and
  repaired by the others (`Registry::invalid_inputs` counts repairs).
- **One name, one kind.** A name registered as one kind is refused for
  another (`MetricsError::KindMismatch`); the unchecked lookups return a
  detached metric and count it in `Registry::kind_conflicts`.
- **Bounded registry.** The number of series plus metadata-only families
  never exceeds `Registry::cardinality_cap`.
- **Valid expositions.** Prometheus and OpenMetrics output always parses:
  one `TYPE` per family, no sample-name clashes (a clashing family is
  skipped and counted in `Registry::exposition_conflicts`), cumulative
  histograms with `+Inf` equal to `_count`, `# EOF` terminated
  OpenMetrics.
- **`Counter`** is monotonic under `inc`/`add` (wrapping at `u64::MAX`);
  `reset`, `set` and `compare_and_swap` are the only ways down. The `try_*`
  methods return `Overflow` and leave the value unchanged.
- **`Gauge`.** `set` stores any value, including NaN and ±∞. Every other
  unchecked method ignores non-finite input and never writes a non-finite
  result; the `try_*` methods report the same cases as errors.
- **`Timer`** sums in 128 bits and clamps durations above `u64::MAX`
  nanoseconds; `count` overflows only past `u64::MAX` recordings, which the
  `try_*` methods report.
- **`RateMeter::tick_if_under_limit`** may overshoot the limit by up to
  `threads − 1` events under contention. `TokenBucket::try_acquire` never
  over-admits.
- **`SystemHealth` readers** are single atomic loads; only the sampler
  thread (or `update()` in manual mode) writes. Values a platform cannot
  provide are `None`.
- **No panics** in library code for any input, including NaN, infinities,
  zero rates and empty bucket lists; see the property tests in
  `tests/proptests.rs`.
