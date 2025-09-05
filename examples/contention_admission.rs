use metrics_lib::{init, metrics};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    init();

    let target = 10_000.0; // desired max rps
    let r = Arc::new(metrics().rate("contention.admission"));

    let threads = 8;
    let run_secs = 2.0;

    let start = Instant::now();
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let r = r.clone();
        handles.push(thread::spawn(move || {
            let mut allowed = 0u64;
            while start.elapsed().as_secs_f64() < run_secs {
                // try to go as fast as possible; admission will gate
                if r.tick_if_under_limit(target) {
                    allowed += 1;
                }
            }
            allowed
        }));
    }

    let mut total_allowed = 0u64;
    for h in handles {
        total_allowed += h.join().unwrap();
    }

    let secs = start.elapsed().as_secs_f64();
    let eff_rps = total_allowed as f64 / secs;
    println!(
        "threads={} duration={:.2}s target_rps={:.0} allowed={} eff_rps={:.0}",
        threads, secs, target, total_allowed, eff_rps
    );
}
