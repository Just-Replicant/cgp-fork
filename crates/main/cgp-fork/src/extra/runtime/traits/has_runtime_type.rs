use crate::core::prelude::*;

#[cgp_type]
/// The abstract type of the context's runtime.
///
/// Wire `RuntimeTypeProviderComponent` to `UseType<YourRuntime>` to choose it.
pub trait HasRuntimeType {
    /// The runtime type. Tokio, async-std, or a test runtime are typical choices.
    type Runtime;
}

/// [`HasRuntimeType::Runtime`](crate::extra::runtime::HasRuntimeType::Runtime) of `Context`.
pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;
