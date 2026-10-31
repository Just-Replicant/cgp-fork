use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::macro_core::types::keyword::Keyword;

/// `#[cgp_new_provider]`: `#[cgp_provider]` that also declares the provider struct.
pub struct CgpNewProviderMacro;

impl Pipeline for CgpNewProviderMacro {
    const NAME: &'static str = "cgp_new_provider";
    const FAMILY: IrFamily = IrFamily::ProviderImpl;
    type Ir = ItemCgpProvider;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse2(body)?;
        let mut args: ProviderArgs = parse2(attr)?;
        args.new = Some(Keyword::default());

        Ok(ItemCgpProvider { args, item_impl })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let lowered = ir.lower()?;

        Ok(quote! {
            #lowered
        })
    }
}

/// `cgp_new_provider` entry point.
pub fn cgp_new_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpNewProviderMacro>(attr, body)
}
