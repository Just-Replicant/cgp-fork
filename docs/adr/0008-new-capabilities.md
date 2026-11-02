# ADR 0008: Checked wiring, mocks, profiles, presets, and log backends

- Status: Accepted
- Date: 2026-11-02
- Plan: [rewrite plan §7.6–7.7](../rewrite-plan.md#76-errors-and-logging), phase 5

## Context

`delegate_components!` wired a context and left the check to a second macro. A table
could name a provider that did not implement the component, and the error showed up
only when a method was called. Presets could not name a version or say which parent
entry they replaced. Tests built a hand-written provider for every component. Nothing
recorded a `CanLog` detail with `tracing` or `defmt`, and nothing printed the wiring
as a graph.

ADR 0001 (hard fork) is still *Proposed*. These features stay in the `cgp-fork` facade,
the same way phase 4 left the runtime adapters there. A `cgp-fork-test` crate is not
added.

## Decision

- A `delegate_components!` whose context has no generic parameters also emits the
  `check_components!` trait `__CanUse{Context}`. `#[unchecked]` skips it. A generic
  table is not checked, because the check parameters would be `()`. Preset entries are
  checked through `with_components!`. `delegate_and_check_components!` does not check
  twice. It does accept `#[cfg_wiring]` on the table and emits the inspect const.
  `#[unchecked]` stays illegal there. `#[check_trait]` names the check trait.
- An `open` or `Key =>` entry is checked. `@Component.A.B` is
  `CanUseComponent<Component, (A, B)>` when this table delegates `Component`.
  A namespace lookup path is not that component, and a `for` loop's provider is
  abstract, so those stay an explicit `check_components!`. A lifetime parameter
  becomes `Life<'a>` in `IsProviderFor` and is not on the `@` path; `open` of that
  component uses `#[unchecked]` plus an explicit check.
- `#[cfg_wiring(predicate)]` is `#[cfg(predicate)]`. On the table it is copied onto
  every generated item. On an entry it gates that entry's wiring and its check.
  On a preset, a non-override key is absent when the predicate is false. An
  `override` forwards to the parent in that configuration, and the consumer check
  still names the key. The same cfg wraps the replayed key and the check.
- `delegate_components!`, `delegate_and_check_components!`, and `cgp_preset!` emit
  their wiring JSON inside `__cgp_inspect!`. The `inspect` feature keeps a private
  const; without the feature the tokens are dropped. `cargo xtask wiring` parses
  the same macros from source and prints JSON, Mermaid, or DOT. Edges run
  context → component → provider, plus preset parents and provider dependencies.
  It does not need the feature.
- `#[cgp_component(Name, mock)]` generates `MockName` and `NameMock` when the `mock`
  feature is on. The context stores `NameMock` in `{snake}_mock` and derives
  `HasField`. Methods return owned values. A closure wins over a queued return, and
  every call is counted. The closure runs without the handler cell borrowed, so it
  may call the same method. A method `where` clause is kept, with `Self` rewritten
  to the context. The generated items go through `__cgp_mock!`, so the feature
  is the facade's.
- `#[version(N)]` on `cgp_preset!` is `pub const VERSION: u64` in the preset module.
  `override` replaces a component a parent already wires. One parent can be omitted;
  several parents must be named. The parent's keys are not known while the macro runs,
  so the expansion emits a `PresetContains` bound that the compiler checks.
- `TracingLogger` and `TracingDebugLogger` (`log-tracing`) and `DefmtLogger`
  (`log-defmt`) implement `CanLog`. They are not in the default prelude. `tracing` is
  built without its default features. Host builds of `defmt` enable `unstable-test`
  so the call links without a device logger or the GNU linker script; firmware
  (`target_os = "none"`) keeps the real logger. `CanLog<Detail>` is generic, so a
  logger table uses `#[unchecked]` or an explicit `check_components!` with
  `#[check_params]`.

## Consequences

- A concrete table that names a provider the component cannot use stops compiling.
  Existing type-level maps and generic components that are not full contexts write
  `#[unchecked]`. That is the behavior change in rewrite plan §8.
- Host builds that enable `log-defmt` link `defmt` with `unstable-test`. A firmware
  target (`target_os = "none"`) does not, and still needs its own logger and linker
  script.
- The hidden JSON is in every expansion. Snapshots gain the `__cgp_inspect!` wrapper.
  The const exists in the caller's crate only with `--features inspect`.
- `cargo xtask wiring` depends on `cgp-fork-macro-impl`. That crate stays an
  implementation detail; the command is the supported interface.
- `#[cgp_auto_error]` from §7.6 and the `book/` from phase 6 are not part of this
  decision.
