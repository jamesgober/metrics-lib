//! OpenMetrics 1.0 text format.
//!
//! Serve the output with [`CONTENT_TYPE`] when the scraper's `Accept` header
//! asks for `application/openmetrics-text`. The family mapping is the same as
//! in [`prometheus`](super::prometheus), with the OpenMetrics rules applied:
//!
//! - counter families are named without `_total` and their samples carry it
//!   (a counter registered as `jobs_total` is the family `jobs`);
//! - `# UNIT` is written only when the family name ends in `_<unit>`;
//! - histograms whose buckets or sum can be negative omit `_sum`/`_count`;
//! - histogram buckets carry their latest exemplar
//!   (see [`Histogram::observe_with_exemplar`](crate::Histogram::observe_with_exemplar));
//! - `le` and `quantile` values use the canonical number format;
//! - the body ends with `# EOF`.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "count")]
//! # {
//! use metrics_lib::MetricsCore;
//! use metrics_lib::exporters::openmetrics;
//!
//! let core = MetricsCore::new();
//! core.counter("jobs_total").add(2);
//! let body = openmetrics::render(core.registry());
//! assert!(body.contains("# TYPE jobs counter\njobs_total 2\n"));
//! assert!(body.ends_with("# EOF\n"));
//! # }
//! ```

use super::text::{Claims, Dialect, write_registry};
use crate::Registry;

/// `Content-Type` of the OpenMetrics text format.
pub const CONTENT_TYPE: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";

/// Render every family of `registry`, terminated by `# EOF`.
#[must_use]
pub fn render(registry: &Registry) -> String {
    let mut out = String::with_capacity(4096);
    render_into(&mut out, registry);
    out
}

/// Append the rendering of `registry` (including `# EOF`) to `out`.
pub fn render_into(out: &mut String, registry: &Registry) {
    write_registry(out, registry, Dialect::OpenMetrics, &mut Claims::default());
    out.push_str("# EOF\n");
}

/// Render the standard `process_*` families followed by every family of
/// `registry` (feature `system`).
#[cfg(feature = "system")]
#[must_use]
pub fn render_with_process(registry: &Registry, process: &crate::ProcessMetrics) -> String {
    let mut out = String::with_capacity(4096);
    let mut claims = Claims::default();
    super::text::write_process(&mut out, process, Dialect::OpenMetrics, &mut claims);
    write_registry(&mut out, registry, Dialect::OpenMetrics, &mut claims);
    out.push_str("# EOF\n");
    out
}

/// Render a [`MetricsCore`](crate::MetricsCore), including the `process_*`
/// families when enabled. See
/// [`prometheus::render_core`](super::prometheus::render_core).
#[must_use]
pub fn render_core(core: &crate::MetricsCore) -> String {
    #[cfg(feature = "system")]
    if let Some(process) = core.sample_process_metrics() {
        return render_with_process(core.registry(), &process);
    }
    render(core.registry())
}
