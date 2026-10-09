use crate::core::field::traits::{HasFields, HasFieldsRef};

/// Consumes `self` and returns its fields as the [`HasFields::Fields`](crate::core::field::traits::HasFields::Fields) list.
pub trait ToFields: HasFields {
    /// Moves every field into the product or sum.
    fn to_fields(self) -> Self::Fields;
}

/// Borrows `self`'s fields as the [`HasFieldsRef`] list.
pub trait ToFieldsRef: HasFieldsRef {
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a;
}
