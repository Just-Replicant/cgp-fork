use core::marker::PhantomData;

/// Adapts `Provider` to a consumer trait through `TypeProvider` or `FieldGetter`.
///
/// `#[cgp_type]` and `#[cgp_getter]` generate a `WithProvider` impl. [`WithType`](crate::core::types::WithType),
/// [`WithField`](crate::core::field::impls::WithField), and [`WithContext`](super::WithContext) are
/// the common aliases.
pub struct WithProvider<Provider>(pub PhantomData<Provider>);
