# Coverage-guided fuzzing for the `cgp-fork` macros

This directory is a [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) crate. It is not a
workspace member and is never published.

libFuzzer cannot link a proc-macro crate, so this crate depends on `cgp-fork-macro-impl`, the
ordinary library behind the `cgp-fork-macro` shim (ADR 0002). The entrypoint list and the seed
inputs come from its `registry` module, which the in-tree robustness fuzzer
(`src/macro_tests/robustness_tests.rs` in that crate) uses too, so both fuzzers cover the same
macros.

## Targets

- `entrypoints`: every macro; the first input byte selects one (modulo their number).
- One target per entrypoint, named after it (`cgp_component`, `delegate_components`, …).
  Type-level macros take a `type_` prefix (`type_symbol`, `type_product`), so `Product` and
  `product` stay distinct on case-insensitive file systems. `cargo +nightly fuzz list` shows them.

The single-entrypoint targets are generated. After adding a macro's `Pipeline` type to
`registry.rs`, run

```sh
cargo run --manifest-path fuzz/Cargo.toml --example write_targets
```

The `registry_tests` in `cgp-fork-macro-impl` fail when a target is missing or stale.

## Running

From the repository root, with a nightly toolchain and `cargo install cargo-fuzz`:

```sh
# Optional: start from the robustness seeds (one directory per target).
cargo +nightly run --manifest-path fuzz/Cargo.toml --example write_seeds -- fuzz/corpus

cargo +nightly fuzz run entrypoints -- -max_total_time=600
cargo +nightly fuzz run cgp_component -- -max_total_time=600
```

Input format: UTF-8 text, split at the first `\x01` byte into the attribute and the item (for
`entrypoints`, after the selector byte). Input that does not lex as Rust tokens is skipped,
because rustc would reject it before a macro runs. A panic is a finding: a macro must turn bad
input into a `syn::Error`.

A crash is saved under `fuzz/artifacts/<target>/`. Reproduce and minimise it with:

```sh
cargo +nightly fuzz run <target> fuzz/artifacts/<target>/crash-…
cargo +nightly fuzz tmin <target> fuzz/artifacts/<target>/crash-…
```

Then add the minimised input as a `macro_tests` regression test before fixing it. The first
finding, raw identifiers in derived names (`check_components! { r#r {} }`), is pinned in
`src/macro_tests/raw_ident_tests.rs`.

## CI

- Every run: the `Macro fuzzing (deep)` job checks that this crate builds
  (`cargo clippy --manifest-path fuzz/Cargo.toml --bins --examples`) and runs the in-tree
  fuzzer.
- Weekly and on demand: the non-blocking `Coverage-guided fuzzing (cargo-fuzz)` job seeds the
  corpora, fuzzes `entrypoints` for 15 minutes and then each single-entrypoint target for
  20 seconds. It uploads `fuzz/artifacts/` if it finds a crash.

## Not done yet

- No structure-aware (`arbitrary`) input generation. The mutator works on raw bytes and most
  inputs fail to lex. The seeds and libFuzzer's dictionary learning offset this, but not fully.
- The one-hour run per target from the old improvement plan is not a phase-1 exit criterion.
  It stays deferred: thirty-eight hour-long runs do not fit this phase, and a 20-second smoke
  (about 280k to 330k executions of `cgp_component`, `type_product`, and `delegate_components`
  on 2026-10-30, no crash) does not by itself show the parsers are clean. The standing check is
  the weekly `Coverage-guided fuzzing (cargo-fuzz)` job: 15 minutes on `entrypoints`, then 20
  seconds on each single-entrypoint target. A crash there is a finding; lengthen a target's run
  when investigating one, rather than blocking the rewrite on a full hour each.
