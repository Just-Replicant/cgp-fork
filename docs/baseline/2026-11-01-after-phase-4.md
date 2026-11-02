# After phase 4: 2026-11-01 (branch `phase-4`)

Measured on the uncommitted `phase-4` tree. Parent is `12e536cc` (`main`).

| Measure | Value |
|---|---|
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Tests passed (`cargo test --workspace --all-features`) | 987 |
| Tests failed | 0 |
| Tests ignored | 58 |
| Tests passed (`--workspace --no-default-features`) | 979 |
| Tests ignored (no default features) | 58 |
| Clippy (`-D warnings`, all targets; all features and no default features) | clean |
| `cargo doc` (`-D warnings --cfg docsrs --all-features --no-deps`) | clean |
| `cargo +nightly fmt --all -- --check`, and `fuzz/` | clean |
| Example `runtime` (`--features tokio`) | prints `spawned`, then `slept` |
| Compile-time bench (200 components, wired and checked) | 26.79 s (budget 30 s, one Windows run) |
| Facade `src` | 201 files, 8662 lines |
| Macro crates `src` | 419 files, 29455 lines |
| Facade integration tests | 391 files, 38639 lines |

Compared with [after phase 3](2026-10-31-after-phase-3.md) (949 passed, 56 ignored): 38 more tests pass and 2 more are ignored. Eight tests run only with `--all-features`: the seven tokio and smol adapter tests, and the tokio module's wiring doctest. The `#[cgp_async]` example stays ignored, because that doctest lives in the proc-macro crate and cannot depend on the facade. The compile-time bench was not re-run on this pass; the figure above is the earlier Windows run and is under the 30 s budget.

`long_names_do_not_overflow_the_stack` walks a 5000-character symbol in a loop. Quoting and dropping that type recurse once per character and abort a debug test on Windows, so the test does neither. It still quotes a name of `MAX_IDENT_LEN`.

Not run here: Rust 1.89 (that toolchain is not installed), `thumbv7m-none-eabi` and `cargo hack` (that target is not installed), coverage, `cargo deny`, `cargo audit`, `cargo semver-checks`, cargo-fuzz runs, and the macOS and Linux matrices. CI already runs `cargo test --workspace --all-features` on those three operating systems.
