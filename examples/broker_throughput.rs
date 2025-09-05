use metrics_lib::metrics;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    metrics_lib::init();
    let m = metrics();

    let prod_r = Arc::new(m.rate("broker.producer_rps"));
    let cons_r = Arc::new(m.rate("broker.consumer_rps"));

    // Simulate a bounded channel with producer/consumer threads
    let (tx, rx) = crossbeam_channel::bounded::<u64>(10_000);

    let prod = {
        let r = prod_r.clone();
        thread::spawn(move || {
            let start = Instant::now();
            let mut sent = 0u64;
            while start.elapsed() < Duration::from_secs(2) {
                if tx.send(sent).is_ok() {
                    r.tick();
                    sent += 1;
                }
            }
            sent
        })
    };

    let cons = {
        let r = cons_r.clone();
        thread::spawn(move || {
            let start = Instant::now();
            let mut recv = 0u64;
            while start.elapsed() < Duration::from_secs(2) {
                if let Ok(_v) = rx.recv_timeout(Duration::from_millis(1)) {
                    r.tick();
                    recv += 1;
                }
            }
            recv
        })
    };

    let sent = prod.join().unwrap();
    let recv = cons.join().unwrap();

    println!(
        "producer: count={} rps~{:.1} | consumer: count={} rps~{:.1}",
        sent,
        prod_r.rate(),
        recv,
        cons_r.rate()
    );
}
