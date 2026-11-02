# Guide

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
