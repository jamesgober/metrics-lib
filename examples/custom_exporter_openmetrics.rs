use metrics_lib::metrics;
use std::fmt::Write;

fn main() {
    metrics_lib::init();

    // Prime a few metrics so the snapshot has content
    let m = metrics();
    m.counter("jobs_total").add(12);
    m.gauge("queue_depth").set(3.0);
    m.timer("latency_ns").record_ns(42_000);
    m.rate("ingest_rps").tick_n(5);

    let body = snapshot_openmetrics();
    println!("{}", body);
}

fn snapshot_openmetrics() -> String {
    let m = metrics();
    let reg = m.registry();
    let mut out = String::new();

    // Counters
    for name in reg.counter_names() {
        let printed = name.clone();
        let v = m.counter(Box::leak(name.into_boxed_str())).get();
        let _ = writeln!(out, "# HELP {} Counter total", printed);
        let _ = writeln!(out, "# TYPE {} counter", printed);
        let _ = writeln!(out, "{} {}", printed, v);
        let _ = writeln!(out);
    }

    // Gauges
    for name in reg.gauge_names() {
        let printed = name.clone();
        let v = m.gauge(Box::leak(name.into_boxed_str())).get();
        let _ = writeln!(out, "# HELP {} Gauge value", printed);
        let _ = writeln!(out, "# TYPE {} gauge", printed);
        let _ = writeln!(out, "{} {}", printed, v);
        let _ = writeln!(out);
    }

    // Timers: expose count and average as gauges for demo
    for name in reg.timer_names() {
        let printed = name.clone();
        let t = m.timer(Box::leak(name.into_boxed_str()));
        let _ = writeln!(out, "# HELP {}_count Timer count", printed);
        let _ = writeln!(out, "# TYPE {}_count gauge", printed);
        let _ = writeln!(out, "{}_count {}", printed, t.count());
        let _ = writeln!(out, "# HELP {}_avg_ns Timer average in ns", printed);
        let _ = writeln!(out, "# TYPE {}_avg_ns gauge", printed);
        let _ = writeln!(out, "{}_avg_ns {}", printed, t.average().as_nanos());
        let _ = writeln!(out);
    }

    // Rate meters: per-second sample
    for name in reg.rate_meter_names() {
        let printed = name.clone();
        let r = m.rate(Box::leak(name.into_boxed_str()));
        let _ = writeln!(out, "# HELP {}_per_second Rate per second", printed);
        let _ = writeln!(out, "# TYPE {}_per_second gauge", printed);
        let _ = writeln!(out, "{}_per_second {}", printed, r.rate());
        let _ = writeln!(out);
    }

    out
}
