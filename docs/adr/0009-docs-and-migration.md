# ADR 0009: The book, the migration table, and derived error enums

- Status: Accepted
- Date: 2026-11-02
- Plan: [rewrite plan](../rewrite-plan.md) phase 6, §7.6 and §11

## Context

Phase 5 left two items from the plan: a guide that compiles, and
`#[cgp_auto_error]` on an error enum. The 0.10 handler names are deprecated and
still the default, so a 1.0 tree needs a single map from each old name to its
replacement and a codemod that applies that map. Publishing a second
documentation crate, or checking the book in with `mdbook` as a CI dependency,
would add a toolchain the rest of the build does not use. The crate version is
already 1.0.0; dropping `compat-0.10` from `default` would be a breaking change
on a released version.

## Decision

- `book/` is an mdBook. Its `rust` and `rust,no_run` fences are extracted by
  `cgp-fork-doctests` and compiled with the rest of the workspace tests. A
  `rust,ignore` fence in the book fails that build. `mdbook` is not a CI
  dependency.
- `xtask` owns the migration table. `cargo xtask migrate` rewrites imports,
  absolute paths, and attributes from that table. `cargo xtask migrate --guide`
  prints it, and `book/src/migration.md` embeds that print between HTML markers.
  A unit test fails when the two differ.
- The codemod does not rewrite method names. `compute`, `try_compute`,
  `produce`, and `send_run` stay. The new methods are `handle` and
  `handle_async`. A path that starts at the new `handler` module is not an old
  name. A rename that appends type arguments runs only when the source writes
  exactly the number of arguments the rule expects; anything else is a warning
  and is left unchanged. Names that are not a one-to-one path (`Promote`,
  `PipeHandlers`, `ComposeHandlers`, the `Build*` pairs, `CanSendRun`) are
  warnings too.
- `#[cgp_auto_error]` on an enum derives `Display`, `core::error::Error`, and
  `From`. It does not emit the error providers; the impl form still does that.
  `#[derive(Debug)]` stays on the user side of the attribute, because the derive
  is expanded after it and `Error` requires `Debug`. The macro does not emit
  `Debug`.
- `compat-0.10` stays in `default`. `cargo xtask ci` runs nightly fmt, clippy
  with and without default features, and the same two test commands. The GitHub
  workflow keeps nextest, the operating-system matrix, semver-checks, deny, and
  fuzz.

## Consequences

- A book example that does not compile fails `cargo test -p cgp-fork-doctests --doc`.
- Changing a rename means changing `xtask` and regenerating the marked section
  of the migration page. The test is the check.
- An application that omitted type arguments on `MatchWithHandlersRef` is not
  rewritten into a different mode. The warning names the line.
- The enum derive is a behavior of an existing attribute. It adds no public
  item to the facade, so it is not a semver change against 1.0.0.
