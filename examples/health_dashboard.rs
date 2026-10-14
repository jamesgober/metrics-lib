//! A small health dashboard from `SystemHealth` with custom thresholds.
//!
//! Run with: `cargo run --example health_dashboard`

use metrics_lib::{HealthConfig, Step, SystemHealth};
use std::time::Duration;

fn main() {
    let config = HealthConfig::default()
        .with_system_cpu(vec![Step::new(90.0, 40.0), Step::new(70.0, 20.0)])
        .with_memory_percent(vec![Step::new(95.0, 30.0), Step::new(85.0, 10.0)]);
    let health = SystemHealth::with_config(Duration::from_millis(250), config);
    std::thread::sleep(Duration::from_millis(600));

    let snap = health.snapshot();
    println!("status:        {}", health.quick_check());
    println!("score:         {:.1}/100", snap.health_score);
    println!("host CPU:      {:.1}%", snap.system_cpu_percent);
    println!(
        "host memory:   {:.0} / {:.0} MiB ({:.1}%)",
        health.mem_used_mb(),
        health.mem_total_mb(),
        health.mem_used_percent()
    );
    println!("process RSS:   {:.1} MiB", health.process_mem_used_mb());
    println!("threads:       {}", optional(snap.thread_count));
    println!("open fds:      {}", optional(snap.fd_count));
    println!("sampled ago:   {:?}", snap.last_update);
}

fn optional(v: Option<u32>) -> String {
    v.map_or_else(|| "n/a on this platform".to_owned(), |v| v.to_string())
}
