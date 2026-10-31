# After phase 1: 2026-10-30 (branch `rewrite/phase-0-1`)

| Measure | Value |
|---|---|
| Commit | `rewrite/phase-0-1`, with all phase 1 code and docs (the commit before this file) |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Tests passed | 867 |
| Tests failed | 0 |
| Tests ignored | 56 |
| Clippy (`-D warnings`, all targets, all features) | clean |
| Compile-time bench (200 components, wired and checked) | 0.70 s |
| Facade `src` | 186 files, 5821 lines |
| Macro crates `src` | 395 files, 25481 lines |
| Facade integration tests | 347 files, 35160 lines |

Compared with the [phase 0 baseline](2026-10-30.md) on the same machine:

| Measure | Baseline | After phase 1 | Change |
|---|---|---|---|
| Tests passed | 824 | 867 | +43: 14 `crate_path`, 4 `registry`, 25 renamed-dependency |
| Tests failed | 0 | 0 | |
| Tests ignored | 56 | 56 | the macro-crate doc examples, compiled in `cgp-fork-doctests` instead |
| Clippy | clean | clean | |
| Compile-time bench | 0.71 s | 0.70 s | within noise (three runs: 0.74, 0.70, 0.71 s) |

Also green on this commit: `cargo +nightly fmt --all -- --check` (workspace and `fuzz/`),
`cargo doc` with `-D warnings --cfg docsrs`, clippy without default features, the MSRV check on
Rust 1.89, the `thumbv7m-none-eabi` no_std checks, `scripts/test-packages.sh` (all three
published crates tested from their `.crate` files), and the fuzz crate's clippy. A 20-second
cargo-fuzz smoke run of `cgp_component`, `type_product` and `delegate_components` found no crash.
