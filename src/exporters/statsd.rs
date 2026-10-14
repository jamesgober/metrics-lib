//! StatsD / `DogStatsD` UDP exporter (feature `statsd`).
//!
//! [`StatsdSink`] renders the registry as StatsD lines
//! (`name:value|type|#tag:value,…`) and pushes them over UDP, batching lines
//! into datagrams no larger than the configured packet size. Labels become
//! `DogStatsD` tags; vanilla StatsD servers that do not understand tags must
//! not be fed labeled series.
//!
//! # Counters
//!
//! StatsD counters (`|c`) carry *increments*, while the registry holds
//! running totals. With [`CounterMode::Delta`] (the default) the sink
//! remembers the value sent last time for each series and sends the
//! difference, which is what StatsD aggregation expects; a counter that went
//! backwards (it was reset) sends its full value. [`CounterMode::Gauge`]
//! sends running totals as gauges instead (stateless, but the server must be
//! told not to sum them).
//!
//! # Mapping
//!
//! | Registry kind | Lines |
//! |---|---|
//! | counter | `name:Δ|c` (or `name:total|g`) |
//! | up/down counter, gauge | `name:value|g` |
//! | timer | `name.count` (counter rule), `name.sum_seconds`, `name.min_seconds`, `name.max_seconds` as `|g` |
//! | rate meter | `name.total` (counter rule), `name.per_second|g` |
//! | histogram | `name.count` (counter rule), `name.sum`, `name.p50`, `name.p95`, `name.p99` as `|g` |
//! | summary | `name.count` (counter rule), `name.sum` and one `name.pNN` per quantile as `|g` |
//!
//! A negative gauge is sent as `name:0|g` followed by `name:-x|g`: in the
//! StatsD protocol a leading sign means "adjust by", so a bare `name:-5|g`
//! would subtract 5 instead of setting -5. NaN and infinite gauges are
//! skipped (StatsD cannot represent them).
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{LabelSet, MetricsCore};
//! use metrics_lib::exporters::statsd::StatsdSink;
//!
//! let core = MetricsCore::new();
//! core.counter_with("requests", &LabelSet::from([("status", "200")])).inc();
//!
//! let sink = StatsdSink::new("127.0.0.1:8125")?.with_prefix("myapp.");
//! sink.send(core.registry())?;
//! # }
//! # Ok::<(), std::io::Error>(())
//! ```

use crate::labels::push_statsd_safe;
use crate::registry::{FamilySnapshot, Handle};
use crate::{LabelSet, MetricKind, Registry};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Mutex, PoisonError};

/// Default datagram payload budget: fits a 1500-byte Ethernet MTU after IP
/// and UDP headers.
pub const DEFAULT_PACKET_SIZE: usize = 1432;
/// Smallest accepted packet size.
pub const MIN_PACKET_SIZE: usize = 64;

/// How counters are sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum CounterMode {
    /// Send the increase since the previous export as `|c`.
    #[default]
    Delta,
    /// Send the running total as `|g`.
    Gauge,
}

/// StatsD UDP exporter.
pub struct StatsdSink {
    socket: UdpSocket,
    target: SocketAddr,
    prefix: String,
    packet_size: usize,
    counter_mode: CounterMode,
    /// Last total sent per counter line, for [`CounterMode::Delta`].
    previous: Mutex<HashMap<(String, LabelSet), u64>>,
}

impl StatsdSink {
    /// Bind an ephemeral local UDP socket and target `addr`.
    ///
    /// # Errors
    ///
    /// Returns the error from resolving `addr` (or an `InvalidInput` error
    /// when it resolves to nothing) or from binding the socket.
    pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let target = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "address resolved to nothing")
        })?;
        let bind = if target.is_ipv6() {
            "[::]:0"
        } else {
            "0.0.0.0:0"
        };
        Ok(Self::with_socket(UdpSocket::bind(bind)?, target))
    }

    /// Use an already bound socket.
    #[must_use]
    pub fn with_socket(socket: UdpSocket, target: SocketAddr) -> Self {
        Self {
            socket,
            target,
            prefix: String::new(),
            packet_size: DEFAULT_PACKET_SIZE,
            counter_mode: CounterMode::Delta,
            previous: Mutex::new(HashMap::new()),
        }
    }

    /// Prefix every metric name (no separator is added; include the trailing
    /// `.` yourself). Characters StatsD reserves are replaced by `_`.
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl AsRef<str>) -> Self {
        self.prefix.clear();
        push_statsd_safe(&mut self.prefix, prefix.as_ref());
        self
    }

    /// Maximum datagram payload (at least [`MIN_PACKET_SIZE`]).
    #[must_use]
    pub fn with_packet_size(mut self, size: usize) -> Self {
        self.packet_size = size.max(MIN_PACKET_SIZE);
        self
    }

    /// Choose how counters are sent.
    #[must_use]
    pub fn with_counter_mode(mut self, mode: CounterMode) -> Self {
        self.counter_mode = mode;
        self
    }

    /// Render `registry` and send it. Returns the number of bytes sent.
    ///
    /// In [`CounterMode::Delta`] the counter baselines advance even if a
    /// send fails part-way, so increments in a failed datagram are lost
    /// rather than double-counted later.
    ///
    /// # Errors
    ///
    /// Returns the first socket error.
    pub fn send(&self, registry: &Registry) -> io::Result<usize> {
        let body = self.render(registry);
        self.send_lines(&body)
    }

    /// Render `registry` as newline-separated StatsD lines without sending.
    ///
    /// In [`CounterMode::Delta`] this advances the counter baselines, exactly
    /// like [`StatsdSink::send`].
    #[must_use]
    pub fn render(&self, registry: &Registry) -> String {
        let mut out = String::with_capacity(2048);
        let mut previous = self.previous.lock().unwrap_or_else(PoisonError::into_inner);
        let mut seen: HashMap<(String, LabelSet), u64> = HashMap::with_capacity(previous.len());
        let mut w = Writer {
            out: &mut out,
            prefix: &self.prefix,
            mode: self.counter_mode,
            previous: &previous,
            seen: &mut seen,
        };
        // Derived names (`<timer>.count`, `<rate>.per_second`, …) can equal
        // another family's name; the first family in name order keeps them,
        // as in the text exporters, and the clash is counted. Delta baselines
        // are therefore never shared between two series.
        let mut claimed: HashSet<String> = HashSet::new();
        for family in registry.collect() {
            let names = emitted_names(&family);
            if names.iter().any(|n| claimed.contains(n)) {
                registry.note_exposition_conflict();
                continue;
            }
            claimed.extend(names);
            for s in &family.series {
                w.series(&family.name, &s.labels, &s.handle);
            }
        }
        *previous = seen;
        out
    }

    fn send_lines(&self, body: &str) -> io::Result<usize> {
        let mut sent = 0;
        let mut packet = String::with_capacity(self.packet_size);
        for line in body.lines().filter(|l| !l.is_empty()) {
            let needed = line.len() + usize::from(!packet.is_empty());
            if !packet.is_empty() && packet.len() + needed > self.packet_size {
                sent += self.socket.send_to(packet.as_bytes(), self.target)?;
                packet.clear();
            }
            if !packet.is_empty() {
                packet.push('\n');
            }
            packet.push_str(line);
        }
        if !packet.is_empty() {
            sent += self.socket.send_to(packet.as_bytes(), self.target)?;
        }
        Ok(sent)
    }
}

impl std::fmt::Debug for StatsdSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatsdSink")
            .field("target", &self.target)
            .field("prefix", &self.prefix)
            .field("packet_size", &self.packet_size)
            .field("counter_mode", &self.counter_mode)
            .finish_non_exhaustive()
    }
}

struct Writer<'a> {
    out: &'a mut String,
    prefix: &'a str,
    mode: CounterMode,
    previous: &'a HashMap<(String, LabelSet), u64>,
    seen: &'a mut HashMap<(String, LabelSet), u64>,
}

impl Writer<'_> {
    fn line(&mut self, name: &str, value: &str, kind: &str, labels: &LabelSet) {
        let _ = write!(self.out, "{}{name}:{value}|{kind}", self.prefix);
        labels.write_statsd(self.out);
        self.out.push('\n');
    }

    fn gauge(&mut self, name: &str, value: f64, labels: &LabelSet) {
        if !value.is_finite() {
            return;
        }
        if value < 0.0 {
            self.line(name, "0", "g", labels);
        }
        self.line(name, &value.to_string(), "g", labels);
    }

    fn gauge_i64(&mut self, name: &str, value: i64, labels: &LabelSet) {
        if value < 0 {
            self.line(name, "0", "g", labels);
        }
        self.line(name, &value.to_string(), "g", labels);
    }

    fn counter(&mut self, name: &str, total: u64, labels: &LabelSet) {
        match self.mode {
            CounterMode::Gauge => self.line(name, &total.to_string(), "g", labels),
            CounterMode::Delta => {
                let key = (name.to_owned(), labels.clone());
                let delta = match self.previous.get(&key) {
                    Some(prev) if *prev <= total => total - prev,
                    _ => total,
                };
                let _ = self.seen.insert(key, total);
                if delta > 0 {
                    self.line(name, &delta.to_string(), "c", labels);
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn series(&mut self, name: &str, labels: &LabelSet, handle: &Handle) {
        match handle {
            #[cfg(feature = "count")]
            Handle::Counter(c) => self.counter(name, c.get(), labels),
            #[cfg(feature = "count")]
            Handle::ShardedCounter(c) => self.counter(name, c.get(), labels),
            #[cfg(feature = "count")]
            Handle::UpDownCounter(c) => self.gauge_i64(name, c.get(), labels),
            #[cfg(feature = "gauge")]
            Handle::Gauge(g) => self.gauge(name, g.get(), labels),
            #[cfg(feature = "timer")]
            Handle::Timer(t) => {
                let count = t.count();
                self.counter(&format!("{name}.count"), count, labels);
                #[allow(clippy::cast_precision_loss)] // seconds as f64 is the contract
                let sum = t.total_nanos() as f64 / 1e9;
                self.gauge(&format!("{name}.sum_seconds"), sum, labels);
                if count > 0 {
                    self.gauge(
                        &format!("{name}.min_seconds"),
                        t.min().as_secs_f64(),
                        labels,
                    );
                    self.gauge(
                        &format!("{name}.max_seconds"),
                        t.max().as_secs_f64(),
                        labels,
                    );
                }
            }
            #[cfg(feature = "meter")]
            Handle::Rate(r) => {
                self.counter(&format!("{name}.total"), r.total(), labels);
                self.gauge(&format!("{name}.per_second"), r.rate(), labels);
            }
            #[cfg(feature = "histogram")]
            Handle::Histogram(h) => {
                let count = h.count();
                self.counter(&format!("{name}.count"), count, labels);
                self.gauge(&format!("{name}.sum"), h.sum(), labels);
                if count > 0 {
                    for (suffix, q) in [("p50", 0.5), ("p95", 0.95), ("p99", 0.99)] {
                        self.gauge(&format!("{name}.{suffix}"), h.quantile(q), labels);
                    }
                }
            }
            #[cfg(feature = "histogram")]
            Handle::Summary(s) => {
                let snap = s.snapshot();
                self.counter(&format!("{name}.count"), snap.count, labels);
                self.gauge(&format!("{name}.sum"), snap.sum, labels);
                for (q, v) in &snap.quantiles {
                    self.gauge(&format!("{name}.{}", quantile_suffix(*q)), *v, labels);
                }
            }
            #[cfg(not(any(
                feature = "count",
                feature = "gauge",
                feature = "timer",
                feature = "meter",
                feature = "histogram"
            )))]
            _ => match *handle {},
        }
    }
}

/// Every metric name `family` writes (before the sink prefix).
fn emitted_names(family: &FamilySnapshot) -> Vec<String> {
    let name = &family.name;
    let derived = |suffixes: &[&str]| -> Vec<String> {
        suffixes.iter().map(|s| format!("{name}.{s}")).collect()
    };
    match family.kind {
        MetricKind::Timer => derived(&["count", "sum_seconds", "min_seconds", "max_seconds"]),
        MetricKind::Rate => derived(&["total", "per_second"]),
        MetricKind::Histogram => derived(&["count", "sum", "p50", "p95", "p99"]),
        MetricKind::Summary => {
            // Summaries only exist with `histogram`.
            #[cfg_attr(not(feature = "histogram"), allow(unused_mut))]
            let mut names = derived(&["count", "sum"]);
            #[cfg(feature = "histogram")]
            for s in &family.series {
                if let Handle::Summary(summary) = &s.handle {
                    for q in summary.quantile_targets() {
                        let n = format!("{name}.{}", quantile_suffix(*q));
                        if !names.contains(&n) {
                            names.push(n);
                        }
                    }
                }
            }
            names
        }
        _ => vec![name.clone()],
    }
}

/// `0.5` → `p50`, `0.999` → `p99_9`.
#[cfg_attr(not(feature = "histogram"), allow(dead_code))]
fn quantile_suffix(q: f64) -> String {
    let pct = (q * 1000.0).round() / 10.0;
    let text = pct.to_string();
    format!("p{}", text.replace('.', "_"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn loopback() -> (StatsdSink, UdpSocket) {
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        recv.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        let target = recv.local_addr().unwrap();
        let send = UdpSocket::bind("127.0.0.1:0").unwrap();
        (StatsdSink::with_socket(send, target), recv)
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_counters_send_deltas_and_handle_resets() {
        let (sink, _recv) = loopback();
        let r = Registry::new();
        let c = r.counter("hits");
        c.add(5);
        assert_eq!(sink.render(&r), "hits:5|c\n");
        c.add(3);
        assert_eq!(sink.render(&r), "hits:3|c\n");
        assert_eq!(sink.render(&r), "", "no increase, no line");
        c.reset();
        c.add(2);
        assert_eq!(sink.render(&r), "hits:2|c\n");
        // Gauge mode sends totals.
        let gauges = loopback().0.with_counter_mode(CounterMode::Gauge);
        assert_eq!(gauges.render(&r), "hits:2|g\n");
    }

    #[cfg(all(feature = "count", feature = "timer"))]
    #[test]
    fn test_derived_name_collisions_are_skipped_regression_ml2_007() {
        let (sink, _recv) = loopback();
        let r = Registry::new();
        // The timer `t` emits `t.count`; the counter `t.count` clashes.
        r.timer("t").record_ns(1_000);
        let clash = r.counter("t.count");
        clash.add(10);
        let first = sink.render(&r);
        assert_eq!(first.matches("t.count:").count(), 1, "{first}");
        assert!(first.contains("t.count:1|c"), "{first}");
        assert_eq!(r.exposition_conflicts(), 1);
        r.timer("t").record_ns(1_000);
        clash.add(10);
        // The timer's delta baseline is its own: one new sample.
        let second = sink.render(&r);
        assert!(second.contains("t.count:1|c"), "{second}");
        assert_eq!(second.matches("t.count:").count(), 1, "{second}");
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_delta_state_is_pruned_for_removed_series() {
        let (sink, _recv) = loopback();
        let r = Registry::new();
        r.counter("gone").add(5);
        let _ = sink.render(&r);
        assert!(r.remove("gone"));
        let _ = sink.render(&r);
        assert!(sink.previous.lock().unwrap().is_empty());
    }

    #[cfg(feature = "gauge")]
    #[test]
    fn test_negative_gauges_are_absolute_regression_ml019() {
        // ML-019: `neg:-5|g` is a relative decrement in StatsD.
        let (sink, _recv) = loopback();
        let r = Registry::new();
        r.gauge("neg").set(-5.0);
        r.gauge("nan").set(f64::NAN);
        r.gauge("pos").set(1.5);
        assert_eq!(sink.render(&r), "neg:0|g\nneg:-5|g\npos:1.5|g\n");
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_up_down_counter_negative_is_absolute() {
        let (sink, _recv) = loopback();
        let r = Registry::new();
        r.up_down_counter("level").add(-3);
        assert_eq!(sink.render(&r), "level:0|g\nlevel:-3|g\n");
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_tags_and_prefix_are_sanitised_regression_ml009() {
        let (sink, _recv) = loopback();
        let sink = sink.with_prefix("app|x.");
        let r = Registry::new();
        r.counter_with("hits", &LabelSet::from([("bad\"key}\n", "v|#@\n")]))
            .inc();
        let out = sink.render(&r);
        assert_eq!(out.lines().count(), 1, "{out:?}");
        assert!(out.starts_with("app_x.hits:1|c|#"), "{out:?}");
    }

    #[cfg(all(feature = "timer", feature = "meter", feature = "histogram"))]
    #[test]
    fn test_every_type_renders() {
        let (sink, _recv) = loopback();
        let r = Registry::new();
        r.timer("t").record_ns(2_000_000_000);
        r.rate_meter("r").tick_n(4);
        r.histogram("h").observe(0.2);
        r.summary("s").observe(0.2);
        r.sharded_counter("sc").add(2);
        let out = sink.render(&r);
        for needle in [
            "t.count:1|c",
            "t.sum_seconds:2|g",
            "t.min_seconds:2|g",
            "t.max_seconds:2|g",
            "r.total:4|c",
            "r.per_second:",
            "h.count:1|c",
            "h.sum:0.2|g",
            "h.p50:",
            "s.count:1|c",
            "s.p50:",
            "s.p99:",
            "sc:2|c",
        ] {
            assert!(out.contains(needle), "{needle} missing in {out}");
        }
    }

    #[test]
    fn test_quantile_suffix() {
        assert_eq!(quantile_suffix(0.5), "p50");
        assert_eq!(quantile_suffix(0.99), "p99");
        assert_eq!(quantile_suffix(0.999), "p99_9");
        assert_eq!(quantile_suffix(1.0), "p100");
    }

    #[cfg(feature = "count")]
    #[test]
    fn test_send_batches_into_packets() {
        let (sink, recv) = loopback();
        let sink = sink.with_packet_size(64);
        let r = Registry::new();
        for i in 0..20 {
            r.counter(&format!("counter_number_{i:02}")).inc();
        }
        let sent = sink.send(&r).unwrap();
        let mut received = 0;
        let mut buf = [0u8; 2048];
        while received < sent {
            let n = recv.recv(&mut buf).unwrap();
            assert!(n <= 64, "packet of {n} bytes");
            received += n;
        }
        assert_eq!(received, sent);
        assert!(format!("{sink:?}").contains("StatsdSink"));
    }

    #[test]
    fn test_new_resolves_and_binds() {
        let sink = StatsdSink::new("127.0.0.1:8125").unwrap();
        assert_eq!(sink.target.port(), 8125);
        assert_eq!(sink.with_packet_size(1).packet_size, MIN_PACKET_SIZE);
        assert!(StatsdSink::new(Vec::<SocketAddr>::new().as_slice()).is_err());
    }
}
