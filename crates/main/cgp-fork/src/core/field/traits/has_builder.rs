/// A value that can be constructed field by field.
///
/// `#[derive(CgpData)]` generates this. `builder` returns a partial value whose
/// fields start out absent; [`BuildField`](crate::core::field::traits::BuildField) fills them and
/// [`FinalizeBuild`](crate::core::field::traits::FinalizeBuild) finishes it.
pub trait HasBuilder {
    type Builder;

    fn builder() -> Self::Builder;
}

pub trait IntoBuilder {
    type Builder;

    fn into_builder(self) -> Self::Builder;
}
