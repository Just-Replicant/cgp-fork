//! Entrypoint for the `ui` target: the diagnostics users see for common mistakes.
//!
//! Each `tests/ui/*.rs` case must fail to compile with exactly the `.stderr` beside
//! it. The deeper "accepted, but the expansion fails to compile" fixtures live in
//! `cargo-cgp`; these few pin the messages this repository is responsible for.
//!
//! Regenerate after an intended change with
//! `TRYBUILD=overwrite cargo test -p cgp-fork --all-features --test ui_tests`.
//! The cases only run with every feature enabled, so the compiler's candidate
//! lists are stable, and with the pinned toolchain.
#![cfg(all(feature = "anyhow", feature = "eyre", feature = "std-error"))]

#[test]
fn ui() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}
