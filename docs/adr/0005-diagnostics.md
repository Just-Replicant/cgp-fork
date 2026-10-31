# ADR 0005: Readable diagnostics with `#[diagnostic::on_unimplemented]`

- Status: Accepted
- Date: 2026-10-31
- Plan: [rewrite plan §7.2](../rewrite-plan.md#72-core-cgp-fork-core), phase 2

## Context

A wiring mistake surfaced as rustc's generic "the trait bound `App: CanUseComponent<…>` is not
satisfied", and a missing field as `HasField<Symbol<4, Chars<'n', Chars<'a', …>>>>`. The README
sent users to an external tool for readable errors. MSRV 1.89 supports
`#[diagnostic::on_unimplemented]` (stable since 1.78), which replaces the first line, the label,
and adds notes when a trait is not implemented. It accepts `{Self}` and the trait's generic
parameter names, and is ignored by older or other compilers instead of failing.

## Decision

- Core traits get a message: `CanUseComponent` ("`App` cannot use the component
  `GreeterComponent`"), `IsProviderFor`, `DelegateComponent`, `HasField`, `HasFieldMut`. The
  field messages never print the `Symbol<…>` tag in the first line; a note explains how to read it.
- The macros attach a message to the traits they generate:
  - the consumer trait of `#[cgp_component]` (and `#[cgp_type]`, `#[cgp_getter]`), naming the
    component to wire;
  - the provider trait, written for both cases its `Self` can be (a context or wiring table
    without an entry, or a provider without an impl);
  - a `#[cgp_auto_getter]` trait, naming the fields it reads.
- Only the emitted traits carry the attribute. Impls derived from those traits do not copy it
  (rustc ignores it on impls and warns).
- A message the user already wrote on the consumer trait wins; it is no longer copied onto the
  generated provider trait, whose `Self` is a different type.

## Consequences

- Every `tests/ui` case reads without a type-level string in its first line (the phase 2 exit
  criterion). New cases: `missing_field`, `unwired_consumer`, `unwired_check`.
- Expansion snapshots of components and auto getters show the new attributes; nothing else in
  them changed.
- Limits:
  - rustc's own `help:` lines (the closest `HasField` impl it found, the list of other types
    implementing a trait) still print type-level strings below the first line.
  - A method call on an unwired context is an `E0599`. Its first line now uses the provider
    trait's message, but rustc still adds "this is an associated function, not a method".
  - Field names in readable form need a nightly-only or future rustc feature; the getter
    messages name them because the macro knows them.
- Compile time: the 200-component benchmark is within noise of `main` (see
  `docs/baseline/2026-10-31-after-phase-2.md`).
