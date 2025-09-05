use metrics_lib::metrics;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    metrics_lib::init();

    let m = metrics();
    let mut cache: HashMap<u64, u64> = HashMap::new();

    // Seed cache with a few entries
    for i in 0..100u64 {
        cache.insert(i, i * 10);
    }

    // Simulate mixed workload
    let keys: Vec<u64> = (0..1_000).map(|i| i % 150).collect(); // 100 in cache, 50 miss

    for k in keys {
        if let Some(v) = cache.get(&k) {
            // Hit path
            m.counter("cache_hits_total").inc();
            let _ = v;
        } else {
            // Miss path - insert
            m.counter("cache_misses_total").inc();
            cache.insert(k, k * 10);
        }
    }

    let hits = m.counter("cache_hits_total").get();
    let misses = m.counter("cache_misses_total").get();
    let total = hits + misses;
    let hit_ratio = if total > 0 { hits as f64 / total as f64 * 100.0 } else { 0.0 };

    println!(
        "cache.hits={} cache.misses={} hit_ratio={:.2}%",
        hits, misses, hit_ratio
    );

    // Example: record average lookup latency
    {
        let t = m.timer("cache_lookup_latency");
        let _g = t.start();
        std::thread::sleep(Duration::from_millis(5));
    }
    println!(
        "cache_lookup_latency_count={} avg_ns={}",
        m.timer("cache_lookup_latency").count(),
        m.timer("cache_lookup_latency").average().as_nanos()
    );
}
