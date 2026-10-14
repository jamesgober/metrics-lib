//! Shorthand macros over the global instance ([`metrics`](crate::metrics)).
//!
//! Each macro takes a name and optional `key => value` labels and returns
//! the metric handle (`Arc<Counter>`, `Arc<Gauge>`, …), in the style of the
//! `metrics` crate's macros:
//!
//! ```
//! # #[cfg(all(feature = "count", feature = "gauge"))]
//! # {
//! use metrics_lib::{counter, gauge};
//!
//! counter!("requests").inc();
//! counter!("requests", "method" => "GET", "status" => "200").add(2);
//! gauge!("queue_depth").set(3.0);
//! let tenant = String::from("acme");
//! gauge!("quota_left", "tenant" => tenant).set(10.0);
//! # }
//! ```
//!
//! Lookups go through the registry on every call; keep the returned handle
//! in hot loops.

/// Build a [`LabelSet`](crate::LabelSet) from `key => value` pairs.
///
/// Keys and values may be any mix of `&'static str` and `String`.
///
/// # Examples
///
/// ```
/// use metrics_lib::labels;
///
/// let region = String::from("eu");
/// let set = labels!("method" => "GET", "region" => region);
/// assert_eq!(set.get("region"), Some("eu"));
/// ```
#[macro_export]
macro_rules! labels {
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut set = $crate::LabelSet::new();
        $( let _ = set.add($key, $value); )*
        set
    }};
}

/// Counter handle from the global registry: `counter!("name")` or
/// `counter!("name", "key" => "value", …)`. Requires the `count` feature.
#[cfg(feature = "count")]
#[macro_export]
macro_rules! counter {
    ($name:expr $(,)?) => {
        $crate::metrics().counter($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().counter_with($name, &$crate::labels!($($key => $value),+))
    };
}

/// Up/down counter handle from the global registry. Requires the `count`
/// feature.
#[cfg(feature = "count")]
#[macro_export]
macro_rules! up_down_counter {
    ($name:expr $(,)?) => {
        $crate::metrics().up_down_counter($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().up_down_counter_with($name, &$crate::labels!($($key => $value),+))
    };
}

/// Gauge handle from the global registry. Requires the `gauge` feature.
#[cfg(feature = "gauge")]
#[macro_export]
macro_rules! gauge {
    ($name:expr $(,)?) => {
        $crate::metrics().gauge($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().gauge_with($name, &$crate::labels!($($key => $value),+))
    };
}

/// Timer handle from the global registry. Requires the `timer` feature.
#[cfg(feature = "timer")]
#[macro_export]
macro_rules! timer {
    ($name:expr $(,)?) => {
        $crate::metrics().timer($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().timer_with($name, &$crate::labels!($($key => $value),+))
    };
}

/// Histogram handle from the global registry. Requires the `histogram`
/// feature.
#[cfg(feature = "histogram")]
#[macro_export]
macro_rules! histogram {
    ($name:expr $(,)?) => {
        $crate::metrics().histogram($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().histogram_with($name, &$crate::labels!($($key => $value),+))
    };
}

/// Summary handle from the global registry. Requires the `histogram`
/// feature.
#[cfg(feature = "histogram")]
#[macro_export]
macro_rules! summary {
    ($name:expr $(,)?) => {
        $crate::metrics().summary($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().summary_with($name, &$crate::labels!($($key => $value),+))
    };
}

/// Rate-meter handle from the global registry. Requires the `meter`
/// feature.
#[cfg(feature = "meter")]
#[macro_export]
macro_rules! rate {
    ($name:expr $(,)?) => {
        $crate::metrics().rate($name)
    };
    ($name:expr, $($key:expr => $value:expr),+ $(,)?) => {
        $crate::metrics().rate_with($name, &$crate::labels!($($key => $value),+))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_labels_macro_builds_sets() {
        let empty = crate::labels!();
        assert!(empty.is_empty());
        let owned = String::from("v2");
        let set = crate::labels!("a" => "v1", "b" => owned,);
        assert_eq!(set.len(), 2);
        assert_eq!(set.get("b"), Some("v2"));
    }

    #[cfg(all(
        feature = "count",
        feature = "gauge",
        feature = "timer",
        feature = "histogram",
        feature = "meter"
    ))]
    #[test]
    fn test_metric_macros_hit_the_global_registry() {
        crate::counter!("macro_test_counter").inc();
        crate::counter!("macro_test_counter", "k" => "v").add(2);
        assert_eq!(crate::metrics().counter("macro_test_counter").get(), 1);
        crate::up_down_counter!("macro_test_updown").dec();
        crate::up_down_counter!("macro_test_updown", "k" => "v").dec();
        crate::gauge!("macro_test_gauge").set(1.0);
        crate::gauge!("macro_test_gauge", "k" => "v").set(2.0);
        crate::timer!("macro_test_timer").record_ns(5);
        crate::timer!("macro_test_timer", "k" => "v").record_ns(5);
        crate::histogram!("macro_test_hist").observe(0.1);
        crate::histogram!("macro_test_hist", "k" => "v").observe(0.1);
        crate::summary!("macro_test_summary").observe(0.1);
        crate::summary!("macro_test_summary", "k" => "v").observe(0.1);
        crate::rate!("macro_test_rate").tick();
        crate::rate!("macro_test_rate", "k" => "v").tick();
        assert_eq!(
            crate::metrics()
                .counter_with("macro_test_counter", &crate::labels!("k" => "v"))
                .get(),
            2
        );
    }
}
