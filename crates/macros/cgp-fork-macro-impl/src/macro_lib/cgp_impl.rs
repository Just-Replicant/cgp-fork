use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_impl::{ImplArgs, ItemCgpImpl};

/// `#[cgp_impl]`: an impl written against the consumer trait becomes a provider impl.
pub struct CgpImplMacro;

impl Pipeline for CgpImplMacro {
    const NAME: &'static str = "cgp_impl";
    const FAMILY: IrFamily = IrFamily::ProviderImpl;
    type Ir = ItemCgpImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let args: ImplArgs = parse2(attr)?;
        let item_impl: ItemImpl = parse2(body)?;

        Ok(ItemCgpImpl { args, item_impl })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let mut lowered = ir.lower()?;

        let bare_impls = lowered.lower()?;

        let helper_items = &lowered.helper_items;
        let default_impls = &lowered.default_impls;

        Ok(quote! {
            #(#helper_items)*
            #bare_impls
            #(#default_impls)*
        })
    }
}

/// `cgp_impl` entry point.
pub fn cgp_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpImplMacro>(attr, body)
}
