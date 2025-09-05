use metrics_lib::metrics;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    metrics_lib::init();

    let sys = metrics().system();

    println!("=== Health Dashboard (5 samples @ 1s) ===");
    for _ in 0..5 {
        // Force update each tick to refresh snapshot
        sys.update();
        let snap = sys.snapshot();

        let cpu_used = snap.system_cpu_percent;
        let cpu_free = 100.0 - cpu_used;
        let proc_cpu = snap.process_cpu_percent;

        let used_mb = snap.system_memory_mb as f64;
        let used_gb = used_mb / 1024.0;
        let health = snap.health_score;

        println!(
            "cpu.used={:>5.1}% cpu.free={:>5.1}% proc.cpu={:>5.1}% mem.used={:>10.2} MB ({:>8.2} GB) load1m={:>4.2} threads={} fds={} health={:>4.1}%",
            cpu_used,
            cpu_free,
            proc_cpu,
            used_mb,
            used_gb,
            snap.load_average,
            snap.thread_count,
            snap.fd_count,
            health
        );

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
