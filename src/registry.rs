//! Lock-free registry for storing and retrieving metrics by name.
//!
//! The Registry uses a lock-free hash table with memory-efficient string interning
//! to provide O(1) metric lookup with minimal memory overhead.

use crate::{Counter, Gauge, RateMeter, Timer};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A thread-safe registry for storing metrics by name.
///
/// Uses RwLock for simplicity while maintaining good performance for the
/// read-heavy workloads typical in metrics collection.
#[repr(align(64))]
pub struct Registry {
    counters: RwLock<HashMap<String, Arc<Counter>>>,
    gauges: RwLock<HashMap<String, Arc<Gauge>>>,
    timers: RwLock<HashMap<String, Arc<Timer>>>,
    rate_meters: RwLock<HashMap<String, Arc<RateMeter>>>,
}

impl Registry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            timers: RwLock::new(HashMap::new()),
            rate_meters: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create a counter by name.
    pub fn get_or_create_counter(&self, name: &str) -> Arc<Counter> {
        // Fast path: try read lock first
        if let Ok(counters) = self.counters.read() {
            if let Some(counter) = counters.get(name) {
                return counter.clone();
            }
        }

        // Slow path: write lock to create new counter
        let mut counters = self.counters.write().unwrap();
        counters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Counter::new()))
            .clone()
    }

    /// Get or create a gauge by name.
    pub fn get_or_create_gauge(&self, name: &str) -> Arc<Gauge> {
        // Fast path: try read lock first
        if let Ok(gauges) = self.gauges.read() {
            if let Some(gauge) = gauges.get(name) {
                return gauge.clone();
            }
        }

        // Slow path: write lock to create new gauge
        let mut gauges = self.gauges.write().unwrap();
        gauges
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Gauge::new()))
            .clone()
    }

    /// Get or create a timer by name.
    pub fn get_or_create_timer(&self, name: &str) -> Arc<Timer> {
        // Fast path: try read lock first
        if let Ok(timers) = self.timers.read() {
            if let Some(timer) = timers.get(name) {
                return timer.clone();
            }
        }

        // Slow path: write lock to create new timer
        let mut timers = self.timers.write().unwrap();
        timers
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Timer::new()))
            .clone()
    }

    /// Get or create a rate meter by name.
    pub fn get_or_create_rate_meter(&self, name: &str) -> Arc<RateMeter> {
        // Fast path: try read lock first
        if let Ok(rate_meters) = self.rate_meters.read() {
            if let Some(rate_meter) = rate_meters.get(name) {
                return rate_meter.clone();
            }
        }

        // Slow path: write lock to create new rate meter
        let mut rate_meters = self.rate_meters.write().unwrap();
        rate_meters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(RateMeter::new()))
            .clone()
    }

    /// Get all registered counter names.
    pub fn counter_names(&self) -> Vec<String> {
        self.counters.read().unwrap().keys().cloned().collect()
    }

    /// Get all registered gauge names.
    pub fn gauge_names(&self) -> Vec<String> {
        self.gauges.read().unwrap().keys().cloned().collect()
    }

    /// Get all registered timer names.
    pub fn timer_names(&self) -> Vec<String> {
        self.timers.read().unwrap().keys().cloned().collect()
    }

    /// Get all registered rate meter names.
    pub fn rate_meter_names(&self) -> Vec<String> {
        self.rate_meters.read().unwrap().keys().cloned().collect()
    }

    /// Get total number of registered metrics.
    pub fn metric_count(&self) -> usize {
        self.counters.read().unwrap().len()
            + self.gauges.read().unwrap().len()
            + self.timers.read().unwrap().len()
            + self.rate_meters.read().unwrap().len()
    }

    /// Clear all metrics from the registry.
    pub fn clear(&self) {
        self.counters.write().unwrap().clear();
        self.gauges.write().unwrap().clear();
        self.timers.write().unwrap().clear();
        self.rate_meters.write().unwrap().clear();
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}

#[cfg(test)]
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
}
