//! Host and process memory, plus the standard `process_*` metrics.
//!
//! Run with: `cargo run --example memory_stats`

use metrics_lib::{ProcessCollector, SystemHealth};

fn main() {
    let health = SystemHealth::manual();
    health.update();
    println!(
        "host memory: {:.0} MiB used of {:.0} MiB ({:.1}%)",
        health.mem_used_mb(),
        health.mem_total_mb(),
        health.mem_used_percent()
    );

    // Allocate and touch 64 MiB so the resident set grows.
    let before = health.process_mem_used_mb();
    let block = vec![1u8; 64 * 1024 * 1024];
    std::hint::black_box(&block);
    health.update();
    println!(
        "process RSS: {before:.1} MiB -> {:.1} MiB after a 64 MiB allocation",
        health.process_mem_used_mb()
    );

    let m = ProcessCollector::new()
        .collect()
        .expect("process metrics are available");
    println!("process_resident_memory_bytes {}", m.resident_memory_bytes);
    if let Some(v) = m.virtual_memory_bytes {
        println!("process_virtual_memory_bytes  {v}");
    }
    if let Some(v) = m.open_fds {
        println!("process_open_fds              {v}");
    }
    if let Some(v) = m.max_fds {
        println!("process_max_fds               {v}");
    }
    if let Some(v) = m.start_time_seconds {
        println!("process_start_time_seconds    {v:.0}");
    }
}
