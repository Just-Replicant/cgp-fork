# From 0.10 to 1.0

The 0.10 names still compile. They live behind `compat-0.10`, which is on by
default, and each one is deprecated. New code uses `Handler` and `AsyncHandler`.

```sh
cargo xtask migrate path/to/crate
cargo xtask migrate --check .
cargo xtask migrate --explicit-only --crate cgp src
```

`--check` writes nothing and exits 1 when a file would change. `--explicit-only`
skips a name that arrived through `use cgp_fork::prelude::*`. `--crate` is the
dependency name in source (`cgp_fork` by default) and may be repeated.
`--keep-async-trait` leaves a single-segment `#[async_trait]`.
`async_trait::async_trait` is never rewritten.

The codemod rewrites imports, absolute paths, and the attributes in the table.
It does not rewrite method names. A glob import is rewritten by replacing the
bare names it brought in, including the keys of `delegate_components!`,
`delegate_and_check_components!`, `check_components!`, and `cgp_preset!`. A
`use` inside a function is left alone. A child module does not inherit the
parent's imports.

A rename that adds type arguments, such as `MatchWithHandlersRef<H>`, is
rewritten only when every argument is written out. A bare
`MatchWithHandlersRef` is left as it is, and the command prints a warning.
`Promote`, `PipeHandlers`, `ComposeHandlers`, and the `Build*` pairs are not
one name each; they are left in place with a warning.

Run the command on an application. Do not run it on this repository: the
compatibility modules still spell the old names on purpose.

The table is generated from the same list the codemod uses.
`cargo xtask migrate --guide` prints it.

<!-- migration-table:start -->
| Old name | Replacement | Codemod |
|---|---|---|
| `CanCompute` | `cgp_fork::handler::CanHandle` | rewrites the path |
| `Computer` | `cgp_fork::handler::Handler` | rewrites the path |
| `ComputerComponent` | `cgp_fork::handler::HandlerComponent` | rewrites the path |
| `CanComputeRef` | `cgp_fork::handler::CanHandle` | rewrites the path |
| `ComputerRef` | `cgp_fork::handler::Handler` | rewrites the path |
| `ComputerRefComponent` | `cgp_fork::handler::HandlerComponent` | rewrites the path |
| `CanTryCompute` | `cgp_fork::handler::CanHandle` | rewrites the path |
| `TryComputer` | `cgp_fork::handler::Handler` | rewrites the path |
| `TryComputerComponent` | `cgp_fork::handler::HandlerComponent` | rewrites the path |
| `CanTryComputeRef` | `cgp_fork::handler::CanHandle` | rewrites the path |
| `TryComputerRef` | `cgp_fork::handler::Handler` | rewrites the path |
| `TryComputerRefComponent` | `cgp_fork::handler::HandlerComponent` | rewrites the path |
| `CanComputeAsync` | `cgp_fork::handler::CanHandleAsync` | rewrites the path |
| `AsyncComputer` | `cgp_fork::handler::AsyncHandler` | rewrites the path |
| `AsyncComputerComponent` | `cgp_fork::handler::AsyncHandlerComponent` | rewrites the path |
| `CanComputeAsyncRef` | `cgp_fork::handler::CanHandleAsync` | rewrites the path |
| `AsyncComputerRef` | `cgp_fork::handler::AsyncHandler` | rewrites the path |
| `AsyncComputerRefComponent` | `cgp_fork::handler::AsyncHandlerComponent` | rewrites the path |
| `CanHandle` | `cgp_fork::handler::CanHandleAsync` | rewrites the path |
| `Handler` | `cgp_fork::handler::AsyncHandler` | rewrites the path |
| `HandlerComponent` | `cgp_fork::handler::AsyncHandlerComponent` | rewrites the path |
| `CanHandleRef` | `cgp_fork::handler::CanHandleAsync` | rewrites the path |
| `HandlerRef` | `cgp_fork::handler::AsyncHandler` | rewrites the path |
| `HandlerRefComponent` | `cgp_fork::handler::AsyncHandlerComponent` | rewrites the path |
| `CanProduce` | `cgp_fork::handler::CanHandle` | rewrites the path |
| `Producer` | `cgp_fork::handler::Handler` | rewrites the path |
| `ProducerComponent` | `cgp_fork::handler::HandlerComponent` | rewrites the path |
| `PromoteAsync` | `cgp_fork::handler::Promote` | rewrites the path |
| `PromoteRef` | `cgp_fork::handler::PromoteDeref` | rewrites the path |
| `TryPromote` | `cgp_fork::handler::PromoteOk` | rewrites the path |
| `ReturnInput` | `cgp_fork::handler::ReturnInput` | rewrites the path |
| `PromoteComputer` | `cgp_fork::compat::Compat` | rewrites the path |
| `PromoteTryComputer` | `cgp_fork::compat::Compat` | rewrites the path |
| `PromoteProducer` | `cgp_fork::compat::Compat` | rewrites the path |
| `PromoteAsyncComputer` | `cgp_fork::compat::Compat` | rewrites the path |
| `PromoteHandler` | `cgp_fork::compat::Compat` | rewrites the path |
| `Promote` | manual: `IgnoreInput`, `PromoteOk`, or `Promote`, depending on which 0.10 promotion this was | left in place |
| `PipeHandlers` | manual: `Pipe` or `TryPipe` when the handlers return `Result` | left in place |
| `ComposeHandlers` | manual: `Pipe<Product![A, B]>` or `TryPipe` | left in place |
| `MatchWithHandlers` | `cgp_fork::handler::MatchWith` | rewrites the path |
| `MatchWithHandlersRef` | `cgp_fork::handler::MatchWith` (`MatchWith<H, All, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchWithHandlersMut` | `cgp_fork::handler::MatchWith` (`MatchWith<H, All, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithHandlers` | `cgp_fork::handler::MatchWith` (`MatchWith<H, First>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithHandlersRef` | `cgp_fork::handler::MatchWith` (`MatchWith<H, First, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithHandlersMut` | `cgp_fork::handler::MatchWith` (`MatchWith<H, First, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchWithFieldHandlers` | `cgp_fork::handler::MatchFields` | rewrites the path |
| `MatchWithValueHandlers` | `cgp_fork::handler::MatchValues` | rewrites the path |
| `MatchWithFieldHandlersRef` | `cgp_fork::handler::MatchFields` (`MatchFields<P, All, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchWithValueHandlersRef` | `cgp_fork::handler::MatchValues` (`MatchValues<P, All, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchWithValueHandlersMut` | `cgp_fork::handler::MatchValues` (`MatchValues<P, All, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchWithFieldHandlersInputs` | `cgp_fork::handler::MatchFields` | rewrites the path |
| `MatchWithFieldHandlersInputsRef` | `cgp_fork::handler::MatchFields` (`MatchFields<P, All, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchWithFieldHandlersInputsMut` | `cgp_fork::handler::MatchFields` (`MatchFields<P, All, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithFieldHandlers` | `cgp_fork::handler::MatchFields` (`MatchFields<P, First>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithValueHandlers` | `cgp_fork::handler::MatchValues` (`MatchValues<P, First>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithFieldHandlersRef` | `cgp_fork::handler::MatchFields` (`MatchFields<P, First, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithValueHandlersRef` | `cgp_fork::handler::MatchValues` (`MatchValues<P, First, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithFieldHandlersMut` | `cgp_fork::handler::MatchFields` (`MatchFields<P, First, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithValueHandlersMut` | `cgp_fork::handler::MatchValues` (`MatchValues<P, First, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithFieldHandlersInputs` | `cgp_fork::handler::MatchFields` (`MatchFields<P, First>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithFieldHandlersInputsRef` | `cgp_fork::handler::MatchFields` (`MatchFields<P, First, Ref>`) | rewrites the path and appends type arguments when they are all written out |
| `MatchFirstWithFieldHandlersInputsMut` | `cgp_fork::handler::MatchFields` (`MatchFields<P, First, Mut>`) | rewrites the path and appends type arguments when they are all written out |
| `ExtractFieldAndHandle` | `cgp_fork::handler::OnField` | rewrites the path |
| `ExtractFirstFieldAndHandle` | `cgp_fork::handler::OnField` (`OnField<Tag, P, First>`) | rewrites the path and appends type arguments when they are all written out |
| `HandleFieldValue` | `cgp_fork::handler::FieldValue` | rewrites the path |
| `HandleFirstFieldValue` | `cgp_fork::handler::FieldValue` (`FieldValue<P, First>`) | rewrites the path and appends type arguments when they are all written out |
| `DowncastAndHandle` | `cgp_fork::handler::Downcast` | rewrites the path |
| `DispatchMatchers` | `cgp_fork::handler::DispatchHandlers` | rewrites the path |
| `BuildWithHandlers` | manual: `BuildWith` or `TryBuildWith` when the handlers return `Result` | left in place |
| `BuildAndMerge` | manual: `Merge` or `TryMerge` | left in place |
| `BuildAndMergeOutputs` | manual: `MergeOutputs` or `TryMergeOutputs` | left in place |
| `ToBuildAndMergeHandler` | manual: `ToMerge` or `ToTryMerge` | left in place |
| `BuildAndSetField` | manual: `SetField` or `TrySetField` | left in place |
| `HasFieldHandlers` | manual: `VariantHandlers<P, S>`; the trait shape changed | left in place |
| `MapFieldHandler` | manual: `VariantHandlers<P, S>`; the trait shape changed | left in place |
| `ToFieldHandlers` | manual: `VariantHandlers<P, S>`; the trait shape changed | left in place |
| `MapExtractFieldAndHandle` | manual: `VariantHandlers<P, All>` | left in place |
| `MapExtractFirstFieldAndHandle` | manual: `VariantHandlers<P, First>` | left in place |
| `IsRef` | `cgp_fork::core::field::types::Ref` | rewrites the path |
| `IsMut` | `cgp_fork::core::field::types::Mut` | rewrites the path |
| `IsOwned` | `cgp_fork::core::field::types::Owned` | rewrites the path |
| `MapTypeRef` | `cgp_fork::core::field::traits::Mode` | rewrites the path |
| `CanSendRun` | manual: put `#[cgp_async(Send)]` on the component that is spawned; `CanRun` stays | left in place |
| `SendRunner` | manual: put `#[cgp_async(Send)]` on the component that is spawned | left in place |
| `SendRunnerComponent` | manual: put `#[cgp_async(Send)]` on the component that is spawned | left in place |
| `#[cgp_computer]` | `#[cgp_handler]` | rewrites the attribute (same arguments; an async function becomes an `AsyncHandler`) |
| `#[cgp_producer]` | `#[cgp_handler]` | rewrites the attribute (a producer is a handler of `()`) |
| `#[derive_promote]` | `#[derive_handler]` | rewrites the attribute (forwards the component method to `Handler`) |
| `#[async_trait]` | `#[cgp_async]` | rewrites the attribute (the cgp-fork attribute, not `async_trait::async_trait`; `--keep-async-trait` leaves it) |

Method names (`compute`, `try_compute`, `produce`, `send_run`) are not rewritten. The new methods are `handle` and `handle_async`.
<!-- migration-table:end -->
