# Benchmarks and Performance Regression

How to run the benchmarks, how CI tracks them, and how the dashboard on
GitHub Pages reads the results. Measured reference figures are in
[`../PERFORMANCE.md`](../PERFORMANCE.md).

## Running benchmarks

Two Criterion targets cover the crate:

- `metrics_bench`: the recording hot paths (counter, sharded counter,
  up/down counter, gauge, timer, rate meter, histogram, summary, token
  bucket) and contended increments at 1, 4, 8 and 16 threads.
- `registry_bench`: registry lookups, concurrent labeled lookups, series
  registration, idle eviction and exposition rendering.

```bash
# Full run
cargo bench --all-features --bench metrics_bench
cargo bench --all-features --bench registry_bench

# Quick run (laptops, pull requests)
cargo bench --all-features --bench metrics_bench -- --warm-up-time 0.3 --measurement-time 1 --sample-size 20
```

Criterion writes HTML and JSON reports to `target/criterion/`; open
`target/criterion/report/index.html` for the overview.

## Comparing results

```bash
# Save a baseline from main, then compare the working tree against it
git switch main && cargo bench --all-features --bench metrics_bench -- --save-baseline main
git switch -   && cargo bench --all-features --bench metrics_bench -- --baseline main

# Or compare two report directories
cargo install critcmp
critcmp target/criterion /path/to/other/criterion
```

## CI and the dashboard

The `Benchmarks` workflow runs nightly and on demand. It aggregates every
`target/criterion/<group>/<function>/new/estimates.json` into one JSON
array whose names are `<group>/<function>` (for example `counter/inc`,
`rate_meter/tick`, `registry_lookup/counter_unlabeled_hit`) and stores the
history on the `gh-pages` branch under `benchmark-data/`.

The viewer (`docs/benchmarks/main.js`) and the regression script
(`tools/bench_regression_check.js`) display those names with dots
(`counter.inc`), shorten `rate_meter.` to `rate.`, and group
`contended_increments/<kind>/<threads>` as thread-scaling series. Each
tracked benchmark has a threshold (8 % for the hot paths, 10 % for registry
lookups, 18 % for the sub-nanosecond gauge operations); the latest run is
compared against the median of the previous five.

Shared CI runners are noisy, so the regression gate reports but does not
fail the workflow. Treat a flagged regression as a prompt to re-measure
locally.

### Cycles per operation

If the viewer knows the CPU frequency it adds a derived `.cycles` row
(`cycles/op = ns/op × MHz`). Provide the frequency with
`<script>window.CPU_MHZ = 3800;</script>`, a
`<meta name="cpu-mhz" content="3800">` tag, or
`BENCHMARK_DATA.meta.cpu_mhz` in the data payload.

## Reducing variance

- Linux: set the CPU governor to `performance` and pin the benchmark to a
  core (`taskset -c 2 cargo bench ...`).
- Close other workloads; a loaded machine only produces upper bounds.
- Report CPU model, OS, `rustc -V` and the Criterion flags next to any
  number you publish.
