//! The `Promote` family: one adapter per axis the 0.10 components split on.
//!
//! | 0.10 | Unified |
//! |---|---|
//! | `PromoteAsync`, `Promote` (`AsyncComputer` → `Handler`) | [`Promote`] (sync → async) |
//! | `TryPromote`, `Promote` (`Computer` → `TryComputer`) | [`PromoteOk`] (infallible → `Result`) |
//! | `PromoteRef` (owned `Deref` input → borrowed provider) | [`PromoteDeref`] |
//! | `Promote` (`Producer` → `Computer`) | [`IgnoreInput`] |
//! | `ReturnInput` | [`ReturnInput`] |
//!
//! Each adapter implements both [`Handler`] and [`AsyncHandler`] where that makes sense, so they
//! compose: `Promote<PromoteOk<P>>` is an asynchronous fallible handler from a synchronous
//! infallible one.

use core::ops::Deref;

use crate::core::prelude::*;
use crate::handler::{AsyncHandler, AsyncHandlerComponent, Handler, HandlerComponent};

/// Runs the synchronous [`Handler`] `Provider` as an [`AsyncHandler`]. The future is ready at
/// once.
pub struct Promote<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, Input> for Promote<Provider>
where
    Provider: Handler<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        Provider::handle(context, code, input)
    }
}

/// Runs the infallible `Provider` and wraps its output in `Ok`, with the context's abstract
/// error as the error type.
pub struct PromoteOk<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, Input> for PromoteOk<Provider>
where
    Context: HasErrorType,
    Provider: Handler<Context, Code, Input>,
{
    type Output = Result<Provider::Output, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Ok(Provider::handle(context, code, input))
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, Input> for PromoteOk<Provider>
where
    Context: HasErrorType,
    Provider: AsyncHandler<Context, Code, Input>,
{
    type Output = Result<Provider::Output, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        Ok(Provider::handle_async(context, code, input).await)
    }
}

/// Calls `Provider` with `&*input`, so an owned input that implements `Deref` can use a
/// handler written for the borrowed target.
pub struct PromoteDeref<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Target, Provider, Output> Handler<Context, Code, Input>
    for PromoteDeref<Provider>
where
    Input: Deref<Target = Target>,
    Target: ?Sized,
    Provider: for<'a> Handler<Context, Code, &'a Target, Output = Output>,
{
    type Output = Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        Provider::handle(context, code, input.deref())
    }
}

#[cgp_provider]
impl<Context, Code, Input, Target, Provider, Output> AsyncHandler<Context, Code, Input>
    for PromoteDeref<Provider>
where
    Input: Deref<Target = Target>,
    Target: ?Sized,
    Provider: for<'a> AsyncHandler<Context, Code, &'a Target, Output = Output>,
{
    type Output = Output;

    async fn handle_async(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        Provider::handle_async(context, code, input.deref()).await
    }
}

/// Runs the producer `Provider` (a handler of `()`) and drops the input.
pub struct IgnoreInput<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, Input> for IgnoreInput<Provider>
where
    Provider: Handler<Context, Code, ()>,
{
    type Output = Provider::Output;

    fn handle(context: &Context, code: PhantomData<Code>, _input: Input) -> Self::Output {
        Provider::handle(context, code, ())
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, Input> for IgnoreInput<Provider>
where
    Provider: AsyncHandler<Context, Code, ()>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Self::Output {
        Provider::handle_async(context, code, ()).await
    }
}

/// Returns its input unchanged.
pub struct ReturnInput;

#[cgp_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for ReturnInput {
    type Output = Input;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: Input) -> Input {
        input
    }
}

#[cgp_provider]
impl<Context, Code, Input> AsyncHandler<Context, Code, Input> for ReturnInput {
    type Output = Input;

    async fn handle_async(_context: &Context, _code: PhantomData<Code>, input: Input) -> Input {
        input
    }
}
