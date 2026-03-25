//! Executable smoke tests mirrored from docs/API.md.
//!
//! Why not `doc_comment::doctest!`?
//! In this repository setup, the previous harness produced `running 0 tests`
//! during `cargo test`, so it did not contribute executable coverage.

use metrics_lib::{init, metrics};
use std::time::Duration;

#[test]
fn api_md_contains_rust_fences() {
    let md = include_str!("../docs/API.md");
    let rust_blocks = md.matches("```rust").count();
    assert!(
        rust_blocks > 0,
        "docs/API.md should contain at least one rust code fence"
    );
}

#[test]
#[cfg(all(feature = "count", feature = "gauge", feature = "timer"))]
fn api_quick_start_smoke_executes() {
    let m = init();

    m.counter("api_smoke.requests").inc();
    m.gauge("api_smoke.cpu").set(42.0);
    let t = m.timer("api_smoke.latency");
    t.record(Duration::from_millis(1));

    assert_eq!(metrics().counter("api_smoke.requests").get(), 1);
    assert!(metrics().gauge("api_smoke.cpu").get().is_finite());
    assert_eq!(metrics().timer("api_smoke.latency").count(), 1);
}

#[cfg(feature = "meter")]
#[test]
fn api_rate_meter_smoke_executes() {
    let m = init();
    let rate = m.rate("api_smoke.qps");
    rate.tick_n(5);
    assert!(rate.total() >= 5);
}
