use core::marker::PhantomData;

use crate::core::component::UseDelegate;
use crate::core::prelude::*;

#[cgp_component(Producer)]
#[prefix(@cgp.extra.handler in DefaultNamespace)]
#[derive_delegate(UseDelegate<Code>)]
/// Produces a value from the context alone. There is no input.
///
/// `#[cgp_producer]` defines a provider for this. `Code` selects which value to produce.
pub trait CanProduce<Code> {
    /// The value `produce` returns.
    type Output;

    /// Builds the value from `self`.
    fn produce(&self, _code: PhantomData<Code>) -> Self::Output;
}
