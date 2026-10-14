<h1 id="top" align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/jamesgober-logo-dark.png">
        <img width="72" alt="James Gober - brand logo. Image displays stylish 'JG' initials encased in a hexagon outline." src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/jamesgober-logo.png">
    </picture>
    <br>
    <b>CONTRIBUTING</b>
  </h1>

Thank you for contributing. This page covers the local workflow, the checks
CI runs, and how to measure performance. The design and testing rules are in
[`docs/GUIDELINES.md`](./docs/GUIDELINES.md).

## Getting started

- MSRV: Rust 1.85. `rust-toolchain.toml` pins 1.85.0 for local builds, so
  `cargo` in this directory uses the MSRV toolchain. Use `cargo +stable`
  for tools that need a newer compiler (`cargo-semver-checks`,
  `cargo-llvm-cov`).
- Install the toolchain and components:

  ```bash
  rustup toolchain install 1.85.0 stable
  rustup component add rustfmt clippy --toolchain 1.85.0
  ```

## Checks before a pull request

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --lib --no-default-features -- -D warnings
cargo test --all-features            # unit, integration and doctests
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo deny check
cargo audit
```

CI additionally runs every feature on its own with
[`cargo-hack`](https://github.com/taiki-e/cargo-hack):

```bash
cargo install cargo-hack
cargo hack clippy --lib --each-feature --no-dev-deps -- -D warnings
cargo hack test --lib --each-feature
```

and checks semver against the last release with
`cargo +stable semver-checks`.

## Tests

- Unit tests sit next to the code; integration tests are in `tests/`:
  `exposition.rs` (every exporter's output through the strict validators
  in `tests/common`, plus a property test over random registries),
  `concurrency.rs` (real-thread stress tests) and `proptests.rs` (arbitrary
  input on public methods).
- Every Rust block in `README.md` and `docs/*.md` is compiled and run as a
  doctest with all features (`src/lib.rs`, `doc_tests`). Keep examples
  self-contained; mark blocks that need a network or a server `no_run`.
- Time-dependent logic is tested with an injected clock, not `sleep`.

## Coverage

```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview --toolchain stable
cargo +stable llvm-cov --all-features --lib --tests --summary-only \
  --ignore-filename-regex '(benches|examples)/'
cargo +stable llvm-cov --all-features --lib --tests --html \
  --ignore-filename-regex '(benches|examples)/'
```

CI enforces the line-coverage threshold set in `.github/workflows/ci.yml`
and uploads `lcov.info` as the `coverage-lcov` artifact.

## Benchmarks

```bash
cargo bench --all-features --bench metrics_bench
cargo bench --all-features --bench registry_bench
# Quick run
cargo bench --all-features --bench metrics_bench -- --warm-up-time 0.3 --measurement-time 1 --sample-size 20
```

Criterion writes reports to `target/criterion/`. To compare a change with
`main`, save a baseline (`-- --save-baseline main`) on `main` and run with
`-- --baseline main` on your branch, or compare two report directories with
[`critcmp`](https://github.com/BurntSushi/critcmp).

Hot-path changes need before/after numbers in the pull request. A
regression above 5 % on a tracked benchmark needs a justification. Numbers
in the docs must come from these benches, with the machine and conditions
stated next to them (see [`docs/PERFORMANCE.md`](./docs/PERFORMANCE.md)).

The `Benchmarks` workflow runs nightly and on pushes to `main`, stores the
history on `gh-pages` and prints a non-blocking regression report; see
[`docs/benchmarks/README.md`](./docs/benchmarks/README.md).

### Reducing variance

- Linux: set the CPU governor to `performance`
  (`echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor`)
  and pin the run to a core (`taskset -c 2 cargo bench ...`). Restore the
  governor afterwards.
- Close other workloads and run on AC power; on Windows use a high
  performance power plan.
- Compare medians over several runs.

## Commit hygiene

- Small, focused commits with lowercase imperative messages
  (`fix token bucket refill after long idle`).
- Format and lint cleanly; never skip hooks.
- Add user-visible changes to `CHANGELOG.md` under `[Unreleased]`.

Thank you for helping make metrics-lib correct, fast and easy to use.
