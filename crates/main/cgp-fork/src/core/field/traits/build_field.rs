use core::marker::PhantomData;

use crate::core::field::impls::{IsNothing, IsPresent};
use crate::core::field::traits::{PartialData, UpdateField};

/// Sets the `Tag` field of a partial builder.
///
/// The builder's map for that field changes from absent ([`IsNothing`](crate::core::field::impls::IsNothing))
/// to present ([`IsPresent`](crate::core::field::impls::IsPresent)). `Output` is the builder with
/// that one field filled.
pub trait BuildField<Tag> {
    /// The type stored under `Tag`.
    type Value;

    /// This builder after `Tag` has been set.
    type Output;

    /// Writes `value` into the `Tag` field.
    fn build_field(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self::Output;
}

impl<Context, Tag> BuildField<Tag> for Context
where
    Context: UpdateField<Tag, IsPresent, Mapper = IsNothing>,
{
    type Value = Context::Value;

    type Output = Context::Output;

    fn build_field(self, tag: PhantomData<Tag>, value: Self::Value) -> Self::Output {
        self.update_field(tag, value).1
    }
}

/// Turns a partial builder into the finished value once every field is present.
pub trait FinalizeBuild: PartialData {
    fn finalize_build(self) -> Self::Target;
}
