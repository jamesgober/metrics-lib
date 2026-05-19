//! OpenMetrics text format ("application/openmetrics-text").
//!
//! Renders the current state of a [`crate::Registry`] to the OpenMetrics
//! exposition format (RFC-compatible superset of the Prometheus text
//! format). Differences from [`super::prometheus`]:
//!
//! - Counter sample names are suffixed with `_total` (mandatory in
//!   OpenMetrics; Prometheus accepts the same convention).
//! - `# UNIT name unit` lines are emitted for metrics whose
//!   [`crate::Unit`] is not [`crate::Unit::None`].
//! - The output terminates with `# EOF\n`.
//!
//! The text is suitable for serving with
//! `Content-Type: application/openmetrics-text; version=1.0.0; charset=utf-8`.

use crate::Registry;
#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
use crate::{LabelSet, MetricKind, MetricMetadata, Unit};
#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
use std::collections::BTreeMap;
#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
use std::fmt::Write as _;

/// Render the entire registry as an OpenMetrics text-format body.
#[must_use]
pub fn render(registry: &Registry) -> String {
    let mut out = String::with_capacity(4096);
    render_into(&mut out, registry);
    out
}

/// Append the OpenMetrics rendering of `registry` to the supplied buffer.
#[allow(clippy::ptr_arg, unused_variables)]
pub fn render_into(out: &mut String, registry: &Registry) {
    #[cfg(feature = "count")]
    render_counters(out, registry);
    #[cfg(feature = "gauge")]
    render_gauges(out, registry);
    #[cfg(feature = "timer")]
    render_timers(out, registry);
    #[cfg(feature = "meter")]
    render_rate_meters(out, registry);
    #[cfg(feature = "histogram")]
    render_histograms(out, registry);
    out.push_str("# EOF\n");
}

#[cfg(feature = "count")]
fn render_counters(out: &mut String, registry: &Registry) {
    let mut by_name: BTreeMap<String, Vec<(LabelSet, u64)>> = BTreeMap::new();
    for (name, labels, c) in registry.counter_entries() {
        by_name.entry(name).or_default().push((labels, c.get()));
    }
    for (name, mut series) in by_name {
        series.sort_by(|a, b| a.0.cmp(&b.0));
        let meta = registry.metadata(&name);
        emit_help_type_unit(out, &name, MetricKind::Counter, meta.as_ref());
        for (labels, value) in series {
            writeln!(out, "{}_total{} {}", name, labels.to_prometheus(), value).unwrap();
        }
    }
}

#[cfg(feature = "gauge")]
fn render_gauges(out: &mut String, registry: &Registry) {
    let mut by_name: BTreeMap<String, Vec<(LabelSet, f64)>> = BTreeMap::new();
    for (name, labels, g) in registry.gauge_entries() {
        by_name.entry(name).or_default().push((labels, g.get()));
    }
    for (name, mut series) in by_name {
        series.sort_by(|a, b| a.0.cmp(&b.0));
        let meta = registry.metadata(&name);
        emit_help_type_unit(out, &name, MetricKind::Gauge, meta.as_ref());
        for (labels, value) in series {
            writeln!(
                out,
                "{}{} {}",
                name,
                labels.to_prometheus(),
                format_f64(value)
            )
            .unwrap();
        }
    }
}

#[cfg(feature = "timer")]
fn render_timers(out: &mut String, registry: &Registry) {
    use std::sync::Arc;
    let mut by_name: BTreeMap<String, Vec<(LabelSet, Arc<crate::Timer>)>> = BTreeMap::new();
    for (name, labels, t) in registry.timer_entries() {
        by_name.entry(name).or_default().push((labels, t));
    }
    for (name, mut series) in by_name {
        series.sort_by(|a, b| a.0.cmp(&b.0));
        let meta = registry.metadata(&name);
        // Timers are exposed as `summary` in OpenMetrics shape (count/sum/min/max).
        emit_help_type_unit(out, &name, MetricKind::Timer, meta.as_ref());
        for (labels, t) in series {
            let labels_str = labels.to_prometheus();
            let count = t.count();
            let sum_s = t.total().as_secs_f64();
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
            writeln!(out, "{}_count{} {}", name, labels_str, count).unwrap();
            writeln!(
                out,
                "{}_sum_seconds{} {}",
                name,
                labels_str,
                format_f64(sum_s)
            )
            .unwrap();
            writeln!(
                out,
                "{}_min_seconds{} {}",
                name,
                labels_str,
                format_f64(min_s)
            )
            .unwrap();
            writeln!(
                out,
                "{}_max_seconds{} {}",
                name,
                labels_str,
                format_f64(max_s)
            )
            .unwrap();
        }
    }
}

#[cfg(feature = "meter")]
fn render_rate_meters(out: &mut String, registry: &Registry) {
    use std::sync::Arc;
    let mut by_name: BTreeMap<String, Vec<(LabelSet, Arc<crate::RateMeter>)>> = BTreeMap::new();
    for (name, labels, r) in registry.rate_meter_entries() {
        by_name.entry(name).or_default().push((labels, r));
    }
    for (name, mut series) in by_name {
        series.sort_by(|a, b| a.0.cmp(&b.0));
        let meta = registry.metadata(&name);
        emit_help_type_unit(out, &name, MetricKind::Rate, meta.as_ref());
        for (labels, r) in series {
            let labels_str = labels.to_prometheus();
            writeln!(out, "{}_total{} {}", name, labels_str, r.total()).unwrap();
            writeln!(
                out,
                "{}_per_second{} {}",
                name,
                labels_str,
                format_f64(r.rate())
            )
            .unwrap();
        }
    }
}

#[cfg(feature = "histogram")]
fn render_histograms(out: &mut String, registry: &Registry) {
    use std::sync::Arc;
    let mut by_name: BTreeMap<String, Vec<(LabelSet, Arc<crate::Histogram>)>> = BTreeMap::new();
    for (name, labels, h) in registry.histogram_entries() {
        by_name.entry(name).or_default().push((labels, h));
    }
    for (name, mut series) in by_name {
        series.sort_by(|a, b| a.0.cmp(&b.0));
        let meta = registry.metadata(&name);
        emit_help_type_unit(out, &name, MetricKind::Histogram, meta.as_ref());
        for (labels, h) in series {
            let snap = h.snapshot();
            for b in &snap.buckets {
                let labels_with_le = merge_le_label(&labels, b.upper_bound);
                writeln!(out, "{}_bucket{} {}", name, labels_with_le, b.count).unwrap();
            }
            writeln!(
                out,
                "{}_sum{} {}",
                name,
                labels.to_prometheus(),
                format_f64(snap.sum)
            )
            .unwrap();
            writeln!(
                out,
                "{}_count{} {}",
                name,
                labels.to_prometheus(),
                snap.count
            )
            .unwrap();
        }
    }
}

#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
fn emit_help_type_unit(
    out: &mut String,
    name: &str,
    kind: MetricKind,
    meta: Option<&MetricMetadata>,
) {
    if let Some(m) = meta {
        if !m.help.is_empty() {
            writeln!(out, "# HELP {} {}", name, escape_help(&m.help)).unwrap();
        }
        writeln!(out, "# TYPE {} {}", name, m.kind.as_prometheus_type()).unwrap();
        if !matches!(m.unit, Unit::None) {
            writeln!(out, "# UNIT {} {}", name, m.unit.as_str()).unwrap();
        }
    } else {
        writeln!(out, "# TYPE {} {}", name, kind.as_prometheus_type()).unwrap();
    }
}

#[cfg(feature = "histogram")]
fn merge_le_label(labels: &LabelSet, upper: f64) -> String {
    let le_value = if upper.is_infinite() {
        "+Inf".to_string()
    } else {
        format_f64(upper)
    };
    let mut merged = labels.clone();
    merged.add("le", le_value);
    merged.to_prometheus()
}

#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
fn escape_help(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(any(
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
fn format_f64(v: f64) -> String {
    if v.is_nan() {
        "NaN".to_string()
    } else if v == f64::INFINITY {
        "+Inf".to_string()
    } else if v == f64::NEG_INFINITY {
        "-Inf".to_string()
    } else {
        format!("{v}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_terminates_with_eof_line() {
        let r = Registry::new();
        let s = render(&r);
        assert!(s.ends_with("# EOF\n"));
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_has_total_suffix() {
        let r = Registry::new();
        r.get_or_create_counter("hits").inc();
        let s = render(&r);
        assert!(s.contains("# TYPE hits counter\n"), "{s}");
        assert!(s.contains("hits_total 1\n"), "{s}");
        assert!(s.ends_with("# EOF\n"));
    }

    #[test]
    #[cfg(all(feature = "count", feature = "gauge"))]
    fn unit_lines_emitted_when_metadata_present() {
        let r = Registry::new();
        r.describe_counter("bytes_total_in", "Bytes received", Unit::Bytes);
        r.describe_gauge("temp_c", "Sensor reading", Unit::Custom("celsius"));
        r.get_or_create_counter("bytes_total_in").add(1024);
        r.get_or_create_gauge("temp_c").set(21.5);
        let s = render(&r);
        assert!(s.contains("# UNIT bytes_total_in bytes\n"), "{s}");
        assert!(s.contains("# UNIT temp_c celsius\n"), "{s}");
    }

    #[test]
    #[cfg(feature = "timer")]
    fn timer_renders_summary_lines_om() {
        let r = Registry::new();
        let t = r.get_or_create_timer("rpc");
        t.record(std::time::Duration::from_millis(7));
        let s = render(&r);
        assert!(s.contains("rpc_count 1"), "{s}");
        assert!(s.contains("rpc_sum_seconds"), "{s}");
        assert!(s.contains("rpc_min_seconds"), "{s}");
        assert!(s.contains("rpc_max_seconds"), "{s}");
        assert!(s.ends_with("# EOF\n"));
    }

    #[test]
    #[cfg(feature = "meter")]
    fn rate_meter_renders_total_and_rate_om() {
        let r = Registry::new();
        r.get_or_create_rate_meter("qps").tick_n(4);
        let s = render(&r);
        assert!(s.contains("qps_total 4"), "{s}");
        assert!(s.contains("qps_per_second"), "{s}");
    }

    #[test]
    fn render_into_appends_and_finishes_with_eof() {
        let r = Registry::new();
        let mut buf = String::from("PREFIX\n");
        render_into(&mut buf, &r);
        assert!(buf.starts_with("PREFIX\n"));
        assert!(buf.ends_with("# EOF\n"));
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn histogram_round_trip() {
        let r = Registry::new();
        r.configure_histogram("rtt_seconds", [0.01, 0.05]);
        let h = r.get_or_create_histogram("rtt_seconds");
        h.observe(0.005);
        h.observe(0.02);
        let s = render(&r);
        assert!(s.contains(r#"rtt_seconds_bucket{le="0.01"} 1"#), "{s}");
        assert!(s.contains(r#"rtt_seconds_bucket{le="0.05"} 2"#), "{s}");
        assert!(s.contains(r#"rtt_seconds_bucket{le="+Inf"} 2"#), "{s}");
        assert!(s.contains("rtt_seconds_count 2"), "{s}");
        assert!(s.ends_with("# EOF\n"));
    }
}
