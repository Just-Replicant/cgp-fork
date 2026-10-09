use crate::core::field::traits::{HasFields, HasFieldsRef};

/// Consumes `self` and returns its fields as the [`HasFields::Fields`](crate::core::field::traits::HasFields::Fields) list.
pub trait ToFields: HasFields {
    fn to_fields(self) -> Self::Fields;
}

pub trait ToFieldsRef: HasFieldsRef {
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a;
}
