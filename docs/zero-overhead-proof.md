# Verifying the Hot-Path Cost

How to check, on your own machine, what recording a metric costs and what
the optional features add. The figures we publish are in
[`PERFORMANCE.md`](./PERFORMANCE.md); this page is the method.

## 1. Inspect the generated code

The hot paths are small enough to read in assembly. With
[`cargo-show-asm`](https://github.com/pacak/cargo-show-asm):

```bash
cargo install cargo-show-asm
cargo asm --release --lib "metrics_lib::counter::Counter::inc"
```

What to expect on x86-64:

- `Counter::inc` / `add`: one `lock xadd` (or `lock add`) and a return.
- `Gauge::set`: one `mov` store. `Gauge::get`: one `mov` load.
- `Timer::record_ns`: two `lock xadd` plus a compare against the current
  minimum and maximum (a `lock cmpxchg` only when a new extreme is seen).
- No calls to the allocator, the formatter or the registry.

`ShardedCounter::inc` adds a thread-local load to pick the stripe.

## 2. Measure what features cost

Every optional component is behind a feature, and the system monitor is not
started until first use, so an unused feature adds compile time and binary
size but no runtime work. To see the size:

```bash
cargo install cargo-bloat
cargo bloat --release --crates --example quick_start --no-default-features --features count,gauge,timer
cargo bloat --release --crates --example quick_start --features full
```

To confirm nothing runs in the background, create a `MetricsCore` and check
your thread count: no `metrics-lib-system-sampler` thread exists until
`MetricsCore::system()` is called.

```rust
let core = metrics_lib::MetricsCore::new();
core.counter("requests").inc();
// No sampler thread, no system call so far. The first call starts it:
let health = core.system();
assert!(health.is_sampling());
```

## 3. Reproduce the benchmarks

```bash
cargo bench --all-features --bench metrics_bench -- --warm-up-time 3 --measurement-time 5
cargo bench --all-features --bench registry_bench -- --warm-up-time 3 --measurement-time 5
```

Results land in `target/criterion/` (open `report/index.html`). Run on a
quiet machine with a fixed CPU frequency, and report the environment with
the numbers: CPU model, OS, `rustc -V`, target triple and the Criterion
flags. Numbers measured on a loaded machine are upper bounds.

## 4. Release profile

The repository's `[profile.release]` (used by `cargo bench`) sets
`lto = "fat"` and `codegen-units = 1`. Your application's profile decides
what you get in production; `codegen-units = 1` mainly helps cross-crate
inlining of the small hot-path functions.
