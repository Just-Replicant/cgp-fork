use core::marker::PhantomData;

use crate::core::component::UseDelegate;
use crate::core::prelude::*;
use crate::extra::handler::UseInputDelegate;

#[async_trait]
#[cgp_component(Handler)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
/// Async computation that returns the context's abstract error on failure.
///
/// `Error` comes from [`HasErrorType`](crate::core::error::HasErrorType). This is the fallible
/// async member of the handler family.
pub trait CanHandle<Code, Input> {
    /// The success value.
    type Output;

    /// Handles an owned `input`, returning the context error on failure.
    async fn handle(&self, _tag: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

#[async_trait]
#[cgp_component(HandlerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
pub trait CanHandleRef<Code, Input> {
    type Output;

    async fn handle_ref(
        &self,
        _tag: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Error>;
}
