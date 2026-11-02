# Guide

The book in [`book/`](../../book/src/SUMMARY.md) is the guide, the reference, the
error catalog, and the 0.10 to 1.0 migration page. `cargo test -p cgp-fork-doctests --doc`
compiles its Rust examples. `mdbook serve book` renders it when mdBook is installed.

These pages cover the phase 5 capabilities. Each one has an example under
`crates/main/cgp-fork/examples/` and tests under `crates/main/cgp-fork/tests/capabilities/`.

| Page | What it covers |
|---|---|
| [Checked wiring](checked-wiring.md) | The check `delegate_components!` emits, and `#[unchecked]` |
| [Profiles](profiles.md) | `#[cfg_wiring]` |
| [Presets](presets.md) | `#[version]` and `override` |
| [Mocks](mocks.md) | `#[cgp_component(..., mock)]` |
| [Tracing and defmt](tracing.md) | `CanLog` backends |
| [Wiring graph](wiring-graph.md) | `cargo xtask wiring` |
