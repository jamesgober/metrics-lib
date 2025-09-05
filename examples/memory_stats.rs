use metrics_lib::metrics;
use sysinfo::{System, SystemExt};

/*
Helper: normalize_sysinfo_memory_to_mb

Purpose
  Convert raw values from sysinfo (which may be reported as KiB or bytes depending on platform/version)
  into a consistent MB scale for display and percentage calculations.

Inputs
  - total_raw: sys.total_memory() as f64
  - used_raw:  sys.used_memory() as f64
  - avail_raw: sys.available_memory() as f64

Outputs (all in MB)
  - (total_mb, used_mb, free_mb)

Invariants
  - Percentages computed from the returned MB values are unit-consistent across platforms.
  - free_mb corresponds to available if provided; otherwise total - used.

Cross‑platform assumptions
  - sysinfo 0.29 often reports memory in KiB; some systems/environments may expose bytes.
  - We evaluate both interpretations and choose the one yielding a plausible total GB in [1, 4096],
    preferring the smaller plausible value when both are acceptable.

Edge cases
  - If neither interpretation falls in [1, 4096] GB (unlikely on modern hosts), we pick the smaller magnitude
    to avoid absurd totals. This only affects example formatting; percentages remain consistent by construction.
  - Negative intermediate values are avoided by construction; saturating arithmetic is used upstream when needed.
*/
fn normalize_sysinfo_memory_to_mb(
    total_raw: f64,
    used_raw: f64,
    avail_raw: f64,
) -> (f64, f64, f64) {
    let free_raw = if avail_raw > 0.0 {
        avail_raw
    } else {
        total_raw - used_raw
    };

    // Consider both KiB and bytes; choose the one yielding plausible total GB [1, 4096]
    let total_mb_kib = total_raw / 1024.0; // KiB -> MB
    let total_gb_kib = total_mb_kib / 1024.0; // KiB -> GB
    let total_mb_bytes = total_raw / (1024.0 * 1024.0); // bytes -> MB
    let total_gb_bytes = total_mb_bytes / 1024.0; // bytes -> GB

    let kib_plausible = (1.0..=4096.0).contains(&total_gb_kib);
    let bytes_plausible = (1.0..=4096.0).contains(&total_gb_bytes);

    let use_bytes = match (kib_plausible, bytes_plausible) {
        (true, false) => false,
        (false, true) => true,
        (true, true) => total_gb_bytes < total_gb_kib, // prefer smaller plausible
        (false, false) => total_gb_bytes < total_gb_kib, // fallback: pick smaller magnitude
    };

    if use_bytes {
        (
            total_mb_bytes,
            used_raw / (1024.0 * 1024.0),
            free_raw / (1024.0 * 1024.0),
        )
    } else {
        (total_mb_kib, used_raw / 1024.0, free_raw / 1024.0)
    }
}

fn main() {
    // Initialize metrics core if needed by your app
    metrics_lib::init();

    let sys_h = metrics().system();

    // Cross-platform totals via sysinfo
    let mut sys = System::new();
    sys.refresh_memory();

    let total_raw = sys.total_memory() as f64;
    let used_raw = sys.used_memory() as f64;
    let avail_raw = sys.available_memory() as f64;
    let (total_mb, used_mb, free_mb) =
        normalize_sysinfo_memory_to_mb(total_raw, used_raw, avail_raw);

    let total_gb = total_mb / 1024.0;
    let used_gb = used_mb / 1024.0;
    let free_gb = free_mb / 1024.0;

    let used_pct = if total_mb > 0.0 {
        (used_mb / total_mb) * 100.0
    } else {
        0.0
    };
    let free_pct = if total_mb > 0.0 {
        (free_mb / total_mb) * 100.0
    } else {
        0.0
    };

    // Process memory via SystemHealth (MB)
    let proc_mb = sys_h.process_mem_used_mb();
    let proc_pct_of_total = if total_mb > 0.0 {
        (proc_mb / total_mb) * 100.0
    } else {
        0.0
    };
    let proc_pct_of_used = if used_mb > 0.0 {
        (proc_mb / used_mb) * 100.0
    } else {
        0.0
    };

    // Adaptive formatter to avoid printing tiny non-zero percentages as 0.00%
    fn fmt_pct(p: f64) -> String {
        if p == 0.0 {
            return "0%".to_string();
        }
        if p < 0.01 {
            format!("{:.4}%", p)
        } else {
            format!("{:.2}%", p)
        }
    }

    println!("=== Memory Overview ===");
    println!(
        "system.mem.total     : {:>10.2} MB ({:>8.2} GB) {}",
        total_mb,
        total_gb,
        fmt_pct(100.0)
    );
    println!(
        "system.mem.used      : {:>10.2} MB ({:>8.2} GB) {}",
        used_mb,
        used_gb,
        fmt_pct(used_pct)
    );
    println!(
        "system.mem.free      : {:>10.2} MB ({:>8.2} GB) {}",
        free_mb,
        free_gb,
        fmt_pct(free_pct)
    );
    println!(
        "process.mem.used     : {:>10.2} MB  (of total: {}, of used: {})",
        proc_mb,
        fmt_pct(proc_pct_of_total),
        fmt_pct(proc_pct_of_used)
    );
}
