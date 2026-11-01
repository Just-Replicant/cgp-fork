use proc_macro2::TokenStream;
use syn::{Ident, ItemFn, parse2};

use crate::extra_macro_core::types::cgp_computer::ItemCgpComputer;
use crate::ir::{Pipeline, ProviderImpl, run};

/// `#[cgp_computer]`: a function becomes a `Computer` provider with its promotions.
pub struct CgpComputerMacro;

impl Pipeline for CgpComputerMacro {
    const NAME: &'static str = "cgp_computer";
    type Ir = ProviderImpl;

    /// The attribute optionally names the provider.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_fn: ItemFn = parse2(body)?;

        let ident: Option<Ident> = parse2(attr)?;

        Ok(ProviderImpl::Computer(Box::new(ItemCgpComputer {
            ident,
            item_fn,
        })))
    }
}

/// `cgp_computer` entry point.
pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpComputerMacro>(attr, body)
}
