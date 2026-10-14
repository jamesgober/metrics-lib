//! Exposition validity: every Prometheus/OpenMetrics output is checked with
//! the strict validators in `tests/common`, including the cases that made
//! 1.x produce invalid scrapes (ML-005) and hostile input (ML-009).

#![cfg(all(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
#![allow(clippy::unwrap_used, clippy::float_cmp)]

mod common;

use common::{Format, validate};
use metrics_lib::exporters::{openmetrics, prometheus};
use metrics_lib::{LabelSet, MetricsCore, Registry, SummaryConfig, Unit};
use proptest::prelude::*;
use std::time::Duration;

fn both(registry: &Registry) -> (common::Exposition, common::Exposition) {
    let prom = prometheus::render(registry);
    let om = openmetrics::render(registry);
    let p = validate(&prom, Format::Prometheus).unwrap_or_else(|e| panic!("{e}\n{prom}"));
    let o = validate(&om, Format::OpenMetrics).unwrap_or_else(|e| panic!("{e}\n{om}"));
    (p, o)
}

#[test]
fn test_empty_registry_is_valid() {
    let r = Registry::new();
    assert_eq!(prometheus::render(&r), "");
    assert_eq!(openmetrics::render(&r), "# EOF\n");
    let _ = both(&r);
}

#[test]
fn test_every_kind_renders_valid_expositions() {
    let r = Registry::new();
    r.describe_counter("requests", "Requests served", Unit::None)
        .unwrap();
    r.counter("requests").add(3);
    r.counter_with("requests", &LabelSet::from([("status", "500")]))
        .inc();
    r.sharded_counter("jobs_total").add(5);
    r.up_down_counter("inflight").add(-2);
    r.describe_gauge("temp_celsius", "Temperature", Unit::Custom("celsius"))
        .unwrap();
    r.gauge("temp_celsius").set(21.5);
    r.gauge("nan_gauge").set(f64::NAN);
    r.describe_timer("db_seconds", "DB time", Unit::Seconds)
        .unwrap();
    r.timer("db_seconds").record(Duration::from_millis(5));
    let _ = r.timer("idle_timer");
    r.rate_meter("qps").tick_n(7);
    r.describe_histogram("latency_seconds", "Latency", Unit::Seconds)
        .unwrap();
    let h = r.histogram_with("latency_seconds", &LabelSet::from([("route", "/a")]));
    h.observe(0.02);
    h.observe(3.0);
    r.configure_summary(
        "size_bytes",
        SummaryConfig::default().with_quantiles([0.5, 0.99]),
    )
    .unwrap();
    r.summary("size_bytes").observe(512.0);
    let _ = r.summary("empty_summary");

    let (p, o) = both(&r);

    // Prometheus: counters keep their registered names.
    assert_eq!(p.family("requests").unwrap().ty, "counter");
    assert_eq!(p.value("requests", &[]), Some(3.0));
    assert_eq!(p.value("jobs_total", &[]), Some(5.0));
    assert_eq!(p.family("inflight").unwrap().ty, "gauge");
    assert_eq!(p.value("inflight", &[]), Some(-2.0));
    assert!(p.value("nan_gauge", &[]).unwrap().is_nan());
    // Timers: summary + min/max gauges; an idle timer has no min/max.
    assert_eq!(p.family("db_seconds").unwrap().ty, "summary");
    assert_eq!(p.value("db_seconds_count", &[]), Some(1.0));
    assert!((p.value("db_seconds_sum", &[]).unwrap() - 0.005).abs() < 1e-12);
    assert_eq!(p.value("db_seconds_min", &[]), Some(0.005));
    assert!(p.family("idle_timer_min").is_none());
    // Rate meters: counter of events plus a rate gauge.
    assert_eq!(p.family("qps_total").unwrap().ty, "counter");
    assert_eq!(p.value("qps_total", &[]), Some(7.0));
    assert_eq!(p.family("qps_per_second").unwrap().ty, "gauge");
    // Histograms and summaries.
    assert_eq!(
        p.value("latency_seconds_count", &[("route", "/a")]),
        Some(2.0)
    );
    assert_eq!(
        p.value(
            "latency_seconds_bucket",
            &[("route", "/a"), ("le", "0.025")]
        ),
        Some(1.0)
    );
    assert!(p.value("size_bytes", &[("quantile", "0.99")]).is_some());
    assert!(
        p.value("empty_summary", &[("quantile", "0.5")])
            .unwrap()
            .is_nan()
    );

    // OpenMetrics: counter families drop `_total`, samples carry it.
    assert_eq!(o.family("requests").unwrap().ty, "counter");
    assert_eq!(o.value("requests_total", &[("status", "500")]), Some(1.0));
    assert_eq!(o.family("jobs").unwrap().ty, "counter");
    assert_eq!(o.value("jobs_total", &[]), Some(5.0));
    assert_eq!(o.family("qps").unwrap().ty, "counter");
    // UNIT only where the name ends with the unit.
    assert_eq!(
        o.family("latency_seconds").unwrap().unit.as_deref(),
        Some("seconds")
    );
    assert_eq!(
        o.family("db_seconds").unwrap().unit.as_deref(),
        Some("seconds")
    );
    assert_eq!(
        o.family("temp_celsius").unwrap().unit.as_deref(),
        Some("celsius")
    );
    assert!(o.family("requests").unwrap().unit.is_none());
    assert_eq!(
        o.family("requests").unwrap().help.as_deref(),
        Some("Requests served")
    );
}

#[test]
fn test_cross_kind_name_reuse_cannot_break_scrapes_regression_ml005() {
    // 1.x emitted `# TYPE dup counter` and `# TYPE dup gauge`.
    let r = Registry::new();
    r.counter("dup").inc();
    r.gauge("dup").set(2.0); // refused: detached
    r.timer("dup").record_ns(1);
    let (p, _) = both(&r);
    assert_eq!(p.families.len(), 1);
    assert_eq!(r.kind_conflicts(), 2);
}

#[test]
fn test_dotted_names_that_collide_after_mapping_are_skipped() {
    let r = Registry::new();
    r.counter("a.b").inc();
    r.gauge("a_b").set(1.0);
    // `timer` named `x` claims `x_sum`/`x_count`/`x_min`/`x_max`.
    r.timer("x").record_ns(10);
    r.gauge("x_min").set(9.0);
    r.counter("x.count").inc();
    let before = r.exposition_conflicts();
    let (p, _) = both(&r);
    assert!(r.exposition_conflicts() > before);
    assert_eq!(p.value("a_b", &[]), Some(1.0), "first family by name wins");
    assert!(p.family("x_count").is_none() || p.family("x").is_some());
}

#[test]
fn test_counter_named_total_is_not_doubled_regression_ml005() {
    let r = Registry::new();
    r.counter("jobs_total").inc();
    let om = openmetrics::render(&r);
    assert!(om.contains("# TYPE jobs counter\njobs_total 1\n"), "{om}");
    assert!(!om.contains("_total_total"));
    let _ = both(&r);
}

#[test]
fn test_negative_histogram_omits_sum_in_openmetrics() {
    let r = Registry::new();
    r.configure_histogram("delta", [-10.0, 0.0, 10.0]).unwrap();
    r.histogram("delta").observe(-3.0);
    let (p, o) = both(&r);
    assert!(p.value("delta_sum", &[]).is_some());
    assert!(o.value("delta_sum", &[]).is_none());
    assert!(o.value("delta_count", &[]).is_none());
    assert_eq!(o.value("delta_bucket", &[("le", "+Inf")]), Some(1.0));
}

#[test]
fn test_exemplars_render_in_openmetrics_only() {
    let r = Registry::new();
    let h = r.histogram("rpc_seconds");
    h.observe_with_exemplar(0.07, &LabelSet::from([("trace_id", "4bf92f3577b34da6")]));
    let (p, o) = both(&r);
    assert!(
        p.families
            .iter()
            .all(|f| f.samples.iter().all(|s| s.exemplar.is_none()))
    );
    let bucket = o
        .family("rpc_seconds")
        .unwrap()
        .samples
        .iter()
        .find(|s| s.exemplar.is_some())
        .unwrap();
    assert!(
        bucket
            .exemplar
            .as_deref()
            .unwrap()
            .contains("trace_id=\"4bf92f3577b34da6\"")
    );
}

#[test]
fn test_hostile_names_and_labels_cannot_inject_lines_regression_ml009() {
    let r = Registry::new();
    let hostile_labels = LabelSet::from([
        ("bad\"key}\n", "v"),
        ("ok", "value with \"quotes\", \\backslash\\ and\nnewline"),
    ]);
    r.counter_with("inj", &hostile_labels).inc();
    r.counter("name\n# TYPE evil gauge\nevil").inc();
    r.gauge("x{y=\"1\"}").set(1.0);
    r.describe_counter("inj", "help with \\ and\nnewline \"q\"", Unit::None)
        .unwrap();
    let (p, o) = both(&r);
    assert!(p.family("evil").is_none());
    assert!(o.family("evil").is_none());
    assert!(r.invalid_inputs() >= 3);
    let value = p
        .families
        .iter()
        .flat_map(|f| &f.samples)
        .flat_map(|s| &s.labels)
        .find(|(k, _)| k == "ok")
        .map(|(_, v)| v.clone())
        .unwrap();
    assert_eq!(value, "value with \"quotes\", \\backslash\\ and\nnewline");
}

#[cfg(feature = "system")]
#[test]
fn test_process_metrics_families() {
    let core = MetricsCore::new();
    core.counter("app_requests").inc();
    core.gauge("process_threads").set(1.0); // clashes with the standard family
    let prom = prometheus::render_core(&core);
    let om = openmetrics::render_core(&core);
    let p = validate(&prom, Format::Prometheus).unwrap_or_else(|e| panic!("{e}\n{prom}"));
    let o = validate(&om, Format::OpenMetrics).unwrap_or_else(|e| panic!("{e}\n{om}"));
    assert_eq!(p.family("process_cpu_seconds_total").unwrap().ty, "counter");
    assert!(p.value("process_resident_memory_bytes", &[]).unwrap() > 0.0);
    assert_eq!(
        o.family("process_cpu_seconds").unwrap().unit.as_deref(),
        Some("seconds")
    );
    assert!(o.value("process_cpu_seconds_total", &[]).is_some());
    assert!(p.value("app_requests", &[]).is_some());
    #[cfg(target_os = "linux")]
    {
        assert!(p.value("process_open_fds", &[]).unwrap() >= 3.0);
        assert!(p.value("process_start_time_seconds", &[]).unwrap() > 1.6e9);
    }
    // The user gauge named `process_threads` is skipped only when the
    // platform reports a thread count.
    let without =
        MetricsCore::with_config(metrics_lib::MetricsConfig::default().with_process_metrics(false))
            .unwrap();
    without.counter("c").inc();
    assert!(!prometheus::render_core(&without).contains("process_"));
}

#[test]
fn test_render_into_reuses_buffer() {
    let r = Registry::new();
    r.counter("c").inc();
    let mut buf = String::from("stale");
    buf.clear();
    prometheus::render_into(&mut buf, &r);
    assert!(buf.starts_with("# TYPE c counter"));
    buf.clear();
    openmetrics::render_into(&mut buf, &r);
    assert!(buf.ends_with("# EOF\n"));
    assert_eq!(
        prometheus::CONTENT_TYPE,
        "text/plain; version=0.0.4; charset=utf-8"
    );
    assert!(openmetrics::CONTENT_TYPE.starts_with("application/openmetrics-text"));
}

#[derive(Debug, Clone)]
enum Op {
    Counter(String, Vec<(String, String)>, u64),
    Gauge(String, f64),
    Timer(String, u64),
    Rate(String, u64),
    Histogram(String, f64),
    Summary(String, f64),
    UpDown(String, i64),
    Describe(String, String),
}

fn name_strategy() -> impl Strategy<Value = String> {
    // Valid, invalid and colliding names alike.
    prop_oneof![
        "[a-c]{1,2}",
        "[a-c]{1,2}(_total|_count|_sum|_bucket|_min|_max|_created|_per_second)",
        "[a-c]\\.[a-c]",
        ".{0,6}",
    ]
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        (
            name_strategy(),
            proptest::collection::vec(("[a-z_]{1,3}|.{0,3}", ".{0,6}"), 0..3),
            any::<u64>()
        )
            .prop_map(|(n, l, v)| Op::Counter(n, l, v)),
        (name_strategy(), any::<f64>()).prop_map(|(n, v)| Op::Gauge(n, v)),
        (name_strategy(), any::<u64>()).prop_map(|(n, v)| Op::Timer(n, v)),
        (name_strategy(), 0u64..1000).prop_map(|(n, v)| Op::Rate(n, v)),
        (name_strategy(), any::<f64>()).prop_map(|(n, v)| Op::Histogram(n, v)),
        (name_strategy(), any::<f64>()).prop_map(|(n, v)| Op::Summary(n, v)),
        (name_strategy(), any::<i64>()).prop_map(|(n, v)| Op::UpDown(n, v)),
        (name_strategy(), ".{0,20}").prop_map(|(n, h)| Op::Describe(n, h)),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Whatever sequence of registrations happens — invalid names, kind
    /// conflicts, suffix collisions, non-finite values — both expositions
    /// stay valid.
    #[test]
    fn prop_any_registry_renders_valid_expositions(ops in proptest::collection::vec(op_strategy(), 0..40)) {
        let r = Registry::new();
        for op in ops {
            match op {
                Op::Counter(n, labels, v) => {
                    let set: LabelSet = labels.into_iter().collect();
                    r.counter_with(&n, &set).add(v);
                }
                Op::Gauge(n, v) => r.gauge(&n).set(v),
                Op::Timer(n, v) => r.timer(&n).record_ns(v),
                Op::Rate(n, v) => r.rate_meter(&n).tick_n(v),
                Op::Histogram(n, v) => r.histogram(&n).observe(v),
                Op::Summary(n, v) => r.summary(&n).observe(v),
                Op::UpDown(n, v) => r.up_down_counter(&n).add(v),
                Op::Describe(n, h) => { let _ = r.describe_gauge(&n, h, Unit::None); }
            }
        }
        let prom = prometheus::render(&r);
        let om = openmetrics::render(&r);
        prop_assert!(validate(&prom, Format::Prometheus).is_ok(), "{:?}\n{}", validate(&prom, Format::Prometheus).err(), prom);
        prop_assert!(validate(&om, Format::OpenMetrics).is_ok(), "{:?}\n{}", validate(&om, Format::OpenMetrics).err(), om);
    }
}
