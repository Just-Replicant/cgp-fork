# ADR 0007: `#[cgp_async]` and tokio/smol runtime adapters

- Status: Accepted
- Date: 2026-11-01
- Plan: [rewrite plan §7.5](../rewrite-plan.md#75-async-run-runtime), phase 4

## Context

`#[async_trait]` desugars `async fn` in a trait to `impl Future` and never adds `Send`. The name
is the popular `async-trait` crate's, and the behaviour is the opposite of that crate's default.
A `Send` future was a second component, `CanSendRun`, whose method was written out by hand as
`fn -> impl Future + Send`. Nothing in the crate could spawn, sleep, or time out on a real
runtime: `HasRuntime` only borrows a value the context stores.

Return-type notation is not stable, so a caller still cannot write `Context::run(..): Send`. A
proxy method on the concrete context is how `Send` is proved. ADR 0001 (hard fork) is still
*Proposed*. This change is additive and keeps the 0.10 names.

## Decision

- `#[cgp_async]` is the desugaring. `#[cgp_async(Send)]` adds `+ Send` to every desugared future.
  Any other argument is an error. On an impl the attribute is a no-op, because a native `async fn`
  already matches the desugared trait method and `#[cgp_component]` copies the attribute onto
  those impls.
- `#[async_trait]` stays, `#[deprecated(since = "0.11.0")]`, and still rejects every argument,
  including `?Send`. `#[cgp_component]` and `#[cgp_fn]` rewrite a bare `#[async_trait]` to
  `#[cgp_async]` and emit one marker (`macro_prelude::ASYNC_TRAIT`) spanned on the attribute, so
  the warning is not repeated on every generated item.
- `CanRun` is the runner, declared with `#[cgp_async]` (not `Send`). `CanSendRun` is the same
  shape declared with `#[cgp_async(Send)]`, and is deprecated: new code puts the flag on the
  component it spawns. The second trait stays because the opaque future of a generic `CanRun::run`
  cannot be named. A generated forwarder for `#[cgp_async(Send)]` returns the delegate's future
  instead of `async { ….await }`. The async block would capture `&Context`, which is not `Send`
  unless `Context: Sync`, and a generic component could not meet the bound.
- `AsyncRuntime` is spawn, sleep, and timeout on the runtime value. `HasAsyncRuntime` implements
  it for every context whose `HasRuntimeType::Runtime` implements it, so those three operations
  need no extra wiring. `SleepWithRuntime`, `SpawnWithRuntime`, and `TimeoutWithRuntime` are the
  providers for when the operation itself is wired and replaceable. A timeout's error is
  `TimeoutElapsed`, raised through `CanRaiseError`.
- Features `tokio` and `smol` (not part of `default` or `full`) implement `AsyncRuntime` for
  `tokio::runtime::Handle` and for an owned `SmolRuntime`. Both need `std`. The `thumbv7m`
  feature powerset excludes them, as it already excludes `eyre`. Sleep and timeout futures must
  be polled on that runtime.

## Consequences

- The components stay in the facade crate. A `cgp-fork-std` crate belongs with the hard fork's
  layout, the same choice phase 3 made for `cgp-fork-handler`.
- `#[async_trait]` and `CanSendRun` keep compiling. Each warns, and the note names the replacement.
- `cargo test --workspace --all-features` runs the tokio and smol tests. That is the command the
  Linux, macOS, and Windows CI jobs already use.
- A `Send` bound is still not free on a generic async forwarder. Only the concrete context, or a
  forwarder that returns the provider's future unchanged, can satisfy it.
