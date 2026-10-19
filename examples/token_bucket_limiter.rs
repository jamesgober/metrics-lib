//! Strict rate limiting with `TokenBucket`.
//!
//! Run with: `cargo run --example token_bucket_limiter`

use metrics_lib::{MetricsError, TokenBucket};
use std::time::Duration;

fn main() {
    // 5 requests per second sustained, bursts of up to 10.
    let bucket = TokenBucket::new(10, 5.0);

    let burst = (0..15).filter(|_| bucket.acquire(1)).count();
    println!("burst: admitted {burst} of 15 (capacity 10)");

    match bucket.try_acquire(1) {
        Err(MetricsError::WouldBlock) => {
            let wait = bucket.time_until_available(1).unwrap_or_default();
            println!("empty; next token in {wait:?}");
            std::thread::sleep(wait);
        }
        other => println!("unexpected: {other:?}"),
    }
    println!("after waiting: acquire = {}", bucket.acquire(1));

    std::thread::sleep(Duration::from_millis(600));
    println!(
        "after 600 ms: {} tokens available (rate {}/s)",
        bucket.available(),
        bucket.refill_per_second()
    );
}
