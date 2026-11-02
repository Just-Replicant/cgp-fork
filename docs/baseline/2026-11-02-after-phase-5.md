# After phase 5: 2026-11-02 (branch `phase-5`)

Measured on the uncommitted `phase-5` tree. Parent is `f7f4e5d8` (`main`).

| Measure | Value |
|---|---|
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |
| Tests passed (`cargo test --workspace --all-features`) | 1003 |
| Tests failed | 0 |
| Tests ignored | 58 |
| Tests passed (`--workspace --no-default-features`) | 992 |
| Tests ignored (no default features) | 58 |
| Clippy (`-D warnings`, all targets; all features and no default features) | clean |
| `cargo doc` (`-D warnings --cfg docsrs --all-features --no-deps`) | clean |
| `cargo +nightly fmt --all -- --check`, and `fuzz/` | clean |
| UI tests (`--all-features --test ui_tests`) | 27 passed |
| Examples (`--all-features`) | `hello` prints `Hello, Ferris!`; `swap_context` prints two `started` lines; `profile` prints the system clock; `preset_override` prints `hello!`; `mock` prints `hello, Ferris`; `tracing` prints `logged`; `defmt` names `DefmtLogger`; `runtime` prints `spawned`, then `slept` |
| `cargo xtask wiring --format json` on `examples/preset_override.rs` | `LoudPreset` version 2, `GreeterComponent` override of `Base` |
| Facade `src` | 206 files, 8859 lines |
| Macro crates `src` | 424 files, 31296 lines |
| Facade integration tests | 399 files, 39078 lines |

Compared with [after phase 4](2026-11-01-after-phase-4.md) (987 passed, 58 ignored): 16 more tests pass with all features, and 13 more pass with no default features. The three that need a feature are the mock, tracing, and defmt capability tests. The other new tests cover checked wiring, profiles, preset overrides, the new parser rejections, and the wiring renderer. 58 stay ignored.

`missing_dependency` now fails twice: the default check on `delegate_components!` (`__CanUseApp`) and the explicit `check_components!` (`__CheckApp`). Host builds that enable `log-defmt` link `defmt` with `unstable-test`. Firmware (`target_os = "none"`) does not.

Not run here: Rust 1.89 (that toolchain is not installed), `thumbv7m-none-eabi` and `cargo hack` (that target is not installed), the compile-time bench, coverage, `cargo deny`, `cargo audit`, `cargo semver-checks`, cargo-fuzz runs, and the macOS and Linux matrices. CI already runs `cargo test --workspace --all-features` on those three operating systems.
