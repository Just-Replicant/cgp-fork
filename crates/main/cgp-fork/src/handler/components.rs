use core::marker::PhantomData;

use crate::core::component::UseDelegate;
use crate::core::prelude::*;
use crate::handler::UseInputDelegate;

#[cgp_component(Handler)]
#[prefix(@cgp.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// Handles an `Input` into an `Output`: the one synchronous handler component.
///
/// `Code` selects the computation when a context runs several. The other axes are part of the
/// types instead of the component name:
///
/// - **borrowing**: a borrowed input is `Input = &'a T` (or `&'a mut T`);
/// - **fallibility**: a fallible handler has `Output = Result<T, E>`;
/// - **producers**: a handler without input has `Input = ()`.
///
/// [`AsyncHandler`] is the asynchronous form. Together they replace the nine 0.10 handler
/// components (`Computer`, `ComputerRef`, `TryComputer`, `TryComputerRef`, `AsyncComputer`,
/// `AsyncComputerRef`, the 0.10 `Handler` and `HandlerRef`, and `Producer`); the
/// `compat-0.10` feature keeps those under `cgp_fork::extra::handler`, and
/// [`Compat`](crate::compat::Compat) bridges providers between the two.
pub trait CanHandle<Code, Input> {
    /// The value `handle` returns.
    type Output;

    /// Runs the handler on `input`.
    fn handle(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[async_trait]
#[cgp_component(AsyncHandler)]
#[prefix(@cgp.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// The asynchronous form of [`CanHandle`]: an `Input` in, a future of `Output` out.
///
/// Borrowing, fallibility and producers are expressed in `Input` and `Output` exactly as for
/// [`CanHandle`]. The future is not required to be `Send`. [`Promote`](crate::handler::Promote)
/// turns any [`Handler`] into an `AsyncHandler`.
pub trait CanHandleAsync<Code, Input> {
    /// The value the future resolves to.
    type Output;

    /// Runs the handler on `input`.
    async fn handle_async(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Output> Handler<Context, Code, Input> for UseField<Tag>
where
    Context: HasField<Tag>,
    Context::Value: CanHandle<Code, Input, Output = Output>,
{
    type Output = Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        context.get_field(PhantomData).handle(code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Output> AsyncHandler<Context, Code, Input> for UseField<Tag>
where
    Context: HasField<Tag>,
    Context::Value: CanHandleAsync<Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle_async(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        context
            .get_field(PhantomData)
            .handle_async(code, input)
            .await
    }
}
