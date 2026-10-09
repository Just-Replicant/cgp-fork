/// The type-level shape of `Self`: a [`Cons`](crate::core::field::types::Cons) or
/// [`Either`](crate::core::field::types::Either) chain of [`Field`](crate::core::field::types::Field)s.
///
/// `#[derive(HasFields)]` and `#[derive(CgpData)]` generate this. Named fields are keyed by
/// `Symbol!`, positional fields by `Index<N>`.
pub trait HasFields {
    /// The product or sum that lists every field.
    type Fields;
}

pub trait HasFieldsRef {
    type FieldsRef<'a>
    where
        Self: 'a;
}
