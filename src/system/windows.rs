//! Windows sampler: six documented kernel32 calls on the current process
//! and the host, declared here directly (no binding crate).
//!
//! Every call writes fixed-size structures through pointers to local values
//! whose layouts are checked at compile time, and uses the current-process
//! pseudo handle, which is not a real handle and never needs closing. A
//! sample takes about a microsecond; the `sysinfo` backend of earlier
//! releases enumerated every process on the host to read one (tens to
//! hundreds of milliseconds).

#![allow(unsafe_code)] // FFI only; every block states why it is sound.

use super::{HostSample, ProcessMetrics};
use std::ffi::c_void;
use std::io;
use std::mem::{align_of, size_of};

type Bool = i32;
type Handle = *mut c_void;

/// `FILETIME`: 100-nanosecond intervals in two 32-bit halves.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct FileTime {
    low: u32,
    high: u32,
}

impl FileTime {
    fn ticks(self) -> u64 {
        (u64::from(self.high) << 32) | u64::from(self.low)
    }
}

/// `PROCESS_MEMORY_COUNTERS_EX`.
#[repr(C)]
#[derive(Default)]
#[allow(dead_code)] // mirrors the Win32 layout; not every field is read
struct ProcessMemoryCountersEx {
    cb: u32,
    page_fault_count: u32,
    peak_working_set_size: usize,
    working_set_size: usize,
    quota_peak_paged_pool_usage: usize,
    quota_paged_pool_usage: usize,
    quota_peak_non_paged_pool_usage: usize,
    quota_non_paged_pool_usage: usize,
    pagefile_usage: usize,
    peak_pagefile_usage: usize,
    private_usage: usize,
}

/// `MEMORYSTATUSEX`.
#[repr(C)]
#[derive(Default)]
#[allow(dead_code)] // mirrors the Win32 layout; not every field is read
struct MemoryStatusEx {
    length: u32,
    memory_load: u32,
    total_phys: u64,
    avail_phys: u64,
    total_page_file: u64,
    avail_page_file: u64,
    total_virtual: u64,
    avail_virtual: u64,
    avail_extended_virtual: u64,
}

// The layouts the Windows SDK headers define (x86, x86-64 and ARM64).
const _: () = {
    assert!(size_of::<FileTime>() == 8 && align_of::<FileTime>() == 4);
    assert!(size_of::<ProcessMemoryCountersEx>() == 8 + 9 * size_of::<usize>());
    assert!(align_of::<ProcessMemoryCountersEx>() == align_of::<usize>());
    assert!(size_of::<MemoryStatusEx>() == 64 && align_of::<MemoryStatusEx>() == 8);
};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> Handle;
    fn GetProcessTimes(
        process: Handle,
        creation: *mut FileTime,
        exit: *mut FileTime,
        kernel: *mut FileTime,
        user: *mut FileTime,
    ) -> Bool;
    fn K32GetProcessMemoryInfo(
        process: Handle,
        counters: *mut ProcessMemoryCountersEx,
        cb: u32,
    ) -> Bool;
    fn GetProcessHandleCount(process: Handle, count: *mut u32) -> Bool;
    fn GetSystemTimes(idle: *mut FileTime, kernel: *mut FileTime, user: *mut FileTime) -> Bool;
    fn GlobalMemoryStatusEx(status: *mut MemoryStatusEx) -> Bool;
}

/// 100 ns ticks between 1601-01-01 (the `FILETIME` epoch) and 1970-01-01.
const EPOCH_DIFFERENCE_TICKS: u64 = 116_444_736_000_000_000;
const TICKS_PER_SECOND: f64 = 10_000_000.0;

fn check(ok: Bool) -> io::Result<()> {
    if ok == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn struct_size<T>() -> u32 {
    // The asserted layouts above are a few dozen bytes.
    u32::try_from(size_of::<T>()).unwrap_or(u32::MAX)
}

fn current_process() -> Handle {
    // SAFETY: `GetCurrentProcess` takes no arguments, cannot fail and
    // returns a constant pseudo handle for the calling process that must not
    // (and need not) be closed.
    unsafe { GetCurrentProcess() }
}

pub(crate) struct Platform {
    /// `(idle, kernel + user)` ticks of the previous host sample.
    prev_cpu: Option<(u64, u64)>,
}

impl Platform {
    pub(crate) fn new() -> Self {
        Self { prev_cpu: None }
    }

    #[allow(clippy::unused_self)] // same signature as the other backends
    #[allow(clippy::cast_precision_loss)] // tick counts are far below 2^53
    pub(crate) fn process(&mut self) -> io::Result<ProcessMetrics> {
        let process = current_process();
        let (mut creation, mut exit, mut kernel, mut user) = (
            FileTime::default(),
            FileTime::default(),
            FileTime::default(),
            FileTime::default(),
        );
        // SAFETY: the four pointers refer to distinct, live, aligned
        // `FILETIME`s that the call only writes; the handle is valid.
        check(unsafe {
            GetProcessTimes(
                process,
                &raw mut creation,
                &raw mut exit,
                &raw mut kernel,
                &raw mut user,
            )
        })?;

        let mut counters = ProcessMemoryCountersEx {
            cb: struct_size::<ProcessMemoryCountersEx>(),
            ..ProcessMemoryCountersEx::default()
        };
        // SAFETY: `counters` is a live `PROCESS_MEMORY_COUNTERS_EX` and `cb`
        // is its exact size, so the call writes inside it.
        check(unsafe { K32GetProcessMemoryInfo(process, &raw mut counters, counters.cb) })?;

        let mut handles = 0u32;
        // SAFETY: `handles` is a live `u32` the call only writes.
        let open_fds = (unsafe { GetProcessHandleCount(process, &raw mut handles) } != 0)
            .then_some(u64::from(handles));

        let cpu_ticks = kernel.ticks().saturating_add(user.ticks());
        let start = creation.ticks();
        Ok(ProcessMetrics {
            cpu_seconds_total: cpu_ticks as f64 / TICKS_PER_SECOND,
            resident_memory_bytes: counters.working_set_size as u64,
            // Private committed bytes, as Prometheus' Windows collector
            // reports for `process_virtual_memory_bytes`.
            virtual_memory_bytes: Some(counters.private_usage as u64).filter(|v| *v > 0),
            open_fds,
            // Windows has no per-process handle limit worth reporting.
            max_fds: None,
            // Counting threads needs a snapshot of every thread on the host.
            threads: None,
            start_time_seconds: (start > EPOCH_DIFFERENCE_TICKS)
                .then(|| (start - EPOCH_DIFFERENCE_TICKS) as f64 / TICKS_PER_SECOND),
        })
    }

    #[allow(clippy::cast_precision_loss)] // tick counts are far below 2^53
    pub(crate) fn host(&mut self) -> io::Result<HostSample> {
        let mut status = MemoryStatusEx {
            length: struct_size::<MemoryStatusEx>(),
            ..MemoryStatusEx::default()
        };
        // SAFETY: `status` is a live `MEMORYSTATUSEX` whose `dwLength` holds
        // its size, as the call requires.
        check(unsafe { GlobalMemoryStatusEx(&raw mut status) })?;

        let (mut idle, mut kernel, mut user) = (
            FileTime::default(),
            FileTime::default(),
            FileTime::default(),
        );
        let times_ok =
            // SAFETY: three distinct, live `FILETIME`s that the call only writes.
            unsafe { GetSystemTimes(&raw mut idle, &raw mut kernel, &raw mut user) } != 0;
        // Kernel time includes idle time.
        let cpu_percent = if times_ok {
            let now = (idle.ticks(), kernel.ticks().saturating_add(user.ticks()));
            let percent = self.prev_cpu.and_then(|(prev_idle, prev_total)| {
                let total = now.1.checked_sub(prev_total)?;
                let idle = now.0.checked_sub(prev_idle)?;
                (total > 0).then(|| {
                    (total.saturating_sub(idle) as f64 / total as f64 * 100.0).clamp(0.0, 100.0)
                })
            });
            self.prev_cpu = Some(now);
            percent
        } else {
            None
        };

        Ok(HostSample {
            cpu_percent,
            memory_total_bytes: status.total_phys,
            memory_used_bytes: status.total_phys.saturating_sub(status.avail_phys),
            // Windows has no load average.
            load_average: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filetime_conversion() {
        let t = FileTime {
            low: 0x89ab_cdef,
            high: 0x0123_4567,
        };
        assert_eq!(t.ticks(), 0x0123_4567_89ab_cdef);
    }

    #[test]
    fn test_samples_are_plausible() {
        let mut p = Platform::new();
        let m = p.process().unwrap();
        assert!(m.resident_memory_bytes > 1024 * 1024);
        assert!(m.open_fds.unwrap() > 0);
        assert!(m.virtual_memory_bytes.unwrap() > 0);
        assert!(m.start_time_seconds.unwrap() > 1.0e9);
        let host = p.host().unwrap();
        assert!(host.memory_total_bytes > host.memory_used_bytes);
        assert!(host.memory_used_bytes > 0);
        assert!(host.cpu_percent.is_none());
        std::thread::sleep(std::time::Duration::from_millis(20));
        let again = p.host().unwrap();
        assert!(again.cpu_percent.is_none_or(|c| (0.0..=100.0).contains(&c)));
    }
}
