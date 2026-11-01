use syn::{ItemImpl, ItemTrait};

use crate::extra_macro_core::types::cgp_handler::ItemCgpHandler;

/// The intermediate representation `#[cgp_auto_dispatch]` evaluates a trait
/// into. Each entry of `handlers` is what the macro would otherwise emit as a
/// `#[cgp_handler(Compute{Method})]` helper function; the entrypoint runs it
/// through the `#[cgp_handler]` pipeline instead.
pub struct EvaluatedCgpAutoDispatch {
    /// The annotated trait, emitted unchanged.
    pub item_trait: ItemTrait,
    /// The impl of the trait for any extensible enum.
    pub blanket_impl: ItemImpl,
    /// One per-variant handler per method, in method order.
    pub handlers: Vec<ItemCgpHandler>,
}
