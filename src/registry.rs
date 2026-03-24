//! Thread-safe registry for storing and retrieving metrics by name.
//!
//! The `Registry` uses one `RwLock<HashMap>` per enabled metric type.
//! Read-heavy workloads (metric *lookups*) proceed with shared read locks and
//! minimal contention. Write locks are only acquired when a new metric name is
//! registered, which is expected to be an infrequent, one-time-per-name event.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[cfg(feature = "count")]
use crate::Counter;
#[cfg(feature = "gauge")]
use crate::Gauge;
#[cfg(feature = "timer")]
use crate::Timer;
#[cfg(feature = "meter")]
use crate::RateMeter;

/// A thread-safe registry for storing metrics by name.
///
/// Stores one `Arc`-wrapped instance per unique metric name for each enabled
/// metric type. Repeated lookups for the same name return the same `Arc`
/// (pointer equality holds). Only metric types enabled via Cargo features are
/// compiled in; attempting to call a method for a disabled type will result in
/// a compile-time error.
///
/// Cache-line aligned to prevent false sharing in concurrent environments.
#[repr(align(64))]
pub struct Registry {
    #[cfg(feature = "count")]
    counters: RwLock<HashMap<String, Arc<Counter>>>,
    #[cfg(feature = "gauge")]
    gauges: RwLock<HashMap<String, Arc<Gauge>>>,
    #[cfg(feature = "timer")]
    timers: RwLock<HashMap<String, Arc<Timer>>>,
    #[cfg(feature = "meter")]
    rate_meters: RwLock<HashMap<String, Arc<RateMeter>>>,
}

impl Registry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "count")]
            counters: RwLock::new(HashMap::new()),
            #[cfg(feature = "gauge")]
            gauges: RwLock::new(HashMap::new()),
            #[cfg(feature = "timer")]
            timers: RwLock::new(HashMap::new()),
            #[cfg(feature = "meter")]
            rate_meters: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create a counter by name.
    ///
    /// Requires the `count` feature.
    #[cfg(feature = "count")]
    pub fn get_or_create_counter(&self, name: &str) -> Arc<Counter> {
        // Fast path: try read lock first
        if let Ok(counters) = self.counters.read() {
            if let Some(counter) = counters.get(name) {
                return counter.clone();
            }
        }

        // Slow path: write lock to create new counter
        let mut counters = self.counters.write().unwrap_or_else(|e| e.into_inner());
        counters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Counter::new()))
            .clone()
    }

    /// Get or create a gauge by name.
    ///
    /// Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn get_or_create_gauge(&self, name: &str) -> Arc<Gauge> {
        // Fast path: try read lock first
        if let Ok(gauges) = self.gauges.read() {
            if let Some(gauge) = gauges.get(name) {
                return gauge.clone();
            }
        }

        // Slow path: write lock to create new gauge
        let mut gauges = self.gauges.write().unwrap_or_else(|e| e.into_inner());
        gauges
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Gauge::new()))
            .clone()
    }

    /// Get or create a timer by name.
    ///
    /// Requires the `timer` feature.
    #[cfg(feature = "timer")]
    pub fn get_or_create_timer(&self, name: &str) -> Arc<Timer> {
        // Fast path: try read lock first
        if let Ok(timers) = self.timers.read() {
            if let Some(timer) = timers.get(name) {
                return timer.clone();
            }
        }

        // Slow path: write lock to create new timer
        let mut timers = self.timers.write().unwrap_or_else(|e| e.into_inner());
        timers
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Timer::new()))
            .clone()
    }

    /// Get or create a rate meter by name.
    ///
    /// Requires the `meter` feature.
    #[cfg(feature = "meter")]
    pub fn get_or_create_rate_meter(&self, name: &str) -> Arc<RateMeter> {
        // Fast path: try read lock first
        if let Ok(rate_meters) = self.rate_meters.read() {
            if let Some(rate_meter) = rate_meters.get(name) {
                return rate_meter.clone();
            }
        }

        // Slow path: write lock to create new rate meter
        let mut rate_meters = self.rate_meters.write().unwrap_or_else(|e| e.into_inner());
        rate_meters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(RateMeter::new()))
            .clone()
    }

    /// Get all registered counter names. Requires the `count` feature.
    #[cfg(feature = "count")]
    pub fn counter_names(&self) -> Vec<String> {
        self.counters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// Get all registered gauge names. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn gauge_names(&self) -> Vec<String> {
        self.gauges
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// Get all registered timer names. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    pub fn timer_names(&self) -> Vec<String> {
        self.timers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// Get all registered rate meter names. Requires the `meter` feature.
    #[cfg(feature = "meter")]
    pub fn rate_meter_names(&self) -> Vec<String> {
        self.rate_meters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// Get total number of registered metrics across all enabled metric types.
    pub fn metric_count(&self) -> usize {
        #[allow(unused_mut)]
        let mut total = 0;
        #[cfg(feature = "count")]
        {
            total += self.counters.read().unwrap_or_else(|e| e.into_inner()).len();
        }
        #[cfg(feature = "gauge")]
        {
            total += self.gauges.read().unwrap_or_else(|e| e.into_inner()).len();
        }
        #[cfg(feature = "timer")]
        {
            total += self.timers.read().unwrap_or_else(|e| e.into_inner()).len();
        }
        #[cfg(feature = "meter")]
        {
            total += self.rate_meters.read().unwrap_or_else(|e| e.into_inner()).len();
        }
        total
    }

    /// Clear all metrics from the registry.
    pub fn clear(&self) {
        #[cfg(feature = "count")]
        self.counters.write().unwrap_or_else(|e| e.into_inner()).clear();
        #[cfg(feature = "gauge")]
        self.gauges.write().unwrap_or_else(|e| e.into_inner()).clear();
        #[cfg(feature = "timer")]
        self.timers.write().unwrap_or_else(|e| e.into_inner()).clear();
        #[cfg(feature = "meter")]
        self.rate_meters.write().unwrap_or_else(|e| e.into_inner()).clear();
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

// `Registry` is Send + Sync automatically because `RwLock<HashMap<…>>` is
// Send + Sync when its contents are Send + Sync. No unsafe impls required.

#[cfg(test)]
// Registry integration tests require all four metric-type features.
#[cfg(all(feature = "count", feature = "gauge", feature = "timer", feature = "meter"))]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_counter_registration() {
        let registry = Registry::new();

        let counter1 = registry.get_or_create_counter("requests");
        let counter2 = registry.get_or_create_counter("requests");

        // Should return the same instance
        assert!(Arc::ptr_eq(&counter1, &counter2));
        assert_eq!(registry.metric_count(), 1);
    }

    #[test]
    fn test_gauge_registration() {
        let registry = Registry::new();

        let gauge1 = registry.get_or_create_gauge("cpu_usage");
        let gauge2 = registry.get_or_create_gauge("cpu_usage");

        // Should return the same instance
        assert!(Arc::ptr_eq(&gauge1, &gauge2));
        assert_eq!(registry.metric_count(), 1);
    }

    #[test]
    fn test_timer_registration() {
        let registry = Registry::new();

        let timer1 = registry.get_or_create_timer("db_query");
        let timer2 = registry.get_or_create_timer("db_query");

        // Should return the same instance
        assert!(Arc::ptr_eq(&timer1, &timer2));
        assert_eq!(registry.metric_count(), 1);
    }

    #[test]
    fn test_rate_meter_registration() {
        let registry = Registry::new();

        let meter1 = registry.get_or_create_rate_meter("api_calls");
        let meter2 = registry.get_or_create_rate_meter("api_calls");

        // Should return the same instance
        assert!(Arc::ptr_eq(&meter1, &meter2));
        assert_eq!(registry.metric_count(), 1);
    }

    #[test]
    fn test_mixed_metrics() {
        let registry = Registry::new();

        let _counter = registry.get_or_create_counter("requests");
        let _gauge = registry.get_or_create_gauge("cpu_usage");
        let _timer = registry.get_or_create_timer("db_query");
        let _meter = registry.get_or_create_rate_meter("api_calls");

        assert_eq!(registry.metric_count(), 4);
        assert_eq!(registry.counter_names().len(), 1);
        assert_eq!(registry.gauge_names().len(), 1);
        assert_eq!(registry.timer_names().len(), 1);
        assert_eq!(registry.rate_meter_names().len(), 1);
    }

    #[test]
    fn test_concurrent_access() {
        let registry = Arc::new(Registry::new());
        let mut handles = vec![];

        // Spawn multiple threads accessing the same counter
        for _i in 0..10 {
            let registry = registry.clone();
            let handle = thread::spawn(move || {
                let counter = registry.get_or_create_counter("concurrent_test");
                counter.inc();
                counter.get()
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have exactly one counter registered
        assert_eq!(registry.metric_count(), 1);
        let counter = registry.get_or_create_counter("concurrent_test");
        assert_eq!(counter.get(), 10);
    }

    #[test]
    fn test_clear() {
        let registry = Registry::new();

        let _counter = registry.get_or_create_counter("requests");
        let _gauge = registry.get_or_create_gauge("cpu_usage");

        assert_eq!(registry.metric_count(), 2);

        registry.clear();
        assert_eq!(registry.metric_count(), 0);
    }

    #[test]
    fn test_metric_names() {
        let registry = Registry::new();

        let _counter1 = registry.get_or_create_counter("requests");
        let _counter2 = registry.get_or_create_counter("errors");
        let _gauge1 = registry.get_or_create_gauge("cpu_usage");

        let counter_names = registry.counter_names();
        let gauge_names = registry.gauge_names();

        assert_eq!(counter_names.len(), 2);
        assert_eq!(gauge_names.len(), 1);
        assert!(counter_names.contains(&"requests".to_string()));
        assert!(counter_names.contains(&"errors".to_string()));
        assert!(gauge_names.contains(&"cpu_usage".to_string()));
    }

    #[test]
    fn test_duplicate_names_across_types_are_independent() {
        let registry = Registry::new();

        let c = registry.get_or_create_counter("same_name");
        let g = registry.get_or_create_gauge("same_name");
        let t = registry.get_or_create_timer("same_name");
        let r = registry.get_or_create_rate_meter("same_name");

        // All should be registered independently (4 metrics total)
        assert_eq!(registry.metric_count(), 4);

        // And they should be distinct types; at least ensure their addresses differ pairwise
        let c_addr = Arc::as_ptr(&c) as usize;
        let g_addr = Arc::as_ptr(&g) as usize;
        let t_addr = Arc::as_ptr(&t) as usize;
        let r_addr = Arc::as_ptr(&r) as usize;

        assert_ne!(c_addr, g_addr);
        assert_ne!(c_addr, t_addr);
        assert_ne!(c_addr, r_addr);
        assert_ne!(g_addr, t_addr);
        assert_ne!(g_addr, r_addr);
        assert_ne!(t_addr, r_addr);
    }

    #[test]
    fn test_clear_then_recreate_returns_new_instances() {
        let registry = Registry::new();

        let counter_before = registry.get_or_create_counter("requests");
        let gauge_before = registry.get_or_create_gauge("cpu");
        assert_eq!(registry.metric_count(), 2);

        // Clear the registry; previously returned Arcs still exist but registry is empty
        registry.clear();
        assert_eq!(registry.metric_count(), 0);

        let counter_after = registry.get_or_create_counter("requests");
        let gauge_after = registry.get_or_create_gauge("cpu");

        // New instances should NOT be ptr-equal to the old ones
        assert!(!Arc::ptr_eq(&counter_before, &counter_after));
        assert!(!Arc::ptr_eq(&gauge_before, &gauge_after));
    }

    #[test]
    fn test_concurrent_duplicate_registration_singleton_per_name() {
        let registry = Arc::new(Registry::new());
        let mut handles = vec![];

        for _ in 0..16 {
            let r = registry.clone();
            handles.push(thread::spawn(move || r.get_or_create_timer("dup")));
        }

        let first = registry.get_or_create_timer("dup");
        for h in handles {
            let timer = h.join().unwrap();
            assert!(Arc::ptr_eq(&first, &timer));
        }
        assert_eq!(registry.metric_count(), 1);
    }
}
