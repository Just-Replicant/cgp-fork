/// Implements a getter component by reading each method from the same-named field.
///
/// `#[cgp_getter]` generates a [`UseFields`] impl next to the per-tag [`UseField`](crate::core::field::impls::UseField)
/// impl. The field tag is `Symbol!("method_name")`.
pub struct UseFields;
