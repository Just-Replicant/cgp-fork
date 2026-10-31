use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

use crate::ir::{IrFamily, Pipeline, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_log::ItemCgpAutoLog;

/// `#[cgp_auto_log]`: a logging trait gets its logging providers.
pub struct CgpAutoLogMacro;

impl Pipeline for CgpAutoLogMacro {
    const NAME: &'static str = "cgp_auto_log";
    const FAMILY: IrFamily = IrFamily::TraitRewrite;
    type Ir = ItemCgpAutoLog;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoLog::parse(&item_trait)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `cgp_auto_log` entry point.
pub fn cgp_auto_log(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoLogMacro>(attr, body)
}
