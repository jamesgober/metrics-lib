//! Labeled series, validation, and the series cap.
//!
//! Run with: `cargo run --example labels_demo`

use metrics_lib::{LabelSet, MetricsCore, MetricsError, Unit, labels};

fn main() -> Result<(), MetricsError> {
    let core = MetricsCore::new();
    core.registry()
        .describe_counter("http_requests", "HTTP requests handled", Unit::None)?;

    let ok = labels!("method" => "GET", "status" => "200");
    let failed = labels!("method" => "POST", "status" => "500");
    core.counter_with("http_requests", &ok).add(10);
    core.counter_with("http_requests", &failed).add(3);

    // Runtime values need no `Box::leak`.
    for tenant in ["acme", "globex", "initech"] {
        core.counter_with("auth_failures", &labels!("tenant" => tenant.to_owned()))
            .add(2);
    }

    // Validation: `try_add` refuses bad keys, `try_*` lookups refuse bad names.
    let mut l = LabelSet::new();
    assert!(l.try_add("bad key", "v").is_err());
    assert!(core.try_counter("bad name").is_err());

    // The series cap bounds memory however many label values arrive.
    core.registry()
        .set_cardinality_cap(core.registry().cardinality_count() + 5);
    for i in 0..100 {
        core.counter_with("per_user", &labels!("user" => format!("u{i}")))
            .inc();
    }
    println!(
        "series={} cap={} overflows={}",
        core.registry().metric_count(),
        core.registry().cardinality_cap(),
        core.registry().cardinality_overflows()
    );
    match core.try_counter_with("per_user", &labels!("user" => "u999")) {
        Err(MetricsError::CardinalityExceeded) => println!("cap reached, as configured"),
        other => println!("unexpected: {other:?}"),
    }
    print!(
        "{}",
        metrics_lib::exporters::prometheus::render(core.registry())
    );
    Ok(())
}
