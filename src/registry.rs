//! Thread-safe, bounded registry of metric families.
//!
//! # Model
//!
//! A *family* is everything registered under one name: its kind, optional
//! help text and unit, optional bucket/quantile configuration, and its
//! *series* — one metric instance per distinct [`LabelSet`] (the empty set
//! is the unlabeled series). One name maps to exactly one kind; registering
//! `"jobs"` as a counter and then as a gauge is refused with
//! [`MetricsError::KindMismatch`], because the two could not share one
//! exposition family.
//!
//! # Concurrency
//!
//! Families live in 16 shards, each a `RwLock<HashMap>`, chosen by a cheap
//! hash of the name, so registrations of different names rarely contend. A
//! lookup of an existing series takes one shard read lock and two hash-map
//! lookups and does **not** allocate, for labeled and unlabeled series alike.
//! Creating a series takes the shard's write lock. For the hottest paths,
//! keep the returned `Arc` and skip the lookup entirely.
//!
//! # Limits
//!
//! - **Series cap** ([`Registry::set_cardinality_cap`], default
//!   [`DEFAULT_CARDINALITY_CAP`]): the total number of series, labeled and
//!   unlabeled, plus families that only carry metadata. Past the cap the
//!   `try_*` lookups return [`MetricsError::CardinalityExceeded`] and the
//!   other lookups hand out a detached *overflow* metric (shared per type,
//!   never exported) and bump [`Registry::cardinality_overflows`].
//! - **Validation**: names and labels are checked (see
//!   [`validate_metric_name`](crate::validate_metric_name)). The `try_*`
//!   lookups return the error; the others repair the input and bump
//!   [`Registry::invalid_inputs`].
//! - **Removal and expiry**: [`Registry::remove`],
//!   [`Registry::remove_series`] and [`Registry::evict_idle`] free series and
//!   their cap slots.
//!
//! # Example
//!
//! ```
//! # #[cfg(all(feature = "count", feature = "gauge"))]
//! # {
//! use metrics_lib::{LabelSet, MetricsError, Registry};
//!
//! let registry = Registry::new();
//! registry.counter("requests").inc();
//! registry
//!     .counter_with("requests", &LabelSet::from([("status", "500")]))
//!     .inc();
//! assert_eq!(registry.metric_count(), 2);
//!
//! // One name, one kind.
//! assert!(matches!(
//!     registry.try_gauge("requests"),
//!     Err(MetricsError::KindMismatch { .. })
//! ));
//! # }
//! ```

use crate::name::{sanitize_metric_name, validate_metric_name};
use crate::{LabelSet, MetricKind, MetricMetadata, MetricsError, Result, Unit};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[cfg(feature = "gauge")]
use crate::Gauge;
#[cfg(feature = "meter")]
use crate::RateMeter;
#[cfg(feature = "timer")]
use crate::Timer;
#[cfg(feature = "count")]
use crate::{Counter, ShardedCounter, UpDownCounter};
#[cfg(feature = "histogram")]
use crate::{Histogram, Summary, SummaryConfig};

/// Default cap on the number of series a [`Registry`] holds.
pub const DEFAULT_CARDINALITY_CAP: usize = 10_000;

const SHARDS: usize = 16;

/// Label set of every unlabeled series, as a `'static` reference.
static EMPTY_LABELS: LabelSet = LabelSet::EMPTY;

/// How a family stores its series. Distinguishes types that share a
/// [`MetricKind`] (a `Counter` and a `ShardedCounter` are both counters but
/// cannot share a family).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Storage {
    #[cfg(feature = "count")]
    Counter,
    #[cfg(feature = "count")]
    ShardedCounter,
    #[cfg(feature = "count")]
    UpDownCounter,
    #[cfg(feature = "gauge")]
    Gauge,
    #[cfg(feature = "timer")]
    Timer,
    #[cfg(feature = "meter")]
    Rate,
    #[cfg(feature = "histogram")]
    Histogram,
    #[cfg(feature = "histogram")]
    Summary,
}

/// A registered metric instance.
#[derive(Clone)]
pub(crate) enum Handle {
    #[cfg(feature = "count")]
    Counter(Arc<Counter>),
    #[cfg(feature = "count")]
    ShardedCounter(Arc<ShardedCounter>),
    #[cfg(feature = "count")]
    UpDownCounter(Arc<UpDownCounter>),
    #[cfg(feature = "gauge")]
    Gauge(Arc<Gauge>),
    #[cfg(feature = "timer")]
    Timer(Arc<Timer>),
    #[cfg(feature = "meter")]
    Rate(Arc<RateMeter>),
    #[cfg(feature = "histogram")]
    Histogram(Arc<Histogram>),
    #[cfg(feature = "histogram")]
    Summary(Arc<Summary>),
}

impl Handle {
    /// Cheap value fingerprint used for idle detection: changes whenever
    /// the metric records something.
    #[allow(clippy::cast_sign_loss)] // a fingerprint only needs to change, not to be meaningful
    fn activity(&self) -> u64 {
        match self {
            #[cfg(feature = "count")]
            Handle::Counter(c) => c.get(),
            #[cfg(feature = "count")]
            Handle::ShardedCounter(c) => c.get(),
            #[cfg(feature = "count")]
            Handle::UpDownCounter(c) => c.get() as u64,
            #[cfg(feature = "gauge")]
            Handle::Gauge(g) => g.get().to_bits(),
            #[cfg(feature = "timer")]
            Handle::Timer(t) => t.count(),
            #[cfg(feature = "meter")]
            Handle::Rate(r) => r.total(),
            #[cfg(feature = "histogram")]
            Handle::Histogram(h) => h.count(),
            #[cfg(feature = "histogram")]
            Handle::Summary(s) => s.count(),
            #[cfg(not(any(
                feature = "count",
                feature = "gauge",
                feature = "timer",
                feature = "meter",
                feature = "histogram"
            )))]
            _ => match *self {},
        }
    }

    /// Number of strong references (1 = only the registry holds it).
    fn strong_count(&self) -> usize {
        match self {
            #[cfg(feature = "count")]
            Handle::Counter(a) => Arc::strong_count(a),
            #[cfg(feature = "count")]
            Handle::ShardedCounter(a) => Arc::strong_count(a),
            #[cfg(feature = "count")]
            Handle::UpDownCounter(a) => Arc::strong_count(a),
            #[cfg(feature = "gauge")]
            Handle::Gauge(a) => Arc::strong_count(a),
            #[cfg(feature = "timer")]
            Handle::Timer(a) => Arc::strong_count(a),
            #[cfg(feature = "meter")]
            Handle::Rate(a) => Arc::strong_count(a),
            #[cfg(feature = "histogram")]
            Handle::Histogram(a) => Arc::strong_count(a),
            #[cfg(feature = "histogram")]
            Handle::Summary(a) => Arc::strong_count(a),
            #[cfg(not(any(
                feature = "count",
                feature = "gauge",
                feature = "timer",
                feature = "meter",
                feature = "histogram"
            )))]
            _ => match *self {},
        }
    }
}

/// Glue between a metric type and the registry.
pub(crate) trait RegistryMetric: Sized + Send + Sync + 'static {
    const STORAGE: Storage;
    const KIND: MetricKind;
    /// Label key this kind reserves for its own exposition (`le`, `quantile`).
    const RESERVED_LABEL: Option<&'static str> = None;
    fn create(family: &Family) -> Self;
    fn wrap(metric: Arc<Self>) -> Handle;
    fn unwrap(handle: &Handle) -> Option<&Arc<Self>>;
    fn overflow_sink() -> &'static Arc<Self>;
}

macro_rules! registry_metric {
    ($feature:literal, $ty:ty, $variant:ident, $kind:expr, $reserved:expr, |$fam:ident| $create:block) => {
        #[cfg(feature = $feature)]
        impl RegistryMetric for $ty {
            const STORAGE: Storage = Storage::$variant;
            const KIND: MetricKind = $kind;
            const RESERVED_LABEL: Option<&'static str> = $reserved;
            #[allow(unused_variables)]
            fn create($fam: &Family) -> Self $create
            fn wrap(metric: Arc<Self>) -> Handle {
                Handle::$variant(metric)
            }
            #[allow(unreachable_patterns)]
            #[inline]
            fn unwrap(handle: &Handle) -> Option<&Arc<Self>> {
                match handle {
                    Handle::$variant(m) => Some(m),
                    _ => None,
                }
            }
            fn overflow_sink() -> &'static Arc<Self> {
                static SINK: OnceLock<Arc<$ty>> = OnceLock::new();
                SINK.get_or_init(|| Arc::new(<$ty as RegistryMetric>::create(&Family::detached())))
            }
        }
    };
}

registry_metric!("count", Counter, Counter, MetricKind::Counter, None, |f| {
    Counter::new()
});
registry_metric!(
    "count",
    ShardedCounter,
    ShardedCounter,
    MetricKind::Counter,
    None,
    |f| { ShardedCounter::new() }
);
registry_metric!(
    "count",
    UpDownCounter,
    UpDownCounter,
    MetricKind::UpDownCounter,
    None,
    |f| { UpDownCounter::new() }
);
registry_metric!("gauge", Gauge, Gauge, MetricKind::Gauge, None, |f| {
    Gauge::new()
});
registry_metric!(
    "timer",
    Timer,
    Timer,
    MetricKind::Timer,
    Some("quantile"),
    |f| { Timer::new() }
);
registry_metric!("meter", RateMeter, Rate, MetricKind::Rate, None, |f| {
    RateMeter::new()
});
registry_metric!(
    "histogram",
    Histogram,
    Histogram,
    MetricKind::Histogram,
    Some("le"),
    |f| {
        match &f.buckets {
            Some(bounds) => Histogram::with_buckets(bounds.iter().copied()),
            None => Histogram::default_seconds(),
        }
    }
);
registry_metric!(
    "histogram",
    Summary,
    Summary,
    MetricKind::Summary,
    Some("quantile"),
    |f| {
        f.summary
            .as_ref()
            .and_then(|cfg| Summary::with_config(SummaryConfig::clone(cfg)).ok())
            .unwrap_or_default()
    }
);

/// Per-series bookkeeping.
pub(crate) struct Series {
    pub(crate) handle: Handle,
    /// Wall-clock creation time (Unix nanoseconds), the OTLP start time.
    pub(crate) created_unix_nanos: u64,
    fingerprint: AtomicU64,
    /// Nanoseconds since the registry epoch of the last observed change.
    last_change_ns: AtomicU64,
}

impl Series {
    fn new(handle: Handle, now_ns: u64) -> Self {
        let fingerprint = handle.activity();
        Self {
            handle,
            created_unix_nanos: unix_nanos_now(),
            fingerprint: AtomicU64::new(fingerprint),
            last_change_ns: AtomicU64::new(now_ns),
        }
    }

    /// Refresh the idle bookkeeping; returns how long the series has been
    /// unchanged.
    fn idle_for(&self, now_ns: u64) -> u64 {
        let current = self.handle.activity();
        if self.fingerprint.swap(current, Ordering::Relaxed) != current {
            self.last_change_ns.store(now_ns, Ordering::Relaxed);
            return 0;
        }
        now_ns.saturating_sub(self.last_change_ns.load(Ordering::Relaxed))
    }
}

/// Everything registered under one name.
pub(crate) struct Family {
    pub(crate) kind: MetricKind,
    storage: Option<Storage>,
    pub(crate) help: Option<Cow<'static, str>>,
    pub(crate) unit: Unit,
    #[cfg(feature = "histogram")]
    buckets: Option<Arc<[f64]>>,
    #[cfg(feature = "histogram")]
    summary: Option<Arc<SummaryConfig>>,
    unlabeled: Option<Series>,
    labeled: HashMap<LabelSet, Series>,
}

impl Family {
    fn new(kind: MetricKind) -> Self {
        Self {
            kind,
            storage: None,
            help: None,
            unit: Unit::None,
            #[cfg(feature = "histogram")]
            buckets: None,
            #[cfg(feature = "histogram")]
            summary: None,
            unlabeled: None,
            labeled: HashMap::new(),
        }
    }

    /// Configuration-free family used to build overflow sinks.
    fn detached() -> Self {
        Self::new(MetricKind::Counter)
    }

    fn series_count(&self) -> usize {
        self.labeled.len() + usize::from(self.unlabeled.is_some())
    }

    /// Cap slots this family occupies: one per series, and one for a family
    /// that only holds metadata or configuration.
    fn slots(&self) -> usize {
        self.series_count().max(1)
    }

    fn has_config(&self) -> bool {
        #[cfg(feature = "histogram")]
        if self.buckets.is_some() || self.summary.is_some() {
            return true;
        }
        self.help.is_some()
    }

    fn get(&self, labels: &LabelSet) -> Option<&Series> {
        if labels.is_empty() {
            self.unlabeled.as_ref()
        } else {
            self.labeled.get(labels)
        }
    }

    fn insert(&mut self, labels: LabelSet, series: Series) {
        if labels.is_empty() {
            self.unlabeled = Some(series);
        } else {
            let _ = self.labeled.insert(labels, series);
        }
    }

    fn remove(&mut self, labels: &LabelSet) -> bool {
        if labels.is_empty() {
            self.unlabeled.take().is_some()
        } else {
            self.labeled.remove(labels).is_some()
        }
    }

    fn series(&self) -> impl Iterator<Item = (&LabelSet, &Series)> {
        self.unlabeled
            .iter()
            .map(|s| (&EMPTY_LABELS, s))
            .chain(self.labeled.iter())
    }

    fn compatible(&self, kind: MetricKind, storage: Option<Storage>) -> bool {
        self.kind == kind
            && (storage.is_none() || self.storage.is_none() || self.storage == storage)
    }
}

/// One family as seen by an exporter: handles are cloned out of the lock,
/// values are read later.
pub(crate) struct FamilySnapshot {
    pub(crate) name: String,
    pub(crate) kind: MetricKind,
    pub(crate) help: Option<Cow<'static, str>>,
    pub(crate) unit: Unit,
    pub(crate) series: Vec<SeriesSnapshot>,
}

pub(crate) struct SeriesSnapshot {
    pub(crate) labels: LabelSet,
    pub(crate) handle: Handle,
    /// Only the OTLP exporter reports series start times.
    #[cfg_attr(not(feature = "otlp"), allow(dead_code))]
    pub(crate) created_unix_nanos: u64,
}

/// Whether a lookup should refuse or repair bad input.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Strict,
    Repair,
}

/// Thread-safe, sharded, bounded registry of metric families.
///
/// See the [module documentation](self) for the model and the limits.
pub struct Registry {
    shards: Box<[RwLock<HashMap<String, Family>>]>,
    cap: AtomicUsize,
    used: AtomicUsize,
    overflows: AtomicU64,
    invalid_inputs: AtomicU64,
    kind_conflicts: AtomicU64,
    exposition_conflicts: AtomicU64,
    epoch: Instant,
}

fn unix_nanos_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
}

/// Constant-time shard selection from the length and four sampled bytes of
/// the name (no copies, no calls). Shard quality only affects contention,
/// never correctness.
#[inline]
fn shard_index(name: &str) -> usize {
    let bytes = name.as_bytes();
    let len = bytes.len();
    let at = |i: usize| u64::from(bytes.get(i).copied().unwrap_or(0));
    let sampled = at(0)
        | (at(len / 2) << 8)
        | (at(len.wrapping_sub(2)) << 16)
        | (at(len.wrapping_sub(1)) << 24)
        | ((len as u64) << 32);
    let mixed = sampled.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    #[allow(clippy::cast_possible_truncation)] // only the top bits are used
    let index = (mixed >> 60) as usize;
    index & (SHARDS - 1)
}

impl Registry {
    /// Create an empty registry with the default series cap.
    #[must_use]
    pub fn new() -> Self {
        Self::with_cardinality_cap(DEFAULT_CARDINALITY_CAP)
    }

    /// Create an empty registry holding at most `cap` series.
    #[must_use]
    pub fn with_cardinality_cap(cap: usize) -> Self {
        Self {
            shards: (0..SHARDS).map(|_| RwLock::new(HashMap::new())).collect(),
            cap: AtomicUsize::new(cap),
            used: AtomicUsize::new(0),
            overflows: AtomicU64::new(0),
            invalid_inputs: AtomicU64::new(0),
            kind_conflicts: AtomicU64::new(0),
            exposition_conflicts: AtomicU64::new(0),
            epoch: Instant::now(),
        }
    }

    #[inline]
    fn read_shard(&self, name: &str) -> RwLockReadGuard<'_, HashMap<String, Family>> {
        self.shards[shard_index(name)]
            .read()
            .unwrap_or_else(PoisonError::into_inner)
    }

    #[inline]
    fn write_shard(&self, name: &str) -> RwLockWriteGuard<'_, HashMap<String, Family>> {
        self.shards[shard_index(name)]
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn now_ns(&self) -> u64 {
        crate::util::duration_to_nanos(self.epoch.elapsed())
    }

    // ------------------------------------------------------------------
    // Limits and diagnostics
    // ------------------------------------------------------------------

    /// Set the series cap. Lowering it below the current count does not
    /// remove anything; it only blocks new series.
    pub fn set_cardinality_cap(&self, cap: usize) {
        self.cap.store(cap, Ordering::Relaxed);
    }

    /// Current series cap.
    #[must_use]
    pub fn cardinality_cap(&self) -> usize {
        self.cap.load(Ordering::Relaxed)
    }

    /// Cap slots in use: one per series plus one per family that only holds
    /// metadata or configuration.
    #[must_use]
    pub fn cardinality_count(&self) -> usize {
        self.used.load(Ordering::Relaxed)
    }

    /// Lookups refused (or sent to the overflow metric) because the cap was
    /// full. Monotonic; not reset by [`Registry::clear`].
    #[must_use]
    pub fn cardinality_overflows(&self) -> u64 {
        self.overflows.load(Ordering::Relaxed)
    }

    /// Unchecked lookups whose name or labels had to be repaired. Monotonic.
    #[must_use]
    pub fn invalid_inputs(&self) -> u64 {
        self.invalid_inputs.load(Ordering::Relaxed)
    }

    /// Unchecked lookups that asked for a different kind than the one
    /// registered under the name, and got a detached metric. Monotonic.
    #[must_use]
    pub fn kind_conflicts(&self) -> u64 {
        self.kind_conflicts.load(Ordering::Relaxed)
    }

    /// Families an exporter skipped because their rendered name collided
    /// with another family's (for example `a.b` and `a_b` both render as
    /// `a_b` in Prometheus). Monotonic.
    #[must_use]
    pub fn exposition_conflicts(&self) -> u64 {
        self.exposition_conflicts.load(Ordering::Relaxed)
    }

    pub(crate) fn note_exposition_conflict(&self) {
        let _ = self.exposition_conflicts.fetch_add(1, Ordering::Relaxed);
    }

    fn try_reserve(&self, slots: usize) -> Result<()> {
        if slots == 0 {
            return Ok(());
        }
        let cap = self.cardinality_cap();
        self.used
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(slots).filter(|next| *next <= cap)
            })
            .map(|_| ())
            .map_err(|_| {
                let _ = self.overflows.fetch_add(1, Ordering::Relaxed);
                MetricsError::CardinalityExceeded
            })
    }

    fn release(&self, slots: usize) {
        if slots > 0 {
            let _ = self
                .used
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                    Some(used.saturating_sub(slots))
                });
        }
    }

    // ------------------------------------------------------------------
    // Core lookup
    // ------------------------------------------------------------------

    /// Fast path: an existing series of the right type.
    #[inline]
    fn find<T: RegistryMetric>(&self, name: &str, labels: &LabelSet) -> Option<Arc<T>> {
        let shard = self.read_shard(name);
        let series = shard.get(name)?.get(labels)?;
        T::unwrap(&series.handle).cloned()
    }

    #[inline]
    pub(crate) fn lookup<T: RegistryMetric>(&self, name: &str, labels: &LabelSet) -> Arc<T> {
        if let Some(found) = self.find::<T>(name, labels) {
            return found;
        }
        self.resolve::<T>(name, labels, Policy::Repair)
            .unwrap_or_else(|_| Arc::clone(T::overflow_sink()))
    }

    pub(crate) fn try_lookup<T: RegistryMetric>(
        &self,
        name: &str,
        labels: &LabelSet,
    ) -> Result<Arc<T>> {
        if let Some(found) = self.find::<T>(name, labels) {
            return Ok(found);
        }
        self.resolve::<T>(name, labels, Policy::Strict)
    }

    /// Slow path: validate (or repair), then find or create under the write
    /// lock. The cap slot is reserved only after the existence re-check, so
    /// concurrent first registrations of one series consume one slot.
    fn resolve<T: RegistryMetric>(
        &self,
        name: &str,
        labels: &LabelSet,
        policy: Policy,
    ) -> Result<Arc<T>> {
        let mut repaired = false;
        let name: Cow<'_, str> = match validate_metric_name(name) {
            Ok(()) => Cow::Borrowed(name),
            Err(e) if policy == Policy::Strict => return Err(e),
            Err(_) => {
                repaired = true;
                sanitize_metric_name(name)
            }
        };
        let mut labels: Cow<'_, LabelSet> = match labels.validate() {
            Ok(()) => Cow::Borrowed(labels),
            Err(e) if policy == Policy::Strict => return Err(e),
            Err(_) => {
                repaired = true;
                labels.sanitized()
            }
        };
        if let Some(reserved) = T::RESERVED_LABEL {
            if labels.contains_key(reserved) {
                if policy == Policy::Strict {
                    return Err(MetricsError::invalid_label(
                        reserved,
                        "this label key is reserved by the metric's exposition format",
                    ));
                }
                repaired = true;
                let owned = labels.to_mut();
                let value = owned.get(reserved).unwrap_or_default().to_owned();
                let _ = owned.remove(reserved);
                let _ = owned.add(format!("{reserved}_"), value);
            }
        }
        if repaired {
            let _ = self.invalid_inputs.fetch_add(1, Ordering::Relaxed);
            if let Some(found) = self.find::<T>(&name, &labels) {
                return Ok(found);
            }
        }

        let mut shard = self.write_shard(&name);
        if let Some(family) = shard.get(name.as_ref()) {
            if !family.compatible(T::KIND, Some(T::STORAGE)) {
                return Err(self.conflict(&name, family.kind, T::KIND, policy));
            }
            if let Some(series) = family.get(&labels) {
                return match T::unwrap(&series.handle) {
                    Some(m) => Ok(Arc::clone(m)),
                    None => Err(self.conflict(&name, family.kind, T::KIND, policy)),
                };
            }
        }
        let now = self.now_ns();
        let family_exists = shard.contains_key(name.as_ref());
        let before = shard.get(name.as_ref()).map_or(0, Family::slots);
        let after = shard
            .get(name.as_ref())
            .map_or(1, |f| (f.series_count() + 1).max(1));
        self.try_reserve(after - before)?;
        if !family_exists {
            let _ = shard.insert(name.to_string(), Family::new(T::KIND));
        }
        let Some(family) = shard.get_mut(name.as_ref()) else {
            self.release(after - before);
            return Err(MetricsError::Config("registry shard lost a family".into()));
        };
        family.storage = Some(T::STORAGE);
        let metric = Arc::new(T::create(family));
        family.insert(
            labels.into_owned(),
            Series::new(T::wrap(Arc::clone(&metric)), now),
        );
        Ok(metric)
    }

    fn conflict(
        &self,
        name: &str,
        existing: MetricKind,
        requested: MetricKind,
        policy: Policy,
    ) -> MetricsError {
        if policy == Policy::Repair {
            let _ = self.kind_conflicts.fetch_add(1, Ordering::Relaxed);
        }
        MetricsError::KindMismatch {
            name: name.to_owned(),
            existing,
            requested,
        }
    }

    // ------------------------------------------------------------------
    // Metadata and configuration
    // ------------------------------------------------------------------

    /// Attach help text and a unit to the family `name`, declaring its kind.
    ///
    /// Describing a name that is not registered yet reserves it for that
    /// kind (and takes one cap slot until its first series exists).
    /// Describing again replaces the help text and unit.
    ///
    /// # Errors
    ///
    /// - [`MetricsError::InvalidName`] when `name` is invalid.
    /// - [`MetricsError::KindMismatch`] when `name` is registered as a
    ///   different kind.
    /// - [`MetricsError::CardinalityExceeded`] when reserving a new name
    ///   would exceed the cap.
    ///
    /// # Examples
    ///
    /// ```
    /// use metrics_lib::{MetricKind, MetricMetadata, Registry, Unit};
    ///
    /// let registry = Registry::new();
    /// registry
    ///     .describe("jobs_done", MetricMetadata::new(MetricKind::Counter, "Jobs completed", Unit::None))
    ///     .unwrap();
    /// assert_eq!(registry.metadata("jobs_done").unwrap().help, "Jobs completed");
    /// ```
    pub fn describe(&self, name: &str, metadata: MetricMetadata) -> Result<()> {
        self.update_family(name, metadata.kind, |family| {
            family.help = Some(metadata.help);
            family.unit = metadata.unit;
        })
    }

    /// Create or update the family `name` with kind `kind` and apply
    /// `apply` to it.
    fn update_family(
        &self,
        name: &str,
        kind: MetricKind,
        apply: impl FnOnce(&mut Family),
    ) -> Result<()> {
        validate_metric_name(name)?;
        let mut shard = self.write_shard(name);
        if let Some(family) = shard.get_mut(name) {
            if !family.compatible(kind, None) {
                return Err(MetricsError::KindMismatch {
                    name: name.to_owned(),
                    existing: family.kind,
                    requested: kind,
                });
            }
            apply(family);
        } else {
            self.try_reserve(1)?;
            let mut family = Family::new(kind);
            apply(&mut family);
            let _ = shard.insert(name.to_owned(), family);
        }
        Ok(())
    }

    /// Describe a counter. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_counter(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(name, MetricMetadata::new(MetricKind::Counter, help, unit))
    }

    /// Describe an up/down counter. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_up_down_counter(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(
            name,
            MetricMetadata::new(MetricKind::UpDownCounter, help, unit),
        )
    }

    /// Describe a gauge. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_gauge(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(name, MetricMetadata::new(MetricKind::Gauge, help, unit))
    }

    /// Describe a timer. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_timer(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(name, MetricMetadata::new(MetricKind::Timer, help, unit))
    }

    /// Describe a rate meter. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_rate(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(name, MetricMetadata::new(MetricKind::Rate, help, unit))
    }

    /// Describe a histogram. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_histogram(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(name, MetricMetadata::new(MetricKind::Histogram, help, unit))
    }

    /// Describe a summary. See [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_summary(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.describe(name, MetricMetadata::new(MetricKind::Summary, help, unit))
    }

    /// Metadata of `name`, when it was described.
    #[must_use]
    pub fn metadata(&self, name: &str) -> Option<MetricMetadata> {
        let shard = self.read_shard(name);
        let family = shard.get(name)?;
        let help = family.help.clone()?;
        Some(MetricMetadata::new(family.kind, help, family.unit))
    }

    /// The kind registered (or described) under `name`.
    #[must_use]
    pub fn kind_of(&self, name: &str) -> Option<MetricKind> {
        self.read_shard(name).get(name).map(|f| f.kind)
    }

    /// Set the bucket bounds for histograms created under `name` from now
    /// on. Existing series keep their buckets.
    ///
    /// # Errors
    ///
    /// - [`MetricsError::InvalidName`] when `name` is invalid.
    /// - [`MetricsError::InvalidValue`] when no finite bound is given.
    /// - [`MetricsError::KindMismatch`] when `name` is not a histogram.
    /// - [`MetricsError::CardinalityExceeded`] when reserving a new name
    ///   would exceed the cap.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "histogram")]
    /// # {
    /// use metrics_lib::Registry;
    ///
    /// let registry = Registry::new();
    /// registry.configure_histogram("payload_bytes", [128.0, 1024.0, 8192.0]).unwrap();
    /// assert_eq!(registry.histogram("payload_bytes").bounds(), &[128.0, 1024.0, 8192.0]);
    /// assert!(registry.configure_histogram("empty", []).is_err());
    /// # }
    /// ```
    #[cfg(feature = "histogram")]
    pub fn configure_histogram(
        &self,
        name: &str,
        buckets: impl IntoIterator<Item = f64>,
    ) -> Result<()> {
        let bounds: Arc<[f64]> = Histogram::validate_bounds(buckets)?.into();
        self.update_family(name, MetricKind::Histogram, |family| {
            family.buckets = Some(bounds);
        })
    }

    /// Set the sketch configuration for summaries created under `name` from
    /// now on. Existing series keep theirs.
    ///
    /// # Errors
    ///
    /// - [`MetricsError::Config`] when the configuration is invalid.
    /// - Otherwise as [`Registry::configure_histogram`].
    #[cfg(feature = "histogram")]
    pub fn configure_summary(&self, name: &str, config: SummaryConfig) -> Result<()> {
        config.validate()?;
        let config = Arc::new(config);
        self.update_family(name, MetricKind::Summary, |family| {
            family.summary = Some(config);
        })
    }

    // ------------------------------------------------------------------
    // Typed lookups
    // ------------------------------------------------------------------
}

macro_rules! typed_lookups {
    (
        $feature:literal, $ty:ty,
        $plain:ident, $with:ident, $try_plain:ident, $try_with:ident, $entries:ident, $names:ident,
        $what:literal
    ) => {
        #[cfg(feature = $feature)]
        impl Registry {
            #[doc = concat!("Get or create the unlabeled ", $what, " `name`.")]
            ///
            /// Invalid names are repaired and a kind conflict or a full cap
            /// returns a detached overflow metric; use the `try_` variant to
            /// get the error instead.
            #[must_use]
            pub fn $plain(&self, name: &str) -> Arc<$ty> {
                self.lookup::<$ty>(name, &LabelSet::EMPTY)
            }

            #[doc = concat!("Get or create the ", $what, " series `(name, labels)`.")]
            ///
            /// Same fallback rules as the unlabeled lookup.
            #[must_use]
            pub fn $with(&self, name: &str, labels: &LabelSet) -> Arc<$ty> {
                self.lookup::<$ty>(name, labels)
            }

            #[doc = concat!("Checked form of the unlabeled ", $what, " lookup.")]
            ///
            /// # Errors
            ///
            /// [`MetricsError::InvalidName`], [`MetricsError::KindMismatch`]
            /// or [`MetricsError::CardinalityExceeded`].
            pub fn $try_plain(&self, name: &str) -> Result<Arc<$ty>> {
                self.try_lookup::<$ty>(name, &LabelSet::EMPTY)
            }

            #[doc = concat!("Checked form of the labeled ", $what, " lookup.")]
            ///
            /// # Errors
            ///
            /// [`MetricsError::InvalidName`], [`MetricsError::InvalidLabel`],
            /// [`MetricsError::KindMismatch`] or
            /// [`MetricsError::CardinalityExceeded`].
            pub fn $try_with(&self, name: &str, labels: &LabelSet) -> Result<Arc<$ty>> {
                self.try_lookup::<$ty>(name, labels)
            }

            #[doc = concat!("Every ", $what, " series as `(name, labels, metric)`, sorted by name and labels.")]
            #[must_use]
            pub fn $entries(&self) -> Vec<(String, LabelSet, Arc<$ty>)> {
                self.entries::<$ty>()
            }

            #[doc = concat!("Names of every ", $what, " family, sorted.")]
            #[must_use]
            pub fn $names(&self) -> Vec<String> {
                self.names_of(<$ty as RegistryMetric>::STORAGE)
            }
        }
    };
}

typed_lookups!(
    "count",
    Counter,
    counter,
    counter_with,
    try_counter,
    try_counter_with,
    counter_entries,
    counter_names,
    "counter"
);
typed_lookups!(
    "count",
    ShardedCounter,
    sharded_counter,
    sharded_counter_with,
    try_sharded_counter,
    try_sharded_counter_with,
    sharded_counter_entries,
    sharded_counter_names,
    "sharded counter"
);
typed_lookups!(
    "count",
    UpDownCounter,
    up_down_counter,
    up_down_counter_with,
    try_up_down_counter,
    try_up_down_counter_with,
    up_down_counter_entries,
    up_down_counter_names,
    "up/down counter"
);
typed_lookups!(
    "gauge",
    Gauge,
    gauge,
    gauge_with,
    try_gauge,
    try_gauge_with,
    gauge_entries,
    gauge_names,
    "gauge"
);
typed_lookups!(
    "timer",
    Timer,
    timer,
    timer_with,
    try_timer,
    try_timer_with,
    timer_entries,
    timer_names,
    "timer"
);
typed_lookups!(
    "meter",
    RateMeter,
    rate_meter,
    rate_meter_with,
    try_rate_meter,
    try_rate_meter_with,
    rate_meter_entries,
    rate_meter_names,
    "rate meter"
);
typed_lookups!(
    "histogram",
    Histogram,
    histogram,
    histogram_with,
    try_histogram,
    try_histogram_with,
    histogram_entries,
    histogram_names,
    "histogram"
);
typed_lookups!(
    "histogram",
    Summary,
    summary,
    summary_with,
    try_summary,
    try_summary_with,
    summary_entries,
    summary_names,
    "summary"
);

/// The 1.x method names, kept as deprecated aliases so upgrading code keeps
/// compiling. They will be removed in 3.0.
macro_rules! deprecated_aliases {
    (
        $feature:literal, $ty:ty,
        $old:ident => $new:ident,
        $old_with:ident => $new_with:ident,
        $old_try_with:ident => $new_try_with:ident
    ) => {
        #[cfg(feature = $feature)]
        impl Registry {
            #[doc = concat!("Deprecated 1.x name of [`Registry::", stringify!($new), "`].")]
            #[deprecated(since = "2.0.0", note = "renamed in 2.0: drop the `get_or_create_` prefix (`try_get_or_create_x_with` is now `try_x_with`)")]
            #[must_use]
            pub fn $old(&self, name: &str) -> Arc<$ty> {
                self.$new(name)
            }

            #[doc = concat!("Deprecated 1.x name of [`Registry::", stringify!($new_with), "`].")]
            #[deprecated(since = "2.0.0", note = "renamed in 2.0: drop the `get_or_create_` prefix (`try_get_or_create_x_with` is now `try_x_with`)")]
            #[must_use]
            pub fn $old_with(&self, name: &str, labels: &LabelSet) -> Arc<$ty> {
                self.$new_with(name, labels)
            }

            #[doc = concat!("Deprecated 1.x name of [`Registry::", stringify!($new_try_with), "`].")]
            ///
            /// # Errors
            ///
            /// As the new method.
            #[deprecated(since = "2.0.0", note = "renamed in 2.0: drop the `get_or_create_` prefix (`try_get_or_create_x_with` is now `try_x_with`)")]
            pub fn $old_try_with(&self, name: &str, labels: &LabelSet) -> Result<Arc<$ty>> {
                self.$new_try_with(name, labels)
            }
        }
    };
}

deprecated_aliases!("count", Counter,
    get_or_create_counter => counter,
    get_or_create_counter_with => counter_with,
    try_get_or_create_counter_with => try_counter_with);
deprecated_aliases!("gauge", Gauge,
    get_or_create_gauge => gauge,
    get_or_create_gauge_with => gauge_with,
    try_get_or_create_gauge_with => try_gauge_with);
deprecated_aliases!("timer", Timer,
    get_or_create_timer => timer,
    get_or_create_timer_with => timer_with,
    try_get_or_create_timer_with => try_timer_with);
deprecated_aliases!("meter", RateMeter,
    get_or_create_rate_meter => rate_meter,
    get_or_create_rate_meter_with => rate_meter_with,
    try_get_or_create_rate_meter_with => try_rate_meter_with);
deprecated_aliases!("histogram", Histogram,
    get_or_create_histogram => histogram,
    get_or_create_histogram_with => histogram_with,
    try_get_or_create_histogram_with => try_histogram_with);

impl Registry {
    fn entries<T: RegistryMetric>(&self) -> Vec<(String, LabelSet, Arc<T>)> {
        let mut out = Vec::new();
        for shard in &*self.shards {
            let shard = shard.read().unwrap_or_else(PoisonError::into_inner);
            for (name, family) in shard.iter() {
                for (labels, series) in family.series() {
                    if let Some(m) = T::unwrap(&series.handle) {
                        out.push((name.clone(), labels.clone(), Arc::clone(m)));
                    }
                }
            }
        }
        out.sort_by(|a, b| (a.0.as_str(), &a.1).cmp(&(b.0.as_str(), &b.1)));
        out
    }

    fn names_of(&self, storage: Storage) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for shard in &*self.shards {
            let shard = shard.read().unwrap_or_else(PoisonError::into_inner);
            out.extend(
                shard
                    .iter()
                    .filter(|(_, f)| f.storage == Some(storage))
                    .map(|(n, _)| n.clone()),
            );
        }
        out.sort();
        out
    }

    /// Clone every family with at least one series, sorted by name, series
    /// sorted by labels. Locks are held only while cloning handles.
    pub(crate) fn collect(&self) -> Vec<FamilySnapshot> {
        let mut out = Vec::new();
        for shard in &*self.shards {
            let shard = shard.read().unwrap_or_else(PoisonError::into_inner);
            for (name, family) in shard.iter() {
                if family.series_count() == 0 {
                    continue;
                }
                let mut series: Vec<SeriesSnapshot> = family
                    .series()
                    .map(|(labels, s)| SeriesSnapshot {
                        labels: labels.clone(),
                        handle: s.handle.clone(),
                        created_unix_nanos: s.created_unix_nanos,
                    })
                    .collect();
                series.sort_by(|a, b| a.labels.cmp(&b.labels));
                out.push(FamilySnapshot {
                    name: name.clone(),
                    kind: family.kind,
                    help: family.help.clone(),
                    unit: family.unit,
                    series,
                });
            }
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        out
    }

    // ------------------------------------------------------------------
    // Removal and expiry
    // ------------------------------------------------------------------

    /// Remove the family `name` with all its series and metadata. Returns
    /// `true` when it existed. Handles already given out keep working but
    /// are no longer exported.
    pub fn remove(&self, name: &str) -> bool {
        let removed = self.write_shard(name).remove(name);
        match removed {
            Some(family) => {
                self.release(family.slots());
                true
            }
            None => false,
        }
    }

    /// Remove one series. Returns `true` when it existed. A family left
    /// without series is removed too, unless it carries metadata or
    /// configuration.
    pub fn remove_series(&self, name: &str, labels: &LabelSet) -> bool {
        let mut shard = self.write_shard(name);
        let Some(family) = shard.get_mut(name) else {
            return false;
        };
        let before = family.slots();
        if !family.remove(labels) {
            return false;
        }
        let keep = family.series_count() > 0 || family.has_config();
        let after = if keep { family.slots() } else { 0 };
        if !keep {
            let _ = shard.remove(name);
        }
        self.release(before - after);
        true
    }

    /// Remove series whose value has not changed for at least `idle` and
    /// that nobody else holds a handle to. Returns how many were removed.
    ///
    /// A series counts as changed when its value fingerprint differs from
    /// the previous sweep (counter value, gauge bits, timer/histogram count,
    /// …), so call this periodically — for example from your scrape handler
    /// or a background task. A gauge set repeatedly to the same value looks
    /// idle. Series whose `Arc` is still held elsewhere are never removed,
    /// because the holder could record into them later.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "count")]
    /// # {
    /// use metrics_lib::{LabelSet, Registry};
    /// use std::time::Duration;
    ///
    /// let registry = Registry::new();
    /// registry.counter_with("hits", &LabelSet::from([("user", "u1")])).inc();
    /// // Nothing has been idle for an hour yet.
    /// assert_eq!(registry.evict_idle(Duration::from_secs(3600)), 0);
    /// // With a zero threshold every unheld series is evicted.
    /// assert_eq!(registry.evict_idle(Duration::ZERO), 1);
    /// assert_eq!(registry.metric_count(), 0);
    /// # }
    /// ```
    pub fn evict_idle(&self, idle: Duration) -> usize {
        self.evict_idle_at(idle, self.now_ns())
    }

    fn evict_idle_at(&self, idle: Duration, now: u64) -> usize {
        let threshold = crate::util::duration_to_nanos(idle);
        let mut removed = 0;
        for shard in &*self.shards {
            // Pass 1 under the read lock: refresh fingerprints, pick victims.
            let victims: Vec<(String, LabelSet)> = {
                let guard = shard.read().unwrap_or_else(PoisonError::into_inner);
                guard
                    .iter()
                    .flat_map(|(name, family)| {
                        family
                            .series()
                            .filter(move |(_, s)| {
                                s.idle_for(now) >= threshold && s.handle.strong_count() == 1
                            })
                            .map(move |(labels, _)| (name.clone(), labels.clone()))
                    })
                    .collect()
            };
            if victims.is_empty() {
                continue;
            }
            // Pass 2 under the write lock: re-check, then remove.
            let mut guard = shard.write().unwrap_or_else(PoisonError::into_inner);
            for (name, labels) in victims {
                let Some(family) = guard.get_mut(&name) else {
                    continue;
                };
                let still_idle = family.get(&labels).is_some_and(|s| {
                    s.handle.strong_count() == 1
                        && s.handle.activity() == s.fingerprint.load(Ordering::Relaxed)
                });
                if !still_idle {
                    continue;
                }
                let before = family.slots();
                let _ = family.remove(&labels);
                let keep = family.series_count() > 0 || family.has_config();
                let after = if keep { family.slots() } else { 0 };
                if !keep {
                    let _ = guard.remove(&name);
                }
                self.release(before - after);
                removed += 1;
            }
        }
        removed
    }

    /// Number of series (labeled and unlabeled) across every family.
    #[must_use]
    pub fn metric_count(&self) -> usize {
        self.shards
            .iter()
            .map(|s| {
                s.read()
                    .unwrap_or_else(PoisonError::into_inner)
                    .values()
                    .map(Family::series_count)
                    .sum::<usize>()
            })
            .sum()
    }

    /// Remove every family, series and piece of metadata. Handles already
    /// given out keep working but are detached. The overflow, invalid-input
    /// and conflict counters are monotonic and are not reset.
    pub fn clear(&self) {
        for shard in &*self.shards {
            shard
                .write()
                .unwrap_or_else(PoisonError::into_inner)
                .clear();
        }
        self.used.store(0, Ordering::Relaxed);
    }

    /// A view that prefixes every name with `prefix`.
    ///
    /// Scopes are only a naming convention: `scoped("http_").counter("hits")`
    /// and `counter("http_hits")` return the same `Arc`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "count")]
    /// # {
    /// use metrics_lib::Registry;
    ///
    /// let registry = Registry::new();
    /// let http = registry.scoped("http.");
    /// http.counter("requests").inc();
    /// assert_eq!(registry.counter("http.requests").get(), 1);
    /// # }
    /// ```
    #[must_use]
    pub fn scoped(&self, prefix: impl Into<String>) -> ScopedRegistry<'_> {
        ScopedRegistry {
            registry: self,
            prefix: prefix.into(),
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("series", &self.metric_count())
            .field("cardinality_count", &self.cardinality_count())
            .field("cardinality_cap", &self.cardinality_cap())
            .finish_non_exhaustive()
    }
}

/// A [`Registry`] view that prepends a fixed prefix to every name. Created
/// with [`Registry::scoped`].
#[derive(Debug)]
pub struct ScopedRegistry<'a> {
    registry: &'a Registry,
    prefix: String,
}

macro_rules! scoped_lookups {
    ($feature:literal, $ty:ty, $plain:ident, $with:ident, $try_plain:ident, $try_with:ident) => {
        #[cfg(feature = $feature)]
        impl ScopedRegistry<'_> {
            /// Prefixed form of the [`Registry`] method of the same name.
            #[must_use]
            pub fn $plain(&self, name: &str) -> Arc<$ty> {
                self.registry.$plain(&self.join(name))
            }

            /// Prefixed form of the [`Registry`] method of the same name.
            #[must_use]
            pub fn $with(&self, name: &str, labels: &LabelSet) -> Arc<$ty> {
                self.registry.$with(&self.join(name), labels)
            }

            /// Prefixed form of the [`Registry`] method of the same name.
            ///
            /// # Errors
            ///
            /// As the [`Registry`] method of the same name.
            pub fn $try_plain(&self, name: &str) -> Result<Arc<$ty>> {
                self.registry.$try_plain(&self.join(name))
            }

            /// Prefixed form of the [`Registry`] method of the same name.
            ///
            /// # Errors
            ///
            /// As the [`Registry`] method of the same name.
            pub fn $try_with(&self, name: &str, labels: &LabelSet) -> Result<Arc<$ty>> {
                self.registry.$try_with(&self.join(name), labels)
            }
        }
    };
}

scoped_lookups!(
    "count",
    Counter,
    counter,
    counter_with,
    try_counter,
    try_counter_with
);
scoped_lookups!(
    "count",
    ShardedCounter,
    sharded_counter,
    sharded_counter_with,
    try_sharded_counter,
    try_sharded_counter_with
);
scoped_lookups!(
    "count",
    UpDownCounter,
    up_down_counter,
    up_down_counter_with,
    try_up_down_counter,
    try_up_down_counter_with
);
scoped_lookups!("gauge", Gauge, gauge, gauge_with, try_gauge, try_gauge_with);
scoped_lookups!("timer", Timer, timer, timer_with, try_timer, try_timer_with);
scoped_lookups!(
    "meter",
    RateMeter,
    rate_meter,
    rate_meter_with,
    try_rate_meter,
    try_rate_meter_with
);
scoped_lookups!(
    "histogram",
    Histogram,
    histogram,
    histogram_with,
    try_histogram,
    try_histogram_with
);
scoped_lookups!(
    "histogram",
    Summary,
    summary,
    summary_with,
    try_summary,
    try_summary_with
);

impl<'a> ScopedRegistry<'a> {
    /// The prefix this scope prepends.
    #[must_use]
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// The underlying registry.
    #[must_use]
    pub fn registry(&self) -> &'a Registry {
        self.registry
    }

    fn join(&self, name: &str) -> String {
        let mut s = String::with_capacity(self.prefix.len() + name.len());
        s.push_str(&self.prefix);
        s.push_str(name);
        s
    }

    /// Prefixed [`Registry::describe`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe(&self, name: &str, metadata: MetricMetadata) -> Result<()> {
        self.registry.describe(&self.join(name), metadata)
    }

    /// Prefixed [`Registry::describe_counter`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_counter(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.registry.describe_counter(&self.join(name), help, unit)
    }

    /// Prefixed [`Registry::describe_gauge`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_gauge(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.registry.describe_gauge(&self.join(name), help, unit)
    }

    /// Prefixed [`Registry::describe_timer`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_timer(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.registry.describe_timer(&self.join(name), help, unit)
    }

    /// Prefixed [`Registry::describe_rate`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_rate(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.registry.describe_rate(&self.join(name), help, unit)
    }

    /// Prefixed [`Registry::describe_histogram`].
    ///
    /// # Errors
    ///
    /// See [`Registry::describe`].
    pub fn describe_histogram(
        &self,
        name: &str,
        help: impl Into<Cow<'static, str>>,
        unit: Unit,
    ) -> Result<()> {
        self.registry
            .describe_histogram(&self.join(name), help, unit)
    }

    /// Prefixed [`Registry::configure_histogram`].
    ///
    /// # Errors
    ///
    /// See [`Registry::configure_histogram`].
    #[cfg(feature = "histogram")]
    pub fn configure_histogram(
        &self,
        name: &str,
        buckets: impl IntoIterator<Item = f64>,
    ) -> Result<()> {
        self.registry.configure_histogram(&self.join(name), buckets)
    }

    /// Prefixed [`Registry::configure_summary`].
    ///
    /// # Errors
    ///
    /// See [`Registry::configure_summary`].
    #[cfg(feature = "histogram")]
    pub fn configure_summary(&self, name: &str, config: SummaryConfig) -> Result<()> {
        self.registry.configure_summary(&self.join(name), config)
    }

    /// A nested scope: `scoped("a.").scoped("b.")` prefixes with `a.b.`.
    #[must_use]
    pub fn scoped(&self, sub_prefix: impl AsRef<str>) -> ScopedRegistry<'a> {
        ScopedRegistry {
            registry: self.registry,
            prefix: self.join(sub_prefix.as_ref()),
        }
    }
}

#[cfg(test)]
#[cfg(all(feature = "count", feature = "gauge", feature = "timer"))]
mod tests {
    use super::*;
    use std::sync::Barrier;
    use std::thread;

    #[test]
    fn test_same_series_returns_same_arc() {
        let r = Registry::new();
        assert!(Arc::ptr_eq(&r.counter("requests"), &r.counter("requests")));
        assert!(Arc::ptr_eq(&r.gauge("cpu"), &r.gauge("cpu")));
        assert!(Arc::ptr_eq(&r.timer("db"), &r.timer("db")));
        let l = LabelSet::from([("k", "v")]);
        assert!(Arc::ptr_eq(
            &r.counter_with("hits", &l),
            &r.counter_with("hits", &l)
        ));
        assert!(Arc::ptr_eq(
            &r.counter("x"),
            &r.counter_with("x", &LabelSet::EMPTY)
        ));
    }

    #[test]
    fn test_labeled_and_unlabeled_series_are_distinct() {
        let r = Registry::new();
        let plain = r.counter("hits");
        let labeled = r.counter_with("hits", &LabelSet::from([("region", "us")]));
        assert!(!Arc::ptr_eq(&plain, &labeled));
        assert_eq!(r.metric_count(), 2);
        assert_eq!(r.cardinality_count(), 2);
    }

    #[test]
    fn test_kind_mismatch_is_refused_regression_ml005() {
        // ML-005: a counter and a gauge sharing a name produced two TYPE lines.
        let r = Registry::new();
        r.counter("dup").inc();
        let err = r.try_gauge("dup").unwrap_err();
        assert_eq!(
            err,
            MetricsError::KindMismatch {
                name: "dup".into(),
                existing: MetricKind::Counter,
                requested: MetricKind::Gauge
            }
        );
        let sink = r.gauge("dup");
        sink.set(5.0);
        assert_eq!(r.kind_conflicts(), 1);
        assert!(r.gauge_entries().is_empty());
        // A sharded counter is a counter, but cannot share a family with one.
        assert!(r.try_sharded_counter("dup").is_err());
        assert_eq!(r.kind_of("dup"), Some(MetricKind::Counter));
    }

    #[test]
    fn test_describe_must_match_kind_regression_ml005() {
        let r = Registry::new();
        r.counter("c2").inc();
        assert!(matches!(
            r.describe_gauge("c2", "help", Unit::None),
            Err(MetricsError::KindMismatch { .. })
        ));
        r.describe_counter("c2", "Counts things", Unit::None)
            .unwrap();
        let meta = r.metadata("c2").unwrap();
        assert_eq!(meta.kind, MetricKind::Counter);
        assert_eq!(meta.help, "Counts things");
        // Describing first reserves the kind.
        r.describe_gauge("temp", "Temperature", Unit::Custom("celsius"))
            .unwrap();
        assert!(r.try_counter("temp").is_err());
        r.gauge("temp").set(1.0);
        assert_eq!(r.cardinality_count(), 2);
    }

    #[test]
    fn test_invalid_names_rejected_or_repaired_regression_ml009() {
        let r = Registry::new();
        assert!(matches!(
            r.try_counter("bad name\n# TYPE x gauge"),
            Err(MetricsError::InvalidName { .. })
        ));
        let repaired = r.counter("bad-name");
        repaired.inc();
        assert_eq!(r.counter_names(), vec!["bad_name".to_string()]);
        assert!(Arc::ptr_eq(&repaired, &r.counter("bad_name")));
        assert_eq!(r.invalid_inputs(), 1);
        let hostile = LabelSet::from([("bad\"key}\n", "v")]);
        assert!(r.try_counter_with("c", &hostile).is_err());
        r.counter_with("c", &hostile).inc();
        let entries = r.counter_entries();
        let labels = &entries.iter().find(|e| e.0 == "c").unwrap().1;
        assert!(labels.validate().is_ok());
        assert!(r.describe_counter("1bad", "", Unit::None).is_err());
    }

    #[cfg(feature = "histogram")]
    #[test]
    fn test_reserved_label_keys_are_refused_or_renamed() {
        let r = Registry::new();
        let le = LabelSet::from([("le", "0.5")]);
        assert!(matches!(
            r.try_histogram_with("lat", &le),
            Err(MetricsError::InvalidLabel { .. })
        ));
        let _ = r.histogram_with("lat", &le);
        let entries = r.histogram_entries();
        assert_eq!(entries[0].1.get("le_"), Some("0.5"));
        let q = LabelSet::from([("quantile", "x")]);
        assert!(r.try_timer_with("t", &q).is_err());
        assert!(r.try_counter_with("ok", &q).is_ok());
    }

    #[test]
    fn test_cap_counts_unlabeled_series_regression_ml008() {
        // ML-008: unlabeled names were not bounded at all.
        let r = Registry::with_cardinality_cap(3);
        for i in 0..3 {
            let _ = r.try_counter(&format!("c{i}")).unwrap();
        }
        assert_eq!(
            r.try_counter("c3").unwrap_err(),
            MetricsError::CardinalityExceeded
        );
        let sink = r.counter("c4");
        sink.inc();
        assert_eq!(r.metric_count(), 3);
        assert_eq!(r.cardinality_overflows(), 2);
        // Existing series still resolve when the cap is full.
        assert!(r.try_counter("c0").is_ok());
        r.set_cardinality_cap(4);
        assert!(r.try_counter("c3").is_ok());
        assert_eq!(r.cardinality_cap(), 4);
    }

    #[test]
    fn test_describe_takes_a_slot_until_first_series() {
        let r = Registry::with_cardinality_cap(1);
        r.describe_counter("a", "help", Unit::None).unwrap();
        assert_eq!(r.cardinality_count(), 1);
        assert!(r.describe_counter("b", "help", Unit::None).is_err());
        // The first series of `a` reuses the slot.
        r.counter("a").inc();
        assert_eq!(r.cardinality_count(), 1);
        assert!(
            r.try_counter_with("a", &LabelSet::from([("k", "v")]))
                .is_err()
        );
    }

    #[test]
    fn test_concurrent_first_registration_takes_one_slot_regression_ml007() {
        // ML-007: histograms reserved the slot before re-checking under the
        // write lock, leaking slots when threads raced on one new series.
        for round in 0..200 {
            let r = Arc::new(Registry::new());
            let barrier = Arc::new(Barrier::new(8));
            let labels = LabelSet::from([("round", round.to_string())]);
            let handles: Vec<_> = (0..8)
                .map(|i| {
                    let r = Arc::clone(&r);
                    let barrier = Arc::clone(&barrier);
                    let labels = labels.clone();
                    thread::spawn(move || {
                        let _ = barrier.wait();
                        match i % 3 {
                            0 => drop(r.counter_with("series", &labels)),
                            1 => drop(r.try_counter_with("series", &labels)),
                            _ => drop(r.counter_with("series", &labels)),
                        }
                    })
                })
                .collect();
            for h in handles {
                h.join().unwrap();
            }
            assert_eq!(r.cardinality_count(), 1, "round {round}");
        }
    }

    #[cfg(feature = "histogram")]
    #[test]
    fn test_concurrent_histogram_registration_takes_one_slot_regression_ml007() {
        for round in 0..200 {
            let r = Arc::new(Registry::new());
            let barrier = Arc::new(Barrier::new(8));
            let handles: Vec<_> = (0..8)
                .map(|_| {
                    let r = Arc::clone(&r);
                    let barrier = Arc::clone(&barrier);
                    thread::spawn(move || {
                        let _ = barrier.wait();
                        r.histogram_with("lat", &LabelSet::from([("op", "x")]))
                    })
                })
                .collect();
            let first = handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .reduce(|a, b| {
                    assert!(Arc::ptr_eq(&a, &b));
                    a
                });
            assert!(first.is_some());
            assert_eq!(r.cardinality_count(), 1, "round {round}");
        }
    }

    #[test]
    fn test_remove_and_remove_series_release_slots() {
        let r = Registry::new();
        let l1 = LabelSet::from([("k", "1")]);
        let l2 = LabelSet::from([("k", "2")]);
        r.counter_with("c", &l1).inc();
        r.counter_with("c", &l2).inc();
        r.gauge("g").set(1.0);
        assert_eq!(r.cardinality_count(), 3);
        assert!(r.remove_series("c", &l1));
        assert!(!r.remove_series("c", &l1));
        assert!(!r.remove_series("nope", &l1));
        assert_eq!(r.cardinality_count(), 2);
        assert!(r.remove_series("c", &l2));
        // The family had no metadata, so it is gone and its kind is free.
        assert_eq!(r.kind_of("c"), None);
        assert!(r.try_gauge("c").is_ok());
        assert!(r.remove("g"));
        assert!(!r.remove("g"));
        assert_eq!(r.cardinality_count(), 1);
    }

    #[test]
    fn test_remove_series_keeps_described_family() {
        let r = Registry::new();
        r.describe_counter("c", "help", Unit::None).unwrap();
        r.counter("c").inc();
        assert!(r.remove_series("c", &LabelSet::EMPTY));
        assert_eq!(r.kind_of("c"), Some(MetricKind::Counter));
        assert_eq!(r.cardinality_count(), 1);
        assert_eq!(r.metric_count(), 0);
    }

    #[test]
    fn test_evict_idle_with_injected_clock() {
        let r = Registry::new();
        let busy = LabelSet::from([("user", "busy")]);
        let quiet = LabelSet::from([("user", "quiet")]);
        r.counter_with("hits", &busy).inc();
        r.counter_with("hits", &quiet).inc();
        let held = r.counter("held");
        let sec = 1_000_000_000;
        // First sweep records fingerprints; nothing is old enough yet.
        assert_eq!(r.evict_idle_at(Duration::from_secs(10), sec), 0);
        r.counter_with("hits", &busy).inc();
        assert_eq!(r.evict_idle_at(Duration::from_secs(10), 20 * sec), 1);
        let names: Vec<_> = r.counter_entries().into_iter().map(|e| e.1).collect();
        assert!(names.contains(&busy));
        assert!(!names.contains(&quiet));
        // A held handle is never evicted.
        assert_eq!(r.evict_idle_at(Duration::ZERO, 100 * sec), 1);
        assert!(r.counter_names().contains(&"held".to_string()));
        drop(held);
        assert_eq!(r.evict_idle_at(Duration::ZERO, 200 * sec), 1);
        assert_eq!(r.metric_count(), 0);
        assert_eq!(r.cardinality_count(), 0);
    }

    #[test]
    fn test_clear_resets_count_but_not_monotonic_counters() {
        let r = Registry::with_cardinality_cap(1);
        r.counter("a").inc();
        let _ = r.counter("b");
        let overflows = r.cardinality_overflows();
        assert!(overflows >= 1);
        let before = r.counter("a");
        r.clear();
        assert_eq!(r.cardinality_count(), 0);
        assert_eq!(r.metric_count(), 0);
        assert_eq!(r.cardinality_overflows(), overflows);
        assert!(!Arc::ptr_eq(&before, &r.counter("a")));
    }

    #[test]
    fn test_entries_and_names_are_sorted() {
        let r = Registry::new();
        r.counter("b").inc();
        r.counter("a").inc();
        r.counter_with("a", &LabelSet::from([("k", "v")])).inc();
        let entries = r.counter_entries();
        let keys: Vec<_> = entries.iter().map(|e| (e.0.as_str(), e.1.len())).collect();
        assert_eq!(keys, vec![("a", 0), ("a", 1), ("b", 0)]);
        assert_eq!(r.counter_names(), vec!["a", "b"]);
        assert!(r.gauge_names().is_empty());
        assert!(format!("{r:?}").contains("Registry"));
    }

    #[test]
    fn test_scoped_registry() {
        let r = Registry::new();
        let http = r.scoped("http.");
        http.counter("requests").inc();
        assert!(Arc::ptr_eq(
            &http.counter("requests"),
            &r.counter("http.requests")
        ));
        let nested = http.scoped("v1.");
        assert_eq!(nested.prefix(), "http.v1.");
        nested.gauge("inflight").set(1.0);
        assert_eq!(r.gauge("http.v1.inflight").get(), 1.0);
        http.describe_counter("requests", "Requests", Unit::None)
            .unwrap();
        assert!(r.metadata("http.requests").is_some());
        assert!(http.try_timer("t").is_ok());
        assert!(
            http.try_counter_with("c", &LabelSet::from([("k", "v")]))
                .is_ok()
        );
        let _ = http.timer_with("t2", &LabelSet::EMPTY);
        let _ = http.gauge_with("g2", &LabelSet::EMPTY);
        assert!(http.try_gauge_with("g3", &LabelSet::EMPTY).is_ok());
        assert!(http.try_gauge("g4").is_ok());
        assert!(http.try_timer_with("t5", &LabelSet::EMPTY).is_ok());
        assert!(std::ptr::eq(http.registry(), &r));
        http.describe_gauge("g", "", Unit::None).unwrap();
        http.describe_timer("t9", "", Unit::None).unwrap();
        http.describe_rate("r9", "", Unit::None).unwrap();
        http.describe_histogram("h9", "", Unit::None).unwrap();
        http.describe(
            "x9",
            MetricMetadata::new(MetricKind::Summary, "", Unit::None),
        )
        .unwrap();
        assert!(format!("{http:?}").contains("http."));
    }

    #[test]
    fn test_shard_index_is_in_range_and_spreads() {
        let mut seen = [false; SHARDS];
        for i in 0..1000 {
            let idx = shard_index(&format!("metric_{i}"));
            assert!(idx < SHARDS);
            seen[idx] = true;
        }
        assert!(seen.iter().filter(|s| **s).count() >= SHARDS / 2);
        assert!(shard_index("") < SHARDS);
    }

    #[test]
    fn test_overflow_sinks_are_detached() {
        let r = Registry::with_cardinality_cap(0);
        let c = r.counter("x");
        c.inc();
        let u = r.up_down_counter("y");
        u.inc();
        let s = r.sharded_counter("z");
        s.inc();
        assert_eq!(r.metric_count(), 0);
        assert!(r.counter_entries().is_empty());
        assert!(r.try_up_down_counter_with("y", &LabelSet::EMPTY).is_err());
        assert!(r.try_sharded_counter_with("z", &LabelSet::EMPTY).is_err());
    }

    #[test]
    fn test_registry_supports_every_counter_flavour() {
        let r = Registry::new();
        r.sharded_counter("sc").add(3);
        r.up_down_counter("ud").add(-2);
        assert_eq!(r.sharded_counter_entries()[0].2.get(), 3);
        assert_eq!(r.up_down_counter_entries()[0].2.get(), -2);
        assert_eq!(r.sharded_counter_names(), vec!["sc"]);
        assert_eq!(r.up_down_counter_names(), vec!["ud"]);
        let l = LabelSet::from([("k", "v")]);
        r.sharded_counter_with("sc", &l).inc();
        r.up_down_counter_with("ud", &l).inc();
        assert!(r.try_sharded_counter("sc").is_ok());
        assert!(r.try_up_down_counter("ud").is_ok());
        r.describe_up_down_counter("ud", "level", Unit::None)
            .unwrap();
        assert_eq!(r.metadata("ud").unwrap().kind, MetricKind::UpDownCounter);
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_1x_aliases_still_work() {
        let r = Registry::new();
        let l = LabelSet::from([("k", "v")]);
        assert!(Arc::ptr_eq(&r.get_or_create_counter("c"), &r.counter("c")));
        assert!(Arc::ptr_eq(
            &r.get_or_create_counter_with("c", &l),
            &r.counter_with("c", &l)
        ));
        assert!(r.try_get_or_create_counter_with("c", &l).is_ok());
        assert!(Arc::ptr_eq(&r.get_or_create_gauge("g"), &r.gauge("g")));
        let _ = r.get_or_create_gauge_with("g", &l);
        assert!(r.try_get_or_create_gauge_with("g", &l).is_ok());
        assert!(Arc::ptr_eq(&r.get_or_create_timer("t"), &r.timer("t")));
        let _ = r.get_or_create_timer_with("t", &l);
        assert!(r.try_get_or_create_timer_with("t", &l).is_ok());
    }

    #[test]
    fn test_registry_collect_snapshot_shape() {
        let r = Registry::new();
        r.describe_counter("b", "help b", Unit::None).unwrap();
        r.counter("b").inc();
        r.counter_with("a", &LabelSet::from([("z", "1")])).inc();
        r.counter_with("a", &LabelSet::from([("y", "1")])).inc();
        r.describe_gauge("empty", "no series", Unit::None).unwrap();
        let snap = r.collect();
        let names: Vec<_> = snap.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b"]);
        assert_eq!(snap[0].series[0].labels.get("y"), Some("1"));
        assert_eq!(snap[1].help.as_deref(), Some("help b"));
        assert!(snap[1].series[0].created_unix_nanos > 0);
    }
}

#[cfg(test)]
#[cfg(all(feature = "histogram", feature = "meter"))]
mod histogram_tests {
    use super::*;

    #[test]
    fn test_configure_histogram_and_summary() {
        let r = Registry::new();
        r.configure_histogram("lat", [0.1, 0.5]).unwrap();
        assert_eq!(r.histogram("lat").bounds(), &[0.1, 0.5]);
        assert!(r.configure_histogram("lat", []).is_err());
        assert!(r.configure_histogram("bad name", [1.0]).is_err());
        r.configure_summary("q", SummaryConfig::default().with_quantiles([0.5]))
            .unwrap();
        assert_eq!(r.summary("q").quantile_targets(), &[0.5]);
        assert!(
            r.configure_summary("q2", SummaryConfig::default().with_relative_accuracy(2.0))
                .is_err()
        );
        r.rate_meter("rm").tick();
        assert!(r.configure_histogram("rm", [1.0]).is_err());
        assert_eq!(r.rate_meter_names(), vec!["rm"]);
        assert_eq!(r.histogram_names(), vec!["lat"]);
        assert_eq!(r.summary_names(), vec!["q"]);
        r.describe_summary("q", "quantiles", Unit::Seconds).unwrap();
        r.describe_histogram("lat", "latency", Unit::Seconds)
            .unwrap();
        r.describe_rate("rm", "rate", Unit::None).unwrap();
        let scoped = r.scoped("s_");
        scoped.configure_histogram("h", [1.0]).unwrap();
        scoped
            .configure_summary("x", SummaryConfig::default())
            .unwrap();
        assert_eq!(scoped.histogram("h").bounds(), &[1.0]);
        let _ = scoped.summary("x");
        let _ = scoped.summary_with("x", &LabelSet::from([("k", "v")]));
        assert!(scoped.try_summary("x").is_ok());
        assert!(scoped.try_summary_with("x", &LabelSet::EMPTY).is_ok());
        let _ = scoped.histogram_with("h", &LabelSet::EMPTY);
        assert!(scoped.try_histogram("h").is_ok());
        assert!(scoped.try_histogram_with("h", &LabelSet::EMPTY).is_ok());
        let _ = scoped.rate_meter("r");
        let _ = scoped.rate_meter_with("r", &LabelSet::EMPTY);
        assert!(scoped.try_rate_meter("r").is_ok());
        assert!(scoped.try_rate_meter_with("r", &LabelSet::EMPTY).is_ok());
        assert_eq!(r.summary_entries().len(), 3);
        assert_eq!(r.rate_meter_entries().len(), 2);
        #[allow(deprecated)]
        {
            let l = LabelSet::from([("k", "v")]);
            let _ = r.get_or_create_rate_meter("rm");
            let _ = r.get_or_create_rate_meter_with("rm", &l);
            assert!(r.try_get_or_create_rate_meter_with("rm", &l).is_ok());
            let _ = r.get_or_create_histogram("lat");
            let _ = r.get_or_create_histogram_with("lat", &l);
            assert!(r.try_get_or_create_histogram_with("lat", &l).is_ok());
        }
    }
}
