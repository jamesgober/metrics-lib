<h1 id="top" align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/jamesgober-logo-dark.png">
        <img width="72" alt="James Gober - brand logo. Image displays stylish 'JG' initials encased in a hexagon outline." src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/jamesgober-logo.png">
    </picture>
    <br>
    <b>CONTRIBUTING</b>
  </h1>

Thank you for contributing! This document summarizes local workflows, benchmarking guidance with Criterion, and tips to reduce variance when you care about repeatable performance measurements.

## Getting Started

- MSRV: Rust 1.70.0
- Install toolchain and components:
  ```bash
  rustup toolchain install 1.70.0 stable
  rustup component add rustfmt clippy
  ```
- Typical fast checks:
  ```bash
  cargo fmt --all -- --check
  cargo clippy --all-features -- -D warnings
  cargo test --all-features
  ```

## Benchmarks

This repository uses Criterion for statistically robust microbenchmarks.

- Run the full suite (default durations):
  ```bash
  cargo bench
  ```
- Run quickly (useful on laptops/PRs):
  ```bash
  cargo bench -- -w 0.3 -m 1.0 -n 20
  # -w warmup time (sec), -m measurement time (sec), -n samples
  ```
- Where to find results:
  - Criterion writes HTML + JSON reports to `target/criterion/`
  - Per-benchmark stats will look like: `time: [low … mean … high]` with outlier detection

### Interpreting Criterion Output

- Prefer comparing the whole distribution rather than a single mean.
- Watch outlier percentages. High outliers can indicate background noise.
- Compare across commits by archiving `target/criterion/` or using CI artifacts.

### CI Behavior for Benchmarks

- Pull Requests: a smoke run executes with short durations to provide quick signal and uploads artifact `criterion-reports` (contents: `target/criterion`).
- Nightly schedule (05:00 UTC): the `Benchmarks` workflow runs full-duration benches on Linux, macOS, and Windows and uploads `benchmark-results-<os>`.

### Reproducing CI Locally

- Use the same shortened settings as the PR smoke job:
  ```bash
  cargo bench -- -w 0.3 -m 1.0 -n 20
  ```
- For parity with nightly, just run `cargo bench` without extra flags.

## Reducing Variance (Linux, self-hosted runners)

When you care about stable, repeatable numbers, system noise and CPU frequency scaling matter. On Linux, consider pinning the CPU governor and minimizing interference. These steps require root and are recommended only on dedicated/self-hosted runners.

1) Check current governor(s):
```bash
for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo "$f: $(cat $f)"; done
```

2) Temporarily set to `performance` for all CPUs:
```bash
# Requires root; applies until reboot or changed back
for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo performance | sudo tee $f; done
```

3) Optionally isolate CPUs for the benchmark process:
- Start the runner with kernel args like `isolcpus=2,3` (requires reboot), then pin the process using `taskset`:
```bash
# Example: pin to CPU 2
sudo taskset -c 2 cargo bench
```

4) After the run, restore to `ondemand` or `powersave` (common defaults):
```bash
for f in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo ondemand | sudo tee $f || echo powersave | sudo tee $f; done
```

Notes:
- Do not attempt these on shared hosted runners.
- Close background apps, disable heavy services, and run on AC power.
- On macOS, enable “High Power”/“Performance” modes where available. On Windows, choose a High Performance power plan.

## Bench-Gated Tests

This repo also includes benchmark-style tests gated behind the `bench-tests` feature and ignored by default to keep CI stable:
```bash
cargo test --features bench-tests -- --ignored
```

## Docs and Doctests

We compile-check Rust code blocks in `docs/API.md` using `doc-comment` in `tests/api_md_doctest.rs`. Use `no_run` on examples that require async runtimes or external services.

## Comparing Criterion Results

There are two recommended approaches:

1) Criterion baselines (no extra tools)

```bash
# Save a baseline (e.g., from main)
cargo bench -- --save-baseline main

# Later, compare current working tree to the saved baseline
cargo bench -- --baseline main
```

This produces per-benchmark comparisons (better/same/worse) in the console and in `target/criterion/`.

2) Directory-to-directory with critcmp (best for CI artifacts)

```bash
cargo install critcmp

# Compare two result directories (local or extracted from CI artifacts)
critcmp path/to/criterion-run-A path/to/criterion-run-B

# Example using local paths
critcmp target/criterion ~/Downloads/criterion-previous
```

Tips:
- Download two CI artifacts (e.g., `benchmark-results-ubuntu-latest` from two runs), extract them, and pass the two extracted directories to `critcmp`.
- Use consistent features and flags across runs (e.g., both default or both `--all-features`).
- When possible, run on similar hardware/OS to avoid skew.

## Commit Hygiene

- Format and lint cleanly.
- Keep `CHANGELOG.md` updated under `[Unreleased]` for user-visible changes.
- Prefer small, focused PRs with clear descriptions and rationale.

Thank you for helping make metrics-lib fast, reliable, and a joy to use!
