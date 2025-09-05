fn main() {
    // Allow using `#[cfg(tarpaulin)]` in code without triggering unexpected cfg warnings.
    // This keeps CI logs clean when tarpaulin isn't present and preserves MSRV 1.70.0.
    println!("cargo:rustc-check-cfg=cfg(tarpaulin)");
    // Allow using `cfg!(coverage)` in tests to relax bounds under coverage instrumentation.
    // Registers the `coverage` cfg with rustc to avoid `unexpected_cfgs` warnings.
    println!("cargo:rustc-check-cfg=cfg(coverage)");
}
