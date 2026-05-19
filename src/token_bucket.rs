//! Strict-admission token bucket (v0.9.5).
//!
//! `TokenBucket` is the strict-admission counterpart to
//! `RateMeter::tick_if_under_limit`. It exposes the classic token-bucket
//! algorithm — capacity, refill rate, atomic acquire — using a single
//! `compare_exchange_weak` loop on a `u64` that packs `tokens` (millitokens)
//! and `last_refill` time together to eliminate the TOCTOU window that
//! `RateMeter` accepts in exchange for hot-path speed.
//!
//! # When to choose `TokenBucket` over `RateMeter`
//!
//! - **`RateMeter::tick_if_under_limit`** is faster (single
//!   `fetch_add`-style hot path) but has known TOCTOU semantics: multiple
//!   threads can each observe the under-limit predicate, then `tick`,
//!   briefly overshooting the limit by up to `num_threads − 1` events.
//!   Suitable for observability use cases (dashboards, alerting).
//! - **`TokenBucket::try_acquire`** uses an atomic CAS on the packed
//!   `(tokens, time)` state, so the limit is **never** exceeded. Suitable
//!   for billing, hard-limit admission control, downstream protection.
//!
//! # Algorithm
//!
//! State is one `AtomicU64` packing:
//! - **upper 32 bits:** `tokens` in millitokens (10⁻³ tokens). Allows
//!   refill rates with sub-token-per-tick resolution.
//! - **lower 32 bits:** milliseconds since `created_at` of the last
//!   refill computation.
//!
//! `try_acquire(n)`:
//! 1. Load packed state.
//! 2. Compute fresh tokens since `last_refill` (refill is monotonic and
//!    capped at `capacity_millitokens`).
//! 3. If `fresh_tokens >= n * 1000`, subtract `n * 1000` and CAS-write the
//!    new packed state. On success return `Ok(())`; on CAS conflict retry
//!    from step 1.
//! 4. If `fresh_tokens < n * 1000`, return `Err(WouldBlock)`.
//!
//! Saturates at `u32::MAX` millitokens (~4.3 M tokens). Time wraps after
//! ~49.7 days of process uptime — a reset call before then is required
//! for very long-running processes that depend on the bucket; in practice
//! restart cadence covers this comfortably.
//!
//! # Example
//!
//! ```
//! use metrics_lib::TokenBucket;
//! use std::time::Duration;
//!
//! // 10 tokens/sec sustained, burst up to 50.
//! let bucket = TokenBucket::new(50, 10.0);
//!
//! // Hot path: drop or downsample if no token is available.
//! if bucket.try_acquire(1).is_ok() {
//!     // serve the request
//! }
//!
//! // Burst of 5 tokens at once.
//! let _ = bucket.try_acquire(5);
//! ```

use crate::{MetricsError, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Strict-admission token bucket with atomic-CAS acquire semantics.
///
/// Cache-line aligned to prevent false sharing in concurrent admission
/// pipelines.
#[repr(align(64))]
pub struct TokenBucket {
    /// Packed state. Upper 32 bits: current `tokens` × 1000 (millitokens).
    /// Lower 32 bits: ms since `created_at` of the last refill computation.
    state: AtomicU64,
    /// Bucket capacity, in millitokens.
    capacity_millitokens: u64,
    /// Refill rate, in millitokens per millisecond. (tokens/sec × 1 / 1 ms.)
    /// Stored as a fixed-point `u64` (millitokens × 1000 per ms = micro-tokens/ms)
    /// so that we can avoid floating-point arithmetic on the hot path.
    refill_micro_per_ms: u64,
    /// Creation timestamp; the "ms since created_at" packed in `state`
    /// is computed relative to this `Instant`.
    created_at: Instant,
}

impl TokenBucket {
    /// Build a new bucket.
    ///
    /// * `capacity` — maximum tokens the bucket can hold (burst size).
    /// * `refill_per_second` — sustained refill rate, tokens per second.
    ///
    /// The bucket starts **full** (`tokens = capacity`). `refill_per_second`
    /// of `0.0` produces a static-capacity bucket (no refill); negative or
    /// non-finite values are coerced to `0.0`.
    pub fn new(capacity: u32, refill_per_second: f64) -> Self {
        let cap_mt = (capacity as u64).saturating_mul(1000);
        let rate = if refill_per_second.is_finite() && refill_per_second > 0.0 {
            // tokens/sec × 1000 ms/sec ⇒ millitokens/sec ⇒ × 1000 ⇒ micro-tokens/ms
            (refill_per_second * 1_000.0).round() as u64
        } else {
            0
        };
        Self {
            state: AtomicU64::new(pack(cap_mt, 0)),
            capacity_millitokens: cap_mt,
            refill_micro_per_ms: rate,
            created_at: Instant::now(),
        }
    }

    /// Bucket capacity in whole tokens.
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> u32 {
        (self.capacity_millitokens / 1000).min(u32::MAX as u64) as u32
    }

    /// Refill rate in tokens per second (reconstructed from internal
    /// fixed-point storage; subject to small rounding).
    #[must_use]
    #[inline]
    pub fn refill_per_second(&self) -> f64 {
        self.refill_micro_per_ms as f64 / 1_000.0
    }

    /// Current available tokens (approximate snapshot — observation has no
    /// retry semantics; treat as advisory).
    #[must_use]
    pub fn available(&self) -> u32 {
        let packed = self.state.load(Ordering::Relaxed);
        let (tokens_mt, last_ms) = unpack(packed);
        let now_ms = self.now_ms();
        let mt = self.refilled(tokens_mt, last_ms, now_ms);
        ((mt / 1000).min(u32::MAX as u64)) as u32
    }

    /// Attempt to acquire `n` tokens. Returns `Ok(())` on success, or
    /// `Err(MetricsError::WouldBlock)` when fewer than `n` tokens are
    /// available even after refill.
    ///
    /// `n == 0` always succeeds without modifying state.
    #[inline]
    pub fn try_acquire(&self, n: u32) -> Result<()> {
        if n == 0 {
            return Ok(());
        }
        let needed = (n as u64) * 1000;
        // CAS loop — single atomic op on the success path under no contention.
        loop {
            let packed = self.state.load(Ordering::Relaxed);
            let (tokens_mt, last_ms) = unpack(packed);
            let now_ms = self.now_ms();
            let mt = self.refilled(tokens_mt, last_ms, now_ms);
            if mt < needed {
                return Err(MetricsError::WouldBlock);
            }
            let new_packed = pack(mt - needed, now_ms);
            if self
                .state
                .compare_exchange_weak(packed, new_packed, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return Ok(());
            }
        }
    }

    /// Acquire-or-don't variant that swallows the error and returns a `bool`.
    #[must_use]
    #[inline]
    pub fn acquire(&self, n: u32) -> bool {
        self.try_acquire(n).is_ok()
    }

    /// Reset the bucket to full capacity.
    pub fn reset(&self) {
        let now_ms = self.now_ms();
        self.state
            .store(pack(self.capacity_millitokens, now_ms), Ordering::SeqCst);
    }

    /// Milliseconds elapsed since `created_at`, saturating into the lower
    /// 32 bits of the packed state.
    #[inline]
    fn now_ms(&self) -> u32 {
        (self.created_at.elapsed().as_millis() as u64).min(u32::MAX as u64) as u32
    }

    /// Compute the bucket's millitoken count after refilling for the
    /// interval `last_ms → now_ms`, capped at capacity.
    #[inline]
    fn refilled(&self, tokens_mt: u64, last_ms: u32, now_ms: u32) -> u64 {
        if self.refill_micro_per_ms == 0 {
            return tokens_mt;
        }
        let elapsed_ms = now_ms.saturating_sub(last_ms) as u64;
        // micro-tokens/ms × ms = micro-tokens. Divide by 1_000 to get millitokens.
        let added_micro = elapsed_ms.saturating_mul(self.refill_micro_per_ms);
        let added_mt = added_micro / 1_000;
        (tokens_mt.saturating_add(added_mt)).min(self.capacity_millitokens)
    }
}

#[inline]
fn pack(tokens_mt: u64, last_ms: u32) -> u64 {
    let tokens_mt = tokens_mt.min(u32::MAX as u64);
    (tokens_mt << 32) | (last_ms as u64)
}

#[inline]
fn unpack(packed: u64) -> (u64, u32) {
    let tokens_mt = packed >> 32;
    let last_ms = (packed & 0xFFFF_FFFF) as u32;
    (tokens_mt, last_ms)
}

impl std::fmt::Debug for TokenBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenBucket")
            .field("capacity", &self.capacity())
            .field("available", &self.available())
            .field("refill_per_second", &self.refill_per_second())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn pack_unpack_round_trip() {
        for &mt in &[0_u64, 1, 1000, 50_000, u32::MAX as u64] {
            for &ms in &[0_u32, 1, 1000, u32::MAX] {
                let (tmt, tms) = unpack(pack(mt, ms));
                assert_eq!(tmt, mt.min(u32::MAX as u64));
                assert_eq!(tms, ms);
            }
        }
    }

    #[test]
    fn new_bucket_starts_full() {
        let b = TokenBucket::new(10, 5.0);
        assert_eq!(b.capacity(), 10);
        assert_eq!(b.available(), 10);
    }

    #[test]
    fn try_acquire_zero_is_noop() {
        let b = TokenBucket::new(5, 1.0);
        b.try_acquire(0).unwrap();
        assert_eq!(b.available(), 5);
    }

    #[test]
    fn drains_then_refuses() {
        let b = TokenBucket::new(3, 0.0); // no refill
        assert!(b.acquire(1));
        assert!(b.acquire(1));
        assert!(b.acquire(1));
        assert!(!b.acquire(1));
        assert!(matches!(b.try_acquire(1), Err(MetricsError::WouldBlock)));
    }

    #[test]
    fn refills_over_time() {
        // 200 tokens per second ⇒ ~0.2 token per ms.
        let b = TokenBucket::new(10, 200.0);
        // Drain.
        assert!(b.acquire(10));
        assert_eq!(b.available(), 0);
        // Sleep enough to refill at least one token (>= 5 ms).
        thread::sleep(Duration::from_millis(50));
        assert!(
            b.available() >= 1,
            "expected ≥ 1 token after 50 ms, got {}",
            b.available()
        );
        // And eventually we can acquire again.
        assert!(b.acquire(1));
    }

    #[test]
    fn refill_caps_at_capacity() {
        let b = TokenBucket::new(5, 1000.0);
        // Drain partially.
        assert!(b.acquire(3));
        thread::sleep(Duration::from_millis(50));
        // After 50 ms at 1000/s we'd refill 50 tokens, but capacity caps at 5.
        assert_eq!(b.available(), 5);
    }

    #[test]
    fn reset_restores_capacity() {
        let b = TokenBucket::new(4, 1.0);
        assert!(b.acquire(4));
        assert_eq!(b.available(), 0);
        b.reset();
        assert_eq!(b.available(), 4);
    }

    #[test]
    fn concurrent_acquire_never_overshoots_capacity() {
        // 100 tokens, no refill. 8 threads each try to acquire 30 tokens.
        // Total demand 240, available 100 — exactly 100 should succeed.
        let b = Arc::new(TokenBucket::new(100, 0.0));
        let threads = 8;
        let per_thread_demand = 30u32;

        let handles: Vec<_> = (0..threads)
            .map(|_| {
                let b = Arc::clone(&b);
                thread::spawn(move || {
                    let mut taken = 0u32;
                    for _ in 0..per_thread_demand {
                        if b.acquire(1) {
                            taken += 1;
                        }
                    }
                    taken
                })
            })
            .collect();

        let total: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert_eq!(total, 100, "atomic-CAS bucket must never exceed capacity");
        assert_eq!(b.available(), 0);
    }

    #[test]
    fn invalid_refill_rate_treated_as_zero() {
        let a = TokenBucket::new(5, f64::NAN);
        assert_eq!(a.refill_per_second(), 0.0);
        let b = TokenBucket::new(5, -1.0);
        assert_eq!(b.refill_per_second(), 0.0);
        let c = TokenBucket::new(5, f64::INFINITY);
        assert_eq!(c.refill_per_second(), 0.0);
    }

    #[test]
    fn debug_impl() {
        let b = TokenBucket::new(7, 2.5);
        let s = format!("{b:?}");
        assert!(s.contains("TokenBucket"));
        assert!(s.contains("capacity: 7"));
    }
}
