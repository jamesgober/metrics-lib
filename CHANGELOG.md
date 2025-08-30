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




### Removed
- The entire codebase.



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
[Unreleased]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.0...HEAD
[0.1.5]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.0...v0.1.5
[0.1.0]: https://github.com/jamesgober/rust-benchmark/releases/tag/v0.1.0
