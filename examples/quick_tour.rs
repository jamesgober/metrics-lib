use metrics_lib::{init, metrics};
use std::time::Duration;

fn main() {
    // Initialize once at startup
    init();

    let m = metrics();

    println!("=== Quick Tour ===");

    // Counter
    m.counter("requests").add(5);
    m.counter("requests").inc();
    println!("counter.requests = {}", m.counter("requests").get());

    // Gauge
    m.gauge("temp_c").set(21.5);
    m.gauge("temp_c").add(0.7);
    println!("gauge.temp_c = {:.2}", m.gauge("temp_c").get());

    // Timer (direct record)
    m.timer("op").record(Duration::from_millis(3));

    // Timer (RAII)
    {
        let t = m.timer("op_raii");
        let _guard = t.start();
        std::thread::sleep(Duration::from_millis(2));
    }

    // RateMeter
    let r = m.rate("ops_per_sec");
    r.tick_n(7);
    println!("rate.ops_per_sec ~{:.2}", r.rate());

    // System health
    let sys = m.system();
    println!(
        "system: cpu={:.1}% mem_mb={:.1}",
        sys.cpu_used(),
        sys.mem_used_mb()
    );
}
