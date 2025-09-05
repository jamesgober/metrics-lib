use metrics_lib::{init, metrics};
use std::time::{Duration, Instant};

fn main() {
    init();

    let limit = 50.0; // ops/sec
    let r = metrics().rate("token_bucket.limiter");

    let start = Instant::now();
    let mut allowed = 0u64;
    let mut denied = 0u64;

    // Simulate ~2 seconds of attempts at up to 200 rps
    for _ in 0..400 {
        if r.tick_if_under_limit(limit) {
            allowed += 1;
        } else {
            denied += 1;
        }
        std::thread::sleep(Duration::from_millis(5)); // 200 rps attempt
    }

    let secs = start.elapsed().as_secs_f64();
    println!(
        "limit={limit}/s over {:.2}s -> allowed={} denied={} eff_rps={:.1}",
        secs,
        allowed,
        denied,
        allowed as f64 / secs
    );
}
