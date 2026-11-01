use crate::core::prelude::*;
use crate::extra::handler::{
    AsyncComputer, AsyncComputerComponent, Computer, Handler, HandlerComponent, TryComputer,
};

/// Lifts a synchronous provider into the async handler family.
///
/// A `Computer` becomes an `AsyncComputer` whose future is ready immediately. A `TryComputer`
/// becomes a `Handler`.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::Promote`, which lifts a unified `Handler` into an `AsyncHandler`"
)]
pub struct PromoteAsync<Provider>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> AsyncComputer<Context, Code, Input>
    for PromoteAsync<Provider>
where
    Provider: Computer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn compute_async(context: &Context, tag: PhantomData<Code>, input: Input) -> Output {
        Provider::compute(context, tag, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Provider> Handler<Context, Code, Input>
    for PromoteAsync<Provider>
where
    Context: HasErrorType,
    Provider: TryComputer<Context, Code, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        Provider::try_compute(context, tag, input)
    }
}
