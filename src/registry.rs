//! Thread-safe registry for storing and retrieving metrics by name and labels.
//!
//! # Architecture
//!
//! The registry maintains two parallel storage tracks for each metric type:
//!
//! 1. **Unlabeled fast path** — `HashMap<String, Arc<T>>` keyed by name only.
//!    This is the original (pre-0.9.3) storage and remains the cheapest
//!    lookup: read-locked `HashMap::get(&str)` returns a cloned `Arc` with
//!    zero allocations on hit.
//! 2. **Labeled path** — `HashMap<(String, LabelSet), Arc<T>>` keyed by
//!    `(name, labels)`. Each unique `(name, labels)` tuple maps to a distinct
//!    `Arc`. Lookups on this path allocate the composite key on every hit
//!    (`String` clone + `LabelSet` clone) — callers should cache the returned
//!    `Arc` in long-lived references for hot paths.
//!
//! # Cardinality
//!
//! The labeled path is subject to a hard cap on the total number of unique
//! `(name, labels)` tuples across **all** metric types. The default is
//! 10 000; configure via [`Registry::set_cardinality_cap`]. When a fresh
//! `(name, labels)` registration would exceed the cap:
//!
//! - The `try_*_with` lookup variants return [`MetricsError::CardinalityExceeded`].
//! - The non-`try` `*_with` variants route to a process-global per-type
//!   "overflow sink" `Arc<T>` (initialised on first use, never registered in
//!   the maps, never exported). Updates land on the sink and are observable
//!   only via [`Registry::cardinality_overflows`].
//!
//! This preserves a panic-free hot path for misbehaving label producers while
//! still surfacing the problem through the overflow counter.
//!
//! # Metadata
//!
//! Optional per-name [`MetricMetadata`] (help text, unit, kind) is stored in
//! a separate map and consumed by exporters. Register via
//! [`Registry::describe`] or the kind-specific shorthands
//! (`describe_counter` / `describe_gauge` / …).

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::RwLock;

/// Convenience alias used throughout this file to gate code that only
/// matters when at least one metric type is compiled in.
#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
use std::sync::{Arc, OnceLock};

#[cfg(feature = "count")]
use crate::Counter;
#[cfg(feature = "gauge")]
use crate::Gauge;
#[cfg(feature = "histogram")]
use crate::Histogram;
#[cfg(feature = "meter")]
use crate::RateMeter;
#[cfg(feature = "timer")]
use crate::Timer;

#[cfg(any(
    feature = "count",
    feature = "gauge",
    feature = "timer",
    feature = "meter",
    feature = "histogram"
))]
use crate::{LabelSet, MetricsError, Result};

use crate::{MetricKind, MetricMetadata, Unit};

/// Default per-registry cardinality cap on unique `(name, labels)` tuples.
pub const DEFAULT_CARDINALITY_CAP: usize = 10_000;

/// A thread-safe registry for storing metrics by name and labels.
///
/// The registry maintains two parallel storage tracks per metric type — an
/// unlabeled fast path (`HashMap<String, Arc<T>>`) and a labeled path keyed
/// by `(name, LabelSet)`. The labeled path is subject to a hard cardinality
/// cap (default 10 000; see [`Registry::set_cardinality_cap`]). Per-metric
/// description, unit, and kind metadata are stored separately and consumed
/// by exporters in [`crate::exporters`].
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

    #[cfg(feature = "count")]
    labeled_counters: RwLock<HashMap<(String, LabelSet), Arc<Counter>>>,
    #[cfg(feature = "gauge")]
    labeled_gauges: RwLock<HashMap<(String, LabelSet), Arc<Gauge>>>,
    #[cfg(feature = "timer")]
    labeled_timers: RwLock<HashMap<(String, LabelSet), Arc<Timer>>>,
    #[cfg(feature = "meter")]
    labeled_rate_meters: RwLock<HashMap<(String, LabelSet), Arc<RateMeter>>>,
    #[cfg(feature = "histogram")]
    histograms: RwLock<HashMap<(String, LabelSet), Arc<Histogram>>>,
    #[cfg(feature = "histogram")]
    histogram_buckets: RwLock<HashMap<String, Vec<f64>>>,

    metadata: RwLock<HashMap<String, MetricMetadata>>,

    cardinality_cap: AtomicUsize,
    cardinality_count: AtomicUsize,
    cardinality_overflows: AtomicU64,
}

impl Registry {
    /// Create a new empty registry with the default cardinality cap
    /// ([`DEFAULT_CARDINALITY_CAP`]).
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

            #[cfg(feature = "count")]
            labeled_counters: RwLock::new(HashMap::new()),
            #[cfg(feature = "gauge")]
            labeled_gauges: RwLock::new(HashMap::new()),
            #[cfg(feature = "timer")]
            labeled_timers: RwLock::new(HashMap::new()),
            #[cfg(feature = "meter")]
            labeled_rate_meters: RwLock::new(HashMap::new()),
            #[cfg(feature = "histogram")]
            histograms: RwLock::new(HashMap::new()),
            #[cfg(feature = "histogram")]
            histogram_buckets: RwLock::new(HashMap::new()),

            metadata: RwLock::new(HashMap::new()),

            cardinality_cap: AtomicUsize::new(DEFAULT_CARDINALITY_CAP),
            cardinality_count: AtomicUsize::new(0),
            cardinality_overflows: AtomicU64::new(0),
        }
    }

    // ---------------------------------------------------------------------
    // Cardinality control
    // ---------------------------------------------------------------------

    /// Set the cardinality cap. New labeled registrations beyond this cap
    /// return overflow sinks (or `Err(CardinalityExceeded)` via the
    /// `try_*_with` paths).
    #[inline]
    pub fn set_cardinality_cap(&self, cap: usize) {
        self.cardinality_cap.store(cap, Ordering::Relaxed);
    }

    /// Current cardinality cap.
    #[must_use]
    #[inline]
    pub fn cardinality_cap(&self) -> usize {
        self.cardinality_cap.load(Ordering::Relaxed)
    }

    /// Count of unique `(name, labels)` tuples currently registered across
    /// all labeled metric types.
    #[must_use]
    #[inline]
    pub fn cardinality_count(&self) -> usize {
        self.cardinality_count.load(Ordering::Relaxed)
    }

    /// Total number of overflow events (labeled registrations that hit the
    /// cap and were routed to the sink).
    #[must_use]
    #[inline]
    pub fn cardinality_overflows(&self) -> u64 {
        self.cardinality_overflows.load(Ordering::Relaxed)
    }

    /// Reserve one cardinality slot. Returns `true` if a slot was acquired;
    /// `false` if the cap is full (caller routes to overflow sink).
    #[cfg(any(
        feature = "count",
        feature = "gauge",
        feature = "timer",
        feature = "meter",
        feature = "histogram"
    ))]
    fn try_acquire_slot(&self) -> bool {
        let cap = self.cardinality_cap();
        loop {
            let current = self.cardinality_count.load(Ordering::Relaxed);
            if current >= cap {
                self.cardinality_overflows.fetch_add(1, Ordering::Relaxed);
                return false;
            }
            if self
                .cardinality_count
                .compare_exchange_weak(current, current + 1, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return true;
            }
        }
    }

    // ---------------------------------------------------------------------
    // Metadata
    // ---------------------------------------------------------------------

    /// Register metadata (help text + unit + kind) for a metric name.
    ///
    /// Calling `describe` again with the same name replaces the prior entry.
    pub fn describe(&self, name: &str, metadata: MetricMetadata) {
        self.metadata
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .insert(name.to_string(), metadata);
    }

    /// Convenience: describe a counter.
    pub fn describe_counter(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.describe(name, MetricMetadata::new(MetricKind::Counter, help, unit));
    }

    /// Convenience: describe a gauge.
    pub fn describe_gauge(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.describe(name, MetricMetadata::new(MetricKind::Gauge, help, unit));
    }

    /// Convenience: describe a timer.
    pub fn describe_timer(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.describe(name, MetricMetadata::new(MetricKind::Timer, help, unit));
    }

    /// Convenience: describe a rate meter.
    pub fn describe_rate(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.describe(name, MetricMetadata::new(MetricKind::Rate, help, unit));
    }

    /// Convenience: describe a histogram.
    pub fn describe_histogram(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.describe(name, MetricMetadata::new(MetricKind::Histogram, help, unit));
    }

    /// Look up metadata for a metric by name.
    #[must_use]
    pub fn metadata(&self, name: &str) -> Option<MetricMetadata> {
        self.metadata
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(name)
            .cloned()
    }

    // ---------------------------------------------------------------------
    // Counter
    // ---------------------------------------------------------------------

    /// Get or create an unlabeled counter.
    ///
    /// Requires the `count` feature.
    #[cfg(feature = "count")]
    pub fn get_or_create_counter(&self, name: &str) -> Arc<Counter> {
        if let Ok(map) = self.counters.read() {
            if let Some(c) = map.get(name) {
                return c.clone();
            }
        }
        let mut map = self.counters.write().unwrap_or_else(|e| e.into_inner());
        map.entry(name.to_string())
            .or_insert_with(|| Arc::new(Counter::new()))
            .clone()
    }

    /// Get or create a counter for the supplied `(name, labels)` tuple.
    /// Routes to the per-type cardinality-overflow sink when the cap is full.
    #[cfg(feature = "count")]
    pub fn get_or_create_counter_with(&self, name: &str, labels: &LabelSet) -> Arc<Counter> {
        if labels.is_empty() {
            return self.get_or_create_counter(name);
        }
        match self.try_get_or_create_counter_with(name, labels) {
            Ok(c) => c,
            Err(_) => counter_overflow_sink().clone(),
        }
    }

    /// Try to get or create a labeled counter. Returns
    /// `Err(CardinalityExceeded)` when the cap is full.
    #[cfg(feature = "count")]
    pub fn try_get_or_create_counter_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<Arc<Counter>> {
        if labels.is_empty() {
            return Ok(self.get_or_create_counter(name));
        }
        if let Ok(map) = self.labeled_counters.read() {
            if let Some(c) = map.get(&(name.to_string(), labels.clone())) {
                return Ok(c.clone());
            }
        }
        let mut map = self
            .labeled_counters
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let key = (name.to_string(), labels.clone());
        if let Some(c) = map.get(&key) {
            return Ok(c.clone());
        }
        if !self.try_acquire_slot() {
            return Err(MetricsError::CardinalityExceeded);
        }
        let c = Arc::new(Counter::new());
        map.insert(key, c.clone());
        Ok(c)
    }

    // ---------------------------------------------------------------------
    // Gauge
    // ---------------------------------------------------------------------

    /// Get or create an unlabeled gauge. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn get_or_create_gauge(&self, name: &str) -> Arc<Gauge> {
        if let Ok(map) = self.gauges.read() {
            if let Some(g) = map.get(name) {
                return g.clone();
            }
        }
        let mut map = self.gauges.write().unwrap_or_else(|e| e.into_inner());
        map.entry(name.to_string())
            .or_insert_with(|| Arc::new(Gauge::new()))
            .clone()
    }

    /// Labeled gauge with overflow-sink fallback. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn get_or_create_gauge_with(&self, name: &str, labels: &LabelSet) -> Arc<Gauge> {
        if labels.is_empty() {
            return self.get_or_create_gauge(name);
        }
        match self.try_get_or_create_gauge_with(name, labels) {
            Ok(g) => g,
            Err(_) => gauge_overflow_sink().clone(),
        }
    }

    /// Labeled gauge returning `Err(CardinalityExceeded)` on overflow.
    /// Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn try_get_or_create_gauge_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<Arc<Gauge>> {
        if labels.is_empty() {
            return Ok(self.get_or_create_gauge(name));
        }
        if let Ok(map) = self.labeled_gauges.read() {
            if let Some(g) = map.get(&(name.to_string(), labels.clone())) {
                return Ok(g.clone());
            }
        }
        let mut map = self
            .labeled_gauges
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let key = (name.to_string(), labels.clone());
        if let Some(g) = map.get(&key) {
            return Ok(g.clone());
        }
        if !self.try_acquire_slot() {
            return Err(MetricsError::CardinalityExceeded);
        }
        let g = Arc::new(Gauge::new());
        map.insert(key, g.clone());
        Ok(g)
    }

    // ---------------------------------------------------------------------
    // Timer
    // ---------------------------------------------------------------------

    /// Get or create an unlabeled timer. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    pub fn get_or_create_timer(&self, name: &str) -> Arc<Timer> {
        if let Ok(map) = self.timers.read() {
            if let Some(t) = map.get(name) {
                return t.clone();
            }
        }
        let mut map = self.timers.write().unwrap_or_else(|e| e.into_inner());
        map.entry(name.to_string())
            .or_insert_with(|| Arc::new(Timer::new()))
            .clone()
    }

    /// Labeled timer with overflow-sink fallback. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    pub fn get_or_create_timer_with(&self, name: &str, labels: &LabelSet) -> Arc<Timer> {
        if labels.is_empty() {
            return self.get_or_create_timer(name);
        }
        match self.try_get_or_create_timer_with(name, labels) {
            Ok(t) => t,
            Err(_) => timer_overflow_sink().clone(),
        }
    }

    /// Labeled timer returning `Err(CardinalityExceeded)` on overflow.
    /// Requires the `timer` feature.
    #[cfg(feature = "timer")]
    pub fn try_get_or_create_timer_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<Arc<Timer>> {
        if labels.is_empty() {
            return Ok(self.get_or_create_timer(name));
        }
        if let Ok(map) = self.labeled_timers.read() {
            if let Some(t) = map.get(&(name.to_string(), labels.clone())) {
                return Ok(t.clone());
            }
        }
        let mut map = self
            .labeled_timers
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let key = (name.to_string(), labels.clone());
        if let Some(t) = map.get(&key) {
            return Ok(t.clone());
        }
        if !self.try_acquire_slot() {
            return Err(MetricsError::CardinalityExceeded);
        }
        let t = Arc::new(Timer::new());
        map.insert(key, t.clone());
        Ok(t)
    }

    // ---------------------------------------------------------------------
    // Rate meter
    // ---------------------------------------------------------------------

    /// Get or create an unlabeled rate meter. Requires the `meter` feature.
    #[cfg(feature = "meter")]
    pub fn get_or_create_rate_meter(&self, name: &str) -> Arc<RateMeter> {
        if let Ok(map) = self.rate_meters.read() {
            if let Some(r) = map.get(name) {
                return r.clone();
            }
        }
        let mut map = self.rate_meters.write().unwrap_or_else(|e| e.into_inner());
        map.entry(name.to_string())
            .or_insert_with(|| Arc::new(RateMeter::new()))
            .clone()
    }

    /// Labeled rate meter with overflow-sink fallback. Requires the `meter`
    /// feature.
    #[cfg(feature = "meter")]
    pub fn get_or_create_rate_meter_with(&self, name: &str, labels: &LabelSet) -> Arc<RateMeter> {
        if labels.is_empty() {
            return self.get_or_create_rate_meter(name);
        }
        match self.try_get_or_create_rate_meter_with(name, labels) {
            Ok(r) => r,
            Err(_) => rate_meter_overflow_sink().clone(),
        }
    }

    /// Labeled rate meter returning `Err(CardinalityExceeded)` on overflow.
    /// Requires the `meter` feature.
    #[cfg(feature = "meter")]
    pub fn try_get_or_create_rate_meter_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<Arc<RateMeter>> {
        if labels.is_empty() {
            return Ok(self.get_or_create_rate_meter(name));
        }
        if let Ok(map) = self.labeled_rate_meters.read() {
            if let Some(r) = map.get(&(name.to_string(), labels.clone())) {
                return Ok(r.clone());
            }
        }
        let mut map = self
            .labeled_rate_meters
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let key = (name.to_string(), labels.clone());
        if let Some(r) = map.get(&key) {
            return Ok(r.clone());
        }
        if !self.try_acquire_slot() {
            return Err(MetricsError::CardinalityExceeded);
        }
        let r = Arc::new(RateMeter::new());
        map.insert(key, r.clone());
        Ok(r)
    }

    // ---------------------------------------------------------------------
    // Histogram
    // ---------------------------------------------------------------------

    /// Pre-configure histogram bucket boundaries for the supplied metric
    /// name. Subsequent `histogram` / `histogram_with` registrations for the
    /// same name will use these bounds. Already-registered histograms are
    /// **not** retroactively rebucketed; configure before first use.
    ///
    /// Requires the `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn configure_histogram(&self, name: &str, buckets: impl IntoIterator<Item = f64>) {
        let buckets: Vec<f64> = buckets.into_iter().collect();
        self.histogram_buckets
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .insert(name.to_string(), buckets);
    }

    /// Get or create an unlabeled histogram.
    ///
    /// Uses buckets configured via [`Self::configure_histogram`] or the
    /// Prometheus default seconds buckets if none configured.
    /// Requires the `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn get_or_create_histogram(&self, name: &str) -> Arc<Histogram> {
        // Histograms always live in the labeled map keyed by `LabelSet::EMPTY`
        // so a single iteration point covers both labeled and unlabeled.
        self.get_or_create_histogram_with(name, &LabelSet::EMPTY)
    }

    /// Labeled histogram with overflow-sink fallback. Requires the
    /// `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn get_or_create_histogram_with(&self, name: &str, labels: &LabelSet) -> Arc<Histogram> {
        match self.try_get_or_create_histogram_with(name, labels) {
            Ok(h) => h,
            Err(_) => histogram_overflow_sink().clone(),
        }
    }

    /// Labeled histogram returning `Err(CardinalityExceeded)` on overflow.
    /// Requires the `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn try_get_or_create_histogram_with(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<Arc<Histogram>> {
        if let Ok(map) = self.histograms.read() {
            if let Some(h) = map.get(&(name.to_string(), labels.clone())) {
                return Ok(h.clone());
            }
        }
        // Only labeled-empty histograms skip the cardinality cap (they are
        // the unlabeled "default" series). Labeled variants consume slots.
        if !labels.is_empty() && !self.try_acquire_slot() {
            return Err(MetricsError::CardinalityExceeded);
        }
        let mut map = self.histograms.write().unwrap_or_else(|e| e.into_inner());
        let key = (name.to_string(), labels.clone());
        if let Some(h) = map.get(&key) {
            return Ok(h.clone());
        }
        // Materialise a histogram with the configured buckets (or the
        // Prometheus default if none configured).
        let buckets = self
            .histogram_buckets
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(name)
            .cloned();
        let h = Arc::new(match buckets {
            Some(b) => Histogram::with_buckets(b),
            None => Histogram::default_seconds(),
        });
        map.insert(key, h.clone());
        Ok(h)
    }

    // ---------------------------------------------------------------------
    // Listing accessors (existing API + new labeled accessors)
    // ---------------------------------------------------------------------

    /// All unlabeled counter names. Requires the `count` feature.
    #[cfg(feature = "count")]
    pub fn counter_names(&self) -> Vec<String> {
        self.counters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// All unlabeled gauge names. Requires the `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn gauge_names(&self) -> Vec<String> {
        self.gauges
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// All unlabeled timer names. Requires the `timer` feature.
    #[cfg(feature = "timer")]
    pub fn timer_names(&self) -> Vec<String> {
        self.timers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// All unlabeled rate meter names. Requires the `meter` feature.
    #[cfg(feature = "meter")]
    pub fn rate_meter_names(&self) -> Vec<String> {
        self.rate_meters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .cloned()
            .collect()
    }

    /// All registered histogram names (labeled + unlabeled). Requires the
    /// `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn histogram_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .histograms
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .keys()
            .map(|(n, _)| n.clone())
            .collect();
        names.sort();
        names.dedup();
        names
    }

    /// Total number of registered metrics across all enabled metric types
    /// and label combinations.
    pub fn metric_count(&self) -> usize {
        #[allow(unused_mut)]
        let mut total = 0;
        #[cfg(feature = "count")]
        {
            total += self
                .counters
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
            total += self
                .labeled_counters
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
        }
        #[cfg(feature = "gauge")]
        {
            total += self.gauges.read().unwrap_or_else(|e| e.into_inner()).len();
            total += self
                .labeled_gauges
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
        }
        #[cfg(feature = "timer")]
        {
            total += self.timers.read().unwrap_or_else(|e| e.into_inner()).len();
            total += self
                .labeled_timers
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
        }
        #[cfg(feature = "meter")]
        {
            total += self
                .rate_meters
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
            total += self
                .labeled_rate_meters
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
        }
        #[cfg(feature = "histogram")]
        {
            total += self
                .histograms
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .len();
        }
        total
    }

    /// Clear every registered metric, all metadata, and reset cardinality
    /// counters. Previously-returned `Arc`s remain valid but become detached
    /// from the registry.
    pub fn clear(&self) {
        #[cfg(feature = "count")]
        {
            self.counters
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            self.labeled_counters
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
        }
        #[cfg(feature = "gauge")]
        {
            self.gauges
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            self.labeled_gauges
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
        }
        #[cfg(feature = "timer")]
        {
            self.timers
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            self.labeled_timers
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
        }
        #[cfg(feature = "meter")]
        {
            self.rate_meters
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            self.labeled_rate_meters
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
        }
        #[cfg(feature = "histogram")]
        {
            self.histograms
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            self.histogram_buckets
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
        }
        self.metadata
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
        self.cardinality_count.store(0, Ordering::Relaxed);
        // Note: `cardinality_overflows` is monotonic and intentionally not reset.
    }

    // ---------------------------------------------------------------------
    // Snapshot accessors (exporter hooks)
    // ---------------------------------------------------------------------

    /// Capture every counter as `(name, labels, Arc<Counter>)`. Requires the
    /// `count` feature.
    #[cfg(feature = "count")]
    pub fn counter_entries(&self) -> Vec<(String, LabelSet, Arc<Counter>)> {
        let mut out = Vec::new();
        for (name, c) in self
            .counters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            out.push((name.clone(), LabelSet::EMPTY, c.clone()));
        }
        for ((name, labels), c) in self
            .labeled_counters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            out.push((name.clone(), labels.clone(), c.clone()));
        }
        out
    }

    /// Capture every gauge as `(name, labels, Arc<Gauge>)`. Requires the
    /// `gauge` feature.
    #[cfg(feature = "gauge")]
    pub fn gauge_entries(&self) -> Vec<(String, LabelSet, Arc<Gauge>)> {
        let mut out = Vec::new();
        for (name, g) in self.gauges.read().unwrap_or_else(|e| e.into_inner()).iter() {
            out.push((name.clone(), LabelSet::EMPTY, g.clone()));
        }
        for ((name, labels), g) in self
            .labeled_gauges
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            out.push((name.clone(), labels.clone(), g.clone()));
        }
        out
    }

    /// Capture every timer as `(name, labels, Arc<Timer>)`. Requires the
    /// `timer` feature.
    #[cfg(feature = "timer")]
    pub fn timer_entries(&self) -> Vec<(String, LabelSet, Arc<Timer>)> {
        let mut out = Vec::new();
        for (name, t) in self.timers.read().unwrap_or_else(|e| e.into_inner()).iter() {
            out.push((name.clone(), LabelSet::EMPTY, t.clone()));
        }
        for ((name, labels), t) in self
            .labeled_timers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            out.push((name.clone(), labels.clone(), t.clone()));
        }
        out
    }

    /// Capture every rate meter as `(name, labels, Arc<RateMeter>)`.
    /// Requires the `meter` feature.
    #[cfg(feature = "meter")]
    pub fn rate_meter_entries(&self) -> Vec<(String, LabelSet, Arc<RateMeter>)> {
        let mut out = Vec::new();
        for (name, r) in self
            .rate_meters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            out.push((name.clone(), LabelSet::EMPTY, r.clone()));
        }
        for ((name, labels), r) in self
            .labeled_rate_meters
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            out.push((name.clone(), labels.clone(), r.clone()));
        }
        out
    }

    /// Capture every histogram as `(name, labels, Arc<Histogram>)`. Requires
    /// the `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn histogram_entries(&self) -> Vec<(String, LabelSet, Arc<Histogram>)> {
        self.histograms
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .map(|((n, l), h)| (n.clone(), l.clone(), h.clone()))
            .collect()
    }

    /// Build a [`ScopedRegistry`] that prepends `prefix` to every metric
    /// name on registration and lookup.
    ///
    /// All scoped lookups land in the same underlying `Registry`, so a scope
    /// is a pure naming convention — there's no separate `Arc` storage. Two
    /// scopes that produce the same effective name (e.g.
    /// `scoped("http_").counter("requests")` and `counter("http_requests")`)
    /// return the **same** `Arc<Counter>`.
    ///
    /// Available since v0.9.5.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(all(feature = "count", feature = "gauge"))]
    /// # {
    /// use metrics_lib::{init, metrics};
    /// init();
    /// let http = metrics().registry().scoped("http.");
    /// http.counter("requests").inc();   // registers "http.requests"
    /// http.gauge("inflight").set(1.0);  // registers "http.inflight"
    /// # }
    /// ```
    pub fn scoped(&self, prefix: impl Into<String>) -> ScopedRegistry<'_> {
        ScopedRegistry {
            registry: self,
            prefix: prefix.into(),
        }
    }
}

/// View into a [`Registry`] that prepends a fixed `prefix` to every metric
/// name. Created via [`Registry::scoped`].
///
/// Available since v0.9.5.
pub struct ScopedRegistry<'a> {
    registry: &'a Registry,
    prefix: String,
}

impl<'a> ScopedRegistry<'a> {
    /// Borrow the prefix this scope prepends.
    #[must_use]
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Borrow the underlying [`Registry`].
    #[must_use]
    pub fn registry(&self) -> &'a Registry {
        self.registry
    }

    #[inline]
    fn join(&self, name: &str) -> String {
        let mut s = String::with_capacity(self.prefix.len() + name.len());
        s.push_str(&self.prefix);
        s.push_str(name);
        s
    }

    // ----- describe shorthands -----

    /// Describe a counter under the scoped name.
    pub fn describe_counter(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.registry.describe_counter(&self.join(name), help, unit);
    }

    /// Describe a gauge under the scoped name.
    pub fn describe_gauge(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.registry.describe_gauge(&self.join(name), help, unit);
    }

    /// Describe a timer under the scoped name.
    pub fn describe_timer(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.registry.describe_timer(&self.join(name), help, unit);
    }

    /// Describe a rate meter under the scoped name.
    pub fn describe_rate(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.registry.describe_rate(&self.join(name), help, unit);
    }

    /// Describe a histogram under the scoped name.
    pub fn describe_histogram(
        &self,
        name: &str,
        help: impl Into<std::borrow::Cow<'static, str>>,
        unit: Unit,
    ) {
        self.registry
            .describe_histogram(&self.join(name), help, unit);
    }

    /// Pre-configure histogram bucket boundaries under the scoped name.
    /// Requires the `histogram` feature.
    #[cfg(feature = "histogram")]
    pub fn configure_histogram(&self, name: &str, buckets: impl IntoIterator<Item = f64>) {
        self.registry.configure_histogram(&self.join(name), buckets);
    }

    // ----- metric lookups (unlabeled) -----

    /// Counter under the scoped name. Requires `count`.
    #[cfg(feature = "count")]
    pub fn counter(&self, name: &str) -> Arc<Counter> {
        self.registry.get_or_create_counter(&self.join(name))
    }

    /// Gauge under the scoped name. Requires `gauge`.
    #[cfg(feature = "gauge")]
    pub fn gauge(&self, name: &str) -> Arc<Gauge> {
        self.registry.get_or_create_gauge(&self.join(name))
    }

    /// Timer under the scoped name. Requires `timer`.
    #[cfg(feature = "timer")]
    pub fn timer(&self, name: &str) -> Arc<Timer> {
        self.registry.get_or_create_timer(&self.join(name))
    }

    /// Rate meter under the scoped name. Requires `meter`.
    #[cfg(feature = "meter")]
    pub fn rate(&self, name: &str) -> Arc<RateMeter> {
        self.registry.get_or_create_rate_meter(&self.join(name))
    }

    /// Histogram under the scoped name. Requires `histogram`.
    #[cfg(feature = "histogram")]
    pub fn histogram(&self, name: &str) -> Arc<Histogram> {
        self.registry.get_or_create_histogram(&self.join(name))
    }

    // ----- labeled lookups -----

    /// Labeled counter under the scoped name. Requires `count`.
    #[cfg(feature = "count")]
    pub fn counter_with(&self, name: &str, labels: &LabelSet) -> Arc<Counter> {
        self.registry
            .get_or_create_counter_with(&self.join(name), labels)
    }

    /// Labeled gauge under the scoped name. Requires `gauge`.
    #[cfg(feature = "gauge")]
    pub fn gauge_with(&self, name: &str, labels: &LabelSet) -> Arc<Gauge> {
        self.registry
            .get_or_create_gauge_with(&self.join(name), labels)
    }

    /// Labeled timer under the scoped name. Requires `timer`.
    #[cfg(feature = "timer")]
    pub fn timer_with(&self, name: &str, labels: &LabelSet) -> Arc<Timer> {
        self.registry
            .get_or_create_timer_with(&self.join(name), labels)
    }

    /// Labeled rate meter under the scoped name. Requires `meter`.
    #[cfg(feature = "meter")]
    pub fn rate_with(&self, name: &str, labels: &LabelSet) -> Arc<RateMeter> {
        self.registry
            .get_or_create_rate_meter_with(&self.join(name), labels)
    }

    /// Labeled histogram under the scoped name. Requires `histogram`.
    #[cfg(feature = "histogram")]
    pub fn histogram_with(&self, name: &str, labels: &LabelSet) -> Arc<Histogram> {
        self.registry
            .get_or_create_histogram_with(&self.join(name), labels)
    }

    /// Nested scope: `scoped("a.").scoped("b.")` is equivalent to
    /// `scoped("a.b.")`. Allocates a single fresh `String`.
    #[must_use]
    pub fn scoped(&self, sub_prefix: impl Into<String>) -> ScopedRegistry<'a> {
        ScopedRegistry {
            registry: self.registry,
            prefix: self.join(&sub_prefix.into()),
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------
// Per-type process-global overflow sinks.
//
// When cardinality is exceeded, the non-`try` `*_with` methods route to these
// process-global metrics so the caller's hot path never panics. These sinks
// are *not* registered in the Registry and are *not* exported. The only way
// to observe cardinality pressure is via `Registry::cardinality_overflows()`.
// ---------------------------------------------------------------------

#[cfg(feature = "count")]
fn counter_overflow_sink() -> &'static Arc<Counter> {
    static SINK: OnceLock<Arc<Counter>> = OnceLock::new();
    SINK.get_or_init(|| Arc::new(Counter::new()))
}

#[cfg(feature = "gauge")]
fn gauge_overflow_sink() -> &'static Arc<Gauge> {
    static SINK: OnceLock<Arc<Gauge>> = OnceLock::new();
    SINK.get_or_init(|| Arc::new(Gauge::new()))
}

#[cfg(feature = "timer")]
fn timer_overflow_sink() -> &'static Arc<Timer> {
    static SINK: OnceLock<Arc<Timer>> = OnceLock::new();
    SINK.get_or_init(|| Arc::new(Timer::new()))
}

#[cfg(feature = "meter")]
fn rate_meter_overflow_sink() -> &'static Arc<RateMeter> {
    static SINK: OnceLock<Arc<RateMeter>> = OnceLock::new();
    SINK.get_or_init(|| Arc::new(RateMeter::new()))
}

#[cfg(feature = "histogram")]
fn histogram_overflow_sink() -> &'static Arc<Histogram> {
    static SINK: OnceLock<Arc<Histogram>> = OnceLock::new();
    SINK.get_or_init(|| Arc::new(Histogram::default_seconds()))
}

// `Registry` is Send + Sync automatically because every field is Send + Sync.
// No unsafe impls required.

#[cfg(test)]
#[cfg(all(feature = "count", feature = "gauge", feature = "timer"))]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_counter_registration() {
        let registry = Registry::new();
        let c1 = registry.get_or_create_counter("requests");
        let c2 = registry.get_or_create_counter("requests");
        assert!(Arc::ptr_eq(&c1, &c2));
    }

    #[test]
    fn test_gauge_registration() {
        let registry = Registry::new();
        let g1 = registry.get_or_create_gauge("cpu_usage");
        let g2 = registry.get_or_create_gauge("cpu_usage");
        assert!(Arc::ptr_eq(&g1, &g2));
    }

    #[test]
    fn test_timer_registration() {
        let registry = Registry::new();
        let t1 = registry.get_or_create_timer("db_query");
        let t2 = registry.get_or_create_timer("db_query");
        assert!(Arc::ptr_eq(&t1, &t2));
    }

    #[test]
    #[cfg(feature = "meter")]
    fn test_rate_meter_registration() {
        let registry = Registry::new();
        let r1 = registry.get_or_create_rate_meter("api_calls");
        let r2 = registry.get_or_create_rate_meter("api_calls");
        assert!(Arc::ptr_eq(&r1, &r2));
    }

    #[test]
    #[cfg(feature = "meter")]
    fn test_mixed_metrics() {
        let registry = Registry::new();
        let _ = registry.get_or_create_counter("a");
        let _ = registry.get_or_create_gauge("b");
        let _ = registry.get_or_create_timer("c");
        let _ = registry.get_or_create_rate_meter("d");
        assert_eq!(registry.metric_count(), 4);
    }

    #[test]
    fn test_concurrent_access() {
        let registry = Arc::new(Registry::new());
        let mut handles = vec![];
        for _ in 0..10 {
            let r = registry.clone();
            handles.push(thread::spawn(move || {
                let c = r.get_or_create_counter("concurrent_test");
                c.inc();
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(registry.get_or_create_counter("concurrent_test").get(), 10);
    }

    #[test]
    fn test_clear() {
        let registry = Registry::new();
        let _ = registry.get_or_create_counter("a");
        let _ = registry.get_or_create_gauge("b");
        assert_eq!(registry.metric_count(), 2);
        registry.clear();
        assert_eq!(registry.metric_count(), 0);
    }

    #[test]
    fn test_metric_names() {
        let registry = Registry::new();
        let _ = registry.get_or_create_counter("requests");
        let _ = registry.get_or_create_counter("errors");
        let _ = registry.get_or_create_gauge("cpu");
        assert_eq!(registry.counter_names().len(), 2);
        assert_eq!(registry.gauge_names().len(), 1);
    }

    #[test]
    #[cfg(feature = "meter")]
    fn test_duplicate_names_across_types_are_independent() {
        let registry = Registry::new();
        let c = registry.get_or_create_counter("x");
        let g = registry.get_or_create_gauge("x");
        let t = registry.get_or_create_timer("x");
        let r = registry.get_or_create_rate_meter("x");
        let addrs = [
            Arc::as_ptr(&c) as usize,
            Arc::as_ptr(&g) as usize,
            Arc::as_ptr(&t) as usize,
            Arc::as_ptr(&r) as usize,
        ];
        for i in 0..addrs.len() {
            for j in (i + 1)..addrs.len() {
                assert_ne!(addrs[i], addrs[j]);
            }
        }
    }

    #[test]
    fn test_clear_then_recreate_returns_new_instances() {
        let registry = Registry::new();
        let c_before = registry.get_or_create_counter("requests");
        registry.clear();
        let c_after = registry.get_or_create_counter("requests");
        assert!(!Arc::ptr_eq(&c_before, &c_after));
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
            let t = h.join().unwrap();
            assert!(Arc::ptr_eq(&first, &t));
        }
    }

    // ---------- v0.9.3 additions ----------

    #[test]
    fn labeled_counter_distinct_from_unlabeled() {
        let r = Registry::new();
        let plain = r.get_or_create_counter("hits");
        let labels = LabelSet::from([("region", "us")]);
        let labeled = r.get_or_create_counter_with("hits", &labels);
        assert!(!Arc::ptr_eq(&plain, &labeled));
        plain.inc();
        labeled.add(5);
        assert_eq!(plain.get(), 1);
        assert_eq!(labeled.get(), 5);
        assert_eq!(r.cardinality_count(), 1);
    }

    #[test]
    fn empty_labelset_routes_to_unlabeled_fast_path() {
        let r = Registry::new();
        let plain = r.get_or_create_counter("x");
        let same = r.get_or_create_counter_with("x", &LabelSet::EMPTY);
        assert!(Arc::ptr_eq(&plain, &same));
        assert_eq!(r.cardinality_count(), 0);
    }

    #[test]
    fn cardinality_cap_routes_overflows_to_sink() {
        let r = Registry::new();
        r.set_cardinality_cap(2);
        let l1 = LabelSet::from([("k", "1")]);
        let l2 = LabelSet::from([("k", "2")]);
        let l3 = LabelSet::from([("k", "3")]);
        let _ = r.get_or_create_counter_with("c", &l1);
        let _ = r.get_or_create_counter_with("c", &l2);
        // Third registration overflows.
        let over = r.get_or_create_counter_with("c", &l3);
        let sink = counter_overflow_sink();
        assert!(Arc::ptr_eq(&over, sink));
        assert_eq!(r.cardinality_count(), 2);
        assert!(r.cardinality_overflows() >= 1);
    }

    #[test]
    fn try_cardinality_cap_returns_error() {
        let r = Registry::new();
        r.set_cardinality_cap(1);
        let _ = r
            .try_get_or_create_counter_with("c", &LabelSet::from([("k", "1")]))
            .unwrap();
        let err = r
            .try_get_or_create_counter_with("c", &LabelSet::from([("k", "2")]))
            .unwrap_err();
        assert_eq!(err, MetricsError::CardinalityExceeded);
    }

    #[test]
    fn metadata_roundtrip() {
        let r = Registry::new();
        r.describe_counter("requests", "Total HTTP requests", Unit::Custom("requests"));
        let meta = r.metadata("requests").unwrap();
        assert_eq!(meta.kind, MetricKind::Counter);
        assert_eq!(meta.help.as_ref(), "Total HTTP requests");
        assert_eq!(meta.unit, Unit::Custom("requests"));
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn histogram_uses_configured_buckets() {
        let r = Registry::new();
        r.configure_histogram("latency", [0.1, 0.5, 1.0]);
        let h = r.get_or_create_histogram("latency");
        // 3 explicit + implicit +Inf = 4.
        let snap = h.snapshot();
        assert_eq!(snap.buckets.len(), 4);
    }

    // ---------- Coverage-completing tests for every labeled metric type ----------

    #[test]
    fn labeled_gauge_distinct_from_unlabeled_and_caps() {
        let r = Registry::new();
        let plain = r.get_or_create_gauge("temp");
        let labels = LabelSet::from([("zone", "a")]);
        let labeled = r.get_or_create_gauge_with("temp", &labels);
        assert!(!Arc::ptr_eq(&plain, &labeled));
        plain.set(1.0);
        labeled.set(2.0);
        assert_eq!(plain.get(), 1.0);
        assert_eq!(labeled.get(), 2.0);

        // Empty label set hits the fast path.
        let same = r.get_or_create_gauge_with("temp", &LabelSet::EMPTY);
        assert!(Arc::ptr_eq(&plain, &same));

        // Try path returns CardinalityExceeded on full cap.
        r.set_cardinality_cap(1);
        assert!(r
            .try_get_or_create_gauge_with("temp", &LabelSet::from([("zone", "b")]))
            .is_err());
        // The non-try path routes to the per-type sink without panicking.
        let _ = r.get_or_create_gauge_with("temp", &LabelSet::from([("zone", "c")]));
        assert!(r.cardinality_overflows() >= 1);
    }

    #[test]
    fn labeled_timer_distinct_from_unlabeled_and_caps() {
        let r = Registry::new();
        let plain = r.get_or_create_timer("rpc");
        let labeled = r.get_or_create_timer_with("rpc", &LabelSet::from([("op", "send")]));
        assert!(!Arc::ptr_eq(&plain, &labeled));
        plain.record(std::time::Duration::from_micros(50));
        labeled.record(std::time::Duration::from_micros(100));
        assert_eq!(plain.count(), 1);
        assert_eq!(labeled.count(), 1);
        let same = r.get_or_create_timer_with("rpc", &LabelSet::EMPTY);
        assert!(Arc::ptr_eq(&plain, &same));

        r.set_cardinality_cap(1);
        assert!(r
            .try_get_or_create_timer_with("rpc", &LabelSet::from([("op", "recv")]))
            .is_err());
        let _ = r.get_or_create_timer_with("rpc", &LabelSet::from([("op", "ack")]));
        assert!(r.cardinality_overflows() >= 1);
    }

    #[test]
    #[cfg(feature = "meter")]
    fn labeled_rate_meter_distinct_from_unlabeled_and_caps() {
        let r = Registry::new();
        let plain = r.get_or_create_rate_meter("qps");
        let labeled = r.get_or_create_rate_meter_with("qps", &LabelSet::from([("tier", "1")]));
        assert!(!Arc::ptr_eq(&plain, &labeled));
        plain.tick_n(3);
        labeled.tick_n(7);
        assert_eq!(plain.total(), 3);
        assert_eq!(labeled.total(), 7);
        let same = r.get_or_create_rate_meter_with("qps", &LabelSet::EMPTY);
        assert!(Arc::ptr_eq(&plain, &same));

        r.set_cardinality_cap(1);
        assert!(r
            .try_get_or_create_rate_meter_with("qps", &LabelSet::from([("tier", "2")]))
            .is_err());
        let _ = r.get_or_create_rate_meter_with("qps", &LabelSet::from([("tier", "3")]));
        assert!(r.cardinality_overflows() >= 1);
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn labeled_histogram_caps_and_observes() {
        let r = Registry::new();
        r.configure_histogram("latency", [0.01, 0.1, 1.0]);
        let h = r.get_or_create_histogram_with("latency", &LabelSet::from([("op", "a")]));
        h.observe(0.005);
        assert_eq!(h.count(), 1);

        r.set_cardinality_cap(1);
        // Already-registered labels should still resolve to the existing Arc.
        let h2 = r.get_or_create_histogram_with("latency", &LabelSet::from([("op", "a")]));
        assert!(Arc::ptr_eq(&h, &h2));

        // New labeled key hits the cap.
        let err = r
            .try_get_or_create_histogram_with("latency", &LabelSet::from([("op", "b")]))
            .unwrap_err();
        assert_eq!(err, MetricsError::CardinalityExceeded);
        let _sink = r.get_or_create_histogram_with("latency", &LabelSet::from([("op", "c")]));
        assert!(r.cardinality_overflows() >= 1);
    }

    #[test]
    fn describe_shorthands_cover_every_kind() {
        let r = Registry::new();
        r.describe_counter("c", "counter help", Unit::Custom("1"));
        r.describe_gauge("g", "gauge help", Unit::Bytes);
        r.describe_timer("t", "timer help", Unit::Seconds);
        #[cfg(feature = "meter")]
        r.describe_rate("rt", "rate help", Unit::Custom("ops"));
        #[cfg(feature = "histogram")]
        r.describe_histogram("h", "histogram help", Unit::Milliseconds);

        assert_eq!(r.metadata("c").unwrap().kind, MetricKind::Counter);
        assert_eq!(r.metadata("g").unwrap().kind, MetricKind::Gauge);
        assert_eq!(r.metadata("t").unwrap().kind, MetricKind::Timer);
        #[cfg(feature = "meter")]
        assert_eq!(r.metadata("rt").unwrap().kind, MetricKind::Rate);
        #[cfg(feature = "histogram")]
        assert_eq!(r.metadata("h").unwrap().kind, MetricKind::Histogram);

        // Re-describe replaces.
        r.describe_counter("c", "new help", Unit::None);
        assert_eq!(r.metadata("c").unwrap().help.as_ref(), "new help");
    }

    #[test]
    fn snapshot_accessors_include_labeled_entries() {
        let r = Registry::new();
        r.get_or_create_counter("c1").inc();
        r.get_or_create_counter_with("c2", &LabelSet::from([("k", "v")]))
            .inc();
        r.get_or_create_gauge("g1").set(1.0);
        r.get_or_create_gauge_with("g2", &LabelSet::from([("k", "v")]))
            .set(2.0);
        r.get_or_create_timer("t1")
            .record(std::time::Duration::from_micros(1));
        r.get_or_create_timer_with("t2", &LabelSet::from([("k", "v")]))
            .record(std::time::Duration::from_micros(2));

        assert_eq!(r.counter_entries().len(), 2);
        assert_eq!(r.gauge_entries().len(), 2);
        assert_eq!(r.timer_entries().len(), 2);

        #[cfg(feature = "meter")]
        {
            r.get_or_create_rate_meter("r1").tick();
            r.get_or_create_rate_meter_with("r2", &LabelSet::from([("k", "v")]))
                .tick();
            assert_eq!(r.rate_meter_entries().len(), 2);
        }

        #[cfg(feature = "histogram")]
        {
            r.get_or_create_histogram("h1").observe(0.1);
            r.get_or_create_histogram_with("h2", &LabelSet::from([("k", "v")]))
                .observe(0.1);
            assert_eq!(r.histogram_entries().len(), 2);
            let names = r.histogram_names();
            assert!(names.contains(&"h1".to_string()));
            assert!(names.contains(&"h2".to_string()));
        }
    }

    #[test]
    fn cardinality_count_is_reset_by_clear_but_overflows_are_monotonic() {
        let r = Registry::new();
        r.set_cardinality_cap(2);
        let _ = r.get_or_create_counter_with("c", &LabelSet::from([("k", "1")]));
        let _ = r.get_or_create_counter_with("c", &LabelSet::from([("k", "2")]));
        let _ = r.get_or_create_counter_with("c", &LabelSet::from([("k", "3")])); // overflow
        assert_eq!(r.cardinality_count(), 2);
        assert!(r.cardinality_overflows() >= 1);

        let prior_overflows = r.cardinality_overflows();
        r.clear();
        assert_eq!(r.cardinality_count(), 0);
        // overflows is monotonic across clears.
        assert_eq!(r.cardinality_overflows(), prior_overflows);
    }

    #[test]
    fn cap_settings_round_trip() {
        let r = Registry::new();
        let default_cap = r.cardinality_cap();
        assert_eq!(default_cap, DEFAULT_CARDINALITY_CAP);
        r.set_cardinality_cap(42);
        assert_eq!(r.cardinality_cap(), 42);
        // Setting a cap below current count doesn't reset the count.
        let _ = r.get_or_create_counter_with("c", &LabelSet::from([("k", "v")]));
        r.set_cardinality_cap(0);
        assert_eq!(r.cardinality_cap(), 0);
        assert_eq!(r.cardinality_count(), 1);
        // But further labeled registrations now overflow.
        let _ = r.get_or_create_counter_with("c", &LabelSet::from([("k", "v2")]));
        assert!(r.cardinality_overflows() >= 1);
    }

    // ---------- v0.9.5 additions: ScopedRegistry ----------

    #[test]
    fn scoped_registry_prepends_prefix() {
        let r = Registry::new();
        let http = r.scoped("http.");
        http.counter("requests").inc();
        // Same Arc resolves both via the scope and via the underlying name.
        let c_scope = http.counter("requests");
        let c_full = r.get_or_create_counter("http.requests");
        assert!(Arc::ptr_eq(&c_scope, &c_full));
        assert_eq!(c_full.get(), 1);
    }

    #[test]
    fn scoped_registry_describe_lands_under_prefixed_name() {
        let r = Registry::new();
        let s = r.scoped("svc.");
        s.describe_counter("hits", "Total hits", Unit::Custom("1"));
        assert!(r.metadata("svc.hits").is_some());
        assert!(r.metadata("hits").is_none());
    }

    #[test]
    #[cfg(feature = "meter")]
    fn scoped_registry_supports_every_metric_type() {
        let r = Registry::new();
        let s = r.scoped("zone_a.");
        s.counter("c").inc();
        s.gauge("g").set(1.0);
        s.timer("t").record(std::time::Duration::from_micros(1));
        s.rate("r").tick();
        assert_eq!(r.get_or_create_counter("zone_a.c").get(), 1);
        assert_eq!(r.get_or_create_gauge("zone_a.g").get(), 1.0);
        assert_eq!(r.get_or_create_timer("zone_a.t").count(), 1);
        assert_eq!(r.get_or_create_rate_meter("zone_a.r").total(), 1);
    }

    #[test]
    fn scoped_registry_labeled_routes_through_cardinality_cap() {
        let r = Registry::new();
        r.set_cardinality_cap(2);
        let s = r.scoped("api.");
        let _ = s.counter_with("hits", &LabelSet::from([("k", "1")]));
        let _ = s.counter_with("hits", &LabelSet::from([("k", "2")]));
        // Third labeled key overflows.
        let _ = s.counter_with("hits", &LabelSet::from([("k", "3")]));
        assert!(r.cardinality_overflows() >= 1);
    }

    #[test]
    fn nested_scopes_compose_prefixes() {
        let r = Registry::new();
        let a = r.scoped("a.");
        let ab = a.scoped("b.");
        ab.counter("x").inc();
        assert_eq!(r.get_or_create_counter("a.b.x").get(), 1);
        assert_eq!(ab.prefix(), "a.b.");
    }

    #[test]
    #[cfg(feature = "histogram")]
    fn scoped_registry_histogram_uses_configured_buckets() {
        let r = Registry::new();
        let s = r.scoped("rtt.");
        s.configure_histogram("seconds", [0.01, 0.1, 1.0]);
        let h = s.histogram("seconds");
        // 3 explicit + +Inf = 4
        assert_eq!(h.snapshot().buckets.len(), 4);
    }
}
