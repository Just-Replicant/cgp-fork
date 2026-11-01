use core::marker::PhantomData;

use crate::core::field::traits::MapType;

/// Replaces the `Tag` field, which is currently wrapped by `M`, and returns the old wrapper.
///
/// [`BuildField`](crate::core::field::traits::BuildField) and [`TakeField`](crate::core::field::traits::TakeField)
/// are the two directions: present to absent, and absent to present.
pub trait UpdateField<Tag, M: MapType> {
    /// The type stored under `Tag`.
    type Value;

    /// The map the field uses before this update. The returned value is wrapped by it.
    type Mapper: MapType;

    /// This builder after the field has been replaced.
    type Output;

    /// Writes `value` into `Tag` and returns the previous contents.
    fn update_field(
        self,
        _tag: PhantomData<Tag>,
        value: M::Map<Self::Value>,
    ) -> (<Self::Mapper as MapType>::Map<Self::Value>, Self::Output);
}
