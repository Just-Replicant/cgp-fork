//! Entrypoint for the `ui` target: the diagnostics users see for common mistakes.
//!
//! Each `tests/ui/*.rs` case must fail to compile. On the minor version pinned in
//! `rust-toolchain.toml` the `.stderr` beside it must match exactly. The CI
//! canary uses stable and beta, and rustc's `help:` lines move between those
//! releases (ADR 0005); there the case must still fail, and the first `error`
//! line — the message this crate owns — must be the one in the snapshot.
//! Each `tests/ui_pass/*.rs` case must compile on every compiler.
//!
//! Regenerate the snapshots after an intended change, on the pinned toolchain,
//! with `TRYBUILD=overwrite cargo test -p cgp-fork --all-features --test ui_tests`.
//! The cases only run with every feature enabled, so the compiler's candidate
//! lists are stable.
#![cfg(all(feature = "anyhow", feature = "eyre", feature = "std-error"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn ui() {
    let cases = trybuild::TestCases::new();
    if snapshot_toolchain() {
        cases.compile_fail("tests/ui/*.rs");
    } else {
        assert_owned_diagnostics();
    }
    // The replacements for the deprecated names compile under `deny(warnings)`.
    cases.pass("tests/ui_pass/*.rs");
}

/// Exact snapshots belong to the pinned minor (`1.99` for `channel = "1.99.1"`).
fn snapshot_toolchain() -> bool {
    let pinned = include_str!("../../../../rust-toolchain.toml");
    let pin = pinned
        .lines()
        .find_map(|line| {
            let line = line.trim();
            line.strip_prefix("channel")
                .and_then(|rest| rest.trim().strip_prefix('='))
                .map(str::trim)
                .map(|value| value.trim_matches('"'))
        })
        .unwrap_or_else(|| panic!("rust-toolchain.toml has no channel"));
    let output = Command::new(toolchain_rustc())
        .arg("--version")
        .output()
        .unwrap_or_else(|error| panic!("rustc --version: {error}"));
    let text = String::from_utf8_lossy(&output.stdout);
    release_minor(&text) == release_minor(pin)
}

fn toolchain_rustc() -> PathBuf {
    let cargo = PathBuf::from(env!("CARGO"));
    let file = if cfg!(windows) { "rustc.exe" } else { "rustc" };
    let rustc = cargo.with_file_name(file);
    if rustc.is_file() {
        rustc
    } else {
        PathBuf::from(file)
    }
}

fn release_minor(text: &str) -> (u32, u32) {
    let token = text
        .split_whitespace()
        .find(|word| word.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .unwrap_or_else(|| panic!("no version in {text:?}"));
    let mut parts = token.split(['.', '-']);
    let major = parts
        .next()
        .and_then(|part| part.parse().ok())
        .unwrap_or_else(|| panic!("bad version {token}"));
    let minor = parts
        .next()
        .and_then(|part| part.parse().ok())
        .unwrap_or_else(|| panic!("bad version {token}"));
    (major, minor)
}

/// Compile every `tests/ui` fixture and require the snapshot's first `error` line.
fn assert_owned_diagnostics() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ui = manifest_dir.join("tests/ui");
    let work = manifest_dir.join("../../../target/ui-canary");
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(work.join("src")).unwrap_or_else(|error| panic!("create {work:?}: {error}"));

    let facade = manifest_dir.to_string_lossy().replace('\\', "/");
    let cargo_toml = format!(
        r#"[package]
name = "cgp-ui-canary"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
cgp-fork = {{ path = "{facade}", features = ["anyhow", "eyre", "std-error", "tokio", "smol", "inspect", "mock", "log-tracing", "log-defmt"] }}

[workspace]
"#
    );
    fs::write(work.join("Cargo.toml"), cargo_toml).unwrap();

    let mut fixtures: Vec<_> = fs::read_dir(&ui)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "no tests/ui fixtures");

    for fixture in fixtures {
        let expected = owned_error_line(
            &ui.join(fixture.file_name().unwrap())
                .with_extension("stderr"),
        );
        fs::copy(&fixture, work.join("src/main.rs")).unwrap();
        let output = Command::new(env!("CARGO"))
            .args(["check", "--manifest-path"])
            .arg(work.join("Cargo.toml"))
            .arg("--color=never")
            .env("CARGO_TERM_COLOR", "never")
            .output()
            .unwrap_or_else(|error| panic!("cargo check: {error}"));
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !output.status.success(),
            "{} compiled, but it must fail",
            fixture.display()
        );
        assert!(
            combined.contains(&expected),
            "{} did not report {expected:?}\n{combined}",
            fixture.display()
        );
    }
}

fn owned_error_line(stderr: &Path) -> String {
    fs::read_to_string(stderr)
        .unwrap_or_else(|error| panic!("read {}: {error}", stderr.display()))
        .lines()
        .find(|line| line.starts_with("error:") || line.starts_with("error["))
        .map(|line| line.trim_end_matches('\r').to_string())
        .unwrap_or_else(|| panic!("{} has no error line", stderr.display()))
}
