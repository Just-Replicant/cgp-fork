/// A value that can be constructed field by field.
///
/// `#[derive(CgpData)]` generates this. `builder` returns a partial value whose
/// fields start out absent; [`BuildField`](crate::core::field::traits::BuildField) fills them and
/// [`FinalizeBuild`](crate::core::field::traits::FinalizeBuild) finishes it.
pub trait HasBuilder {
    /// The partial value, one slot per field.
    type Builder;

    /// An empty builder, with every field absent.
    fn builder() -> Self::Builder;
}

/// Turns an existing value into a builder that already holds its fields.
pub trait IntoBuilder {
    /// The partial value populated from `self`.
    type Builder;

    /// Moves `self` into a builder without clearing its fields.
    fn into_builder(self) -> Self::Builder;
}
