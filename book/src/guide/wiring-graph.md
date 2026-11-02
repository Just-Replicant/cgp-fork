# Wiring graph

`cargo xtask wiring` reads `delegate_components!`, `delegate_and_check_components!`,
and `cgp_preset!` and prints one graph. The same command is `cargo wiring` when
`.cargo/config.toml` aliases it, and `cargo run -p xtask -- wiring`.

```sh
cargo xtask wiring --format mermaid crates/main/cgp-fork/examples
cargo xtask wiring --format dot path/to/app.rs
cargo xtask wiring --format json .
```

The default format is Mermaid. An edge runs context, then component, then
provider. `wires` labels the first hop and `provides` the second. A cfg
predicate replaces `wires`. An override is `provides override {Parent}`. A
preset draws a `parent` edge, and a context that uses the preset gets the
preset's entries with parents flattened in. A preset that is not in the scan
stays `context → {Preset}::Provider`, labeled `preset`.

`#[cgp_provider]` and `#[cgp_impl]` add a `needs` edge for each context bound
that is not a marker or formatting trait.

JSON is one object per file: `path`, `tables`, `presets`, and `providers`. The
same per-table JSON is what those macros emit inside `__cgp_inspect!`. The
`inspect` feature keeps that const in the caller's module. The xtask does not
need the feature: it parses the source.
