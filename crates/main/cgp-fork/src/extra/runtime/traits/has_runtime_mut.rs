use crate::core::prelude::*;
use crate::extra::runtime::HasRuntimeType;

#[cgp_getter]
#[use_type(HasRuntimeType.Runtime)]
/// Mutably borrows the context's [`HasRuntimeType::Runtime`](crate::extra::runtime::HasRuntimeType::Runtime).
///
/// `#[cgp_getter]` reads a field named `runtime_mut` unless a different provider is wired.
/// Wire `RuntimeMutGetterComponent` to `UseField<Symbol!("runtime")>` when the field is `runtime`.
pub trait HasRuntimeMut {
    /// Mutably borrows the runtime.
    fn runtime_mut(&mut self) -> &mut Runtime;
}
