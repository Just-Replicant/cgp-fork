# ADR 0003: Resolve the facade's path in generated code

- Status: Accepted
- Date: 2026-10-30
- Plan: [rewrite plan §3 item 2 and §7.1](../rewrite-plan.md#71-macro-pipeline-cgp-fork-macro-impl), phase 1

## Context

Every macro expansion named the facade as `::cgp_fork::macro_prelude::…`
(`macro_core/macros/export.rs`). A user who renames the dependency,

```toml
[dependencies]
cgp = { package = "cgp-fork", version = "0.10" }
```

got `error[E0433]: cannot find cgp_fork in the list of imported crates` from every macro.

## Options

1. Require an explicit `crate = path` argument on every macro call. Precise, but every invocation
   in a renamed project needs it, and declarative helpers cannot pass it along.
2. Read the calling crate's `Cargo.toml` at expansion time and find the name `cgp-fork` is
   imported under. This is what `proc-macro-crate` does; serde-adjacent and Bevy-style ecosystems
   use the same approach.
3. A global override through an environment variable.

## Decision

Option 2, written in-crate rather than with `proc-macro-crate`, to keep the macro crates'
dependency set at `syn`, `quote` and `proc-macro2` (and so keep downstream build times flat).
The resolver:

- reads `$CARGO_MANIFEST_DIR/Cargo.toml` and looks for a key whose table sets
  `package = "cgp-fork"` in `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]` and
  `[target.*.…dependencies]`; a match yields `::<key with - replaced by _>`;
- falls back to `::cgp_fork` in every other case (no manifest, not found, the facade itself whose
  `lib.rs` has `extern crate self as cgp_fork`, or a parse it does not understand);
- also follows `name = { workspace = true }` to `[workspace.dependencies]` in the nearest
  ancestor manifest that declares `[workspace]`;
- caches the answer per manifest directory and thread, and re-reads the manifest when its
  modification time changes (rust-analyzer's proc-macro server expands many crates in one
  process); only strings are cached, never token handles;
- can be forced with the environment variable `CGP_FORK_CRATE`, set to a crate name (`cgp`) or a
  path (`::reexports::cgp`), for build systems without Cargo such as Bazel or Buck. Cargo does not
  track this variable, so changing it needs a clean rebuild of the affected crates.

The parse is a small line-oriented scan of the TOML subset Cargo manifests use for dependencies,
covering both `name = { package = "cgp-fork", … }` and `[dependencies.name]` tables.

## Consequences

- Renamed dependencies work; `crates/tests/renamed-dep` is a workspace crate that imports
  `cgp-fork` as `cgp` and exercises the main macros.
- Expansion for ordinary users is byte-for-byte unchanged (still `::cgp_fork`).
- A manifest edit that only renames the dependency is picked up on the next build, because Cargo
  rebuilds the crate when its manifest changes.
- `cgp_preset!` generates a `macro_rules!` that expands in the crates using the preset. Callers
  pass their own facade path into `with_components!` (`crates/tests/preset-consumer` defines the
  preset under the name `cgp` and uses it under `cgp_fork`).
- A normal `[dependencies]` entry wins over a target-specific table, then a dev-dependency.
  Build-dependencies apply only when the crate being compiled is a build script. A
  `[target.'cfg(...)'.…]` table counts only when that `cfg` matches the build. A key of
  `cgp-fork` with no `package` field is a real hit, so it is not hidden by a rename in another
  table.
- Implementation: `crates/macros/cgp-fork-macro-impl/src/crate_path.rs`; unit tests in
  `src/macro_tests/crate_path_tests.rs`.
