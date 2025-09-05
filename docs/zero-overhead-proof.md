# Zero-Overhead Proof

This document shows how to independently verify the zero-overhead claims for `metrics-lib` using assembly inspection, binary size analysis, and reproducible nanosecond benchmarks.

## 1) Assembly Inspection (inlining and hot path)

Goal: demonstrate that hot-path metric calls compile down to a handful of instructions with no function call overhead.

Tools:
- cargo-asm: https://github.com/gnzlbg/cargo-asm

Install:
```bash
cargo install cargo-asm
```

Inspect a hot path (Counter::inc):
```bash
# Build in release
cargo build --release

# Show the assembly for Counter::inc (adapt the symbol if necessary)
cargo asm metrics_lib::counter::Counter::inc --rust --target x86_64-unknown-linux-gnu \
  | sed -n '1,120p'
```

What to look for:
- A single atomic RMW instruction (e.g., `lock xadd` or equivalent) or small CAS loop depending on architecture.
- No calls to allocation or formatting.
- No indirect jumps through registries or dispatchers.

Repeat for Gauge::set and Timer::record_ns to verify minimal instruction sequences.

## 2) Binary Size Analysis

Goal: demonstrate minimal code footprint and the impact of enabling features.

Tools:
- cargo-bloat: https://github.com/RazrFalcon/cargo-bloat

Install:
```bash
cargo install cargo-bloat
```

Measure size contributions (top symbols):
```bash
cargo bloat --release -n 50 --crates
```

Compare with features:
```bash
# Minimal
cargo build --release
cargo bloat --release -n 50 --crates

# With async feature
cargo build --release --features async
cargo bloat --release -n 50 --crates --features async
```

What to look for:
- Core metric types contribute a small, stable portion of total size.
- Optional features add size only when enabled.

## 3) Nanosecond Benchmarks (Reproducibility)

Goal: reproduce the sub-nanosecond Gauge and multi-nanosecond Counter/Timer numbers with Criterion in a controlled environment.

Methodology:
- Fix CPU frequency (performance governor) and ensure a quiescent system.
- Use Criterion with longer warmup/measurement to reduce variance.

Commands:
```bash
# Stabilize results
cargo bench -- -w 3.0 -m 5.0 -n 100

# Extract means
python3 - << 'PY'
import json,glob
for f in glob.glob('target/criterion/**/new/estimates.json', recursive=True):
    d=json.load(open(f))
    m=d.get('mean',{}).get('point_estimate');
    if m is not None:
        print(f,m)
PY
```

Interpreting results:
- Gauge set should measure around sub-ns on modern CPUs.
- Counter increment and Timer record should fall into low single-digit and low tens of ns respectively, depending on CPU.

## 4) Environment Disclosure

To ensure fair comparisons, document:
- CPU model and MHz
- Rust toolchain version
- Target triple
- Criterion flags

Example:
```
CPU: Apple M1 Pro (Performance cores) @ 3.2 GHz
rustc: 1.78.0 (stable)
Target: aarch64-apple-darwin
Criterion: -w 3.0 -m 5.0 -n 100
```

## 5) Optional: Codegen Units and LTO

For maximum optimization:
```toml
# Cargo.toml (profile)
[profile.release]
codegen-units = 1
lto = "thin"
opt-level = 3
```

Rebuild and re-check assembly/bloat to observe any additional improvements.
