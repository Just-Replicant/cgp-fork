# Coverage-guided fuzzing for `cgp-fork-macro`

This directory is a [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) crate. It is not a
workspace member and is never published.

libFuzzer cannot link a proc-macro crate, so `src/lib.rs` compiles the macro implementation
modules of `crates/macros/cgp-fork-macro` directly, by `#[path]`. Only the `#[proc_macro*]`
wrappers in that crate's `lib.rs` are left out. The entrypoint list and seed inputs come from the
in-tree robustness fuzzer (`src/macro_tests/robustness_tests.rs`). Both fuzzers therefore cover
the same macros, and a new entrypoint added there is fuzzed here too.

## Running

From the repository root, with a nightly toolchain and `cargo install cargo-fuzz`:

```sh
# Optional: start from the robustness seeds (one input per seed and entrypoint).
cargo +nightly run --manifest-path fuzz/Cargo.toml --example write_seeds -- fuzz/corpus/entrypoints

cargo +nightly fuzz run entrypoints -- -max_total_time=600
```

The input format: the first byte selects the entrypoint (modulo the number of entrypoints). The
rest is UTF-8 text, split at the first `\x01` byte into the attribute and the item. Input that does
not lex as Rust tokens is skipped, because rustc would reject it before a macro runs. A panic is
a finding: a macro must turn bad input into a `syn::Error`.

A crash is saved under `fuzz/artifacts/entrypoints/`. Reproduce and minimise it with:

```sh
cargo +nightly fuzz run entrypoints fuzz/artifacts/entrypoints/crash-…
cargo +nightly fuzz tmin entrypoints fuzz/artifacts/entrypoints/crash-…
```

Then add the minimised input as a `macro_tests` regression test before fixing it. The first
finding, raw identifiers in derived names (`check_components! { r#r {} }`), is pinned in
`src/macro_tests/raw_ident_tests.rs`.

## CI

- Every run: the `Macro fuzzing (deep)` job checks that this crate still builds
  (`cargo clippy --manifest-path fuzz/Cargo.toml --bins --examples`) and runs the in-tree
  fuzzer.
- Weekly and on demand: the non-blocking `Coverage-guided fuzzing (cargo-fuzz)` job seeds the
  corpus and fuzzes for 30 minutes. It uploads `fuzz/artifacts/` if it finds a crash.

## What is not done yet

The improvement plan (I-9) proposed moving the parsers into an internal non-proc-macro library
crate, with one fuzz target per entrypoint. Including the modules by path gets the same
coverage-guided fuzzing without restructuring the published crates. It has these limits:

- One target dispatches over all entrypoints by its first byte. It is not split into one target
  per entrypoint.
- There is no structure-aware (`arbitrary`) input generation. The mutator works on raw bytes and
  most inputs fail to lex. The seeds and libFuzzer's dictionary learning offset this, but not
  fully.
- The plan's verification run, one hour per target without crashes, has not been done. Locally,
  a 300-second run (365k executions, about 18k coverage edges) found no crash after the
  raw-identifier fix.
