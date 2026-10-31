//! The unified handler family: one synchronous component, [`Handler`], and one asynchronous
//! component, [`AsyncHandler`] (rewrite plan phase 3, `docs/rewrite-plan.md` §7.4).
//!
//! The 0.10 family split every axis into its own component: owned or borrowed input
//! (`Computer`/`ComputerRef`), fallible or not (`TryComputer`), sync or async
//! (`AsyncComputer`, `Handler`), with or without input (`Producer`). Here the axes live in the
//! types:
//!
//! | 0.10 | Unified |
//! |---|---|
//! | `Computer<Ctx, Code, Input>` | `Handler<Ctx, Code, Input>` |
//! | `ComputerRef<Ctx, Code, Input>` | `Handler<Ctx, Code, &'a Input>` |
//! | `TryComputer<Ctx, Code, Input>` | `Handler<Ctx, Code, Input, Output = Result<T, Ctx::Error>>` |
//! | `TryComputerRef` | `Handler<Ctx, Code, &'a Input, Output = Result<…>>` |
//! | `AsyncComputer` | `AsyncHandler<Ctx, Code, Input>` |
//! | `AsyncComputerRef` | `AsyncHandler<Ctx, Code, &'a Input>` |
//! | `Handler` (async, fallible) | `AsyncHandler<…, Output = Result<T, Ctx::Error>>` |
//! | `HandlerRef` | `AsyncHandler<Ctx, Code, &'a Input, Output = Result<…>>` |
//! | `Producer<Ctx, Code>` | `Handler<Ctx, Code, ()>` |
//!
//! The combinators follow the same rule: one [`Promote`] family ([`Promote`], [`PromoteOk`],
//! [`PromoteDeref`], [`IgnoreInput`]) instead of five promote modules, [`Pipe`] and
//! [`TryPipe`] for composition, and one dispatcher, [`MatchWith`], generic over the match
//! strategy ([`All`]/[`First`]) and the access mode
//! ([`Owned`](crate::core::field::types::Owned), [`Ref`](crate::core::field::types::Ref),
//! [`Mut`](crate::core::field::types::Mut)), instead of six.
//!
//! The 0.10 names are still available under [`crate::extra::handler`] and
//! [`crate::extra::dispatch`] with the `compat-0.10` feature (on by default), and
//! [`Compat`](crate::compat::Compat) adapts a provider of either family to the other. Because
//! the 0.10 prelude already exports a `Handler`, the unified names are not in
//! [`crate::prelude`]; import them with `use cgp_fork::handler::prelude::*;`.

mod adapters;
mod components;
mod dispatch;
mod pipe;
mod types;

pub use adapters::*;
pub use components::*;
pub use dispatch::*;
pub use pipe::*;
pub use types::*;

/// The unified handler names: `use cgp_fork::handler::prelude::*;` next to
/// `use cgp_fork::prelude::*;`.
///
/// Like the 0.10 prelude, it exports the provider traits and not the consumer traits
/// [`CanHandle`] and [`CanHandleAsync`]:
/// both pairs name their method `handle`/`handle_async`, so with both in scope a call such
/// as `Pipe::<H>::handle(&context, code, input)` would be ambiguous. Import the consumer trait
/// where a context calls it.
pub mod prelude {
    pub use crate::core::field::traits::{Mode, ModeExtractor, ModeFields};
    pub use crate::core::field::types::{Mut, Owned, Ref};
    pub use crate::handler::{
        All, AsyncHandler, AsyncHandlerComponent, BuildWith, FieldValue, First, Handler,
        HandlerComponent, IgnoreInput, MatchFields, MatchValues, MatchWith, Merge, MergeOutputs,
        NoCode, OnField, Pipe, Promote, PromoteDeref, PromoteOk, ReturnInput, SetField,
        TryBuildWith, TryMerge, TryMergeOutputs, TryPipe, TrySetField, UseInputDelegate,
    };
}
