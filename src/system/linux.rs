//! Linux sampler: reads `/proc` directly, no extra dependencies.

use super::procfs::{
    CpuTimes, parse_auxv, parse_boot_time, parse_cpu_times, parse_loadavg, parse_max_open_files,
    parse_meminfo, parse_self_stat,
};
use super::{HostSample, ProcessMetrics};
use std::fs;
use std::io;

/// Kernel ABI default for `USER_HZ` on every mainstream architecture; only
/// used when `/proc/self/auxv` is unreadable.
const FALLBACK_CLK_TCK: u64 = 100;
const FALLBACK_PAGE_SIZE: u64 = 4096;

pub(crate) struct Platform {
    clk_tck: u64,
    page_size: u64,
    boot_time: Option<u64>,
    prev_cpu: Option<CpuTimes>,
}

impl Platform {
    pub(crate) fn new() -> Self {
        let (clk_tck, page_size) =
            fs::read("/proc/self/auxv").map_or((None, None), |bytes| parse_auxv(&bytes));
        let boot_time = fs::read_to_string("/proc/stat")
            .ok()
            .and_then(|s| parse_boot_time(&s));
        Self {
            clk_tck: clk_tck.unwrap_or(FALLBACK_CLK_TCK),
            page_size: page_size.unwrap_or(FALLBACK_PAGE_SIZE),
            boot_time,
            prev_cpu: None,
        }
    }

    #[allow(clippy::cast_precision_loss)] // tick and page counts are far below 2^53
    pub(crate) fn process(&mut self) -> io::Result<ProcessMetrics> {
        let raw = fs::read_to_string("/proc/self/stat")?;
        let stat = parse_self_stat(&raw).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "unparseable /proc/self/stat")
        })?;
        let clk_tck = self.clk_tck as f64;
        let open_fds = fs::read_dir("/proc/self/fd").ok().map(|dir| {
            u64::try_from(dir.count())
                .unwrap_or(u64::MAX)
                .saturating_sub(1)
        });
        let max_fds = fs::read_to_string("/proc/self/limits")
            .ok()
            .and_then(|l| parse_max_open_files(&l));
        Ok(ProcessMetrics {
            cpu_seconds_total: stat.utime.saturating_add(stat.stime) as f64 / clk_tck,
            resident_memory_bytes: stat.rss_pages.saturating_mul(self.page_size),
            virtual_memory_bytes: Some(stat.vsize),
            open_fds,
            max_fds,
            threads: Some(stat.num_threads),
            start_time_seconds: self
                .boot_time
                .map(|boot| boot as f64 + stat.starttime as f64 / clk_tck),
        })
    }

    pub(crate) fn host(&mut self) -> io::Result<HostSample> {
        let stat = fs::read_to_string("/proc/stat")?;
        let cpu = parse_cpu_times(&stat);
        let cpu_percent = match (cpu, self.prev_cpu) {
            (Some(now), Some(before)) => now.busy_percent_since(&before),
            _ => None,
        };
        if cpu.is_some() {
            self.prev_cpu = cpu;
        }
        let mem = fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|m| parse_meminfo(&m));
        let load = fs::read_to_string("/proc/loadavg")
            .ok()
            .and_then(|l| parse_loadavg(&l));
        Ok(HostSample {
            cpu_percent,
            memory_total_bytes: mem.map_or(0, |m| m.total_kb.saturating_mul(1024)),
            memory_used_bytes: mem.map_or(0, |m| {
                m.total_kb
                    .saturating_sub(m.available_kb)
                    .saturating_mul(1024)
            }),
            load_average: load,
        })
    }
}
