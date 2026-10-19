//! Count cache hits and misses and time lookups.
//!
//! Run with: `cargo run --example cache_hit_miss`

use metrics_lib::{LabelSet, MetricsCore};
use std::collections::HashMap;

fn main() {
    let core = MetricsCore::new();
    // Resolve handles once, outside the loop.
    let hits = core.counter_with("cache_requests", &LabelSet::from([("result", "hit")]));
    let misses = core.counter_with("cache_requests", &LabelSet::from([("result", "miss")]));
    let lookup = core.timer("cache_lookup_seconds");

    let mut cache: HashMap<u64, u64> = (0..100).map(|i| (i, i * 10)).collect();
    for key in (0..1_000u64).map(|i| i % 150) {
        let _t = lookup.start();
        match cache.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => hits.inc(),
            std::collections::hash_map::Entry::Vacant(slot) => {
                misses.inc();
                let _ = slot.insert(key * 10);
            }
        }
    }

    let (h, m) = (hits.get(), misses.get());
    #[allow(clippy::cast_precision_loss)]
    let ratio = h as f64 / (h + m) as f64 * 100.0;
    println!(
        "hits={h} misses={m} hit_ratio={ratio:.1}% avg_lookup={:?}",
        lookup.average()
    );
}
