//! Prometheus text exposition format (version 0.0.4).
//!
//! Serve the output from a `GET /metrics` handler with
//! [`CONTENT_TYPE`] as the `Content-Type`.
//!
//! # Mapping
//!
//! | Registry kind | Exposition |
//! |---|---|
//! | `Counter`, `ShardedCounter` | `counter` family named as registered |
//! | `UpDownCounter`, `Gauge` | `gauge` |
//! | `Timer` | `summary` (`_sum` in seconds, `_count`) plus `<name>_min` / `<name>_max` gauges in seconds |
//! | `RateMeter` | `<name>_total` counter of events plus `<name>_per_second` gauge |
//! | `Histogram` | `histogram` with cumulative `_bucket{le=…}`, `_sum`, `_count` |
//! | `Summary` | `summary` with `{quantile=…}` samples, `_sum`, `_count` |
//!
//! Families are sorted by name and series by labels, so the output is
//! deterministic. `.` in names is written as `_`. When two families would
//! produce the same name the later one is skipped and counted in
//! [`Registry::exposition_conflicts`], so the scrape always parses.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::{LabelSet, MetricsCore};
//! use metrics_lib::exporters::prometheus;
//!
//! let core = MetricsCore::new();
//! core.counter("requests").inc();
//! core.counter_with("requests", &LabelSet::from([("status", "200")])).add(3);
//!
//! let body = prometheus::render(core.registry());
//! assert!(body.contains("# TYPE requests counter"));
//! assert!(body.contains("requests 1"));
//! assert!(body.contains(r#"requests{status="200"} 3"#));
//! # }
//! ```

use super::text::{Claims, Dialect, write_registry};
use crate::Registry;

/// `Content-Type` of the Prometheus text format.
pub const CONTENT_TYPE: &str = "text/plain; version=0.0.4; charset=utf-8";

/// Render every family of `registry`.
#[must_use]
pub fn render(registry: &Registry) -> String {
    let mut out = String::with_capacity(4096);
    render_into(&mut out, registry);
    out
}

/// Append the rendering of `registry` to `out` (reuse one buffer across
/// scrapes).
pub fn render_into(out: &mut String, registry: &Registry) {
    write_registry(out, registry, Dialect::Prometheus, &mut Claims::default());
}

/// Render the standard `process_*` families followed by every family of
/// `registry` (feature `system`).
///
/// Registry families whose names clash with a `process_*` family are
/// skipped and counted in [`Registry::exposition_conflicts`].
#[cfg(feature = "system")]
#[must_use]
pub fn render_with_process(registry: &Registry, process: &crate::ProcessMetrics) -> String {
    let mut out = String::with_capacity(4096);
    let mut claims = Claims::default();
    super::text::write_process(&mut out, process, Dialect::Prometheus, &mut claims);
    write_registry(&mut out, registry, Dialect::Prometheus, &mut claims);
    out
}

/// Render a [`MetricsCore`](crate::MetricsCore): its registry plus, when the
/// core was configured with
/// [`MetricsConfig::process_metrics`](crate::MetricsConfig::process_metrics)
/// (the default with feature `system`), the `process_*` families sampled
/// right now.
#[must_use]
pub fn render_core(core: &crate::MetricsCore) -> String {
    #[cfg(feature = "system")]
    if let Some(process) = core.sample_process_metrics() {
        return render_with_process(core.registry(), &process);
    }
    render(core.registry())
}
