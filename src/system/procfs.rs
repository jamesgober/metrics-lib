//! Pure parsers for the Linux `/proc` files the sampler reads.
//!
//! Kept free of I/O so every parser is unit-tested against fixtures on every
//! platform, including the awkward cases (process names containing spaces
//! and parentheses, missing fields, empty files).

/// Aggregate CPU jiffies from the first line of `/proc/stat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CpuTimes {
    /// Jiffies spent doing work: user, nice, system, irq, softirq, steal.
    pub(crate) busy: u64,
    /// All jiffies, idle and iowait included. `guest` and `guest_nice` are
    /// already counted in `user` and `nice`, so they are not added again.
    pub(crate) total: u64,
}

impl CpuTimes {
    /// Busy share between two samples, in percent. `None` when no time
    /// passed (or the counters went backwards).
    #[allow(clippy::cast_precision_loss)] // jiffy deltas are far below 2^53
    pub(crate) fn busy_percent_since(&self, earlier: &CpuTimes) -> Option<f64> {
        let total = self.total.checked_sub(earlier.total)?;
        let busy = self.busy.checked_sub(earlier.busy)?;
        if total == 0 {
            return None;
        }
        Some((busy as f64 / total as f64 * 100.0).clamp(0.0, 100.0))
    }
}

/// Parse the aggregate `cpu` line of `/proc/stat`.
pub(crate) fn parse_cpu_times(stat: &str) -> Option<CpuTimes> {
    let line = stat.lines().find(|l| l.starts_with("cpu "))?;
    let fields: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()?;
    // user nice system idle [iowait irq softirq steal guest guest_nice]
    let get = |i: usize| fields.get(i).copied().unwrap_or(0);
    if fields.len() < 4 {
        return None;
    }
    let idle = get(3).saturating_add(get(4));
    let busy = get(0)
        .saturating_add(get(1))
        .saturating_add(get(2))
        .saturating_add(get(5))
        .saturating_add(get(6))
        .saturating_add(get(7));
    Some(CpuTimes {
        busy,
        total: busy.saturating_add(idle),
    })
}

/// Parse the `btime` (boot time, Unix seconds) line of `/proc/stat`.
pub(crate) fn parse_boot_time(stat: &str) -> Option<u64> {
    stat.lines()
        .find_map(|l| l.strip_prefix("btime "))
        .and_then(|v| v.trim().parse().ok())
}

/// The fields of `/proc/self/stat` the sampler uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelfStat {
    /// User-mode CPU time, in clock ticks.
    pub(crate) utime: u64,
    /// Kernel-mode CPU time, in clock ticks.
    pub(crate) stime: u64,
    /// Threads in the process.
    pub(crate) num_threads: u64,
    /// Start time after boot, in clock ticks.
    pub(crate) starttime: u64,
    /// Virtual memory size, in bytes.
    pub(crate) vsize: u64,
    /// Resident set size, in pages.
    pub(crate) rss_pages: u64,
}

/// Parse `/proc/self/stat`.
///
/// The second field is the command name in parentheses and may itself
/// contain spaces and `)`, so parsing starts after the **last** `)`.
pub(crate) fn parse_self_stat(stat: &str) -> Option<SelfStat> {
    let rest = &stat[stat.rfind(')')? + 1..];
    let fields: Vec<&str> = rest.split_whitespace().collect();
    // `fields[0]` is field 3 (state); field N is at index N - 3.
    let num = |field: usize| -> Option<u64> { fields.get(field - 3)?.parse().ok() };
    Some(SelfStat {
        utime: num(14)?,
        stime: num(15)?,
        num_threads: num(20)?,
        starttime: num(22)?,
        vsize: num(23)?,
        rss_pages: num(24)?,
    })
}

/// Memory totals from `/proc/meminfo`, in kibibytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MemInfo {
    pub(crate) total_kb: u64,
    pub(crate) available_kb: u64,
}

/// Parse `/proc/meminfo`. Uses `MemAvailable` when present (kernel 3.14+),
/// otherwise `MemFree + Buffers + Cached`.
pub(crate) fn parse_meminfo(meminfo: &str) -> Option<MemInfo> {
    let field = |name: &str| -> Option<u64> {
        meminfo
            .lines()
            .find_map(|l| l.strip_prefix(name))
            .and_then(|rest| rest.trim_start_matches(':').split_whitespace().next())
            .and_then(|v| v.parse().ok())
    };
    let total_kb = field("MemTotal")?;
    let available_kb = field("MemAvailable").or_else(|| {
        Some(field("MemFree")? + field("Buffers").unwrap_or(0) + field("Cached").unwrap_or(0))
    })?;
    Some(MemInfo {
        total_kb,
        available_kb: available_kb.min(total_kb),
    })
}

/// Parse the one-minute load average from `/proc/loadavg`.
pub(crate) fn parse_loadavg(loadavg: &str) -> Option<f64> {
    loadavg
        .split_whitespace()
        .next()?
        .parse()
        .ok()
        .filter(|v: &f64| v.is_finite() && *v >= 0.0)
}

/// Parse the soft "Max open files" limit from `/proc/self/limits`. `None`
/// for `unlimited` or a missing line.
pub(crate) fn parse_max_open_files(limits: &str) -> Option<u64> {
    limits
        .lines()
        .find_map(|l| l.strip_prefix("Max open files"))?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

/// `AT_PAGESZ` in the ELF auxiliary vector.
const AT_PAGESZ: u64 = 6;
/// `AT_CLKTCK` in the ELF auxiliary vector.
const AT_CLKTCK: u64 = 17;

/// Clock ticks per second and page size from `/proc/self/auxv`, which holds
/// native-endian `(type, value)` word pairs ending with `AT_NULL`.
pub(crate) fn parse_auxv(auxv: &[u8]) -> (Option<u64>, Option<u64>) {
    const WORD: usize = std::mem::size_of::<usize>();
    let mut clk_tck = None;
    let mut page_size = None;
    for pair in auxv.chunks_exact(2 * WORD) {
        let (key, value) = pair.split_at(WORD);
        let (Some(key), Some(value)) = (read_word(key), read_word(value)) else {
            break;
        };
        match key {
            0 => break,
            AT_CLKTCK => clk_tck = Some(value).filter(|v| *v > 0),
            AT_PAGESZ => page_size = Some(value).filter(|v| *v > 0),
            _ => {}
        }
    }
    (clk_tck, page_size)
}

fn read_word(bytes: &[u8]) -> Option<u64> {
    match bytes.len() {
        8 => Some(u64::from_ne_bytes(bytes.try_into().ok()?)),
        4 => Some(u64::from(u32::from_ne_bytes(bytes.try_into().ok()?))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROC_STAT: &str = "cpu  4705 150 1120 16250 520 30 45 10 0 0\n\
cpu0 2000 70 500 8000 260 15 20 5 0 0\n\
intr 12345\nctxt 999\nbtime 1700000000\nprocesses 4242\n";

    #[test]
    fn test_parse_cpu_times_counts_iowait_as_idle_and_includes_irq_steal() {
        let t = parse_cpu_times(PROC_STAT).unwrap();
        assert_eq!(t.busy, 4705 + 150 + 1120 + 30 + 45 + 10);
        assert_eq!(t.total, t.busy + 16250 + 520);
    }

    #[test]
    fn test_parse_cpu_times_old_kernels_with_four_fields() {
        let t = parse_cpu_times("cpu 10 0 10 80\n").unwrap();
        assert_eq!((t.busy, t.total), (20, 100));
        assert_eq!(parse_cpu_times("cpu 1 2\n"), None);
        assert_eq!(parse_cpu_times(""), None);
        assert_eq!(parse_cpu_times("cpu a b c d\n"), None);
    }

    #[test]
    fn test_busy_percent_is_a_delta_regression_ml003() {
        // ML-003: CPU% used the since-boot totals instead of a delta.
        let before = CpuTimes {
            busy: 1_000_000,
            total: 10_000_000,
        };
        let after = CpuTimes {
            busy: 1_000_000 + 75,
            total: 10_000_000 + 100,
        };
        assert_eq!(after.busy_percent_since(&before), Some(75.0));
        assert_eq!(before.busy_percent_since(&before), None);
        assert_eq!(before.busy_percent_since(&after), None);
    }

    #[test]
    fn test_parse_boot_time() {
        assert_eq!(parse_boot_time(PROC_STAT), Some(1_700_000_000));
        assert_eq!(parse_boot_time("cpu 1 2 3 4\n"), None);
    }

    fn self_stat(comm: &str) -> String {
        // Fields 1..=24 followed by more fields; values encode their index.
        let mut s = format!("4242 ({comm}) S");
        for field in 4..=52 {
            let value = match field {
                14 => 111,
                15 => 222,
                20 => 7,
                22 => 333,
                23 => 4_096_000,
                24 => 250,
                n => n,
            };
            s.push(' ');
            s.push_str(&value.to_string());
        }
        s.push('\n');
        s
    }

    #[test]
    fn test_parse_self_stat_plain_name() {
        let s = parse_self_stat(&self_stat("server")).unwrap();
        assert_eq!(
            s,
            SelfStat {
                utime: 111,
                stime: 222,
                num_threads: 7,
                starttime: 333,
                vsize: 4_096_000,
                rss_pages: 250
            }
        );
    }

    #[test]
    fn test_parse_self_stat_name_with_spaces_and_parens_regression_ml013() {
        // ML-013: splitting the whole line on whitespace shifted every field
        // when the command name contained a space.
        let s = parse_self_stat(&self_stat("my worker) (x y")).unwrap();
        assert_eq!(s.utime, 111);
        assert_eq!(s.stime, 222);
        assert_eq!(s.rss_pages, 250);
    }

    #[test]
    fn test_parse_self_stat_rejects_truncated_input() {
        assert_eq!(parse_self_stat("4242 (x) S 1 2 3"), None);
        assert_eq!(parse_self_stat(""), None);
        assert_eq!(parse_self_stat("no parens here"), None);
    }

    #[test]
    fn test_parse_meminfo() {
        let m = parse_meminfo(
            "MemTotal:       16000000 kB\nMemFree:         1000000 kB\nMemAvailable:    6000000 kB\n",
        )
        .unwrap();
        assert_eq!((m.total_kb, m.available_kb), (16_000_000, 6_000_000));
        let old =
            parse_meminfo("MemTotal: 1000 kB\nMemFree: 100 kB\nBuffers: 50 kB\nCached: 250 kB\n")
                .unwrap();
        assert_eq!(old.available_kb, 400);
        assert_eq!(parse_meminfo("MemFree: 1 kB\n"), None);
        assert_eq!(parse_meminfo(""), None);
        let clamped = parse_meminfo("MemTotal: 10 kB\nMemAvailable: 99 kB\n").unwrap();
        assert_eq!(clamped.available_kb, 10);
    }

    #[test]
    fn test_parse_loadavg() {
        assert_eq!(parse_loadavg("0.52 0.58 0.59 1/389 12345\n"), Some(0.52));
        assert_eq!(parse_loadavg(""), None);
        assert_eq!(parse_loadavg("nan 1 1"), None);
        assert_eq!(parse_loadavg("-1 1 1"), None);
    }

    #[test]
    fn test_parse_max_open_files() {
        let limits = "Limit                     Soft Limit           Hard Limit           Units\n\
Max cpu time              unlimited            unlimited            seconds\n\
Max open files            1024                 1048576              files\n";
        assert_eq!(parse_max_open_files(limits), Some(1024));
        assert_eq!(
            parse_max_open_files("Max open files unlimited unlimited files\n"),
            None
        );
        assert_eq!(parse_max_open_files(""), None);
    }

    #[test]
    fn test_parse_auxv_native_words() {
        let mut bytes = Vec::new();
        for (k, v) in [(3usize, 64usize), (6, 4096), (17, 100), (0, 0), (17, 999)] {
            bytes.extend_from_slice(&k.to_ne_bytes());
            bytes.extend_from_slice(&v.to_ne_bytes());
        }
        assert_eq!(parse_auxv(&bytes), (Some(100), Some(4096)));
        assert_eq!(parse_auxv(&[]), (None, None));
        assert_eq!(read_word(&[1, 2, 3]), None);
        assert_eq!(read_word(&1u32.to_ne_bytes()), Some(1));
    }
}
