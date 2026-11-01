use core::marker::PhantomData;

use crate::core::component::UseDelegate;
use crate::core::prelude::*;
use crate::extra::handler::UseInputDelegate;

#[cgp_async]
#[cgp_component(AsyncComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// Async form of [`CanCompute`](crate::extra::handler::CanCompute): an owned `Input` in, a future out.
#[deprecated(
    since = "0.11.0",
    note = "use the unified `AsyncHandler` component (`cgp_fork::handler::AsyncHandler`, called through `cgp_fork::handler::CanHandleAsync`)"
)]
pub trait CanComputeAsync<Code, Input> {
    /// The value the future resolves to.
    type Output;

    /// Runs the async computation on an owned `input`.
    async fn compute_async(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[cgp_async]
#[cgp_component(AsyncComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// Async form of [`CanComputeRef`](crate::extra::handler::CanComputeRef).
#[deprecated(
    since = "0.11.0",
    note = "use the unified `AsyncHandler` component (`cgp_fork::handler::AsyncHandler`) with a `&T` input"
)]
pub trait CanComputeAsyncRef<Code, Input> {
    /// The value the future resolves to.
    type Output;

    /// Runs the async computation on a borrowed `input`.
    async fn compute_async_ref(&self, _code: PhantomData<Code>, input: &Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Output> AsyncComputer<Context, Code, Input> for UseField<Tag>
where
    Context: HasField<Tag>,
    Context::Value: CanComputeAsync<Code, Input, Output = Output>,
{
    type Output = Output;

    async fn compute_async(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        context
            .get_field(PhantomData)
            .compute_async(code, input)
            .await
    }
}
