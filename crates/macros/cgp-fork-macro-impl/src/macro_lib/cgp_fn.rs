use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_fn::ItemCgpFn;

/// `#[cgp_fn]`: a function becomes a component with one provider.
pub struct CgpFnMacro;

impl Pipeline for CgpFnMacro {
    const NAME: &'static str = "cgp_fn";
    const FAMILY: IrFamily = IrFamily::ComponentDef;
    type Ir = ItemCgpFn;

    /// The attribute optionally names the generated trait.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_fn: ItemFn = parse2(body)?;

        let ident: Option<Ident> = parse2(attr)?;

        Ok(ItemCgpFn { ident, item_fn })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.preprocess()?.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `cgp_fn` entry point.
pub fn cgp_fn(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpFnMacro>(attr, body)
}
