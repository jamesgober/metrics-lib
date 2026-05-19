//! Prometheus text exposition format.
//!
//! Renders the current state of a [`crate::Registry`] to the text format
//! consumed by `prometheus`-compatible scrapers. The output is a `String`
//! ready to be served from a `GET /metrics` HTTP handler with
//! `Content-Type: text/plain; version=0.0.4`.
//!
//! # Layout
//!
//! Each metric block emits, in order:
//!
//! ```text
//! # HELP <name> <help text>
//! # TYPE <name> <type>
//! <name>{labels} <value>
//! ```
//!
//! - **Counter** → Prometheus `counter`. One sample per `(name, labels)`.
//! - **Gauge** → Prometheus `gauge`. One sample per `(name, labels)`.
//! - **Timer** → Prometheus `summary` shape: `_count`, `_sum_seconds`,
//!   `_min_seconds`, `_max_seconds`. Real percentile quantiles are
//!   delegated to the `Histogram` type.
//! - **RateMeter** → Prometheus `gauge` reporting `_per_second` events.
//! - **Histogram** → Prometheus `histogram` with cumulative `_bucket{le="…"}`
//!   rows, `_sum`, and `_count`.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{init, metrics, LabelSet};
//! use metrics_lib::exporters::prometheus;
//!
//! init();
//! metrics().counter("requests").inc();
//! metrics().counter_with("requests", &LabelSet::from([("status", "200")])).add(3);
//!
//! let body = prometheus::render(metrics().registry());
//! assert!(body.contains("# TYPE requests counter"));
//! assert!(body.contains("requests 1"));
//! assert!(body.contains(r#"requests{status="200"} 3"#));
//! # }
//! ```

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

/// Render the entire registry as a Prometheus text-format body.
///
/// The output:
/// - Sorts every metric block by name for deterministic, diff-friendly
///   output.
/// - Sorts each metric's `(labels)` rows for deterministic output within a
///   block.
/// - Always emits a final newline.
#[must_use]
pub fn render(registry: &Registry) -> String {
    let mut out = String::with_capacity(4096);
    render_into(&mut out, registry);
    out
}

/// Append the Prometheus rendering of `registry` to the supplied buffer.
/// Useful for re-using a buffer across scrapes.
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
        emit_help_and_type(out, &name, MetricKind::Counter, meta.as_ref());
        for (labels, value) in series {
            writeln!(
                out,
                "{}{} {}",
                name,
                labels.to_prometheus(),
                format_u64(value)
            )
            .unwrap();
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
        emit_help_and_type(out, &name, MetricKind::Gauge, meta.as_ref());
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
        emit_help_and_type(out, &name, MetricKind::Timer, meta.as_ref());
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
        emit_help_and_type(out, &name, MetricKind::Rate, meta.as_ref());
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
        emit_help_and_type(out, &name, MetricKind::Histogram, meta.as_ref());
        for (labels, h) in series {
            let snap = h.snapshot();
            for b in &snap.buckets {
                // Inject `le="..."` alongside user labels.
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
fn emit_help_and_type(
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
    } else {
        writeln!(out, "# TYPE {} {}", name, kind.as_prometheus_type()).unwrap();
    }
    // Bonus: emit unit comment when known (Prometheus ignores `# UNIT`, but
    // operators read it).
    if let Some(m) = meta {
        if !matches!(m.unit, Unit::None) {
            writeln!(out, "# UNIT {} {}", name, m.unit.as_str()).unwrap();
        }
    }
}

/// Insert/replace an `le="..."` label inside an existing label set, returning
/// the Prometheus-formatted suffix (`{labels,le="..."}`).
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
    // Prometheus help text: `\` → `\\`, `\n` → `\n` (literal backslash-n).
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

#[cfg(any(feature = "count", feature = "meter"))]
#[inline]
fn format_u64(v: u64) -> String {
    v.to_string()
}

/// Prometheus value formatter: `+Inf` / `-Inf` / `NaN` / fixed decimal.
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
        // Use Display: emits e.g. "42", "42.5", "1.234e-5", etc.
        format!("{v}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_renders_empty() {
        let r = Registry::new();
        let s = render(&r);
        assert!(s.is_empty());
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_renders_with_type_line() {
        let r = Registry::new();
        let c = r.get_or_create_counter("hits");
        c.add(7);
        let s = render(&r);
        assert!(s.contains("# TYPE hits counter\n"));
        assert!(s.contains("hits 7\n"));
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_with_help_and_unit_emits_metadata() {
        let r = Registry::new();
        r.describe_counter("hits", "Total hits served", Unit::Custom("hits"));
        r.get_or_create_counter("hits").inc();
        let s = render(&r);
        assert!(s.contains("# HELP hits Total hits served\n"));
        assert!(s.contains("# TYPE hits counter\n"));
        assert!(s.contains("# UNIT hits hits\n"));
        assert!(s.contains("hits 1\n"));
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_with_labels_renders_braces() {
        let r = Registry::new();
        let labels = LabelSet::from([("method", "GET"), ("status", "200")]);
        r.get_or_create_counter_with("requests", &labels).add(4);
        let s = render(&r);
        assert!(s.contains(r#"requests{method="GET",status="200"} 4"#));
    }

    #[test]
    #[cfg(feature = "gauge")]
    fn gauge_with_non_finite_renders_special_tokens() {
        let r = Registry::new();
        r.get_or_create_gauge("temp").set(f64::INFINITY);
        let s = render(&r);
        assert!(s.contains("# TYPE temp gauge\n"));
        assert!(s.contains("temp +Inf\n"));
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn histogram_renders_cumulative_buckets() {
        let r = Registry::new();
        r.configure_histogram("latency_seconds", [0.1, 0.5, 1.0]);
        let h = r.get_or_create_histogram("latency_seconds");
        h.observe(0.05);
        h.observe(0.2);
        h.observe(0.8);
        h.observe(5.0);
        let s = render(&r);
        // Buckets are cumulative.
        assert!(s.contains(r#"latency_seconds_bucket{le="0.1"} 1"#));
        assert!(s.contains(r#"latency_seconds_bucket{le="0.5"} 2"#));
        assert!(s.contains(r#"latency_seconds_bucket{le="1"} 3"#));
        assert!(s.contains(r#"latency_seconds_bucket{le="+Inf"} 4"#));
        assert!(s.contains("latency_seconds_count 4"));
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn labeled_histogram_merges_le_with_user_labels() {
        let r = Registry::new();
        r.configure_histogram("rtt", [0.01, 0.05]);
        let labels = LabelSet::from([("region", "us")]);
        let h = r.get_or_create_histogram_with("rtt", &labels);
        h.observe(0.005);
        let s = render(&r);
        // `le` must coexist with `region`; sorted alphabetically: le before region.
        assert!(
            s.contains(r#"rtt_bucket{le="0.01",region="us"} 1"#),
            "got: {s}"
        );
        assert!(
            s.contains(r#"rtt_bucket{le="+Inf",region="us"} 1"#),
            "got: {s}"
        );
    }

    #[test]
    #[cfg(feature = "count")]
    fn counter_overflow_sink_not_exported() {
        let r = Registry::new();
        r.set_cardinality_cap(1);
        let _ = r.get_or_create_counter_with("c", &LabelSet::from([("k", "1")]));
        // This routes to the sink.
        r.get_or_create_counter_with("c", &LabelSet::from([("k", "2")]))
            .inc();
        let s = render(&r);
        // Only one labeled series for "c" should appear.
        assert_eq!(s.matches(r#"c{k=""#).count(), 1, "got: {s}");
    }

    #[test]
    #[cfg(any(
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram"
    ))]
    fn format_f64_special_values() {
        assert_eq!(format_f64(f64::INFINITY), "+Inf");
        assert_eq!(format_f64(f64::NEG_INFINITY), "-Inf");
        assert_eq!(format_f64(f64::NAN), "NaN");
        assert_eq!(format_f64(0.0), "0");
        assert_eq!(format_f64(42.5), "42.5");
    }
}
