//! Parity of the in-house Windows and macOS samplers with the `sysinfo`
//! crate they replaced. `sysinfo` is a dev-dependency on these targets only,
//! so these tests run on the Windows and macOS CI runners.
//!
//! Values that move while the test runs (memory, CPU time) are compared with
//! tolerances; values that do not (total memory, start time) closely.

#![cfg(all(feature = "system", any(windows, target_os = "macos")))]
#![allow(clippy::unwrap_used, clippy::cast_precision_loss)]

use metrics_lib::{ProcessCollector, SystemHealth};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

fn sysinfo_process() -> (System, Pid) {
    let mut sys = System::new();
    let pid = sysinfo::get_current_pid().unwrap();
    let _ = sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::nothing().with_memory().with_cpu(),
    );
    (sys, pid)
}

fn within_factor(a: u64, b: u64, factor: f64) -> bool {
    let (a, b) = (a as f64, b as f64);
    a > 0.0 && b > 0.0 && a / b <= factor && b / a <= factor
}

#[test]
fn test_process_values_match_sysinfo() {
    // Touch some memory and CPU so the readings are not trivially small.
    let ballast = vec![1u8; 16 * 1024 * 1024];
    let mut x = 0u64;
    for i in 0..5_000_000u64 {
        x = x.wrapping_mul(31).wrapping_add(i);
    }
    let _ = std::hint::black_box((&ballast, x));

    let ours = ProcessCollector::new().collect().unwrap();
    let (sys, pid) = sysinfo_process();
    let theirs = sys.process(pid).unwrap();

    assert!(
        within_factor(ours.resident_memory_bytes, theirs.memory(), 2.0),
        "resident: ours {} vs sysinfo {}",
        ours.resident_memory_bytes,
        theirs.memory()
    );
    let their_cpu = theirs.accumulated_cpu_time() as f64 / 1000.0;
    assert!(
        (ours.cpu_seconds_total - their_cpu).abs() < 1.0,
        "cpu seconds: ours {} vs sysinfo {their_cpu}",
        ours.cpu_seconds_total
    );
    let start = ours.start_time_seconds.unwrap();
    assert!(
        (start - theirs.start_time() as f64).abs() < 2.0,
        "start time: ours {start} vs sysinfo {}",
        theirs.start_time()
    );
    if let (Some(v), virt) = (ours.virtual_memory_bytes, theirs.virtual_memory()) {
        if virt > 0 {
            // Windows: private bytes on both sides, but sysinfo's own state
            // grows between the samples; macOS: the same task field.
            assert!(
                within_factor(v, virt, 4.0),
                "virtual: ours {v} vs sysinfo {virt}"
            );
        }
    }
    if let (Some(open), Some(their_open)) = (ours.open_fds, theirs.open_files()) {
        let their_open = their_open as u64;
        if cfg!(windows) {
            // Both read the process handle count.
            assert!(
                open.abs_diff(their_open) <= 32 + their_open / 4,
                "open handles: ours {open} vs sysinfo {their_open}"
            );
        } else {
            // sysinfo 0.36 reports the size `proc_pidinfo(PROC_PIDLISTFDS)`
            // asks for when probed with an empty buffer, which is the
            // descriptor table's capacity; we list the descriptors and count
            // them, so ours is the true count and never above it.
            assert!(
                (3..=their_open).contains(&open),
                "open descriptors: ours {open} vs sysinfo's table size {their_open}"
            );
        }
    }
    #[cfg(target_os = "macos")]
    {
        assert!(ours.threads.unwrap() >= 1);
        assert!(ours.max_fds.unwrap() >= ours.open_fds.unwrap());
    }
}

#[test]
fn test_host_memory_matches_sysinfo() {
    let health = SystemHealth::manual();
    health.update();
    let mut sys = System::new();
    sys.refresh_memory();
    let total_mb = sys.total_memory() as f64 / (1024.0 * 1024.0);
    let used_mb = sys.used_memory() as f64 / (1024.0 * 1024.0);
    assert!(
        (health.mem_total_mb() - total_mb).abs() <= total_mb * 0.01,
        "total: ours {} MB vs sysinfo {total_mb} MB",
        health.mem_total_mb()
    );
    // Windows uses the same definition (total - available); macOS counts
    // Activity Monitor's "Memory Used", which sysinfo approximates
    // differently, so allow a wider band there.
    let band = if cfg!(windows) { 0.05 } else { 0.35 };
    assert!(
        (health.mem_used_mb() - used_mb).abs() <= total_mb * band,
        "used: ours {} MB vs sysinfo {used_mb} MB (total {total_mb} MB)",
        health.mem_used_mb()
    );
    assert!(health.mem_used_mb() > 0.0 && health.mem_used_mb() <= health.mem_total_mb());
}

#[test]
fn test_host_cpu_is_a_percentage_after_two_samples() {
    let health = SystemHealth::manual();
    health.update();
    let start = std::time::Instant::now();
    let mut x = 0u64;
    while start.elapsed() < std::time::Duration::from_millis(100) {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    }
    let _ = std::hint::black_box(x);
    health.update();
    let cpu = health.cpu_used();
    assert!((0.0..=100.0).contains(&cpu), "{cpu}");
}
