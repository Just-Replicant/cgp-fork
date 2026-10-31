# ADR 0006: One `Handler` and one `AsyncHandler` instead of nine handler components

- Status: Accepted
- Date: 2026-10-31
- Plan: [rewrite plan §7.3, §7.4](../rewrite-plan.md#74-handlers-and-dispatch-cgp-fork-handler), phase 3

## Context

The 0.10 handler family has nine components (`Computer`, `ComputerRef`, `TryComputer`,
`TryComputerRef`, `AsyncComputer`, `AsyncComputerRef`, `Handler`, `HandlerRef`, `Producer`), one
per combination of owned/borrowed input, fallible or not, and sync or async. Every adapter and
dispatcher exists once per combination: five `promote*` modules and six `Match*With*Handlers*`
dispatchers. The data side is split the same way (`HasExtractor`/`HasExtractorRef`/
`HasExtractorMut`, `ToFields`/`ToFieldsRef`).

ADR 0001 (hard fork before phase 3) is still *Proposed*. Nothing in this record depends on it:
the change is additive and keeps the 0.10 API.

## Decision

- A new module, `cgp_fork::handler`, holds two components:
  `Handler<Context, Code, Input> { type Output; fn handle(..) }` and
  `AsyncHandler<Context, Code, Input> { type Output; async fn handle_async(..) }`. Borrowing is in
  the `Input` type (`&'a T`), fallibility in `Output = Result<_, Context::Error>`, a producer is
  `Input = ()`.
- Combinators: `Promote<P>` (sync to async), `PromoteOk<P>` (infallible to fallible),
  `PromoteDeref<P>`, `IgnoreInput<P>`, `ReturnInput`, `Pipe<Handlers>` and `TryPipe<Handlers>`.
  Each implements both components where that makes sense, so there is one adapter per job.
- Dispatch: `MatchWith<Handlers, Strategy = All | First, M = Owned | Ref | Mut>` with the handler
  lists `OnField`, `FieldValue`, `VariantHandlers`, `MatchFields`/`MatchValues`, and the record
  builders `BuildWith`, `TryBuildWith`, `SetField`, `Merge`, `MergeOutputs`. It replaces the six
  dispatchers.
- Data: a sealed access mode (`Owned`, `Ref`, `Mut`) with `ModeExtractor<Input>` and
  `ModeFields<Input>`. These are the mode-generic interface; the per-mode traits stay as the
  primitives the derives implement, so `#[derive(CgpData)]` output is unchanged.
- The 0.10 family (`extra::handler`, `extra::dispatch`, `extra::monad`, `#[cgp_computer]`,
  `#[cgp_producer]`, `#[cgp_auto_dispatch]`) moves behind a new `compat-0.10` feature, enabled by
  `full` and therefore by default. `cgp_fork::compat` bridges the families: `Compat<P>`
  turns a 0.10 provider into a unified one and a unified provider into every 0.10 component;
  `CompatTry`, `CompatRef`, `CompatTryRef` and `CompatProducer` cover the cases a single
  impl cannot.
- The unified names are not in `cgp_fork::prelude`, which already exports the 0.10 `Handler`;
  they are in `cgp_fork::handler::prelude`. Like the 0.10 prelude it exports provider traits
  but not the consumer traits (`CanHandle`, `CanHandleAsync`), because a provider and its
  consumer trait share the method name and `Provider::handle(..)` would be ambiguous.
- `FinalizeExtract` carries `#[diagnostic::on_unimplemented]`, so a `MatchWith` that misses a
  variant says so (`tests/ui/match_with_missing_variant.rs`).

## Consequences

- Default users see no change. A crate with `default-features = false` that uses the 0.10
  handlers must enable `compat-0.10`.
- Every 0.10 test still passes with `compat-0.10`; without it the 0.10 tests are compiled out.
  The ported tests are in `tests/unified_handlers/` and run with and without the feature
  (the `compat_bridge` cases need it).
- Follow-up (0.11.0 deprecations):
  - The 0.10 names are `#[deprecated(since = "0.11.0")]`, each note naming its replacement
    (see the CHANGELOG). The macro blocker is resolved: `#[cgp_handler]` replaces
    `#[cgp_computer]`/`#[cgp_producer]`, `#[derive_handler]` replaces `#[derive_promote]`, and
    `#[cgp_auto_dispatch]` generates the unified API (its 0.10 wiring through `Compat` is
    emitted inside `__cgp_compat_0_10!`, which expands to nothing without `compat-0.10`). The
    deprecated macros stay and their expansions carry `#[allow(deprecated)]`, so a user is
    warned once, at the attribute. `#[derive_promote]` is an attribute inside
    `#[cgp_component]`, which `#[deprecated]` cannot reach; the expansion names the hidden
    deprecated constant `macro_prelude::DERIVE_PROMOTE` with the attribute's span instead.
  - The 0.10 modules, `compat` and the 0.10 tests allow `deprecated` locally (module or file
    level), never crate-wide. `tests/ui/deprecated_*.rs` pin the warnings and
    `tests/ui_pass/` checks the replacements compile under `deny(warnings)`.
  - `MapTypeRef` is merged into `Mode`: the access-mode trait carries the borrow-aware
    `type Map<'a, T: 'a>`, `Owned`/`Ref`/`Mut` implement it, and the derived extractors use it.
    `MapTypeRef` remains as a deprecated trait implemented for every `Mode`, and
    `IsRef`/`IsMut`/`IsOwned` as deprecated aliases. `MapType` (`type Map<T>`) cannot be merged
    with it: a GAT without a lifetime cannot be `&'a T` in an impl (E0309), and `MapType` serves
    the state and type-function maps (`MapFields`, `ToMerge`), not access modes.
  - The unified items the generated code names are in the hidden `macro_prelude::unified`
    module, not renamed re-exports, so diagnostics keep the public paths
    (`cgp_fork::handler::Handler`).
- Not done, deliberately:
  - The monad layer is neither deprecated nor behind its own `unstable-monad` feature: the
    unified family has no monadic replacement, and the 0.10 dispatchers are built on it, so it
    shares `compat-0.10`.
  - The components stay in the facade crate; the `cgp-fork-handler` crate split belongs with
    the crate layout of the hard fork.
  - The async method is `handle_async`, not `handle`, so a type can implement both components
    and be called without naming the trait.
