//! Small time, rate and atomic helpers shared by every metric type.

use std::sync::atomic::{AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

/// Read-modify-write through a compare-and-swap loop.
///
/// Same contract as the standard `fetch_update`: `f` is called with the
/// current value until it returns `None` (the update is abandoned and the
/// current value returned as `Err`) or the swap succeeds (the previous value
/// is returned as `Ok`). The standard method was renamed `try_update` after
/// the MSRV, and the old name is deprecated on current compilers, so the
/// crate uses this one name on every supported toolchain.
pub(crate) trait CasUpdate {
    /// The integer type held by the atomic.
    type Value: Copy;

    /// See the trait documentation.
    fn cas_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::Value) -> Option<Self::Value>,
    ) -> Result<Self::Value, Self::Value>;
}

macro_rules! cas_update_impl {
    ($($atomic:ty => $value:ty),* $(,)?) => {$(
        impl CasUpdate for $atomic {
            type Value = $value;

            #[inline]
            fn cas_update(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                mut f: impl FnMut($value) -> Option<$value>,
            ) -> Result<$value, $value> {
                let mut prev = self.load(fetch_order);
                while let Some(next) = f(prev) {
                    match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                        Ok(previous) => return Ok(previous),
                        Err(actual) => prev = actual,
                    }
                }
                Err(prev)
            }
        }
    )*};
}

cas_update_impl!(AtomicU64 => u64, AtomicI64 => i64, AtomicUsize => usize);

/// Clamp a `Duration` to whole nanoseconds in `u64`.
#[inline]
pub(crate) fn duration_to_nanos(d: Duration) -> u64 {
    u64::try_from(d.as_nanos()).unwrap_or(u64::MAX)
}

/// Build a `Duration` from a 128-bit nanosecond count, saturating.
#[inline]
#[cfg_attr(not(feature = "timer"), allow(dead_code))]
pub(crate) fn duration_from_nanos_u128(nanos: u128) -> Duration {
    let secs = u64::try_from(nanos / 1_000_000_000).unwrap_or(u64::MAX);
    // The remainder is below 1e9, so it always fits in a u32.
    let sub = u32::try_from(nanos % 1_000_000_000).unwrap_or(0);
    Duration::new(secs, sub)
}

/// `value / age` in events per second; `0.0` for a zero age.
#[allow(clippy::cast_precision_loss)] // statistics: f64 precision is the documented contract
#[cfg_attr(
    not(any(feature = "count", feature = "timer", feature = "meter")),
    allow(dead_code)
)]
pub(crate) fn per_second(value: u64, age: Duration) -> f64 {
    let secs = age.as_secs_f64();
    if secs > 0.0 { value as f64 / secs } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers() {
        assert_eq!(duration_to_nanos(Duration::MAX), u64::MAX);
        assert_eq!(duration_to_nanos(Duration::from_micros(3)), 3_000);
        assert_eq!(duration_from_nanos_u128(u128::MAX).as_secs(), u64::MAX);
        assert_eq!(
            duration_from_nanos_u128(1_500_000_000),
            Duration::from_millis(1500)
        );
        assert!(per_second(10, Duration::ZERO).abs() < f64::EPSILON);
        assert!((per_second(10, Duration::from_secs(2)) - 5.0).abs() < f64::EPSILON);
    }
}
