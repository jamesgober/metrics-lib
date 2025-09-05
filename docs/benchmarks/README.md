# Benchmarks and Performance Regression

This document explains how to run and interpret benchmarks for `metrics-lib`, how CI guards against performance regressions, and how to run the heavier chaos/longevity suites.

## Running Benchmarks

- Full run (higher confidence):
  ```bash
  cargo bench
  ```
- Quick run (useful on laptops/PRs):
  ```bash
  cargo bench -- -w 0.3 -m 1.0 -n 20
  # -w warmup (s), -m measurement (s), -n samples
  ```
- Results location:
  - Criterion HTML + JSON reports: `target/criterion/`
  - Open `target/criterion/report/index.html` for the overview

## Comparing Results

- Using Criterion baselines:
  ```bash
  # Save a baseline from main
  cargo bench -- --save-baseline main
  # Compare current working tree to baseline
  cargo bench -- --baseline main
  ```

- Using `critcmp` (compare two directories or baselines):
  ```bash
  cargo install critcmp
  critcmp target/criterion /path/to/other/criterion
  ```

## CI Performance Guardrails

- PRs: a smoke Criterion pass runs with short durations and uploads `criterion-reports`.
- Pushes to `main`: results are analyzed by `benchmark-action/github-action-benchmark` and stored on `gh-pages` under `benchmark-data/`.
- Regressions beyond the alert threshold will fail the job and comment on PRs.

Notes:
- Ensure `gh-pages` exists and GitHub Pages is enabled if you want to keep result history.
- Thresholds can be tuned after we establish baseline variance.

## Chaos and Longevity Suites

These are heavy tests to validate concurrency correctness and long-term stability. They are gated behind the `bench-tests` feature and ignored by default to keep standard CI stable.

- Chaos tests:
  ```bash
  cargo test --features bench-tests -- --ignored --test chaos_tests
  ```

- Longevity tests (1B+ configurable via env):
  ```bash
  OPS=1000000000 cargo test --features bench-tests -- --ignored --test longevity_tests
  ```

CI-safe defaults automatically reduce operation counts when the `CI` env variable is present.

## Reducing Variance

On Linux self-hosted runners you can stabilize CPU frequency and reduce noise. See `CONTRIBUTING.md` → "Reducing Variance" for details.
