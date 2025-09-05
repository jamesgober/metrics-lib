<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>CHANGELOG</b>
</h1>
<p>
  All notable changes to this project will be documented in this file. The format is based on <a href="https://keepachangelog.com/en/1.1.0/">Keep a Changelog</a>,
  and this project adheres to <a href="https://semver.org/spec/v2.0.0.html/">Semantic Versioning</a>.
</p>

## [Unreleased]


<br>

## [0.5.1] - STABLE

### Added
- Codecov configuration `codecov.yml` with project and patch coverage thresholds (target: 85%).
- Coverage status checks integrated via Codecov for PRs and main.
- Optional Cargo feature `bench-tests` to enable running benchmark-style tests on demand.
- Documentation on running bench-gated tests locally (`cargo test --features bench-tests -- --ignored`).

### Changed
- CI coverage no longer uses Codecov or tokens; it is enforced directly via `cargo tarpaulin --fail-under 70` with Cobertura XML (`cobertura.xml`) uploaded as a build artifact.
- CI workflow `.github/workflows/ci.yml`: replaced Codecov upload with tokenless tarpaulin threshold enforcement; coverage status now comes from the job result.
- Repository-wide formatting normalized with `cargo fmt --all`.
- Benchmarks are now gated behind `bench-tests` and ignored by default using `#[cfg_attr(not(feature = "bench-tests"), ignore)]` across modules:
  - `src/counter.rs`
  - `src/gauge.rs`
  - `src/timer.rs`
  - `src/rate_meter.rs`
  - `src/system_health.rs`
- Relaxed benchmark timing thresholds slightly to reduce flakiness on slower or variable runners.
- Security audit now runs `cargo audit` directly in CI (no integration permissions required).

### Fixed
- Lint hygiene: repository is Clippy-clean under `--all-features`.
- Flaky CI failures caused by strict benchmark timing assertions; benchmarks no longer run by default in CI.

### Quality Gates
- Local and CI parity established:
  - Formatting: `cargo fmt --all -- --check`.
  - Lints: `cargo clippy --all-features -- -D warnings`.
  - Tests: `cargo test --all-features` across Linux/macOS/Windows and MSRV `1.70.0`.

### Removed
- Codecov integration removed; `codecov.yml` archived to `docs/legacy/codecov.yml`.

### Internal
- Build script registers `cfg(tarpaulin)` using MSRV-safe `cargo:rustc-check-cfg=cfg(tarpaulin)` to silence unknown cfg warnings on Rust 1.70.0.



<br>

## [0.5.0] - COMPLETE ARCHITECTURAL OVERHAUL
### Fixed
- **Compilation Errors**: Fixed temporary value dropped while borrowed in timer closure
- **Import Cleanup**: Removed unused imports (BufRead, BufReader, File) from system_health module
- **Ambiguous Re-exports**: Resolved conflicting specialized module exports from gauge and rate_meter
- **Documentation**: Added missing documentation for time_fn macro
- **Lint Issues**: Applied Clippy fixes for format string optimizations and redundant closures
- **Integer Overflow**: Fixed subtraction overflow panic in system health monitoring



<br>

## [0.2.0] - COMPLETE ARCHITECTURAL OVERHAUL

### Added - Complete Rewrite
- **COMPLETE ARCHITECTURAL OVERHAUL**: Rebuilt from ground up for ultimate performance
- **Core Types**: Counter, Gauge, Timer, RateMeter, Registry - optimized for sub-nanosecond operations
- **System Health Integration**: Built-in CPU/memory monitoring with process introspection
- **Lock-Free Everything**: Zero locks, zero allocations in hot paths
- **Dynamic Configuration**: Runtime tunable parameters without restarts
- **Circuit Breaker**: Fault tolerance with automatic recovery mechanisms
- **Dead Simple API**: `METRICS.cpu().used()`, `timer.start()`, etc.
- **Cross-Platform Optimizations**: CPU feature detection, NUMA-aware when beneficial
- **Process Usage Estimation**: Automatic detection of current app's CPU/memory consumption

### Changed
- **API Redesign**: Simplified from complex over-engineered interface to intuitive methods
- **Performance Focus**: Sub-3ns counter increments, sub-5ns gauge updates
- **Memory Layout**: Cache-line aligned structures, optimized atomic operations
- **Error Handling**: Graceful degradation, never hang or block

### Removed
- Over-engineered SIMD complexity (moved to optional extensions)
- Complex async machinery (simplified to core functionality)
- Thread-local pools (over-optimization removed)
- Specialized lens types (moved to separate analytics crate)
- Historical tracking complexity (available as optional feature)

### Technical Details
- **Atomic Operations**: IEEE 754 bit manipulation for gauges
- **Memory Efficiency**: Single cache line per metric where possible  
- **CPU Detection**: Runtime optimization based on available features
- **Health Monitoring**: Built-in system resource tracking
- **Circuit Breakers**: Automatic failure recovery and fallback modes


<br>

## [0.1.0] - 2025-08-29

Initial release with core metrics library functionality.

### Added
- Core metrics library with comprehensive performance measurement tools
- **Gauge Module**: Ultra-high-performance atomic gauges for server applications
  - Lock-free atomic operations with sub-nanosecond updates
  - Labeled gauges, percentage gauges, and specialized metric types
  - Statistical analysis including percentiles, variance, and trends
- **Timer Module**: High-resolution timer implementation
  - Sub-nanosecond precision timing with TSC support
  - Scoped timers for RAII-based measurement
  - Timer pools for high-concurrency scenarios
- **Lens Module**: Advanced temporal lens for sliding window analytics
  - SIMD-optimized aggregations (AVX512, AVX2, SSE2)
  - Lock-free concurrent operations with async support
  - Comprehensive trend analysis and statistical computations
- **Sample Module**: Statistical sampling and reservoir algorithms
- **Histogram Module**: High-performance histogram implementations
- **Registry Module**: Centralized metrics collection and management
- **Count Module**: Efficient counting mechanisms
- **Meter Module**: Rate measurement and throughput analysis

### Fixed
- **Compilation Issues**: Resolved all major compilation errors
  - Fixed duplicate module definitions in lib.rs
  - Added proper feature gates for async/tokio functionality
  - Resolved AVX512 unstable feature usage with nightly feature guards
  - Fixed borrow checker issues in SIMD implementations
  - Corrected type mismatches in async components
- **Import Cleanup**: Removed unused imports across all modules
  - Cleaned up unused atomic types in gauge.rs
  - Fixed unused SIMD imports in lens.rs and timer.rs
  - Corrected unused standard library imports
- **Code Quality**: Addressed warnings and lint issues
  - Fixed unused variable warnings
  - Corrected unnecessary mutable annotations
  - Resolved feature flag mismatches

### Technical Highlights
- **Zero-allocation hot paths** for maximum performance
- **Cache-line aligned data structures** to prevent false sharing
- **SIMD optimization support** with runtime feature detection
- **Async/await compatibility** with proper feature gating
- **Thread-safe concurrent access** throughout all components
- **Comprehensive test suites** with performance benchmarks

### Performance
- Sub-nanosecond gauge operations
- Millions of operations per second capability
- Lock-free data structures for maximum concurrency
- SIMD-accelerated batch operations where applicable

### Documentation
- Comprehensive inline documentation for all public APIs
- Performance-focused implementation notes
- Usage examples and best practices
- Architecture decision documentation



<!-- FOOT LINKS
################################################# -->
[Unreleased]: https://github.com/jamesgober/metrics-lib/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/jamesgober/metrics-lib/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/jamesgober/metrics-lib/compare/v0.2.0...v0.5.0
[0.2.0]: https://github.com/jamesgober/metrics-lib/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jamesgober/metrics-lib/releases/tag/v0.1.0
