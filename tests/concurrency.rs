//! Real-clock, multi-threaded stress tests for every claim of correctness
//! under concurrency: registry registration storms, rendering and eviction
//! while recording, and the lock-free primitives under contention.

#![cfg(all(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
#![allow(clippy::unwrap_used)]
// Test arithmetic on counts far below 2^52.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod common;

use common::{Format, validate};
use metrics_lib::exporters::{openmetrics, prometheus};
use metrics_lib::{LabelSet, MetricsCore, Registry, TokenBucket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

const THREADS: usize = 8;

fn spawn_all<F>(threads: usize, f: F) -> Vec<thread::JoinHandle<()>>
where
    F: Fn(usize) + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let barrier = Arc::new(Barrier::new(threads));
    (0..threads)
        .map(|t| {
            let f = Arc::clone(&f);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                let _ = barrier.wait();
                f(t);
            })
        })
        .collect()
}

fn join(handles: Vec<thread::JoinHandle<()>>) {
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_registration_storm_counts_every_series_once() {
    let registry = Arc::new(Registry::new());
    let r = Arc::clone(&registry);
    join(spawn_all(THREADS, move |t| {
        for i in 0..500 {
            // Every thread registers the same 500 series of each kind, in a
            // different order.
            let i = (i + t * 61) % 500;
            let labels = LabelSet::from([("i", i.to_string())]);
            r.counter_with("storm_c", &labels).inc();
            r.gauge_with("storm_g", &labels).add(1.0);
            r.timer_with("storm_t", &labels).record_ns(1);
            r.histogram_with("storm_h", &labels).observe(0.1);
            r.rate_meter_with("storm_r", &labels).tick();
        }
    }));
    assert_eq!(registry.metric_count(), 5 * 500);
    assert_eq!(registry.cardinality_count(), 5 * 500);
    for (_, _, c) in registry.counter_entries() {
        assert_eq!(c.get(), THREADS as u64);
    }
    for (_, _, h) in registry.histogram_entries() {
        assert_eq!(h.count(), THREADS as u64);
    }
}

#[test]
fn test_clear_racing_registrations_keeps_the_count_exact_regression_ml2_005() {
    let registry = Arc::new(Registry::with_cardinality_cap(1_000));
    let stop = Arc::new(AtomicBool::new(false));
    let (r, done) = (Arc::clone(&registry), Arc::clone(&stop));
    // A bounded number of clears that end while registrations go on, so a
    // drift in the count is not wiped by a final quiet clear.
    let clearer = thread::spawn(move || {
        for _ in 0..2_000 {
            if done.load(Ordering::Acquire) {
                break;
            }
            r.clear();
        }
    });
    let r = Arc::clone(&registry);
    join(spawn_all(4, move |t| {
        for i in 0..60_000 {
            let name = format!("c{t}_{}", i % 97);
            r.counter(&name).inc();
        }
    }));
    stop.store(true, Ordering::Release);
    clearer.join().unwrap();
    // 1.x-style `used.store(0)` let racing registrations drift the count.
    assert_eq!(registry.cardinality_count(), registry.metric_count());
    assert!(registry.cardinality_count() <= registry.cardinality_cap());
    registry.clear();
    assert_eq!(registry.cardinality_count(), 0);
}

#[test]
fn test_cap_is_never_exceeded_under_contention() {
    let registry = Arc::new(Registry::with_cardinality_cap(100));
    let r = Arc::clone(&registry);
    join(spawn_all(THREADS, move |t| {
        for i in 0..200 {
            let labels = LabelSet::from([("t", t.to_string()), ("i", i.to_string())]);
            r.counter_with("capped", &labels).inc();
        }
    }));
    assert_eq!(registry.metric_count(), 100);
    assert_eq!(registry.cardinality_count(), 100);
    assert_eq!(
        registry.cardinality_overflows(),
        (THREADS * 200 - 100) as u64
    );
}

#[test]
fn test_rendering_while_recording_is_always_valid() {
    let core = Arc::new(MetricsCore::new());
    let stop = Arc::new(AtomicBool::new(false));
    let writers: Vec<_> = (0..4)
        .map(|t| {
            let core = Arc::clone(&core);
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                let mut i = 0u64;
                while !stop.load(Ordering::Relaxed) {
                    let labels =
                        LabelSet::from([("w", t.to_string()), ("k", (i % 50).to_string())]);
                    core.counter_with("live_c", &labels).inc();
                    core.histogram_with("live_h", &labels)
                        .observe((i % 7) as f64 * 0.1);
                    core.summary_with("live_s", &labels)
                        .observe((i % 11) as f64);
                    core.timer_with("live_t", &labels).record_ns(i);
                    i += 1;
                }
            })
        })
        .collect();
    let deadline = Instant::now() + Duration::from_millis(500);
    let mut renders = 0;
    while Instant::now() < deadline {
        let prom = prometheus::render(core.registry());
        validate(&prom, Format::Prometheus).unwrap_or_else(|e| panic!("{e}"));
        let om = openmetrics::render(core.registry());
        validate(&om, Format::OpenMetrics).unwrap_or_else(|e| panic!("{e}"));
        renders += 1;
    }
    stop.store(true, Ordering::Relaxed);
    for w in writers {
        w.join().unwrap();
    }
    assert!(renders > 0);
}

#[test]
fn test_evict_idle_while_recording_never_loses_held_series() {
    let registry = Arc::new(Registry::new());
    let held: Vec<_> = (0..THREADS)
        .map(|t| registry.counter_with("held", &LabelSet::from([("t", t.to_string())])))
        .collect();
    let stop = Arc::new(AtomicBool::new(false));
    let sweeper = {
        let registry = Arc::clone(&registry);
        let stop = Arc::clone(&stop);
        thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                let _ = registry.evict_idle(Duration::ZERO);
            }
        })
    };
    let r = Arc::clone(&registry);
    join(spawn_all(THREADS, move |t| {
        for i in 0..2_000 {
            // Transient series come and go; nothing must panic.
            r.counter_with(
                "transient",
                &LabelSet::from([("i", (i % 20).to_string()), ("t", t.to_string())]),
            )
            .inc();
        }
    }));
    for (t, c) in held.iter().enumerate() {
        c.add(t as u64 + 1);
    }
    stop.store(true, Ordering::Relaxed);
    sweeper.join().unwrap();
    // Held series survived every sweep and are still the registered ones.
    for (t, c) in held.iter().enumerate() {
        let again = registry.counter_with("held", &LabelSet::from([("t", t.to_string())]));
        assert!(Arc::ptr_eq(c, &again));
        assert_eq!(again.get(), t as u64 + 1);
    }
    assert!(registry.cardinality_count() >= THREADS);
}

#[test]
fn test_global_lookup_and_held_handle_agree_under_contention() {
    let core = Arc::new(MetricsCore::new());
    let held = core.counter("shared");
    let c = Arc::clone(&core);
    join(spawn_all(THREADS, move |_| {
        for _ in 0..10_000 {
            c.counter("shared").inc();
        }
    }));
    assert_eq!(held.get(), (THREADS * 10_000) as u64);
}

#[test]
fn test_sharded_and_plain_counters_agree() {
    let core = Arc::new(MetricsCore::new());
    let c = Arc::clone(&core);
    join(spawn_all(16, move |_| {
        let sharded = c.sharded_counter("sharded");
        let plain = c.counter("plain");
        for _ in 0..25_000 {
            sharded.inc();
            plain.inc();
        }
    }));
    assert_eq!(core.sharded_counter("sharded").get(), 16 * 25_000);
    assert_eq!(core.counter("plain").get(), 16 * 25_000);
}

#[test]
fn test_token_bucket_never_over_admits_with_refill() {
    // 2000 tokens/s, burst 50, hammered by 8 threads for 300 ms.
    let bucket = Arc::new(TokenBucket::new(50, 2000.0));
    let start = Instant::now();
    let granted = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let (b, g) = (Arc::clone(&bucket), Arc::clone(&granted));
    join(spawn_all(THREADS, move |_| {
        let until = Instant::now() + Duration::from_millis(300);
        while Instant::now() < until {
            if b.acquire(1) {
                let _ = g.fetch_add(1, Ordering::Relaxed);
            }
        }
    }));
    let elapsed = start.elapsed().as_secs_f64();
    let bound = 50.0 + 2000.0 * elapsed + 1.0;
    let granted = granted.load(Ordering::Relaxed) as f64;
    assert!(granted <= bound, "granted {granted} > bound {bound}");
    assert!(granted >= 50.0, "the burst alone allows 50, got {granted}");
}

#[test]
fn test_rate_meter_total_is_exact_and_windows_bounded() {
    let core = Arc::new(MetricsCore::new());
    let c = Arc::clone(&core);
    join(spawn_all(THREADS, move |_| {
        let meter = c.rate("events");
        for _ in 0..20_000 {
            meter.tick();
        }
    }));
    let meter = core.rate("events");
    let total = (THREADS * 20_000) as f64;
    assert_eq!(meter.total(), total as u64);
    assert!(meter.rate_per_hour() <= total + 1.0);
    assert!(meter.rate_per_hour() >= total * 0.99);
}

#[test]
fn test_summary_and_histogram_counts_are_exact() {
    let core = Arc::new(MetricsCore::new());
    let c = Arc::clone(&core);
    join(spawn_all(THREADS, move |t| {
        let h = c.histogram("dist_h");
        let s = c.summary("dist_s");
        for i in 0..10_000 {
            let v = ((i + t) % 100) as f64 / 100.0;
            h.observe(v);
            s.observe(v);
        }
    }));
    assert_eq!(core.histogram("dist_h").count(), (THREADS * 10_000) as u64);
    assert_eq!(core.summary("dist_s").count(), (THREADS * 10_000) as u64);
    let median = core.summary("dist_s").quantile(0.5);
    assert!((0.48..=0.52).contains(&median), "{median}");
}
