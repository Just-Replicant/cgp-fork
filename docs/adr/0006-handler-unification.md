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
- Not done, deliberately:
  - `#[deprecated]` on the 0.10 names. The macros above and `#[derive_promote]` still generate
    0.10 code, so a deprecation would warn in code that has no replacement yet. It comes with
    the macro port.
  - The monad layer is not behind its own `unstable-monad` feature: the 0.10 dispatchers are
    built on it, so it shares `compat-0.10`.
  - `MapType`/`MapTypeRef` are not merged, and the components stay in the facade crate; the
    `cgp-fork-handler` crate split belongs with the crate layout of the hard fork.
  - The async method is `handle_async`, not `handle`, so a type can implement both components
    and be called without naming the trait.
