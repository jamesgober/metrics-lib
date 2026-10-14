//! Sampler for targets without an in-house backend (FreeBSD and the other
//! BSDs, …): backed by `sysinfo`, which is compiled only for these targets.
//!
//! All sizes from `sysinfo` 0.26+ are in **bytes**; the 1.x crate divided by
//! 1024 once and reported KiB as MiB (ML-002).

use super::{HostSample, ProcessMetrics};
use std::io;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

pub(crate) struct Platform {
    system: System,
    pid: Option<Pid>,
    cpu_primed: bool,
}

impl Platform {
    pub(crate) fn new() -> Self {
        Self {
            system: System::new(),
            pid: sysinfo::get_current_pid().ok(),
            cpu_primed: false,
        }
    }

    #[allow(clippy::cast_precision_loss)] // millisecond and second counts are far below 2^53
    pub(crate) fn process(&mut self) -> io::Result<ProcessMetrics> {
        let pid = self
            .pid
            .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, "current pid unavailable"))?;
        let _ = self.system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[pid]),
            false,
            ProcessRefreshKind::nothing().with_memory().with_cpu(),
        );
        let process = self
            .system
            .process(pid)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "current process not found"))?;
        let start = process.start_time();
        Ok(ProcessMetrics {
            cpu_seconds_total: process.accumulated_cpu_time() as f64 / 1000.0,
            resident_memory_bytes: process.memory(),
            virtual_memory_bytes: Some(process.virtual_memory()).filter(|v| *v > 0),
            open_fds: process.open_files().and_then(|n| u64::try_from(n).ok()),
            max_fds: process
                .open_files_limit()
                .and_then(|n| u64::try_from(n).ok()),
            threads: process
                .tasks()
                .and_then(|tasks| u64::try_from(tasks.len()).ok()),
            start_time_seconds: (start > 0).then_some(start as f64),
        })
    }

    #[allow(clippy::unnecessary_wraps)] // same signature as the Linux backend, which can fail
    pub(crate) fn host(&mut self) -> io::Result<HostSample> {
        self.system.refresh_memory();
        self.system.refresh_cpu_usage();
        // CPU usage is a delta: the first refresh only primes the counters.
        let cpu_percent = if self.cpu_primed {
            let v = f64::from(self.system.global_cpu_usage());
            v.is_finite().then(|| v.clamp(0.0, 100.0))
        } else {
            self.cpu_primed = true;
            None
        };
        Ok(HostSample {
            cpu_percent,
            memory_total_bytes: self.system.total_memory(),
            memory_used_bytes: self.system.used_memory(),
            load_average: load_average(),
        })
    }
}

/// Windows has no load average; `sysinfo` emulates one with a background
/// performance-counter thread, which a metrics library must not start
/// behind the caller's back. Report it as unavailable instead.
#[cfg(windows)]
fn load_average() -> Option<f64> {
    None
}

#[cfg(not(windows))]
fn load_average() -> Option<f64> {
    let one = System::load_average().one;
    (one.is_finite() && one >= 0.0).then_some(one)
}
