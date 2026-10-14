//! macOS sampler: `getrusage`, `getrlimit`, `getloadavg`, `sysconf`,
//! `sysctlbyname`, `proc_pidinfo` (libproc) and the Mach host statistics,
//! declared here directly (no binding crate). All of them live in
//! libSystem, which every macOS binary links.
//!
//! Every call writes fixed-size structures through pointers to local values
//! whose layouts are checked at compile time. The one kernel resource held,
//! the host port right, is released by [`HostPort`]'s `Drop`. A sample takes
//! microseconds; the `sysinfo` backend of earlier releases refreshed process
//! tables to read one.

#![allow(unsafe_code)] // FFI only; every block states why it is sound.

use super::{HostSample, ProcessMetrics};
use std::ffi::{c_char, c_int, c_long, c_void};
use std::io;
use std::mem::{align_of, size_of};

/// `struct timeval` (64-bit Darwin: `time_t` seconds, 32-bit microseconds).
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TimeVal {
    sec: i64,
    usec: i32,
}

/// `struct rusage`.
#[repr(C)]
#[derive(Default)]
#[allow(dead_code)] // mirrors the C layout; not every field is read
struct RUsage {
    utime: TimeVal,
    stime: TimeVal,
    rest: [c_long; 14],
}

/// `struct rlimit`.
#[repr(C)]
#[derive(Default)]
#[allow(dead_code)] // mirrors the C layout; not every field is read
struct RLimit {
    cur: u64,
    max: u64,
}

/// `struct proc_taskinfo` (`PROC_PIDTASKINFO`).
#[repr(C)]
#[derive(Default)]
#[allow(dead_code)] // mirrors the C layout; not every field is read
struct ProcTaskInfo {
    virtual_size: u64,
    resident_size: u64,
    total_user: u64,
    total_system: u64,
    threads_user: u64,
    threads_system: u64,
    policy: i32,
    faults: i32,
    pageins: i32,
    cow_faults: i32,
    messages_sent: i32,
    messages_received: i32,
    syscalls_mach: i32,
    syscalls_unix: i32,
    csw: i32,
    threadnum: i32,
    numrunning: i32,
    priority: i32,
}

/// `struct proc_bsdinfo` (`PROC_PIDTBSDINFO`).
#[repr(C)]
#[allow(dead_code)] // mirrors the C layout; not every field is read
struct ProcBsdInfo {
    ids: [u32; 12],
    comm: [u8; 16],
    name: [u8; 32],
    counts: [u32; 6],
    start_tvsec: u64,
    start_tvusec: u64,
}

/// `struct vm_statistics64` (`HOST_VM_INFO64`).
#[repr(C)]
#[derive(Default)]
#[allow(dead_code)] // mirrors the C layout; not every field is read
struct VmStatistics64 {
    free_count: u32,
    active_count: u32,
    inactive_count: u32,
    wire_count: u32,
    counters: [u64; 9],
    purgeable_count: u32,
    speculative_count: u32,
    swap_counters: [u64; 4],
    compressor_page_count: u32,
    throttled_count: u32,
    external_page_count: u32,
    internal_page_count: u32,
    total_uncompressed_pages_in_compressor: u64,
}

/// `host_cpu_load_info` (`HOST_CPU_LOAD_INFO`): user, system, idle, nice.
#[repr(C)]
#[derive(Default)]
struct HostCpuLoadInfo {
    ticks: [u32; 4],
}

// The layouts of the macOS SDK headers (x86-64 and arm64 are identical).
const _: () = {
    assert!(size_of::<TimeVal>() == 16 && align_of::<TimeVal>() == 8);
    assert!(size_of::<RUsage>() == 144);
    assert!(size_of::<RLimit>() == 16);
    assert!(size_of::<ProcTaskInfo>() == 96 && align_of::<ProcTaskInfo>() == 8);
    assert!(size_of::<ProcBsdInfo>() == 136 && align_of::<ProcBsdInfo>() == 8);
    assert!(size_of::<VmStatistics64>() == 152 && align_of::<VmStatistics64>() == 8);
    assert!(size_of::<HostCpuLoadInfo>() == 16);
};

const RUSAGE_SELF: c_int = 0;
const RLIMIT_NOFILE: c_int = 8;
const RLIM_INFINITY: u64 = (1 << 63) - 1;
const SC_PAGESIZE: c_int = 29;
const PROC_PIDLISTFDS: c_int = 1;
const PROC_PIDTBSDINFO: c_int = 3;
const PROC_PIDTASKINFO: c_int = 4;
/// Size of one `struct proc_fdinfo`.
const PROC_FDINFO_SIZE: usize = 8;
const HOST_CPU_LOAD_INFO: c_int = 3;
const HOST_VM_INFO64: c_int = 4;
const KERN_SUCCESS: c_int = 0;
/// `HOST_CPU_LOAD_INFO_COUNT`, in 32-bit words.
const HOST_CPU_LOAD_INFO_COUNT: u32 = 4;
/// `HOST_VM_INFO64_COUNT`, in 32-bit words.
const HOST_VM_INFO64_COUNT: u32 = 38;

unsafe extern "C" {
    static mach_task_self_: u32;
    fn getpid() -> c_int;
    fn getrusage(who: c_int, usage: *mut RUsage) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut RLimit) -> c_int;
    fn getloadavg(loadavg: *mut f64, nelem: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut usize,
        newp: *mut c_void,
        newlen: usize,
    ) -> c_int;
    fn proc_pidinfo(
        pid: c_int,
        flavor: c_int,
        arg: u64,
        buffer: *mut c_void,
        buffersize: c_int,
    ) -> c_int;
    fn mach_host_self() -> u32;
    fn mach_port_deallocate(task: u32, name: u32) -> c_int;
    fn host_statistics(host: u32, flavor: c_int, info: *mut c_int, count: *mut u32) -> c_int;
    fn host_statistics64(host: u32, flavor: c_int, info: *mut c_int, count: *mut u32) -> c_int;
}

/// The host port send right, released on drop.
struct HostPort(u32);

impl HostPort {
    fn new() -> Self {
        // SAFETY: `mach_host_self` takes no arguments and cannot fail; it
        // returns a send right that `Drop` releases.
        Self(unsafe { mach_host_self() })
    }
}

impl Drop for HostPort {
    fn drop(&mut self) {
        // SAFETY: `mach_task_self_` is the process's own task port, set up by
        // libSystem before `main`; `self.0` is a right this value owns and
        // releases exactly once. A failure leaves only a leaked right.
        let _ = unsafe { mach_port_deallocate(mach_task_self_, self.0) };
    }
}

fn c_size<T>() -> c_int {
    // The asserted layouts are at most a few hundred bytes.
    c_int::try_from(size_of::<T>()).unwrap_or(c_int::MAX)
}

/// `proc_pidinfo` into a value of type `T`; `None` unless it was filled.
fn pid_info<T>(pid: c_int, flavor: c_int, out: &mut T) -> Option<()> {
    // SAFETY: `out` is a live, aligned `T` and `buffersize` is its exact
    // size, so the kernel writes at most `size_of::<T>()` bytes into it.
    let written = unsafe {
        proc_pidinfo(
            pid,
            flavor,
            0,
            std::ptr::from_mut(out).cast(),
            c_size::<T>(),
        )
    };
    (written == c_size::<T>()).then_some(())
}

fn open_fd_count(pid: c_int) -> Option<u64> {
    // SAFETY: a null buffer of size 0 only asks for the size needed.
    let needed = unsafe { proc_pidinfo(pid, PROC_PIDLISTFDS, 0, std::ptr::null_mut(), 0) };
    let needed = usize::try_from(needed).ok().filter(|n| *n > 0)?;
    // Room for descriptors opened between the two calls.
    let mut buffer = vec![0u8; needed + 32 * PROC_FDINFO_SIZE];
    let len = c_int::try_from(buffer.len()).ok()?;
    // SAFETY: `buffer` is live and writable for `len` bytes, the size passed.
    let written = unsafe { proc_pidinfo(pid, PROC_PIDLISTFDS, 0, buffer.as_mut_ptr().cast(), len) };
    let written = usize::try_from(written).ok()?;
    u64::try_from(written / PROC_FDINFO_SIZE).ok()
}

fn last_error(context: &str) -> io::Error {
    let os = io::Error::last_os_error();
    io::Error::new(os.kind(), format!("{context}: {os}"))
}

pub(crate) struct Platform {
    pid: c_int,
    host: HostPort,
    page_size: u64,
    /// `(busy, total)` CPU ticks of the previous host sample.
    prev_cpu: Option<(u64, u64)>,
}

impl Platform {
    pub(crate) fn new() -> Self {
        // SAFETY: `getpid` takes no arguments and cannot fail.
        let pid = unsafe { getpid() };
        // SAFETY: `sysconf` only reads a configuration value.
        let page = unsafe { sysconf(SC_PAGESIZE) };
        Self {
            pid,
            host: HostPort::new(),
            page_size: u64::try_from(page).ok().filter(|p| *p > 0).unwrap_or(4096),
            prev_cpu: None,
        }
    }

    #[allow(clippy::cast_precision_loss)] // second counts are far below 2^53
    pub(crate) fn process(&mut self) -> io::Result<ProcessMetrics> {
        let mut usage = RUsage::default();
        // SAFETY: `usage` is a live `struct rusage` the call only writes.
        if unsafe { getrusage(RUSAGE_SELF, &raw mut usage) } != 0 {
            return Err(last_error("getrusage"));
        }
        let seconds = |t: TimeVal| t.sec as f64 + f64::from(t.usec) / 1e6;
        let mut task = ProcTaskInfo::default();
        pid_info(self.pid, PROC_PIDTASKINFO, &mut task)
            .ok_or_else(|| last_error("proc_pidinfo(PROC_PIDTASKINFO)"))?;
        let mut bsd = ProcBsdInfo {
            ids: [0; 12],
            comm: [0; 16],
            name: [0; 32],
            counts: [0; 6],
            start_tvsec: 0,
            start_tvusec: 0,
        };
        let start_time_seconds = pid_info(self.pid, PROC_PIDTBSDINFO, &mut bsd)
            .filter(|()| bsd.start_tvsec > 0)
            .map(|()| bsd.start_tvsec as f64 + bsd.start_tvusec as f64 / 1e6);
        let mut limit = RLimit::default();
        // SAFETY: `limit` is a live `struct rlimit` the call only writes.
        let max_fds = (unsafe { getrlimit(RLIMIT_NOFILE, &raw mut limit) } == 0
            && limit.cur < RLIM_INFINITY)
            .then_some(limit.cur);
        Ok(ProcessMetrics {
            cpu_seconds_total: seconds(usage.utime) + seconds(usage.stime),
            resident_memory_bytes: task.resident_size,
            virtual_memory_bytes: Some(task.virtual_size).filter(|v| *v > 0),
            open_fds: open_fd_count(self.pid),
            max_fds,
            threads: u64::try_from(task.threadnum).ok().filter(|t| *t > 0),
            start_time_seconds,
        })
    }

    #[allow(clippy::cast_precision_loss)] // tick counts are far below 2^53
    pub(crate) fn host(&mut self) -> io::Result<HostSample> {
        let mut total = 0u64;
        let mut len = size_of::<u64>();
        // SAFETY: the name is a NUL-terminated literal, `total` is a live
        // `u64`, `len` holds its size, and no new value is written.
        let rc = unsafe {
            sysctlbyname(
                c"hw.memsize".as_ptr(),
                std::ptr::from_mut(&mut total).cast(),
                &raw mut len,
                std::ptr::null_mut(),
                0,
            )
        };
        if rc != 0 || len != size_of::<u64>() {
            return Err(last_error("sysctlbyname(hw.memsize)"));
        }

        let mut vm = VmStatistics64::default();
        let mut count = HOST_VM_INFO64_COUNT;
        // SAFETY: `vm` is a live `vm_statistics64`; `count` is its size in
        // 32-bit words, the unit the call expects, so it writes inside `vm`.
        let vm_rc = unsafe {
            host_statistics64(
                self.host.0,
                HOST_VM_INFO64,
                std::ptr::from_mut(&mut vm).cast(),
                &raw mut count,
            )
        };
        // "Memory Used" as Activity Monitor shows it: app memory (anonymous
        // pages that are not purgeable), wired memory and compressed pages.
        let used_pages = u64::from(vm.internal_page_count.saturating_sub(vm.purgeable_count))
            + u64::from(vm.wire_count)
            + u64::from(vm.compressor_page_count);
        if vm_rc != KERN_SUCCESS {
            return Err(io::Error::other(format!(
                "host_statistics64(HOST_VM_INFO64) returned {vm_rc}"
            )));
        }
        let memory_used_bytes = used_pages.saturating_mul(self.page_size).min(total);

        let mut load = HostCpuLoadInfo::default();
        let mut load_count = HOST_CPU_LOAD_INFO_COUNT;
        // SAFETY: `load` is a live `host_cpu_load_info` of four 32-bit words,
        // the count passed in.
        let cpu_rc = unsafe {
            host_statistics(
                self.host.0,
                HOST_CPU_LOAD_INFO,
                std::ptr::from_mut(&mut load).cast(),
                &raw mut load_count,
            )
        };
        let cpu_percent = if cpu_rc == KERN_SUCCESS {
            let [user, system, idle, nice] = load.ticks.map(u64::from);
            let now = (user + system + nice, user + system + nice + idle);
            let percent = self.prev_cpu.and_then(|(prev_busy, prev_total)| {
                let total = now.1.checked_sub(prev_total)?;
                let busy = now.0.checked_sub(prev_busy)?;
                (total > 0).then(|| (busy as f64 / total as f64 * 100.0).clamp(0.0, 100.0))
            });
            self.prev_cpu = Some(now);
            percent
        } else {
            None
        };

        let mut averages = [0.0f64; 3];
        // SAFETY: `averages` holds three `f64`s and we ask for one.
        let load_average = (unsafe { getloadavg(averages.as_mut_ptr(), 1) } == 1)
            .then_some(averages[0])
            .filter(|l| l.is_finite() && *l >= 0.0);

        Ok(HostSample {
            cpu_percent,
            memory_total_bytes: total,
            memory_used_bytes,
            load_average,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_are_plausible() {
        let mut p = Platform::new();
        let m = p.process().unwrap();
        assert!(m.resident_memory_bytes > 1024 * 1024);
        assert!(m.threads.unwrap() >= 1);
        assert!(m.open_fds.unwrap() >= 3);
        assert!(m.max_fds.unwrap_or(u64::MAX) >= m.open_fds.unwrap());
        assert!(m.start_time_seconds.unwrap() > 1.0e9);
        let host = p.host().unwrap();
        assert!(host.memory_total_bytes > 0);
        assert!(host.memory_used_bytes > 0);
        assert!(host.memory_used_bytes <= host.memory_total_bytes);
        assert!(host.load_average.is_some());
        std::thread::sleep(std::time::Duration::from_millis(20));
        let again = p.host().unwrap();
        assert!(again.cpu_percent.is_none_or(|c| (0.0..=100.0).contains(&c)));
    }

    #[test]
    fn test_platforms_release_their_host_port() {
        // Creating and dropping many platforms must not exhaust port rights.
        for _ in 0..10_000 {
            drop(Platform::new());
        }
    }
}
