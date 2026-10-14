//! Property tests over the public API: arithmetic edges, invariants, and
//! panic-freedom for arbitrary inputs (ML-011).

#![cfg(all(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram",
    feature = "sample"
))]
#![allow(clippy::unwrap_used)]

use metrics_lib::{
    AdaptiveSampler, Counter, Gauge, Histogram, LabelSet, RateMeter, Registry, SamplingStrategy,
    Summary, SummaryConfig, Timer, TokenBucket, UpDownCounter,
};
use proptest::prelude::*;
use std::time::Duration;

proptest! {
    #[test]
    fn prop_counter_adds_are_exact(adds in proptest::collection::vec(0u64..1_000_000, 0..64)) {
        let c = Counter::new();
        for a in &adds {
            c.add(*a);
        }
        prop_assert_eq!(c.get(), adds.iter().sum::<u64>());
    }

    #[test]
    fn prop_gauge_unchecked_api_never_panics(ops in proptest::collection::vec((0u8..12, any::<f64>(), any::<f64>()), 0..64)) {
        let g = Gauge::new();
        for (op, a, b) in ops {
            match op {
                0 => g.set(a),
                1 => g.add(a),
                2 => g.sub(a),
                3 => g.set_max(a),
                4 => g.set_min(a),
                5 => g.multiply(a),
                6 => g.divide(a),
                7 => g.clamp(a, b),
                8 => g.update_ema(a, b),
                9 => g.abs(),
                10 => { let _ = g.try_add(a); let _ = g.try_set(b); }
                _ => { let _ = g.compare_and_swap(a, b); }
            }
        }
        let _ = g.stats();
    }

    #[test]
    fn prop_updown_matches_i64_wrapping(deltas in proptest::collection::vec(any::<i64>(), 0..32)) {
        let c = UpDownCounter::new();
        let mut expected = 0i64;
        for d in &deltas {
            c.add(*d);
            expected = expected.wrapping_add(*d);
        }
        prop_assert_eq!(c.get(), expected);
    }

    #[test]
    fn prop_timer_accepts_any_duration(nanos in proptest::collection::vec(any::<u64>(), 0..32), secs in any::<u64>()) {
        let t = Timer::new();
        for n in &nanos {
            t.record_ns(*n);
        }
        t.record(Duration::new(secs, 999_999_999));
        let stats = t.stats();
        prop_assert_eq!(stats.count, nanos.len() as u64 + 1);
        prop_assert!(stats.max >= stats.min);
    }

    #[test]
    fn prop_histogram_constructors_never_panic(
        bounds in proptest::collection::vec(any::<f64>(), 0..40),
        start in any::<f64>(), width in any::<f64>(), count in 0usize..5000,
        values in proptest::collection::vec(any::<f64>(), 0..40),
    ) {
        for h in [
            Histogram::with_buckets(bounds.clone()),
            Histogram::linear(start, width, count),
            Histogram::exponential(start, width, count),
        ] {
            for v in &values {
                h.observe(*v);
                let _ = h.try_observe(*v);
            }
            let snap = h.snapshot();
            prop_assert_eq!(snap.buckets.last().map(|b| b.count), Some(snap.count));
            let _ = h.quantile(start);
        }
    }

    #[test]
    fn prop_summary_config_and_observe_never_panic(
        alpha in any::<f64>(), lo in any::<f64>(), hi in any::<f64>(),
        quantiles in proptest::collection::vec(any::<f64>(), 0..5),
        values in proptest::collection::vec(any::<f64>(), 0..40),
    ) {
        let cfg = SummaryConfig::default().with_relative_accuracy(alpha).with_range(lo, hi).with_quantiles(quantiles);
        let s = Summary::with_config(cfg).unwrap_or_default();
        for v in &values {
            s.observe(*v);
        }
        let snap = s.snapshot();
        prop_assert!(snap.count <= values.len() as u64);
    }

    #[test]
    fn prop_samplers_never_panic(rate in any::<u32>(), min in any::<u32>(), max in any::<u32>(), target in any::<u64>(), interval in any::<u64>()) {
        for strategy in [
            SamplingStrategy::Fixed { rate },
            SamplingStrategy::Dynamic { min_rate: min, max_rate: max, target_throughput: target },
            SamplingStrategy::TimeBased { min_interval: interval },
        ] {
            let s = AdaptiveSampler::with_seed(strategy, u64::from(rate));
            for _ in 0..16 {
                let _ = s.should_sample();
            }
            let _ = s.stats().sampling_percentage();
        }
    }

    #[test]
    fn prop_token_bucket_never_panics(capacity in any::<u64>(), rate in any::<f64>(), n in any::<u64>()) {
        let b = TokenBucket::new(capacity, rate);
        let _ = b.try_acquire(n);
        let _ = b.time_until_available(n);
        prop_assert!(b.available() <= b.capacity());
        b.reset();
    }

    #[test]
    fn prop_rate_meter_total_is_exact(ticks in proptest::collection::vec(0u64..u64::from(u32::MAX) * 4, 0..16)) {
        let m = RateMeter::with_window(Duration::from_millis(250));
        let mut total = 0u64;
        for t in &ticks {
            if m.try_tick_n(*t).is_ok() {
                total += t;
            }
        }
        prop_assert_eq!(m.total(), total);
        prop_assert!(m.rate() >= 0.0);
    }

    #[test]
    fn prop_registry_never_panics_on_arbitrary_names_and_labels(
        names in proptest::collection::vec(".{0,40}", 1..20),
        labels in proptest::collection::vec((".{0,20}", ".{0,40}"), 0..5),
    ) {
        let r = Registry::with_cardinality_cap(50);
        let set: LabelSet = labels.into_iter().collect();
        for n in &names {
            r.counter_with(n, &set).inc();
            let _ = r.try_gauge_with(n, &set);
            let _ = r.describe_counter(n, "h", metrics_lib::Unit::None);
            let _ = r.remove_series(n, &set);
        }
        prop_assert!(r.cardinality_count() <= 50);
        let _ = r.evict_idle(Duration::ZERO);
    }
}
