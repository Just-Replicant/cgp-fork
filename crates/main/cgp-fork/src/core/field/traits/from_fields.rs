use crate::core::field::traits::HasFields;

/// Builds `Self` from its type-level [`HasFields::Fields`](crate::core::field::traits::HasFields::Fields) list.
pub trait FromFields: HasFields {
    fn from_fields(fields: Self::Fields) -> Self;
}
