//! The jsonx-lib JSON exporter (feature `json`) against the serde_json one
//! (feature `serde`): for the same snapshot both must write the same bytes,
//! compact and indented. Runs when both features are on (`--all-features`).
//!
//! The one documented difference, U+2028 and U+2029 in strings, has its own
//! test at the end.

#![cfg(all(
    feature = "serde",
    feature = "json",
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
#![allow(clippy::unwrap_used)]

use metrics_lib::exporters::json::{self, RegistrySnapshot};
use metrics_lib::{LabelSet, MetricKind, Registry, SummaryConfig, Unit};
use std::time::Duration;

/// Strings that need every kind of escape JSON has, plus characters that
/// need none (`/`, `<`, DEL, non-ASCII, a character outside the BMP).
const AWKWARD: [&str; 8] = [
    "quote \" and backslash \\",
    "line\nfeed\rreturn\ttab",
    "nul \u{0} unit sep \u{1f} bell \u{7} backspace \u{8} form feed \u{c}",
    "del \u{7f} slash / html <script>&amp;</script>",
    "caf\u{e9} \u{4e2d}\u{6587} \u{1F600} \u{FFFF}",
    "",
    " ",
    "\\u0041 is not an escape here",
];

fn assert_same(snap: &RegistrySnapshot) {
    let serde_compact = serde_json::to_string(snap).unwrap();
    let jsonx_compact = snap.to_json();
    assert_eq!(jsonx_compact, serde_compact);
    let serde_pretty = serde_json::to_string_pretty(snap).unwrap();
    let jsonx_pretty = snap.to_json_pretty();
    assert_eq!(jsonx_pretty, serde_pretty);
}

/// A registry with every metric kind, labels and help text that need
/// escaping, empty and full histograms and summaries, exemplars, and the
/// extreme values each kind can hold.
fn every_kind() -> Registry {
    let r = Registry::new();

    r.describe_counter("c_help", AWKWARD[0], Unit::None)
        .unwrap();
    r.counter("c_help").inc();
    let _zero = r.counter("c_zero");
    r.counter("c_max").add(u64::MAX);
    for (i, value) in AWKWARD.iter().enumerate() {
        r.counter_with(
            "c_labels",
            &LabelSet::from([("awkward", *value), ("i", "x")]),
        )
        .add(i as u64);
    }
    r.sharded_counter("sc").add(41);
    r.up_down_counter("ud_neg").add(-7);
    r.up_down_counter("ud_min").add(i64::MIN);
    r.up_down_counter("ud_max").add(i64::MAX);

    r.describe_gauge("g_units", AWKWARD[1], Unit::Bytes)
        .unwrap();
    r.gauge("g_units").set(1.5);
    r.describe_gauge("g_custom", "custom unit", Unit::Custom("req/\"s\"\n"))
        .unwrap();
    r.gauge("g_custom").set(2.0);
    let gauges = [
        0.0,
        -0.0,
        -1.0,
        1.0,
        0.1 + 0.2,
        21.5,
        -273.15,
        1e15,
        1e16,
        1e-5,
        1e-6,
        123_456_789.012_345_68,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        5e-324,
        f64::EPSILON,
        // Exactly halfway between two shortest texts: serde_json picks the
        // even last digit.
        164_696_460_098_373.0 + 0.625,
        -(140_737_488_355_328.0 + 0.875),
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    for (i, v) in gauges.iter().enumerate() {
        r.gauge_with("g", &LabelSet::from([("i", i.to_string())]))
            .set(*v);
    }

    r.describe_timer("t", AWKWARD[4], Unit::Seconds).unwrap();
    let _empty = r.timer("t_empty");
    let t = r.timer("t");
    t.record_ns(1);
    t.record(Duration::from_millis(1_500));
    t.record_ns(u64::MAX);
    r.timer_with("t_labels", &LabelSet::from([("awkward", AWKWARD[2])]))
        .record(Duration::from_secs(3));

    let _empty = r.rate_meter("r_empty");
    r.rate_meter("r").tick_n(1_000);
    r.rate_meter("r_max").tick_n(u64::MAX);

    r.describe_histogram("h", AWKWARD[3], Unit::Milliseconds)
        .unwrap();
    r.configure_histogram("h", [0.5, 1.0, 1e300]).unwrap();
    let _empty = r.histogram("h_empty");
    let h = r.histogram("h");
    h.observe(0.25);
    h.observe(0.75);
    h.observe(f64::MAX);
    h.observe(-1.0);
    h.observe_with_exemplar(
        0.3,
        &LabelSet::from([("trace_id", "4bf92f35\"\\\n"), ("span", AWKWARD[4])]),
    );
    let overflow = r.histogram("h_overflow");
    overflow.observe(f64::MAX);
    overflow.observe(f64::MAX);

    r.describe_summary("s", "", Unit::Ratio).unwrap();
    let _empty = r.summary("s_empty");
    let s = r.summary("s");
    for v in [0.001, 0.5, 0.5, 2.0, 1e6] {
        s.observe(v);
    }
    r.configure_summary(
        "s_custom",
        SummaryConfig::default().with_quantiles([0.0, 0.25, 0.999, 1.0]),
    )
    .unwrap();
    let _empty = r.summary_with("s_custom", &LabelSet::from([("awkward", AWKWARD[5])]));
    r.summary_with("s_custom", &LabelSet::from([("awkward", AWKWARD[0])]))
        .observe(7.0);
    r
}

#[test]
fn test_every_kind_is_byte_identical() {
    let r = every_kind();
    let snap = json::snapshot(&r);
    assert!(snap.series.len() > 40);
    assert_same(&snap);
    // The registry-level functions write the same snapshot.
    let text = json::to_json(&r);
    assert!(text.contains(r#""type":"histogram""#), "{text}");
    assert!(text.contains(r#""exemplars":[{"labels":{"span""#), "{text}");
    assert!(text.contains(r#"},null,null,null]"#), "{text}");
    assert!(
        text.contains(r#""unit":{"Custom":"req/\"s\"\n"}"#),
        "{text}"
    );
    assert!(json::to_json_pretty(&r).contains("\n    {\n      \"name\": "));
}

#[test]
fn test_empty_registry_is_byte_identical() {
    assert_same(&json::snapshot(&Registry::new()));
    let capped = Registry::with_cardinality_cap(1);
    capped.counter("a").inc();
    capped.counter("b").inc();
    assert_same(&json::snapshot(&capped));
}

/// Fields the registry never produces, set by hand on a snapshot: every
/// kind tag, every unit, a help text on every series, non-finite bucket
/// bounds and quantiles.
#[test]
fn test_edited_snapshot_is_byte_identical() {
    let mut snap = json::snapshot(&every_kind());
    let kinds = [
        MetricKind::Counter,
        MetricKind::UpDownCounter,
        MetricKind::Gauge,
        MetricKind::Timer,
        MetricKind::Rate,
        MetricKind::Histogram,
        MetricKind::Summary,
    ];
    let units = [
        None,
        Some(Unit::None),
        Some(Unit::Seconds),
        Some(Unit::Milliseconds),
        Some(Unit::Microseconds),
        Some(Unit::Nanoseconds),
        Some(Unit::Bytes),
        Some(Unit::Kilobytes),
        Some(Unit::Megabytes),
        Some(Unit::Gigabytes),
        Some(Unit::Percent),
        Some(Unit::Ratio),
        Some(Unit::Custom("")),
        Some(Unit::Custom("\u{0}\"")),
    ];
    for (i, series) in snap.series.iter_mut().enumerate() {
        series.kind = kinds[i % kinds.len()];
        series.unit = units[i % units.len()];
        series.help = Some(AWKWARD[i % AWKWARD.len()].to_owned());
        match &mut series.value {
            json::JsonValue::Histogram(h) => {
                for (j, bucket) in h.buckets.iter_mut().enumerate() {
                    bucket.upper_bound = [f64::NAN, f64::NEG_INFINITY, -0.0, 1e-7][j % 4];
                }
                h.sum = f64::NAN;
            }
            json::JsonValue::Summary { sum, quantiles, .. } => {
                *sum = f64::INFINITY;
                for q in quantiles.iter_mut() {
                    q.quantile = f64::NAN;
                    q.value = f64::NEG_INFINITY;
                }
            }
            json::JsonValue::Timer(t) => {
                t.total = Duration::MAX;
                t.rate_per_second = f64::NAN;
            }
            json::JsonValue::Rate(r) => {
                r.rate = f64::INFINITY;
                r.per_second = -0.0;
            }
            _ => {}
        }
    }
    snap.cardinality.cap = usize::MAX;
    snap.cardinality.overflows = u64::MAX;
    assert_same(&snap);
}

/// Floats spread over every exponent: the number text of the two writers
/// must agree on every one (shortest round-trip digits, the same switch
/// between decimal and exponent form).
#[test]
fn test_random_floats_are_byte_identical() {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = move || {
        // xorshift64*
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    };
    for _round in 0..20 {
        let r = Registry::new();
        for i in 0..1_000 {
            let bits = next();
            // Half arbitrary bit patterns (every exponent, subnormals, NaNs),
            // half 53-bit integers scaled by a power of two, which use every
            // bit of the significand and so meet ties between two shortest
            // texts.
            let v = if i % 2 == 0 {
                f64::from_bits(bits)
            } else {
                let scale = i32::try_from(bits % 120).unwrap() - 100;
                ((bits >> 11) | (1 << 52)) as f64 * 2f64.powi(scale)
            };
            r.gauge_with("g", &LabelSet::from([("i", i.to_string())]))
                .set(v);
        }
        assert_same(&json::snapshot(&r));
    }
}

/// The one difference: jsonx-lib escapes U+2028 and U+2029, serde_json
/// copies them. The texts differ only there and decode to the same value.
#[test]
fn test_line_separators_are_escaped_and_decode_the_same() {
    let r = Registry::new();
    let value = "a\u{2028}b\u{2029}c";
    r.counter_with("c", &LabelSet::from([("sep", value)])).inc();
    r.describe_counter("c", "help\u{2028}", Unit::None).unwrap();
    let snap = json::snapshot(&r);
    let serde_text = serde_json::to_string(&snap).unwrap();
    let jsonx_text = snap.to_json();
    assert!(
        jsonx_text.contains(r#""sep":"a\u2028b\u2029c""#),
        "{jsonx_text}"
    );
    assert!(!jsonx_text.contains('\u{2028}') && !jsonx_text.contains('\u{2029}'));
    assert_eq!(
        jsonx_text
            .replace("\\u2028", "\u{2028}")
            .replace("\\u2029", "\u{2029}"),
        serde_text
    );
    let a: serde_json::Value = serde_json::from_str(&serde_text).unwrap();
    let b: serde_json::Value = serde_json::from_str(&jsonx_text).unwrap();
    assert_eq!(a, b);
    assert_eq!(b["series"][0]["labels"]["sep"], value);
}
