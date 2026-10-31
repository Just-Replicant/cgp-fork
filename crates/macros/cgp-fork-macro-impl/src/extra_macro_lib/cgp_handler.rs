use proc_macro2::TokenStream;
use syn::{Ident, ItemFn, parse2};

use crate::extra_macro_core::types::cgp_handler::ItemCgpHandler;
use crate::ir::{Pipeline, ProviderImpl, run};

/// `#[cgp_handler]`: a function becomes a provider of the unified `Handler` (or, for an
/// `async` function, `AsyncHandler`).
pub struct CgpHandlerMacro;

impl Pipeline for CgpHandlerMacro {
    const NAME: &'static str = "cgp_handler";
    type Ir = ProviderImpl;

    /// The attribute optionally names the provider.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_fn: ItemFn = parse2(body)?;

        let ident: Option<Ident> = parse2(attr)?;

        Ok(ProviderImpl::Handler(Box::new(ItemCgpHandler {
            ident,
            item_fn,
            legacy_wiring: false,
        })))
    }
}

/// `cgp_handler` entry point.
pub fn cgp_handler(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpHandlerMacro>(attr, body)
}
