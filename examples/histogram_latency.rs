//! Latency distributions with `Histogram` (fixed buckets, aggregatable) and
//! `Summary` (streaming quantiles with a relative-error bound).
//!
//! Run with: `cargo run --example histogram_latency --features histogram`

use metrics_lib::{LabelSet, MetricsCore, Unit};

/// Deterministic pseudo-random value in `[0, 1)`.
fn next(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    #[allow(clippy::cast_precision_loss)]
    let v = (*state >> 11) as f64 / (1u64 << 53) as f64;
    v
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core = MetricsCore::new();
    let registry = core.registry();
    registry.describe_histogram("request_duration_seconds", "Handler latency", Unit::Seconds)?;

    let fast = core.histogram_with(
        "request_duration_seconds",
        &LabelSet::from([("route", "/healthz")]),
    );
    let slow = core.histogram_with(
        "request_duration_seconds",
        &LabelSet::from([("route", "/search")]),
    );
    let all = core.summary("request_duration_quantiles_seconds");

    let mut rng = 7u64;
    for _ in 0..1000 {
        let v = 0.005 + next(&mut rng) * 0.005;
        fast.observe(v);
        all.observe(v);
    }
    for _ in 0..200 {
        let v = 0.1 + next(&mut rng) * 1.9;
        slow.observe(v);
        all.observe(v);
    }

    for (route, h) in [("/healthz", &fast), ("/search", &slow)] {
        println!(
            "{route:<9} count={:>4} mean={:.4}s p50~{:.4}s p99~{:.4}s (bucket interpolation)",
            h.count(),
            h.mean(),
            h.quantile(0.5),
            h.quantile(0.99)
        );
    }
    for (q, v) in all.snapshot().quantiles {
        println!("summary   p{:<4} = {v:.4}s (within 2%)", q * 100.0);
    }
    Ok(())
}
