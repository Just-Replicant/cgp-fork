# After phases 1b and 2: 2026-10-31 (branch `phase-1b-2`)

Measured with `bash scripts/baseline.sh` on the commit before the documentation commit
(`89345e8e`; every phase 1b and phase 2 code change).

| Measure | Value |
|---|---|
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Tests passed | 895 |
| Tests failed | 0 |
| Tests ignored | 56 |
| Clippy (`-D warnings`, all targets, all features) | clean |
| Compile-time bench (200 components, wired and checked) | 0.87 to 0.93 s (four runs) |
| Facade `src` | 186 files, 5850 lines |
| Macro crates `src` | 402 files, 27408 lines |
| Facade integration tests | 350 files, 35720 lines |

Compared with `main` (`d38dc8bc`, which includes phases 0 and 1) on the same machine and day:

| Measure | `main` | After 1b and 2 | Change |
|---|---|---|---|
| Tests passed | 877 | 895 | +18: 11 `ir`, 2 `registry`, 5 `diagnostic_attrs`; the 3 new `tests/ui` cases run inside the single `ui` test |
| Tests failed | 0 | 0 | |
| Tests ignored | 56 | 56 | the macro-crate doc examples, compiled in `cgp-fork-doctests` instead |
| Clippy | clean | clean | |
| Compile-time bench | 0.80, 0.84, 0.97 s | 0.87, 0.89, 0.89, 0.93 s | within noise on this machine |

The phase 1 baseline recorded 0.70 s on a quieter day; `main` measured 0.80 to 0.97 s alongside
this branch, so the comparison above is the one to read.

Also green on this commit: `cargo +nightly fmt --all -- --check` (workspace and `fuzz/`),
`cargo doc` with `-D warnings --cfg docsrs`, clippy without default features, the MSRV check on
Rust 1.89 (all features and no default features), the `thumbv7m-none-eabi` no_std checks, the
`cargo hack` feature powerset (host and `thumbv7m-none-eabi`), `scripts/test-packages.sh` (all
three published crates tested from their `.crate` files), the fuzz crate's clippy, the doctests,
and the examples build.

Not run here: the coverage job (`cargo llvm-cov`), `cargo deny`, `cargo audit`,
`cargo semver-checks`, cargo-fuzz runs, the knowledge-base reference check, and the 3-OS and
beta/nightly matrices.
