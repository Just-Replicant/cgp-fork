# Wiring graph

`cargo xtask wiring` reads `delegate_components!`, `delegate_and_check_components!`,
and `cgp_preset!` from Rust source and prints one graph. The alias is
`.cargo/config.toml`. `cargo run -p xtask -- wiring` is the same command.

```text
cargo xtask wiring --format mermaid crates/main/cgp-fork/examples
cargo xtask wiring --format dot path/to/app.rs
cargo xtask wiring --format json .
```

The default format is Mermaid. An edge runs context → component → provider.
`wires` labels the first hop and `provides` the second. A cfg predicate replaces
`wires`. An override is `provides override {Parent}`. A preset draws a `parent`
edge to each parent's provider, and a context that uses a preset gets the preset's
entries with parents flattened in (a child's key replaces the parent's). A preset
that is not in the scan stays `context → {Preset}::Provider`, labeled `preset`.
Identical edges are drawn once. `#[cgp_provider]` and `#[cgp_impl]` add a `needs`
edge from the provider to each context bound that is not a marker or formatting
trait. `#[check_trait]` is not a profile.

JSON is one object per file: `path`, `tables`, `presets`, and `providers`.
Component and provider text is a single line. Mermaid and DOT escape quotes,
backslashes, and newlines.

The same per-table JSON is what `delegate_components!`, `delegate_and_check_components!`,
and `cgp_preset!` emit inside `__cgp_inspect!`. The `inspect` feature keeps a
private const in the caller's module (`dead_code` is allowed; nothing in the
program reads it). The xtask does not need the feature: it parses the source
with the macro crate. Providers are part of the xtask JSON only.

The renderer is tested in `cgp-fork-macro-impl` (`wiring::tests`).
