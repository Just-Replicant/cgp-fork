//! Bridges between the 0.10 handler family ([`crate::extra::handler`]) and the unified one
//! ([`crate::handler`]), available with the `compat-0.10` feature.
//!
//! A provider of either family can be wired where the other is expected:
//!
//! | Provider written as | Wire it as | With |
//! |---|---|---|
//! | `Computer` / `AsyncComputer` | `Handler` / `AsyncHandler` | [`Compat<P>`](Compat) |
//! | `TryComputer` / 0.10 `Handler` | `Handler` / `AsyncHandler` returning `Result` | [`CompatTry<P>`](CompatTry) |
//! | `ComputerRef` / `AsyncComputerRef` | `Handler` / `AsyncHandler` of `&'a T` | [`CompatRef<P>`](CompatRef) |
//! | `TryComputerRef` / `HandlerRef` | `Handler` / `AsyncHandler` of `&'a T` returning `Result` | [`CompatTryRef<P>`](CompatTryRef) |
//! | `Producer` | `Handler` / `AsyncHandler` of any input | [`CompatProducer<P>`](CompatProducer) |
//! | `Handler` / `AsyncHandler` | any of the nine 0.10 components | [`Compat<P>`](Compat) |
//!
//! `Compat<P>` implements every 0.10 component from a unified provider, so one wiring entry
//! such as `[ComputerComponent, TryComputerComponent]: Compat<MyHandler>` serves 0.10
//! consumers while the provider is ported.

use crate::core::prelude::*;
use crate::extra::handler as legacy;
use crate::extra::handler::{
    AsyncComputer, AsyncComputerComponent, AsyncComputerRef, AsyncComputerRefComponent, Computer,
    ComputerComponent, ComputerRef, ComputerRefComponent, Producer, ProducerComponent, TryComputer,
    TryComputerComponent, TryComputerRef, TryComputerRefComponent,
};
use crate::handler::{AsyncHandler, AsyncHandlerComponent, Handler, HandlerComponent};

/// Adapts a provider between the two families: a 0.10 `Computer`/`AsyncComputer` as a unified
/// [`Handler`]/[`AsyncHandler`], and a unified handler as any 0.10 component.
pub struct Compat<Provider>(pub PhantomData<Provider>);

/// A 0.10 `TryComputer` or async fallible `Handler` as a unified handler returning
/// `Result<_, Context::Error>`.
pub struct CompatTry<Provider>(pub PhantomData<Provider>);

/// A 0.10 `ComputerRef` or `AsyncComputerRef` as a unified handler of `&'a T`.
pub struct CompatRef<Provider>(pub PhantomData<Provider>);

/// A 0.10 `TryComputerRef` or `HandlerRef` as a unified handler of `&'a T` returning
/// `Result<_, Context::Error>`.
pub struct CompatTryRef<Provider>(pub PhantomData<Provider>);

/// A 0.10 `Producer` as a unified handler that ignores its input.
pub struct CompatProducer<Provider>(pub PhantomData<Provider>);

// 0.10 providers as unified handlers.

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, Input> for Compat<Provider>
where
    Provider: Computer<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Provider::compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, Input> for Compat<Provider>
where
    Provider: AsyncComputer<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        Provider::compute_async(context, code, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, Input> for CompatTry<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input>,
{
    type Output = Result<Provider::Output, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Provider::try_compute(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, Input> for CompatTry<Provider>
where
    Context: HasErrorType,
    Provider: legacy::Handler<Context, Code, Input>,
{
    type Output = Result<Provider::Output, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        Provider::handle(context, code, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, &Input> for CompatRef<Provider>
where
    Provider: ComputerRef<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: &Input) -> Self::Output {
        Provider::compute_ref(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, &Input> for CompatRef<Provider>
where
    Provider: AsyncComputerRef<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Self::Output {
        Provider::compute_async_ref(context, code, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, &Input> for CompatTryRef<Provider>
where
    Context: HasErrorType,
    Provider: TryComputerRef<Context, Code, Input>,
{
    type Output = Result<Provider::Output, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, input: &Input) -> Self::Output {
        Provider::try_compute_ref(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, &Input> for CompatTryRef<Provider>
where
    Context: HasErrorType,
    Provider: legacy::HandlerRef<Context, Code, Input>,
{
    type Output = Result<Provider::Output, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Self::Output {
        Provider::handle_ref(context, code, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> Handler<Context, Code, Input> for CompatProducer<Provider>
where
    Provider: Producer<Context, Code>,
{
    type Output = Provider::Output;

    fn handle(context: &Context, code: PhantomData<Code>, _input: Input) -> Self::Output {
        Provider::produce(context, code)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncHandler<Context, Code, Input> for CompatProducer<Provider>
where
    Provider: Producer<Context, Code>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Self::Output {
        Provider::produce(context, code)
    }
}

// Unified handlers as 0.10 providers.

#[cgp_provider]
impl<Context, Code, Input, Provider> Computer<Context, Code, Input> for Compat<Provider>
where
    Provider: Handler<Context, Code, Input>,
{
    type Output = Provider::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Provider::handle(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> ComputerRef<Context, Code, Input> for Compat<Provider>
where
    Provider: for<'a> Handler<Context, Code, &'a Input, Output = Output>,
{
    type Output = Output;

    fn compute_ref(context: &Context, code: PhantomData<Code>, input: &Input) -> Output {
        Provider::handle(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> TryComputer<Context, Code, Input> for Compat<Provider>
where
    Context: HasErrorType,
    Provider: Handler<Context, Code, Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    fn try_compute(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::handle(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> TryComputerRef<Context, Code, Input>
    for Compat<Provider>
where
    Context: HasErrorType,
    Provider: for<'a> Handler<Context, Code, &'a Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    fn try_compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        Provider::handle(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider> AsyncComputer<Context, Code, Input> for Compat<Provider>
where
    Provider: AsyncHandler<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn compute_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        Provider::handle_async(context, code, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Output> AsyncComputerRef<Context, Code, Input>
    for Compat<Provider>
where
    Provider: for<'a> AsyncHandler<Context, Code, &'a Input, Output = Output>,
{
    type Output = Output;

    async fn compute_async_ref(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Output {
        Provider::handle_async(context, code, input).await
    }
}

#[cgp_provider(legacy::HandlerComponent)]
impl<Context, Code, Input, Provider, Output> legacy::Handler<Context, Code, Input>
    for Compat<Provider>
where
    Context: HasErrorType,
    Provider: AsyncHandler<Context, Code, Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::handle_async(context, code, input).await
    }
}

#[cgp_provider(legacy::HandlerRefComponent)]
impl<Context, Code, Input, Provider, Output> legacy::HandlerRef<Context, Code, Input>
    for Compat<Provider>
where
    Context: HasErrorType,
    Provider:
        for<'a> AsyncHandler<Context, Code, &'a Input, Output = Result<Output, Context::Error>>,
{
    type Output = Output;

    async fn handle_ref(
        context: &Context,
        code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Output, Context::Error> {
        Provider::handle_async(context, code, input).await
    }
}

#[cgp_provider]
impl<Context, Code, Provider> Producer<Context, Code> for Compat<Provider>
where
    Provider: Handler<Context, Code, ()>,
{
    type Output = Provider::Output;

    fn produce(context: &Context, code: PhantomData<Code>) -> Self::Output {
        Provider::handle(context, code, ())
    }
}
