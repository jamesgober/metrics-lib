//! Striped counter for heavily contended increments.
//!
//! A plain [`Counter`](crate::Counter) is one cache line. When many threads
//! increment it at once, that line bounces between cores and every increment
//! pays for the transfer. [`ShardedCounter`] spreads increments over several
//! cache-line-padded stripes; each thread is pinned to one stripe, so
//! concurrent writers mostly touch different lines. Reading sums the stripes.
//!
//! Trade-offs, measured in `benches/metrics_bench.rs`:
//!
//! - an uncontended increment is slightly slower than `Counter::inc` (one
//!   thread-local lookup on top of the same atomic add);
//! - contended increments scale with the number of stripes instead of
//!   collapsing onto one cache line;
//! - [`ShardedCounter::get`] costs one load per stripe and is not an atomic
//!   snapshot: increments racing with the read may or may not be included.
//!
//! Use it for counters bumped from many threads in tight loops (request
//! counters on a busy server, per-message counters in a broker). For
//! everything else, [`Counter`](crate::Counter) is simpler and cheaper.

use std::cell::Cell;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Upper bound on the stripe count.
pub const MAX_SHARDS: usize = 64;

/// One stripe, padded to 128 bytes so adjacent-line prefetching on x86 does
/// not reintroduce false sharing between neighbouring stripes.
#[repr(align(128))]
struct Stripe(AtomicU64);

/// Monotonic counter striped across cache lines for contended workloads.
///
/// # Examples
///
/// ```
/// use metrics_lib::ShardedCounter;
/// use std::sync::Arc;
///
/// let hits = Arc::new(ShardedCounter::new());
/// let workers: Vec<_> = (0..4)
///     .map(|_| {
///         let hits = Arc::clone(&hits);
///         std::thread::spawn(move || {
///             for _ in 0..1000 {
///                 hits.inc();
///             }
///         })
///     })
///     .collect();
/// for w in workers {
///     w.join().unwrap();
/// }
/// assert_eq!(hits.get(), 4000);
/// ```
pub struct ShardedCounter {
    stripes: Box<[Stripe]>,
    mask: usize,
    created_at: Instant,
}

/// Process-wide round-robin source of stripe slots for new threads.
static NEXT_SLOT: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    /// This thread's stripe slot, assigned on first use. Masked per counter,
    /// so it works for counters with any (power-of-two) stripe count.
    static SLOT: Cell<usize> = const { Cell::new(usize::MAX) };
}

#[inline]
fn thread_slot() -> usize {
    SLOT.with(|slot| {
        let current = slot.get();
        if current == usize::MAX {
            let assigned = NEXT_SLOT.fetch_add(1, Ordering::Relaxed) & (usize::MAX >> 1);
            slot.set(assigned);
            assigned
        } else {
            current
        }
    })
}

impl ShardedCounter {
    /// Create a counter with one stripe per available CPU (rounded up to a
    /// power of two, capped at [`MAX_SHARDS`]).
    #[must_use]
    pub fn new() -> Self {
        let cpus = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        Self::with_shards(cpus)
    }

    /// Create a counter with `shards` stripes, rounded up to a power of two
    /// and clamped to `1..=`[`MAX_SHARDS`].
    #[must_use]
    pub fn with_shards(shards: usize) -> Self {
        let count = shards.clamp(1, MAX_SHARDS).next_power_of_two();
        let stripes: Box<[Stripe]> = (0..count).map(|_| Stripe(AtomicU64::new(0))).collect();
        Self {
            stripes,
            mask: count - 1,
            created_at: Instant::now(),
        }
    }

    /// Number of stripes.
    #[inline]
    #[must_use]
    pub fn shards(&self) -> usize {
        self.stripes.len()
    }

    #[inline]
    fn stripe(&self) -> &AtomicU64 {
        // `mask` is `len - 1` with `len` a power of two, so the index is in
        // bounds; `get` keeps the hot path free of a panic branch anyway.
        match self.stripes.get(thread_slot() & self.mask) {
            Some(stripe) => &stripe.0,
            None => &self.stripes[0].0,
        }
    }

    /// Increment by one. Wraps at `u64::MAX` (summed across stripes).
    #[inline]
    pub fn inc(&self) {
        let _ = self.stripe().fetch_add(1, Ordering::Relaxed);
    }

    /// Add `amount`. Wraps at `u64::MAX` (summed across stripes).
    #[inline]
    pub fn add(&self, amount: u64) {
        let _ = self.stripe().fetch_add(amount, Ordering::Relaxed);
    }

    /// Sum of all stripes.
    ///
    /// Not an atomic snapshot: increments that race with the read may or may
    /// not be counted. Once writers are quiescent the result is exact.
    #[must_use]
    pub fn get(&self) -> u64 {
        self.stripes
            .iter()
            .fold(0u64, |acc, s| acc.wrapping_add(s.0.load(Ordering::Relaxed)))
    }

    /// Reset every stripe to zero.
    ///
    /// Increments racing with the reset may survive it.
    pub fn reset(&self) {
        for stripe in &*self.stripes {
            stripe.0.store(0, Ordering::Relaxed);
        }
    }

    /// Time since the counter was created.
    #[inline]
    #[must_use]
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Average increments per second since creation.
    #[must_use]
    pub fn rate_per_second(&self) -> f64 {
        crate::util::per_second(self.get(), self.age())
    }
}

impl Default for ShardedCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ShardedCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShardedCounter")
            .field("value", &self.get())
            .field("shards", &self.shards())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_sharded_counter_rounds_shards_to_power_of_two() {
        assert_eq!(ShardedCounter::with_shards(0).shards(), 1);
        assert_eq!(ShardedCounter::with_shards(3).shards(), 4);
        assert_eq!(ShardedCounter::with_shards(1000).shards(), MAX_SHARDS);
        assert!(ShardedCounter::new().shards().is_power_of_two());
    }

    #[test]
    fn test_sharded_counter_single_thread_ops() {
        let c = ShardedCounter::with_shards(4);
        c.inc();
        c.add(9);
        assert_eq!(c.get(), 10);
        c.reset();
        assert_eq!(c.get(), 0);
        assert!(c.rate_per_second() >= 0.0);
        assert!(format!("{c:?}").contains("ShardedCounter"));
        let _ = c.age();
        assert_eq!(ShardedCounter::default().get(), 0);
    }

    #[test]
    fn test_sharded_counter_is_exact_under_real_contention() {
        let threads = 16;
        let per_thread = 20_000u64;
        let c = Arc::new(ShardedCounter::with_shards(8));
        let barrier = Arc::new(Barrier::new(threads));
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let c = Arc::clone(&c);
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    let _ = barrier.wait();
                    for _ in 0..per_thread {
                        c.inc();
                    }
                    c.add(1);
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(c.get(), threads as u64 * (per_thread + 1));
    }

    #[test]
    fn test_sharded_counter_wraps_like_counter() {
        let c = ShardedCounter::with_shards(1);
        c.add(u64::MAX);
        c.add(2);
        assert_eq!(c.get(), 1);
    }
}
