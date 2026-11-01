# After the phase 1b follow-up fixes and phase 3: 2026-10-31 (branch `phase-3-fixes`)

Measured with `bash scripts/baseline.sh` on the commit before the documentation commit
(`6781d166`; every fix and phase 3 code change).

| Measure | Value |
|---|---|
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Tests passed | 949 |
| Tests failed | 0 |
| Tests ignored | 56 |
| Clippy (`-D warnings`, all targets, all features) | clean |
| Compile-time bench (200 components, wired and checked) | 0.79 to 1.06 s (three runs) |
| Facade `src` | 194 files, 7547 lines |
| Macro crates `src` | 412 files, 28441 lines |
| Facade integration tests | 369 files, 37362 lines |

Compared with `main` (`1b17cc1b`) on the same machine and day:

| Measure | `main` | This branch | Change |
|---|---|---|---|
| Tests passed | 909 | 949 | +11 `validation_tests` (fixes), +29 `unified_handlers_tests` (phase 3); the 5 new `tests/ui` cases run inside the single `ui` test |
| Tests failed | 0 | 0 | |
| Tests ignored | 56 | 56 | |
| Clippy | clean | clean | |
| Compile-time bench | 0.77, 1.05 s | 0.79, 0.83, 1.06 s | within noise on this machine |

`cargo test -p cgp-fork --no-default-features` passes 456 tests: the 0.10 handler tests are
compiled out without `compat-0.10`, and the unified tests that do not use the bridges still run.

Also green on this commit: `cargo +nightly fmt --all -- --check` (workspace and `fuzz/`),
`cargo doc` with `-D warnings --cfg docsrs`, clippy without default features, the MSRV check on
Rust 1.89 (all features and no default features), the `thumbv7m-none-eabi` no_std checks (with
and without `compat-0.10`), the `cargo hack` feature powerset (host and `thumbv7m-none-eabi`),
`scripts/test-packages.sh`, and the fuzz crate's `cargo check`.

Not run here: the coverage job, `cargo deny`, `cargo audit`, `cargo semver-checks`, cargo-fuzz
runs, the knowledge-base reference check, and the 3-OS and beta/nightly matrices.
