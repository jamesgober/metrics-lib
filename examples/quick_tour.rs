//! A tour of every metric type and the main exporters.
//!
//! Run with: `cargo run --example quick_tour --features meter`

use metrics_lib::exporters::openmetrics;
use metrics_lib::{MetricsConfig, MetricsCore, MetricsError, Unit};
use std::time::Duration;

fn main() -> Result<(), MetricsError> {
    let core = MetricsCore::with_config(MetricsConfig::default().with_max_series(1_000))?;
    let r = core.registry();

    r.describe_counter("requests", "Requests served", Unit::None)?;
    core.counter("requests").add(5);
    core.sharded_counter("hot_path_events").add(1_000);
    core.up_down_counter("connections").add(3);
    core.gauge("temperature_celsius").set(21.5);
    core.timer("op_seconds").record(Duration::from_millis(3));
    core.rate("ops").tick_n(7);

    // One name, one kind: a second kind is refused.
    assert!(matches!(
        core.try_gauge("requests"),
        Err(MetricsError::KindMismatch { .. })
    ));

    println!("rate of ops: {:.1}/s", core.rate("ops").rate());
    #[cfg(feature = "system")]
    println!(
        "host memory used: {:.0} MiB of {:.0} MiB",
        core.system().mem_used_mb(),
        core.system().mem_total_mb()
    );
    print!("{}", openmetrics::render(r));
    Ok(())
}
