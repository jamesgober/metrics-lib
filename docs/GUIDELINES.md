<h1 id="top" align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/jamesgober-logo-dark.png">
        <img width="72" alt="James Gober - brand logo. Image displays stylish 'JG' initials encased in a hexagon outline." src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/jamesgober-logo.png">
    </picture>
    <br><b>DEVELOPMENT</b><br>
    <sup><sub>STANDARDS &amp; GUIDELINES</sub></sup>
</h1>
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./README.md" title="Documentation"><b>DOCS</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./API.md" title="API Reference"><b>API</b></a>
        <span>&nbsp;│&nbsp;</span>
        <span>GUIDELINES</span>
    </sup>
</div>
<br>

How code in this repository is designed, written and verified. The
portfolio-wide engineering standard (REPS) applies in full; this page
covers what is specific to metrics-lib.

## Design rules

1. **The hot path is one atomic operation where possible.** Recording into
   a held handle must not lock, allocate, call the OS or read the clock
   unless the metric's meaning requires it (timers and rate meters read
   `Instant`). Anything slower belongs in lookups, exporters or samplers.
2. **Measure, then claim.** Every performance statement in the docs comes
   from `benches/` with the machine and conditions stated next to it.
3. **Every input is hostile.** Names, label keys and values come from
   callers and sometimes from requests. They are validated or repaired at
   registration and escaped again at render time, so no input can inject
   lines into an exposition or a datagram.
4. **Bounded by default.** Every collection that grows with input has a cap
   (series, label count and length, bucket count, sketch size).
5. **No panics in library code.** Invalid numbers, empty configurations and
   impossible states are handled; the property tests in `tests/` try
   arbitrary inputs on every public method.
6. **Exposition is validated, not assumed.** Output of the text exporters is
   parsed by the strict validators in `tests/common` in every test run,
   including thousands of random registries.
7. **Features are additive.** Every feature compiles and passes clippy on
   its own and with the defaults (`cargo hack --each-feature` in CI).
8. **Platform data is honest.** If a platform cannot provide a value, the
   API returns `None` and the exporter omits the series.

## Verification

Before a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo hack clippy --each-feature --no-dev-deps -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo +1.85 check --all-features
cargo deny check
cargo audit
```

Benchmarks for hot-path changes:

```bash
cargo bench --all-features --bench metrics_bench
cargo bench --all-features --bench registry_bench
```

Record before/after numbers in the pull request; a regression above 5 % on
a tracked benchmark needs a justification.

## Testing conventions

- Unit tests live next to the code (`#[cfg(test)] mod tests`), integration
  tests in `tests/`. Names follow `test_<subject>_<condition>_<expected>`.
- Every fixed bug gets a regression test named after its audit finding
  (`..._regression_ml004`) that fails on the old code.
- Concurrency claims get real-thread stress tests with a `Barrier` start
  (`tests/concurrency.rs`); time-dependent logic takes an injected clock in
  unit tests instead of sleeping.
- Platform parsers are pure functions tested against fixtures on every
  platform (`src/system/procfs.rs`).

## Documentation conventions

- Every public item has rustdoc with the behaviour, `# Errors` /
  `# Panics` where relevant, and an example for non-trivial APIs.
- Every Rust block in `README.md` and `docs/*.md` is compiled as a doctest
  with all features (`src/lib.rs`, `doc_tests`).
- No marketing adjectives; state the mechanism and the measured number.
