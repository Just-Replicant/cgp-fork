use proc_macro2::TokenStream;
use syn::{Ident, ItemFn, parse2};

use crate::ir::{ComponentDef, Pipeline, run};
use crate::macro_core::types::cgp_fn::ItemCgpFn;

/// `#[cgp_fn]`: a function becomes a trait with one blanket implementation.
pub struct CgpFnMacro;

impl Pipeline for CgpFnMacro {
    const NAME: &'static str = "cgp_fn";
    type Ir = ComponentDef;

    /// The attribute optionally names the generated trait.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_fn: ItemFn = parse2(body)?;

        let ident: Option<Ident> = parse2(attr)?;

        Ok(ComponentDef::Function(ItemCgpFn { ident, item_fn }))
    }
}

/// `cgp_fn` entry point.
pub fn cgp_fn(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpFnMacro>(attr, body)
}
