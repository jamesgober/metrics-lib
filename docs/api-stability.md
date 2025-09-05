# API Stability Guarantees

This document defines the stability promises for `metrics-lib` across versions. It clarifies what is considered a breaking change, what is semver-patch/minor safe, and how deprecations are handled.

## Versioning Policy

- `metrics-lib` follows Semantic Versioning (SemVer): `MAJOR.MINOR.PATCH`.
- MSRV (Minimum Supported Rust Version): `1.70`.
- Within a given `MAJOR` version, we strive to avoid breaking changes. Breaking changes trigger a `MAJOR` bump.

## Backward Compatibility Guarantees

- Public types and functions in `src/` are covered by SemVer guarantees once released on crates.io.
- Adding new APIs is a `MINOR` bump.
- Bug fixes and performance improvements that do not alter API signatures or documented behavior are `PATCH`.

## What Counts as Breaking

- Removing or renaming public items (types/functions/modules).
- Changing function signatures, return types, trait bounds, or visibility.
- Altering documented behavior or invariants in a way that can break consumers.
- Tightening trait bounds or introducing new required features on existing APIs.

## Non‑Breaking Changes (Examples)

- Adding new functions, structs, modules, or optional features.
- Adding `#[must_use]` to functions where ignoring the result is almost certainly a bug, accompanied by rustdoc notes (already implemented across several APIs).
- Internal performance optimizations without observable behavioral change.

## Deprecation Policy

- When feasible, we prefer to deprecate before removing.
- Deprecations include rustdoc notes explaining the migration path and expected removal timeline.
- Removals occur in the next `MAJOR` release.

## Feature Flags

- Optional features are additive and opt‑in.
- The `bench-tests` feature gates heavy or timing-sensitive tests to keep CI reliable.
- The `async` feature enables async helpers; absence of this feature keeps the core runtime‑agnostic and small.

## Error Semantics

- `try_` methods return `Result<_, MetricsError>` and never panic under valid inputs.
- Non‑`try_` methods prioritize speed and may saturate or assume valid inputs; they are not expected to panic in normal operation.

## MSRV

- The MSRV is currently `1.70` and may be bumped in a `MINOR` release with justification and changelog notes (following Rust community guidance), though we attempt to keep it stable.

## Stability of Metric Names

- This crate is in‑process metrics focused; metric names are user‑defined. We recommend treating names as part of your app’s API contract. The crate does not automatically rename or mutate metric names.

## Migration Guidance

- Breaking changes are documented in `CHANGELOG.md` with explicit migration steps.
- A dedicated guide for migrating from `metrics-rs` is available at `docs/migrating-from-metrics-rs.md`.

## Contact

- If you depend on a specific behavior and are unsure if a change affects you, please open an issue or start a discussion. We aim to keep the surface predictable and fast.
