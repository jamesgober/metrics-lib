//! Minimal custom OpenMetrics-style exporter built on top of the public registry API.
//!
//! Demonstrates how to take a one-shot snapshot of every registered metric and
//! render it to a Prometheus-compatible text body without allocating leaked
//! `&'static str`s. As of 0.9.2, `MetricsCore::counter/gauge/timer/rate` accept
//! `&str`, so runtime-derived names (from `registry().*_names()`) are passed
//! through directly.
//!
//! For a production exporter that handles labels, units, and HELP text out of
//! the box, the in-crate exporters introduced in 0.9.3 are recommended.

use metrics_lib::metrics;
use std::fmt::Write;

fn main() {
    metrics_lib::init();

    // Prime a few metrics so the snapshot has content.
    let m = metrics();
    m.counter("jobs_total").add(12);
    m.gauge("queue_depth").set(3.0);
    m.timer("latency_ns").record_ns(42_000);
    m.rate("ingest_rps").tick_n(5);

    println!("{}", snapshot_openmetrics());
}

/// Render the global registry as a Prometheus-compatible text body.
fn snapshot_openmetrics() -> String {
    let m = metrics();
    let reg = m.registry();
    let mut out = String::new();

    // Counters
    for name in reg.counter_names() {
        let v = m.counter(&name).get();
        let _ = writeln!(out, "# HELP {name} Counter total");
        let _ = writeln!(out, "# TYPE {name} counter");
        let _ = writeln!(out, "{name} {v}");
        let _ = writeln!(out);
    }

    // Gauges
    for name in reg.gauge_names() {
        let v = m.gauge(&name).get();
        let _ = writeln!(out, "# HELP {name} Gauge value");
        let _ = writeln!(out, "# TYPE {name} gauge");
        let _ = writeln!(out, "{name} {v}");
        let _ = writeln!(out);
    }

    // Timers: expose count and average as gauges for demo purposes.
    for name in reg.timer_names() {
        let t = m.timer(&name);
        let _ = writeln!(out, "# HELP {name}_count Timer count");
        let _ = writeln!(out, "# TYPE {name}_count gauge");
        let _ = writeln!(out, "{name}_count {}", t.count());
        let _ = writeln!(out, "# HELP {name}_avg_ns Timer average in ns");
        let _ = writeln!(out, "# TYPE {name}_avg_ns gauge");
        let _ = writeln!(out, "{name}_avg_ns {}", t.average().as_nanos());
        let _ = writeln!(out);
    }

    // Rate meters: per-second sample.
    for name in reg.rate_meter_names() {
        let r = m.rate(&name);
        let _ = writeln!(out, "# HELP {name}_per_second Rate per second");
        let _ = writeln!(out, "# TYPE {name}_per_second gauge");
        let _ = writeln!(out, "{name}_per_second {}", r.rate());
        let _ = writeln!(out);
    }

    out
}
