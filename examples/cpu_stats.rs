use metrics_lib::metrics;

fn main() {
    // Initialize metrics core if needed by your app
    metrics_lib::init();

    let sys = metrics().system();

    // System CPU usage
    let used = sys.cpu_used();
    let free = sys.cpu_free();
    let load = sys.load_avg();
    let threads = sys.thread_count();
    let fds = sys.fd_count();

    // Process CPU usage (instantaneous)
    let proc_cpu_instant = sys.process_cpu_used();

    // Warm-up sampling window: wait briefly, force update, then sample again
    std::thread::sleep(std::time::Duration::from_millis(600));
    sys.update();
    let proc_cpu_after_sleep = sys.process_cpu_used();

    // Brief CPU activity to demonstrate a non-zero sample
    let start_spin = std::time::Instant::now();
    while start_spin.elapsed().as_millis() < 250 {
        // busy-wait for ~150ms
        std::hint::spin_loop();
    }
    sys.update();
    let proc_cpu_after_work = sys.process_cpu_used();

    println!("=== CPU Overview ===");
    println!("system.cpu.used      : {:>6.2}%", used);
    println!("system.cpu.free      : {:>6.2}%", free);
    println!("system.load.1m       : {:>6.2}", load);
    println!(
        "process.cpu.used     : {:>6.2}% (instant)",
        proc_cpu_instant
    );
    println!(
        "process.cpu.used     : {:>6.2}% (after 300ms)",
        proc_cpu_after_sleep
    );
    println!(
        "process.cpu.used     : {:>6.2}% (after brief work)",
        proc_cpu_after_work
    );
    println!("process.threads      : {}", threads);
    println!("process.fds          : {}", fds);
}
