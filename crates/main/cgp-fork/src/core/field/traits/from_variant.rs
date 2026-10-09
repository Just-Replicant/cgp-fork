use core::marker::PhantomData;

/// Builds an enum from the payload of the `Tag` variant.
pub trait FromVariant<Tag> {
    /// The payload stored in the `Tag` variant.
    type Value;

    fn from_variant(_tag: PhantomData<Tag>, value: Self::Value) -> Self;
}
