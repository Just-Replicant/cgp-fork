# ADR 0002: Move the macro implementation into a library crate

- Status: Accepted
- Date: 2026-10-30
- Plan: [rewrite plan §6 and §7.1](../rewrite-plan.md#6-new-architecture), phase 1

## Context

All parsers and code generators lived inside the `cgp-fork-macro` proc-macro crate. A proc-macro
crate can only export `#[proc_macro*]` functions and cannot be linked into an ordinary binary, so:

- the cargo-fuzz crate compiled the macro sources a second time by `#[path]` (`fuzz/src/lib.rs`);
- the fuzzer could only have one target, selecting the entrypoint from the first input byte;
- no other tool (a wiring inspector, a migration codemod) could reuse the parsers.

## Decision

Split the macro crate in two:

- `cgp-fork-macro-impl`: an ordinary library with every parser, code generator, the
  `async_trait` desugaring, the snapshot helpers (feature `snapshot`) and the macro unit tests.
  Its public surface is `#[doc(hidden)]`-level internals with no stability promise; it is published
  only because `cgp-fork-macro` depends on it.
- `cgp-fork-macro`: the proc-macro shim. It keeps the user-facing documentation and each entrypoint
  is a one-line call into the library.

The entrypoint list that both fuzzers use (`registry`) is public in the library, so the cargo-fuzz
crate depends on the library instead of including files by path, and has one target per
entrypoint.

## Consequences

- Users still add only `cgp-fork`; the facade, its prelude and every macro name are unchanged.
- One more crate to publish, in the same release-plz version group.
- Generated code is unchanged; every expansion snapshot in the test suite passes unmodified.
