//! Shared writer for the Prometheus and OpenMetrics text formats.
//!
//! Both formats share one model: every registry family becomes one or more
//! *exposition families* (a timer, for example, becomes a `summary` plus
//! `_min` / `_max` gauges). Before a family is written, its name and every
//! sample name it would produce are checked against the names already
//! written; a family that would clash is skipped and counted in
//! [`Registry::exposition_conflicts`](crate::Registry::exposition_conflicts),
//! so the output is always a valid exposition.

use crate::labels::escape_label_value;
use crate::name::prometheus_name;
use crate::registry::{FamilySnapshot, Handle, SeriesSnapshot};
use crate::{LabelSet, MetricKind, Registry, Unit};
use std::collections::HashSet;
use std::fmt::Write as _;

/// Which text format to produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Dialect {
    /// Prometheus text format 0.0.4.
    Prometheus,
    /// OpenMetrics 1.0.0 text format.
    OpenMetrics,
}

/// Names already claimed by written families.
#[derive(Default)]
pub(crate) struct Claims {
    names: HashSet<String>,
}

impl Claims {
    /// Claim `names` if none of them is taken yet.
    fn try_claim(&mut self, names: &[String]) -> bool {
        if names.iter().any(|n| self.names.contains(n)) {
            return false;
        }
        for n in names {
            let _ = self.names.insert(n.clone());
        }
        true
    }
}

/// `TYPE` token of an exposition family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Counter,
    Gauge,
    Summary,
    Histogram,
}

impl Type {
    fn token(self) -> &'static str {
        match self {
            Type::Counter => "counter",
            Type::Gauge => "gauge",
            Type::Summary => "summary",
            Type::Histogram => "histogram",
        }
    }
}

/// One exposition family about to be written.
struct Family<'a> {
    name: String,
    ty: Type,
    help: Option<&'a str>,
    unit: Option<&'static str>,
    body: String,
    /// Sample names (besides `name`) this family produces.
    suffixes: &'static [&'static str],
}

/// Format a sample value. Uses Rust's shortest round-trip representation,
/// which both formats accept, plus the special spellings for NaN/±Inf.
#[cfg_attr(
    not(any(
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram",
        feature = "system"
    )),
    allow(dead_code)
)]
pub(crate) fn fmt_value(out: &mut String, v: f64) {
    if v.is_nan() {
        out.push_str("NaN");
    } else if v.is_infinite() {
        out.push_str(if v > 0.0 { "+Inf" } else { "-Inf" });
    } else {
        let _ = write!(out, "{v:?}");
    }
}

/// Canonical rendering for `le` / `quantile` label values (OpenMetrics
/// "Canonical Numbers"): Go's `%g` with `.0` appended when the result has
/// neither a decimal point nor an exponent.
#[cfg_attr(not(feature = "histogram"), allow(dead_code))]
pub(crate) fn canonical_float(v: f64) -> String {
    if v.is_nan() {
        return "NaN".to_owned();
    }
    if v.is_infinite() {
        return if v > 0.0 { "+Inf" } else { "-Inf" }.to_owned();
    }
    if v == 0.0 {
        return if v.is_sign_negative() { "-0.0" } else { "0.0" }.to_owned();
    }
    // `{:e}` gives the shortest round-trip digits: "1.2345e-5", "1e6".
    let sci = format!("{:e}", v.abs());
    let (mantissa, exp) = sci.split_once('e').unwrap_or((sci.as_str(), "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let mut out = String::with_capacity(digits.len() + 8);
    if v < 0.0 {
        out.push('-');
    }
    if !(-4..6).contains(&exp) {
        out.push_str(&digits[..1]);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        let _ = write!(
            out,
            "e{}{:02}",
            if exp < 0 { '-' } else { '+' },
            exp.unsigned_abs()
        );
    } else if exp < 0 {
        out.push_str("0.");
        for _ in 0..(-exp - 1) {
            out.push('0');
        }
        out.push_str(&digits);
    } else {
        #[allow(clippy::cast_sign_loss)] // exp is in 0..6 here
        let int_len = exp as usize + 1;
        if digits.len() <= int_len {
            out.push_str(&digits);
            for _ in digits.len()..int_len {
                out.push('0');
            }
            out.push_str(".0");
        } else {
            out.push_str(&digits[..int_len]);
            out.push('.');
            out.push_str(&digits[int_len..]);
        }
    }
    out
}

/// Escape HELP text. Prometheus escapes `\` and line feed; OpenMetrics also
/// escapes `"`.
fn escape_help(out: &mut String, help: &str, dialect: Dialect) {
    match dialect {
        Dialect::OpenMetrics => escape_label_value(out, help),
        Dialect::Prometheus => {
            for c in help.chars() {
                match c {
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push(' '),
                    c => out.push(c),
                }
            }
        }
    }
}

/// Append one sample line.
fn sample(out: &mut String, name: &str, labels: &LabelSet, extra: Option<(&str, &str)>) {
    out.push_str(name);
    labels.write_prometheus(out, extra);
    out.push(' ');
}

#[cfg_attr(
    not(any(
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram",
        feature = "system"
    )),
    allow(dead_code)
)]
fn sample_f64(
    out: &mut String,
    name: &str,
    labels: &LabelSet,
    extra: Option<(&str, &str)>,
    v: f64,
) {
    sample(out, name, labels, extra);
    fmt_value(out, v);
    out.push('\n');
}

fn sample_int(
    out: &mut String,
    name: &str,
    labels: &LabelSet,
    extra: Option<(&str, &str)>,
    v: impl std::fmt::Display,
) {
    sample(out, name, labels, extra);
    let _ = write!(out, "{v}");
    out.push('\n');
}

/// Write `fam` unless one of its names is already claimed.
fn emit(
    out: &mut String,
    fam: &Family<'_>,
    dialect: Dialect,
    claims: &mut Claims,
    registry: Option<&Registry>,
) {
    if fam.body.is_empty() {
        return;
    }
    let mut names = vec![fam.name.clone()];
    names.extend(fam.suffixes.iter().map(|s| format!("{}{s}", fam.name)));
    if !claims.try_claim(&names) {
        if let Some(r) = registry {
            r.note_exposition_conflict();
        }
        return;
    }
    let ty = fam.ty.token();
    match dialect {
        Dialect::Prometheus => {
            if let Some(help) = fam.help.filter(|h| !h.is_empty()) {
                let _ = write!(out, "# HELP {} ", fam.name);
                escape_help(out, help, dialect);
                out.push('\n');
            }
            let _ = writeln!(out, "# TYPE {} {ty}", fam.name);
        }
        Dialect::OpenMetrics => {
            let _ = writeln!(out, "# TYPE {} {ty}", fam.name);
            if let Some(unit) = fam.unit {
                let _ = writeln!(out, "# UNIT {} {unit}", fam.name);
            }
            if let Some(help) = fam.help.filter(|h| !h.is_empty()) {
                let _ = write!(out, "# HELP {} ", fam.name);
                escape_help(out, help, dialect);
                out.push('\n');
            }
        }
    }
    out.push_str(&fam.body);
}

/// OpenMetrics `# UNIT` value for `family_name`, if `unit` is set, made of
/// metric-name characters, and a suffix of the name (as the spec requires).
fn om_unit(family_name: &str, unit: Unit) -> Option<&'static str> {
    let u = unit.as_str();
    let valid = !u.is_empty() && u.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    (valid && family_name.len() > u.len() + 1 && family_name.ends_with(u) && {
        let cut = family_name.len() - u.len() - 1;
        family_name.as_bytes()[cut] == b'_'
    })
    .then_some(u)
}

/// Family name for a counter: OpenMetrics names the family without the
/// `_total` suffix its samples carry.
fn counter_family_name(base: &str, dialect: Dialect) -> String {
    match dialect {
        Dialect::OpenMetrics => base
            .strip_suffix("_total")
            .filter(|s| !s.is_empty())
            .unwrap_or(base)
            .to_owned(),
        Dialect::Prometheus => base.to_owned(),
    }
}

const COUNTER_SUFFIXES_OM: &[&str] = &["_total", "_created"];
const SUMMARY_SUFFIXES: &[&str] = &["_sum", "_count", "_created"];
const HISTOGRAM_SUFFIXES: &[&str] = &["_bucket", "_sum", "_count", "_created"];
const NO_SUFFIXES: &[&str] = &[];

/// Write every family of `registry`.
pub(crate) fn write_registry(
    out: &mut String,
    registry: &Registry,
    dialect: Dialect,
    claims: &mut Claims,
) {
    for family in registry.collect() {
        write_family(out, &family, dialect, claims, registry);
    }
}

#[allow(clippy::too_many_lines)] // one match arm per metric kind
#[allow(unused_mut)] // the gauge arm's body stays empty without count/gauge
fn write_family(
    out: &mut String,
    family: &FamilySnapshot,
    dialect: Dialect,
    claims: &mut Claims,
    registry: &Registry,
) {
    let base = prometheus_name(&family.name).into_owned();
    let help = family.help.as_deref();
    let unit = family.unit;
    match family.kind {
        MetricKind::Counter => {
            let name = counter_family_name(&base, dialect);
            let sample_name = match dialect {
                Dialect::OpenMetrics => format!("{name}_total"),
                Dialect::Prometheus => name.clone(),
            };
            let mut body = String::new();
            for s in &family.series {
                if let Some(v) = counter_value(s) {
                    sample_int(&mut body, &sample_name, &s.labels, None, v);
                }
            }
            let fam = Family {
                unit: om_unit(&name, unit),
                name,
                ty: Type::Counter,
                help,
                body,
                suffixes: if dialect == Dialect::OpenMetrics {
                    COUNTER_SUFFIXES_OM
                } else {
                    NO_SUFFIXES
                },
            };
            emit(out, &fam, dialect, claims, Some(registry));
        }
        MetricKind::UpDownCounter | MetricKind::Gauge => {
            let mut body = String::new();
            for s in &family.series {
                match &s.handle {
                    #[cfg(feature = "count")]
                    Handle::UpDownCounter(c) => {
                        sample_int(&mut body, &base, &s.labels, None, c.get());
                    }
                    #[cfg(feature = "gauge")]
                    Handle::Gauge(g) => sample_f64(&mut body, &base, &s.labels, None, g.get()),
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
            let fam = Family {
                unit: om_unit(&base, unit),
                name: base,
                ty: Type::Gauge,
                help,
                body,
                suffixes: NO_SUFFIXES,
            };
            emit(out, &fam, dialect, claims, Some(registry));
        }
        MetricKind::Timer => write_timer(out, family, &base, dialect, claims, registry),
        MetricKind::Rate => write_rate(out, family, &base, dialect, claims, registry),
        MetricKind::Histogram => write_histogram(out, family, &base, dialect, claims, registry),
        MetricKind::Summary => write_summary(out, family, &base, dialect, claims, registry),
    }
}

fn counter_value(s: &SeriesSnapshot) -> Option<u64> {
    match &s.handle {
        #[cfg(feature = "count")]
        Handle::Counter(c) => Some(c.get()),
        #[cfg(feature = "count")]
        Handle::ShardedCounter(c) => Some(c.get()),
        #[allow(unreachable_patterns)]
        _ => None,
    }
}

#[allow(unused_variables, unused_mut, irrefutable_let_patterns)]
fn write_timer(
    out: &mut String,
    family: &FamilySnapshot,
    base: &str,
    dialect: Dialect,
    claims: &mut Claims,
    registry: &Registry,
) {
    let mut summary = String::new();
    let mut min = String::new();
    let mut max = String::new();
    let (sum_name, count_name) = (format!("{base}_sum"), format!("{base}_count"));
    let (min_name, max_name) = (format!("{base}_min"), format!("{base}_max"));
    #[cfg(feature = "timer")]
    for s in &family.series {
        if let Handle::Timer(t) = &s.handle {
            #[allow(clippy::cast_precision_loss)] // seconds as f64 is the exposition contract
            let sum = t.total_nanos() as f64 / 1e9;
            let count = t.count();
            sample_f64(&mut summary, &sum_name, &s.labels, None, sum);
            sample_int(&mut summary, &count_name, &s.labels, None, count);
            if count > 0 {
                sample_f64(&mut min, &min_name, &s.labels, None, t.min().as_secs_f64());
                sample_f64(&mut max, &max_name, &s.labels, None, t.max().as_secs_f64());
            }
        }
    }
    let seconds_unit = om_unit(base, Unit::Seconds);
    emit(
        out,
        &Family {
            name: base.to_owned(),
            ty: Type::Summary,
            help: family.help.as_deref(),
            unit: seconds_unit,
            body: summary,
            suffixes: SUMMARY_SUFFIXES,
        },
        dialect,
        claims,
        Some(registry),
    );
    for (suffix, body) in [("_min", min), ("_max", max)] {
        let name = format!("{base}{suffix}");
        emit(
            out,
            &Family {
                name,
                ty: Type::Gauge,
                help: None,
                unit: None,
                body,
                suffixes: NO_SUFFIXES,
            },
            dialect,
            claims,
            Some(registry),
        );
    }
}

#[allow(unused_variables, unused_mut, irrefutable_let_patterns)]
fn write_rate(
    out: &mut String,
    family: &FamilySnapshot,
    base: &str,
    dialect: Dialect,
    claims: &mut Claims,
    registry: &Registry,
) {
    let counter_name = match dialect {
        Dialect::OpenMetrics => counter_family_name(base, dialect),
        Dialect::Prometheus => format!("{}_total", base.strip_suffix("_total").unwrap_or(base)),
    };
    let total_sample = match dialect {
        Dialect::OpenMetrics => format!("{counter_name}_total"),
        Dialect::Prometheus => counter_name.clone(),
    };
    let rate_name = format!("{base}_per_second");
    let mut totals = String::new();
    let mut rates = String::new();
    #[cfg(feature = "meter")]
    for s in &family.series {
        if let Handle::Rate(r) = &s.handle {
            sample_int(&mut totals, &total_sample, &s.labels, None, r.total());
            sample_f64(&mut rates, &rate_name, &s.labels, None, r.rate());
        }
    }
    emit(
        out,
        &Family {
            name: counter_name,
            ty: Type::Counter,
            help: family.help.as_deref(),
            unit: None,
            body: totals,
            suffixes: if dialect == Dialect::OpenMetrics {
                COUNTER_SUFFIXES_OM
            } else {
                NO_SUFFIXES
            },
        },
        dialect,
        claims,
        Some(registry),
    );
    emit(
        out,
        &Family {
            name: rate_name,
            ty: Type::Gauge,
            help: None,
            unit: None,
            body: rates,
            suffixes: NO_SUFFIXES,
        },
        dialect,
        claims,
        Some(registry),
    );
}

#[allow(unused_variables, unused_mut, irrefutable_let_patterns)]
fn write_histogram(
    out: &mut String,
    family: &FamilySnapshot,
    base: &str,
    dialect: Dialect,
    claims: &mut Claims,
    registry: &Registry,
) {
    let mut body = String::new();
    let bucket_name = format!("{base}_bucket");
    let (sum_name, count_name) = (format!("{base}_sum"), format!("{base}_count"));
    // Series of one family almost always share their bounds: format the
    // `le` values once and reuse them while the bounds stay the same.
    let mut le_bounds: Vec<f64> = Vec::new();
    let mut le_values: Vec<String> = Vec::new();
    #[cfg(feature = "histogram")]
    for s in &family.series {
        if let Handle::Histogram(h) = &s.handle {
            // Only OpenMetrics carries exemplars; Prometheus text skips them.
            let snap = match dialect {
                Dialect::OpenMetrics => h.snapshot(),
                Dialect::Prometheus => h.snapshot_counts(),
            };
            let same = le_bounds.len() == snap.buckets.len()
                && le_bounds
                    .iter()
                    .zip(&snap.buckets)
                    .all(|(a, b)| a.to_bits() == b.upper_bound.to_bits());
            if !same {
                le_bounds = snap.buckets.iter().map(|b| b.upper_bound).collect();
                le_values = le_bounds.iter().map(|b| canonical_float(*b)).collect();
            }
            for (i, (b, le)) in snap.buckets.iter().zip(&le_values).enumerate() {
                sample(&mut body, &bucket_name, &s.labels, Some(("le", le)));
                let _ = write!(body, "{}", b.count);
                if let Some(ex) = snap.exemplar(i) {
                    body.push_str(" # ");
                    let mut labels = String::new();
                    ex.labels.write_prometheus(&mut labels, None);
                    body.push_str(if labels.is_empty() { "{}" } else { &labels });
                    body.push(' ');
                    fmt_value(&mut body, ex.value);
                    body.push(' ');
                    fmt_value(&mut body, ex.timestamp);
                }
                body.push('\n');
            }
            // OpenMetrics forbids a sum once negative values are possible,
            // and `_count` may only appear together with `_sum`.
            let negative = h.bounds().first().is_some_and(|b| *b < 0.0) || snap.sum < 0.0;
            if dialect == Dialect::Prometheus || !negative {
                sample_f64(&mut body, &sum_name, &s.labels, None, snap.sum);
                sample_int(&mut body, &count_name, &s.labels, None, snap.count);
            }
        }
    }
    emit(
        out,
        &Family {
            unit: om_unit(base, family.unit),
            name: base.to_owned(),
            ty: Type::Histogram,
            help: family.help.as_deref(),
            body,
            suffixes: HISTOGRAM_SUFFIXES,
        },
        dialect,
        claims,
        Some(registry),
    );
}

#[allow(unused_variables, unused_mut, irrefutable_let_patterns)]
fn write_summary(
    out: &mut String,
    family: &FamilySnapshot,
    base: &str,
    dialect: Dialect,
    claims: &mut Claims,
    registry: &Registry,
) {
    let mut body = String::new();
    let (sum_name, count_name) = (format!("{base}_sum"), format!("{base}_count"));
    #[cfg(feature = "histogram")]
    for s in &family.series {
        if let Handle::Summary(q) = &s.handle {
            let snap = q.snapshot();
            for (quantile, value) in &snap.quantiles {
                let label = canonical_float(*quantile);
                sample_f64(
                    &mut body,
                    base,
                    &s.labels,
                    Some(("quantile", &label)),
                    *value,
                );
            }
            sample_f64(&mut body, &sum_name, &s.labels, None, snap.sum);
            sample_int(&mut body, &count_name, &s.labels, None, snap.count);
        }
    }
    emit(
        out,
        &Family {
            unit: om_unit(base, family.unit),
            name: base.to_owned(),
            ty: Type::Summary,
            help: family.help.as_deref(),
            body,
            suffixes: SUMMARY_SUFFIXES,
        },
        dialect,
        claims,
        Some(registry),
    );
}

/// Write the standard `process_*` families.
#[cfg(feature = "system")]
pub(crate) fn write_process(
    out: &mut String,
    m: &crate::ProcessMetrics,
    dialect: Dialect,
    claims: &mut Claims,
) {
    let empty = LabelSet::EMPTY;
    let counter_name = match dialect {
        Dialect::OpenMetrics => "process_cpu_seconds",
        Dialect::Prometheus => "process_cpu_seconds_total",
    };
    let mut cpu = String::new();
    sample_f64(
        &mut cpu,
        "process_cpu_seconds_total",
        &empty,
        None,
        m.cpu_seconds_total,
    );
    emit(
        out,
        &Family {
            name: counter_name.to_owned(),
            ty: Type::Counter,
            help: Some("Total user and system CPU time spent in seconds."),
            unit: (dialect == Dialect::OpenMetrics).then_some("seconds"),
            body: cpu,
            suffixes: if dialect == Dialect::OpenMetrics {
                COUNTER_SUFFIXES_OM
            } else {
                NO_SUFFIXES
            },
        },
        dialect,
        claims,
        None,
    );
    #[allow(clippy::cast_precision_loss)] // exposition values are f64
    let gauges: [(&str, &str, Option<&'static str>, Option<f64>); 6] = [
        (
            "process_resident_memory_bytes",
            "Resident memory size in bytes.",
            Some("bytes"),
            Some(m.resident_memory_bytes as f64),
        ),
        (
            "process_virtual_memory_bytes",
            "Virtual memory size in bytes.",
            Some("bytes"),
            m.virtual_memory_bytes.map(|v| v as f64),
        ),
        (
            "process_open_fds",
            "Number of open file descriptors.",
            None,
            m.open_fds.map(|v| v as f64),
        ),
        (
            "process_max_fds",
            "Maximum number of open file descriptors.",
            None,
            m.max_fds.map(|v| v as f64),
        ),
        (
            "process_threads",
            "Number of OS threads in the process.",
            None,
            m.threads.map(|v| v as f64),
        ),
        (
            "process_start_time_seconds",
            "Start time of the process since unix epoch in seconds.",
            Some("seconds"),
            m.start_time_seconds,
        ),
    ];
    for (name, help, unit, value) in gauges {
        let Some(value) = value else { continue };
        let mut body = String::new();
        sample_f64(&mut body, name, &empty, None, value);
        emit(
            out,
            &Family {
                name: name.to_owned(),
                ty: Type::Gauge,
                help: Some(help),
                unit: unit.filter(|_| dialect == Dialect::OpenMetrics),
                body,
                suffixes: NO_SUFFIXES,
            },
            dialect,
            claims,
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_float_matches_openmetrics_examples() {
        let cases: &[(f64, &str)] = &[
            (0.0, "0.0"),
            (0.001, "0.001"),
            (0.002, "0.002"),
            (0.01, "0.01"),
            (0.1, "0.1"),
            (0.9, "0.9"),
            (0.95, "0.95"),
            (0.99, "0.99"),
            (0.999, "0.999"),
            (1.0, "1.0"),
            (1.7, "1.7"),
            (10.0, "10.0"),
            (1e-10, "1e-10"),
            (1e-9, "1e-09"),
            (1e-5, "1e-05"),
            (1e-4, "0.0001"),
            (100_000.0, "100000.0"),
            (1e6, "1e+06"),
            (1e10, "1e+10"),
            (2.5, "2.5"),
            (0.005, "0.005"),
            (0.025, "0.025"),
            (123_456.0, "123456.0"),
            (1_234_567.0, "1.234567e+06"),
            (-0.5, "-0.5"),
            (-2e7, "-2e+07"),
            (f64::INFINITY, "+Inf"),
            (f64::NEG_INFINITY, "-Inf"),
            (f64::NAN, "NaN"),
            (-0.0, "-0.0"),
            (1.5e300, "1.5e+300"),
        ];
        for (v, want) in cases {
            assert_eq!(canonical_float(*v), *want, "{v}");
        }
    }

    #[test]
    fn test_fmt_value_special_cases() {
        let mut s = String::new();
        for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1.0, 1e-7] {
            fmt_value(&mut s, v);
            s.push(' ');
        }
        assert_eq!(s, "NaN +Inf -Inf 1.0 1e-7 ");
    }

    #[test]
    fn test_escape_help_per_dialect() {
        let mut p = String::new();
        escape_help(&mut p, "a\\b\n\"c\"", Dialect::Prometheus);
        assert_eq!(p, "a\\\\b\\n\"c\"");
        let mut o = String::new();
        escape_help(&mut o, "a\\b\n\"c\"", Dialect::OpenMetrics);
        assert_eq!(o, "a\\\\b\\n\\\"c\\\"");
    }

    #[test]
    fn test_om_unit_requires_suffix() {
        assert_eq!(om_unit("latency_seconds", Unit::Seconds), Some("seconds"));
        assert_eq!(om_unit("latency", Unit::Seconds), None);
        assert_eq!(om_unit("seconds", Unit::Seconds), None);
        assert_eq!(om_unit("xseconds", Unit::Seconds), None);
        assert_eq!(om_unit("size_bytes", Unit::None), None);
        assert_eq!(om_unit("q_req_s", Unit::Custom("req-s")), None);
    }

    #[test]
    fn test_counter_family_name_strips_total_for_openmetrics() {
        assert_eq!(
            counter_family_name("hits_total", Dialect::OpenMetrics),
            "hits"
        );
        assert_eq!(
            counter_family_name("hits_total", Dialect::Prometheus),
            "hits_total"
        );
        assert_eq!(
            counter_family_name("_total", Dialect::OpenMetrics),
            "_total"
        );
        assert_eq!(counter_family_name("hits", Dialect::OpenMetrics), "hits");
    }

    #[test]
    fn test_claims_reject_any_overlap() {
        let mut c = Claims::default();
        assert!(c.try_claim(&["a".into(), "a_sum".into()]));
        assert!(!c.try_claim(&["b".into(), "a_sum".into()]));
        assert!(c.try_claim(&["b".into()]));
    }
}
