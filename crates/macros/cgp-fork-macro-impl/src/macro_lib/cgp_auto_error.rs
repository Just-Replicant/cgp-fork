use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemImpl;

use crate::ir::{IrFamily, Pipeline, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_error::ItemCgpAutoError;

/// `#[cgp_auto_error]`: error-raising methods on a context become error providers.
pub struct CgpAutoErrorMacro;

impl Pipeline for CgpAutoErrorMacro {
    const NAME: &'static str = "cgp_auto_error";
    const FAMILY: IrFamily = IrFamily::ProviderImpl;
    type Ir = ItemCgpAutoError;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoError::parse(&item_impl)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `cgp_auto_error` entry point.
pub fn cgp_auto_error(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoErrorMacro>(attr, body)
}
