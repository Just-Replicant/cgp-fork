use core::marker::PhantomData;

use crate::core::component::UseDelegate;
use crate::core::prelude::*;
use crate::extra::handler::UseInputDelegate;

#[cgp_component(TryComputer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
/// Fallible form of [`CanCompute`](crate::extra::handler::CanCompute).
///
/// Failure is the context's abstract error from [`HasErrorType`](crate::core::error::HasErrorType).
pub trait CanTryCompute<Code, Input> {
    /// The success value.
    type Output;

    /// Runs the computation, returning the context error on failure.
    fn try_compute(&self, _code: PhantomData<Code>, input: Input) -> Result<Self::Output, Error>;
}

#[cgp_component(TryComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
#[use_type(HasErrorType.Error)]
pub trait CanTryComputeRef<Code, Input> {
    type Output;

    fn try_compute_ref(
        &self,
        _code: PhantomData<Code>,
        input: &Input,
    ) -> Result<Self::Output, Error>;
}
