//! Running a list of handlers one after the other.

use crate::core::prelude::*;
use crate::handler::{AsyncHandler, AsyncHandlerComponent, Handler, HandlerComponent};

/// Runs the handlers of a [`Cons`] list (`Product![A, B, C]`) in order, each on the output of
/// the one before. An empty list returns its input.
pub struct Pipe<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for Pipe<Nil> {
    type Output = Input;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: Input) -> Input {
        input
    }
}

#[cgp_provider]
impl<Context, Code, Input, Head, Tail> Handler<Context, Code, Input> for Pipe<Cons<Head, Tail>>
where
    Head: Handler<Context, Code, Input>,
    Pipe<Tail>: Handler<Context, Code, Head::Output>,
{
    type Output = <Pipe<Tail> as Handler<Context, Code, Head::Output>>::Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let output = Head::handle(context, code, input);
        Pipe::<Tail>::handle(context, code, output)
    }
}

#[cgp_provider]
impl<Context, Code, Input> AsyncHandler<Context, Code, Input> for Pipe<Nil> {
    type Output = Input;

    async fn handle_async(_context: &Context, _code: PhantomData<Code>, input: Input) -> Input {
        input
    }
}

#[cgp_provider]
impl<Context, Code, Input, Head, Tail> AsyncHandler<Context, Code, Input> for Pipe<Cons<Head, Tail>>
where
    Head: AsyncHandler<Context, Code, Input>,
    Pipe<Tail>: AsyncHandler<Context, Code, Head::Output>,
{
    type Output = <Pipe<Tail> as AsyncHandler<Context, Code, Head::Output>>::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        let output = Head::handle_async(context, code, input).await;
        Pipe::<Tail>::handle_async(context, code, output).await
    }
}

/// [`Pipe`] for fallible handlers: each returns `Result<_, Context::Error>`, the next runs on
/// the `Ok` value, and the first `Err` is returned. An empty list returns `Ok(input)`.
pub struct TryPipe<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for TryPipe<Nil>
where
    Context: HasErrorType,
{
    type Output = Result<Input, Context::Error>;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: Input) -> Self::Output {
        Ok(input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Head, Tail, Value, Output> Handler<Context, Code, Input>
    for TryPipe<Cons<Head, Tail>>
where
    Context: HasErrorType,
    Head: Handler<Context, Code, Input, Output = Result<Value, Context::Error>>,
    TryPipe<Tail>: Handler<Context, Code, Value, Output = Result<Output, Context::Error>>,
{
    type Output = Result<Output, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let value = Head::handle(context, code, input)?;
        TryPipe::<Tail>::handle(context, code, value)
    }
}

#[cgp_provider]
impl<Context, Code, Input> AsyncHandler<Context, Code, Input> for TryPipe<Nil>
where
    Context: HasErrorType,
{
    type Output = Result<Input, Context::Error>;

    async fn handle_async(
        _context: &Context,
        _code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        Ok(input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Head, Tail, Value, Output> AsyncHandler<Context, Code, Input>
    for TryPipe<Cons<Head, Tail>>
where
    Context: HasErrorType,
    Head: AsyncHandler<Context, Code, Input, Output = Result<Value, Context::Error>>,
    TryPipe<Tail>: AsyncHandler<Context, Code, Value, Output = Result<Output, Context::Error>>,
{
    type Output = Result<Output, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        let value = Head::handle_async(context, code, input).await?;
        TryPipe::<Tail>::handle_async(context, code, value).await
    }
}
