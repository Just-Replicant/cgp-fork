use core::marker::PhantomData;

use crate::core::component::UseDelegate;
use crate::core::prelude::*;
use crate::extra::handler::UseInputDelegate;

#[cgp_component(Computer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
/// Computes an owned `Input` into `Output`.
///
/// `Code` selects which computation this is when one context runs several. Wire
/// `ComputerComponent` to a provider, or derive one with `#[cgp_computer]`.
pub trait CanCompute<Code, Input> {
    /// The value `compute` returns.
    type Output;

    /// Runs the computation on an owned `input`.
    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}

#[cgp_component(ComputerRef)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanComputeRef<Code, Input> {
    type Output;

    fn compute_ref(&self, _code: PhantomData<Code>, input: &Input) -> Self::Output;
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Output> Computer<Context, Code, Input> for UseField<Tag>
where
    Context: HasField<Tag>,
    Context::Value: CanCompute<Code, Input, Output = Output>,
{
    type Output = Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        context.get_field(PhantomData).compute(code, input)
    }
}
