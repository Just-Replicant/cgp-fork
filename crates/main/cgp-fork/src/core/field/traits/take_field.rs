use core::marker::PhantomData;

use crate::core::field::impls::{IsNothing, IsPresent};
use crate::core::field::traits::UpdateField;

/// Removes a present `Tag` field from a builder, leaving that slot empty.
///
/// The inverse of [`BuildField`](crate::core::field::traits::BuildField): the map changes from
/// [`IsPresent`](crate::core::field::impls::IsPresent) to [`IsNothing`](crate::core::field::impls::IsNothing).
pub trait TakeField<Tag> {
    /// The type stored under `Tag`.
    type Value;

    /// This builder after `Tag` has been cleared.
    type Remainder;

    /// Takes the `Tag` field and returns the builder with that slot empty.
    fn take_field(self, _tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder);
}

impl<Context, Tag> TakeField<Tag> for Context
where
    Context: UpdateField<Tag, IsNothing, Mapper = IsPresent>,
{
    type Value = Context::Value;

    type Remainder = Context::Output;

    fn take_field(self, tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder) {
        self.update_field(tag, ())
    }
}
