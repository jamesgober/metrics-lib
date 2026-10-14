# Performance Tuning

Practical advice for the lowest overhead in production. Measured figures
are in [`PERFORMANCE.md`](./PERFORMANCE.md).

## Hold handles on hot paths

A registry lookup (`core.counter("name")`) takes a shard read lock and two
hash lookups; recording into a held `Arc<Counter>` is one atomic add.
Resolve handles once, at start-up or per connection:

```rust
use metrics_lib::{LabelSet, MetricsCore};

let core = MetricsCore::new();
let ok = core.counter_with("requests", &LabelSet::from([("status", "200")]));
for _ in 0..1_000 {
    ok.inc(); // no lookup, no allocation
}
assert_eq!(ok.get(), 1_000);
```

## Pick the right type under contention

- Many threads incrementing one counter: use `ShardedCounter`. On the
  reference machine eight threads hammering one `Counter` take 7 to 9 times as
  long as with a `ShardedCounter` (see `contended_increments`).
- Strict rate limits: `TokenBucket` (one compare-and-swap per acquire), not
  `RateMeter::tick_if_under_limit`.
- Percentiles: `Histogram` when you aggregate across instances in
  Prometheus; `Summary` when you need accurate per-process quantiles.

## Batch where the data is already batched

`Counter::add(n)`, `RateMeter::tick_n(n)` and `Timer::record_batch(&[..])`
update each atomic once per batch.

## Keep cardinality bounded

Every distinct label set is a series with its own memory. Use templates
(`/users/{id}`), not raw values, as labels; cap the registry with
`MetricsConfig::with_max_series`; and evict stale series with
`Registry::evict_idle` from a periodic task.

## Export off the hot path

Rendering walks every series. Scrape from a separate thread (the
`http` feature's `MetricsServer` does), reuse a buffer with
`prometheus::render_into`, and keep the scrape interval reasonable (10–60 s).

## Benchmarking on your machine

- Fix the CPU frequency (Linux: `performance` governor) and close other
  workloads.
- Use longer Criterion runs:

```bash
cargo bench --all-features --bench metrics_bench -- --warm-up-time 3 --measurement-time 5
```

- Compare medians across several runs; report CPU, OS and `rustc -V` with
  the numbers.
