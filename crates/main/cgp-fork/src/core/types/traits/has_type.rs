use cgp_fork_macro::cgp_component;

use crate::core::base::macro_prelude::*;

/// An abstract type chosen per `Tag`.
///
/// Wire `TypeProviderComponent` to [`UseType`](crate::core::types::UseType)`<T>` to set the type to
/// `T`, or to [`UseDelegatedType`](crate::core::types::UseDelegatedType) to look it up.
#[cgp_component(TypeProvider)]
#[derive_delegate(UseDelegate<Tag>)]
pub trait HasType<Tag> {
    /// The concrete type stored under `Tag`.
    type Type;
}

/// [`HasType::Type`](crate::core::types::HasType::Type) of `Context` at `Tag`.
pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
