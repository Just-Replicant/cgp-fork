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
- Limits: see "Phase 1b follow-up" below.
- Compile time: the 200-component benchmark is within noise of `main` (see
  `docs/baseline/2026-10-31-after-phase-2.md`).

## Phase 1b follow-up: the lower error lines

The first line of every case was readable; the `help:` and `note:` lines below it still printed
type-level strings. What stable Rust (MSRV 1.89) allows was tried case by case.

### Done

- `#[diagnostic::do_not_recommend]` (stable since 1.85) on every `DelegateComponent` impl that
  `delegate_components!` emits. A missing key no longer lists up to ten unrelated wiring tables
  (`BuildAndMergeOutputs<…>`, `MatchFirstWithFieldHandlersInputs<…>`, "and N others") under
  "the following other types implement trait `DelegateComponent<Key>`". `unwired_check` and
  `unwired_consumer` lost those ten lines each; the new `missing_entry` case pins the remaining
  output.
- The attribute's `#` and `[ ]` take the entry's span, like the impl's own boundary tokens
  (`override_item_span`). Left at `call_site`, they widened the impl's span and the "but trait
  `DelegateComponent<NamerComponent>` is implemented" help pointed at the whole
  `delegate_components!` block instead of the entry. The path inside stays at `call_site`, so
  rust-analyzer navigation on the key is unaffected.
- `missing_dependency` is unchanged: the "required for `ProviderX` to implement …" chain that
  names the unmet requirement comes from `IsProviderFor` impls, which are left alone.

### Not possible on stable

| Line | Why it stays |
| --- | --- |
| "but trait `HasField<Symbol<5, Chars<…>>>` is implemented for it" (`missing_field`) | rustc's closest-impl help. `on_unimplemented` cannot change `help:` lines. `do_not_recommend` on the derived `HasField` impls was tried: the help stayed and its span moved from the field to the `#[derive]`, so it was reverted. |
| `Symbol<N, Chars<…>>` spelled out as a type | rustc prints the normalized type. Type aliases are not kept in diagnostics, and a readable name needs a const-generic `&str` (`adt_const_params`, nightly). |
| "this is an associated function, not a method" (`E0599` on an unwired context) | rustc's label for provider methods taking `context: &Context`. Removing it means renaming or reshaping provider methods, a breaking change. |
| Second error on the `IsProviderFor` supertrait and its "other types implement `IsProviderFor`" list (`unwired_consumer`) | `do_not_recommend` on `IsProviderFor` impls would also hide the "required for … to implement …" chain that `missing_dependency` relies on. The dependency chain is worth more than the shorter error. |
| `#[diagnostic::on_unimplemented]` text per impl | rustc reads it only on traits, so a message cannot depend on which table or entry is missing beyond the trait's generic names. |
| Generic tables whose entry has a `where` clause | With `do_not_recommend`, rustc may report the outer `DelegateComponent` obligation instead of the inner unmet bound. No test hits this; noted as a known edge case. |

`#[diagnostic::on_unimplemented]` and `#[diagnostic::do_not_recommend]` are ignored with a lint
by compilers that do not know them, so neither raises the MSRV.
