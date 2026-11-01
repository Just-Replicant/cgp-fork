use crate::core::prelude::*;
use crate::extra::runtime::HasRuntimeType;

#[cgp_getter]
#[use_type(HasRuntimeType.Runtime)]
/// Borrows the context's [`HasRuntimeType::Runtime`](crate::extra::runtime::HasRuntimeType::Runtime).
///
/// `#[cgp_getter]` reads it from the `runtime` field unless a different provider is wired.
pub trait HasRuntime {
    /// Borrows the runtime.
    fn runtime(&self) -> &Runtime;
}
