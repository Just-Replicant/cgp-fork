use crate::core::prelude::*;

#[cgp_type]
/// The abstract type of the context's runtime.
///
/// Wire `RuntimeTypeProviderComponent` to `UseType<YourRuntime>` to choose it.
pub trait HasRuntimeType {
    type Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;
