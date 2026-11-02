# After phase 6: 2026-11-02 (branch `phase-6`)

Measured on the uncommitted `phase-6` tree. Parent is `e2acb568` (`phase-5`).

| Measure | Value |
|---|---|
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Tests passed (`cargo test --workspace --all-features`) | 1037 |
| Tests failed | 0 |
| Tests ignored | 59 |
| Tests passed (`--workspace --no-default-features`) | 1028 |
| Tests ignored (no default features) | 59 |
| Clippy (`-D warnings`, all targets; all features and no default features) | clean |
| `cargo +nightly fmt --all`, and `fuzz/` | clean |
| Book examples compiled by `cgp-fork-doctests` | 15 |
| `cargo xtask migrate` unit tests | 10 passed, including the check that `book/src/migration.md` embeds `migration_table()` |
| Example `error_enum` | prints `read note.txt: disk` |
| `cargo semver-checks` 0.51.0, registry baseline | `v0.10.0 -> v1.0.0`, major, 260 checks skipped, no update required. crates.io has no `cgp-fork` 1.0.0 |
| `cargo semver-checks --baseline-rev e10cf821 --release-type patch` | 229 checks passed, 31 skipped, no update required |
| Facade `src` | 206 files, 8899 lines (unchanged by this phase) |
| Macro crates | 426 files, 32990 lines |
| Facade integration tests | 400 files, 39508 lines |

Compared with [after phase 5](2026-11-02-after-phase-5.md) (1003 passed with all features, 992 with no default features, 58 ignored): 34 more tests pass with all features, and 36 more pass with no default features. The new tests are the error-enum derive, its parser rejections, the migrate codemod, and the 15 book doctests. Both of today's runs report 59 ignored. This phase adds no `#[ignore]`.

`cargo clippy --workspace --all-targets -- -D warnings` reported `large_enum_variant` on `DelegateKey`. That enum now has the same allow as the other syn 3 enums. Both feature sets are clean after that.

Not run here: Rust 1.89 (that toolchain is not installed), `thumbv7m-none-eabi` and `cargo hack` (that target is not installed), the compile-time bench, coverage, `cargo deny`, `cargo audit`, cargo-fuzz runs, and the macOS and Linux matrices. `mdbook` is not installed; the book examples are the doctests above.
