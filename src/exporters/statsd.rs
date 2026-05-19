//! StatsD (DogStatsD-compatible) UDP exporter (behind the `statsd` feature).
//!
//! Push-style exporter that serialises the registry into the StatsD wire
//! format (`<metric>:<value>|<type>|#tag:value,tag:value`) and ships it over
//! UDP. The DogStatsD tag extension is always used so [`LabelSet`]s round-trip
//! into native StatsD tags; vanilla StatsD agents that ignore unknown
//! suffixes still accept the lines.
//!
//! # Value mapping
//!
//! StatsD counter (`|c`) semantics are *delta* — `name:1|c` means "increment
//! by 1". The metrics-lib registry stores cumulative values, so emitting
//! deltas would require tracking `prev_value` state across exports. To keep
//! the exporter stateless, every metric is published as a **gauge** (`|g`),
//! which StatsD accepts as an absolute reading. Downstream agents that
//! prefer counter semantics can configure aggregation rules accordingly.
//!
//! - **Counter** → `name:VALUE|g` (cumulative total).
//! - **Gauge** → `name:VALUE|g`.
//! - **Timer** → four series: `name.count`, `name.sum_seconds`,
//!   `name.min_seconds`, `name.max_seconds`, all `|g`.
//! - **RateMeter** → two series: `name.total`, `name.per_second`, both `|g`.
//! - **Histogram** → `name.count`, `name.sum`, `name.p50`, `name.p95`,
//!   `name.p99`, all `|g`. Quantiles come from
//!   [`Histogram::quantile`](crate::Histogram::quantile).
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{init, metrics, LabelSet};
//! use metrics_lib::exporters::statsd::StatsdSink;
//!
//! init();
//! metrics().counter_with("requests", &LabelSet::from([("status", "200")])).inc();
//!
//! let sink = StatsdSink::new("127.0.0.1:8125").expect("bind UDP");
//! sink.send(metrics().registry()).expect("statsd push");
//! # }
//! ```

use crate::{LabelSet, Registry};
use std::fmt::Write as _;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

/// Conservative MTU budget for UDP datagrams. Picked to fit comfortably
/// inside the typical 1500-byte Ethernet MTU minus IP/UDP headers.
const DEFAULT_PACKET_SIZE: usize = 1432;

/// Push-style StatsD UDP exporter.
pub struct StatsdSink {
    socket: UdpSocket,
    target: SocketAddr,
    prefix: Option<String>,
    packet_size: usize,
}

impl StatsdSink {
    /// Bind a local UDP socket and prepare to push to `addr`.
    ///
    /// The local bind is `0.0.0.0:0` (any free ephemeral port). Use
    /// [`StatsdSink::with_socket`] for explicit control over the local
    /// binding.
    pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let target = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "empty SocketAddr list"))?;
        let bind = if target.is_ipv6() {
            "[::]:0"
        } else {
            "0.0.0.0:0"
        };
        let socket = UdpSocket::bind(bind)?;
        Ok(Self {
            socket,
            target,
            prefix: None,
            packet_size: DEFAULT_PACKET_SIZE,
        })
    }

    /// Construct from an explicit pre-bound socket + target address.
    pub fn with_socket(socket: UdpSocket, target: SocketAddr) -> Self {
        Self {
            socket,
            target,
            prefix: None,
            packet_size: DEFAULT_PACKET_SIZE,
        }
    }

    /// Set a global prefix prepended to every metric name (e.g.
    /// `"myapp."`). The exporter does NOT insert a separator — include
    /// trailing `.` in `prefix` if you want one.
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the maximum UDP datagram payload size used when batching lines.
    /// Defaults to 1432 (typical Ethernet-safe).
    #[must_use]
    pub fn with_packet_size(mut self, size: usize) -> Self {
        self.packet_size = size.max(64);
        self
    }

    /// Render every metric in `registry` and push to the configured target.
    /// Returns the total number of bytes sent across all datagrams.
    pub fn send(&self, registry: &Registry) -> io::Result<usize> {
        let body = self.render(registry);
        self.flush_lines(&body)
    }

    /// Render the registry to a single newline-separated string in StatsD
    /// wire format. Useful for unit tests, dry runs, or piping into a
    /// different transport.
    #[must_use]
    pub fn render(&self, registry: &Registry) -> String {
        let mut out = String::with_capacity(2048);
        let prefix = self.prefix.as_deref().unwrap_or("");

        #[cfg(feature = "count")]
        for (name, labels, c) in registry.counter_entries() {
            emit_gauge_line(&mut out, prefix, &name, &labels, c.get() as f64);
        }
        #[cfg(feature = "gauge")]
        for (name, labels, g) in registry.gauge_entries() {
            emit_gauge_line(&mut out, prefix, &name, &labels, g.get());
        }
        #[cfg(feature = "timer")]
        for (name, labels, t) in registry.timer_entries() {
            let count = t.count();
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.count"),
                &labels,
                count as f64,
            );
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.sum_seconds"),
                &labels,
                t.total().as_secs_f64(),
            );
            let min_s = if count == 0 {
                0.0
            } else {
                t.min().as_secs_f64()
            };
            let max_s = if count == 0 {
                0.0
            } else {
                t.max().as_secs_f64()
            };
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.min_seconds"),
                &labels,
                min_s,
            );
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.max_seconds"),
                &labels,
                max_s,
            );
        }
        #[cfg(feature = "meter")]
        for (name, labels, r) in registry.rate_meter_entries() {
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.total"),
                &labels,
                r.total() as f64,
            );
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.per_second"),
                &labels,
                r.rate(),
            );
        }
        #[cfg(feature = "histogram")]
        for (name, labels, h) in registry.histogram_entries() {
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.count"),
                &labels,
                h.count() as f64,
            );
            emit_gauge_line(&mut out, prefix, &format!("{name}.sum"), &labels, h.sum());
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.p50"),
                &labels,
                h.quantile(0.50),
            );
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.p95"),
                &labels,
                h.quantile(0.95),
            );
            emit_gauge_line(
                &mut out,
                prefix,
                &format!("{name}.p99"),
                &labels,
                h.quantile(0.99),
            );
        }
        out
    }

    /// Split `body` into MTU-bounded datagrams and send each one.
    fn flush_lines(&self, body: &str) -> io::Result<usize> {
        let mut total_sent = 0usize;
        let mut buf = String::with_capacity(self.packet_size);
        for line in body.split('\n').filter(|l| !l.is_empty()) {
            // +1 for the separating newline if buf is non-empty.
            let needed = line.len() + if buf.is_empty() { 0 } else { 1 };
            if !buf.is_empty() && buf.len() + needed > self.packet_size {
                total_sent += self.socket.send_to(buf.as_bytes(), self.target)?;
                buf.clear();
            }
            if !buf.is_empty() {
                buf.push('\n');
            }
            buf.push_str(line);
        }
        if !buf.is_empty() {
            total_sent += self.socket.send_to(buf.as_bytes(), self.target)?;
        }
        Ok(total_sent)
    }
}

fn emit_gauge_line(out: &mut String, prefix: &str, name: &str, labels: &LabelSet, value: f64) {
    let value_str = if value.is_nan() {
        "0".to_string()
    } else if value == f64::INFINITY {
        f64::MAX.to_string()
    } else if value == f64::NEG_INFINITY {
        f64::MIN.to_string()
    } else {
        format!("{value}")
    };
    write!(out, "{prefix}{name}:{value_str}|g").unwrap();
    if !labels.is_empty() {
        out.push_str(&labels.to_statsd());
    }
    out.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Registry;

    fn loopback_sink() -> (StatsdSink, std::net::SocketAddr) {
        // Bind both ends to ephemeral ports on loopback. The sink "sends" to
        // a receiver we also own so tests don't rely on an external StatsD.
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        let target = recv.local_addr().unwrap();
        let send = UdpSocket::bind("127.0.0.1:0").unwrap();
        (StatsdSink::with_socket(send, target), target)
    }

    #[test]
    fn render_emits_gauge_lines_for_counters_and_gauges() {
        let r = Registry::new();
        #[cfg(feature = "count")]
        r.get_or_create_counter("hits").add(5);
        #[cfg(feature = "gauge")]
        r.get_or_create_gauge("temp_c").set(21.5);

        let send = UdpSocket::bind("127.0.0.1:0").unwrap();
        let sink = StatsdSink::with_socket(send, "127.0.0.1:0".parse().unwrap());
        let body = sink.render(&r);

        #[cfg(feature = "count")]
        assert!(body.contains("hits:5|g\n"), "{body}");
        #[cfg(feature = "gauge")]
        assert!(body.contains("temp_c:21.5|g\n"), "{body}");
    }

    #[test]
    #[cfg(feature = "count")]
    fn labels_render_as_tags() {
        let r = Registry::new();
        let labels = LabelSet::from([("region", "us"), ("env", "prod")]);
        r.get_or_create_counter_with("requests", &labels).inc();

        let send = UdpSocket::bind("127.0.0.1:0").unwrap();
        let sink = StatsdSink::with_socket(send, "127.0.0.1:0".parse().unwrap());
        let body = sink.render(&r);
        // labels sort alphabetically: env, region.
        assert!(
            body.contains("requests:1|g|#env:prod,region:us\n"),
            "{body}"
        );
    }

    #[test]
    fn prefix_is_prepended() {
        let r = Registry::new();
        #[cfg(feature = "gauge")]
        r.get_or_create_gauge("temp").set(42.0);
        let send = UdpSocket::bind("127.0.0.1:0").unwrap();
        let sink =
            StatsdSink::with_socket(send, "127.0.0.1:0".parse().unwrap()).with_prefix("svc.");
        let body = sink.render(&r);
        #[cfg(feature = "gauge")]
        assert!(body.contains("svc.temp:42|g\n"), "{body}");
        let _ = body;
    }

    #[test]
    fn send_writes_at_least_one_datagram() {
        let r = Registry::new();
        #[cfg(feature = "count")]
        r.get_or_create_counter("hits").inc();
        let (sink, _target) = loopback_sink();
        // Even when nothing is to send, this is fine. With content, expect > 0 bytes.
        let sent = sink.send(&r).unwrap();
        #[cfg(feature = "count")]
        assert!(sent > 0);
        let _ = sent;
    }

    #[test]
    fn send_packetises_long_bodies() {
        let r = Registry::new();
        #[cfg(feature = "count")]
        {
            for i in 0..200 {
                r.get_or_create_counter(&format!("metric_{i}"))
                    .add(i as u64);
            }
        }
        let (sink, _) = loopback_sink();
        let sink = sink.with_packet_size(256); // small MTU to force splitting
        let sent = sink.send(&r).unwrap();
        let _ = sent;
    }
}
