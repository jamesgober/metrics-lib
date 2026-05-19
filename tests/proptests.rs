//! Property-based tests for core invariants (v0.9.5+).
//!
//! These tests use [`proptest`] to exercise a wider input space than the
//! hand-written unit suite. They focus on cross-cutting invariants that
//! would silently break if a refactor introduced subtle off-by-one or
//! ordering bugs.
//!
//! Skipped under `--no-default-features` because most properties require
//! at least one metric type.

#![cfg(all(feature = "count", feature = "gauge", feature = "timer"))]

use metrics_lib::{Counter, Gauge, LabelSet, MetricsError, Registry, Timer, TokenBucket};
use proptest::prelude::*;
use std::sync::Arc;

proptest! {
    /// **Counter monotonicity.** A `Counter` exposed only to `inc()` and
    /// `add(N: u64)` is monotonic non-decreasing.
    #[test]
    fn counter_inc_add_is_monotonic(adds in proptest::collection::vec(any::<u64>(), 0..32)) {
        let c = Counter::new();
        let mut last = 0u64;
        for n in adds {
            // try_add may fail on overflow; either it succeeds (and value
            // increases, or stays the same for n==0) or it fails (and
            // value is unchanged).
            let before = c.get();
            match c.try_add(n) {
                Ok(()) => {
                    let after = c.get();
                    prop_assert!(
                        after >= before,
                        "counter must be monotonic: before={before}, after={after}"
                    );
                    last = after;
                }
                Err(MetricsError::Overflow) => {
                    // Counter is unchanged on overflow.
                    prop_assert_eq!(c.get(), before);
                }
                Err(e) => panic!("unexpected error: {e:?}"),
            }
            prop_assert_eq!(c.get(), last);
        }
    }

    /// **Counter try_add overflow guard.** `try_add` returns `Overflow`
    /// iff the cumulative addition would exceed `u64::MAX`.
    #[test]
    fn counter_try_add_overflow_iff_arithmetic_overflows(
        start in any::<u64>(),
        n in any::<u64>(),
    ) {
        let c = Counter::with_value(start);
        match c.try_add(n) {
            Ok(()) => {
                let expected = start.checked_add(n).expect("Ok ⇒ no overflow");
                prop_assert_eq!(c.get(), expected);
            }
            Err(MetricsError::Overflow) => {
                prop_assert!(start.checked_add(n).is_none());
                prop_assert_eq!(c.get(), start, "counter unchanged on overflow");
            }
            Err(e) => panic!("unexpected error: {e:?}"),
        }
    }

    /// **Gauge `try_*` rejects non-finite inputs.** Any operation that
    /// would put the gauge in a non-finite state returns `InvalidValue`
    /// without mutating the gauge.
    #[test]
    fn gauge_try_set_rejects_non_finite(initial in -1e9_f64..1e9_f64) {
        let g = Gauge::with_value(initial);
        // `prop_assert!` reads the expression as a format string; pre-bind
        // booleans so the `{ .. }` in the `matches!` arm doesn't confuse it.
        let nan_rejected = matches!(
            g.try_set(f64::NAN),
            Err(MetricsError::InvalidValue { .. })
        );
        let inf_rejected = matches!(
            g.try_set(f64::INFINITY),
            Err(MetricsError::InvalidValue { .. })
        );
        prop_assert!(nan_rejected);
        prop_assert!(inf_rejected);
        prop_assert_eq!(g.get(), initial);
    }

    /// **LabelSet equality is insertion-order-independent (unique keys).**
    /// Two sets built from the same unique-keyed pairs in any order compare
    /// and hash equal. Duplicate-key inputs use last-write-wins semantics,
    /// which is intentionally order-sensitive — `Cargo.toml` documents this
    /// in the `LabelSet::add` contract.
    #[test]
    fn labelset_insertion_order_independence(
        pairs in proptest::collection::vec(("[a-z]{1,8}", "[a-zA-Z0-9_]{0,12}"), 0..8),
    ) {
        // Deduplicate by key (keep first occurrence) so the test only
        // exercises the order-independent case.
        let mut seen = std::collections::HashSet::<String>::new();
        let unique: Vec<(String, String)> = pairs
            .into_iter()
            .filter(|(k, _)| seen.insert(k.clone()))
            .collect();

        let a: LabelSet = unique.iter().cloned().collect();
        let b: LabelSet = unique.iter().rev().cloned().collect();
        prop_assert_eq!(&a, &b);
        prop_assert_eq!(a.to_prometheus(), b.to_prometheus());

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut ha = DefaultHasher::new();
        a.hash(&mut ha);
        let mut hb = DefaultHasher::new();
        b.hash(&mut hb);
        prop_assert_eq!(ha.finish(), hb.finish());
    }

    /// **LabelSet duplicate-key is last-write-wins.** Documents the
    /// intentional order-sensitive behaviour of `LabelSet::add` on
    /// duplicate keys (proptest discovered this in v0.9.5).
    #[test]
    fn labelset_duplicate_key_keeps_last_value(
        key in "[a-z]{1,8}",
        values in proptest::collection::vec("[a-zA-Z0-9_]{0,12}", 1..6),
    ) {
        let mut set = LabelSet::new();
        for v in &values {
            set.add(key.clone(), v.clone());
        }
        prop_assert_eq!(set.len(), 1);
        prop_assert_eq!(set.get(&key), Some(values.last().unwrap().as_str()));
    }

    /// **Timer batch is associative.** `record_batch(&[a, b])` agrees with
    /// two sequential `record_ns` calls on `count`/`total`/`min`/`max`.
    #[test]
    fn timer_record_batch_matches_sequential(
        durations in proptest::collection::vec(0u64..1_000_000u64, 1..16),
    ) {
        let dur_vec: Vec<_> = durations
            .iter()
            .map(|&n| std::time::Duration::from_nanos(n))
            .collect();

        let batched = Timer::new();
        batched.record_batch(&dur_vec);

        let seq = Timer::new();
        for d in &dur_vec {
            seq.record(*d);
        }

        prop_assert_eq!(batched.count(), seq.count());
        prop_assert_eq!(batched.total(), seq.total());
        prop_assert_eq!(batched.min(), seq.min());
        prop_assert_eq!(batched.max(), seq.max());
    }

    /// **Registry singleton-per-name.** Repeated `get_or_create_counter`
    /// for the same name returns `Arc::ptr_eq` references.
    #[test]
    fn registry_returns_same_arc_for_same_name(name in "[a-z]{1,20}") {
        let r = Registry::new();
        let a = r.get_or_create_counter(&name);
        let b = r.get_or_create_counter(&name);
        prop_assert!(Arc::ptr_eq(&a, &b));
    }

    /// **TokenBucket never overshoots capacity.** No matter how many
    /// `acquire(1)` attempts are interleaved against `acquire(n>1)`
    /// attempts, the total successful tokens cannot exceed the bucket
    /// capacity when refill is disabled.
    #[test]
    fn token_bucket_no_overshoot_no_refill(
        capacity in 1u32..1000u32,
        attempts in proptest::collection::vec(1u32..50u32, 1..64),
    ) {
        let b = TokenBucket::new(capacity, 0.0);
        let mut taken = 0u32;
        for n in attempts {
            if b.acquire(n) {
                taken = taken.saturating_add(n);
            }
        }
        prop_assert!(taken <= capacity, "taken={taken} > capacity={capacity}");
    }
}

#[cfg(feature = "histogram")]
mod histogram_props {
    use super::*;
    use metrics_lib::Histogram;

    proptest! {
        /// **Histogram count == sum of bucket counts** (the trailing +Inf
        /// bucket equals the total).
        #[test]
        fn count_matches_inf_bucket(values in proptest::collection::vec(-1e3_f64..1e3_f64, 0..32)) {
            let h = Histogram::with_buckets([0.0, 0.1, 1.0, 10.0]);
            let mut finite = 0u64;
            for v in &values {
                if v.is_finite() {
                    finite += 1;
                    h.observe(*v);
                }
            }
            let snap = h.snapshot();
            prop_assert_eq!(snap.count, finite);
            prop_assert_eq!(
                snap.buckets.last().expect("at least the +Inf bucket").count,
                finite
            );
        }

        /// **Histogram quantile monotonicity.** `quantile(a) <= quantile(b)`
        /// whenever `a <= b`.
        #[test]
        fn quantile_is_monotone(
            values in proptest::collection::vec(0.001_f64..100.0_f64, 1..64),
            a in 0.0_f64..1.0,
            b in 0.0_f64..1.0,
        ) {
            let h = Histogram::default_seconds();
            for v in &values {
                h.observe(*v);
            }
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let q_lo = h.quantile(lo);
            let q_hi = h.quantile(hi);
            prop_assert!(
                q_lo <= q_hi + 1e-9,
                "quantile({lo}) = {q_lo} > quantile({hi}) = {q_hi}"
            );
        }
    }
}
