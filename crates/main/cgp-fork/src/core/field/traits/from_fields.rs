use crate::core::field::traits::HasFields;

/// Builds `Self` from its type-level [`HasFields::Fields`](crate::core::field::traits::HasFields::Fields) list.
pub trait FromFields: HasFields {
    /// Reconstructs `Self` from the product (or sum) of its fields.
    fn from_fields(fields: Self::Fields) -> Self;
}
