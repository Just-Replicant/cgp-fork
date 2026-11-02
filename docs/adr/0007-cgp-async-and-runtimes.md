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
- `AsyncRuntime` is spawn, sleep, and timeout on the runtime value. `SleepFuture` and
  `TimeoutFuture<T>` are associated types: `Send + 'static` futures that do not borrow the
  runtime. The wait starts when the future is first polled, and dropping it cancels the wait.
  `HasAsyncRuntime` implements the same operations for every context whose
  `HasRuntimeType::Runtime` implements `AsyncRuntime`, by returning those associated types, so
  the three operations need no extra wiring. `CanSleep` and `CanTimeout` are components with
  the same shape (`type Sleep`, `type Timeout<T>`). They are not `#[cgp_async(Send)]`, because
  that attribute's future still captures `&self` and would require `Context: Sync` to be
  spawned. `impl Future + use<Self>` cannot express this on a trait method either: `Self` is an
  alias, and the opaque type must mention `Self`. `SleepWithRuntime` returns the runtime's
  `SleepFuture`. `TimeoutWithRuntime` wraps it in `RaisedTimeout`, which stores
  `fn(TimeoutElapsed) -> Context::Error` and does not store the context, so `Context::Error:
  'static`. `spawn_task` detaches the task; a panic inside it is discarded.
- Features `tokio` and `smol` (not part of `default` or `full`) implement `AsyncRuntime` for
  `tokio::runtime::Handle` and for an owned `SmolRuntime`. Both need `std`. The `thumbv7m`
  feature powerset excludes them, as it already excludes `eyre`. Tokio clones the handle into
  the future and, on first poll, spawns a task that constructs `tokio::time::sleep` or
  `tokio::time::timeout` on that handle, so the future can be polled on any thread. Dropping
  the future aborts the task: dropping a `JoinHandle` only detaches it. Shutting the runtime
  down while a sleep or timeout is still running panics, and a panic in the timed task is
  resumed on the waiter. `current_thread` and `multi_thread` build that runtime with the time
  driver. `SmolRuntime` is `async-executor` plus `async-io` timers. The timeout polls the task
  first, then the timer. It does not depend on the `smol` meta-crate. Smol sleep and timeout
  run on the `async-io` reactor; spawned tasks run only while `SmolRuntime::block_on` polls
  the executor.

## Consequences

- The components stay in the facade crate. A `cgp-fork-std` crate belongs with the hard fork's
  layout, the same choice phase 3 made for `cgp-fork-handler`.
- `#[async_trait]` and `CanSendRun` keep compiling. Each warns, and the note names the replacement.
- `cargo test --workspace --all-features` runs the tokio and smol tests. That is the command the
  Linux, macOS, and Windows CI jobs already use.
- A `Send` bound is still not free on a generic async forwarder. Only the concrete context, or a
  forwarder that returns the provider's future unchanged, can satisfy it. `CanSleep` and
  `CanTimeout` name their futures as associated types, so the returned future does not capture
  `&self`.
