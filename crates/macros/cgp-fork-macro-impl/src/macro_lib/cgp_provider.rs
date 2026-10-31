use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};

/// `#[cgp_provider]`: a provider-trait impl gets its `IsProviderFor` impl.
pub struct CgpProviderMacro;

impl Pipeline for CgpProviderMacro {
    const NAME: &'static str = "cgp_provider";
    const FAMILY: IrFamily = IrFamily::ProviderImpl;
    type Ir = ItemCgpProvider;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse2(body)?;
        let args: ProviderArgs = parse2(attr)?;

        Ok(ItemCgpProvider { args, item_impl })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let lowered = ir.lower()?;

        Ok(quote! {
            #lowered
        })
    }
}

/// `cgp_provider` entry point.
pub fn cgp_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpProviderMacro>(attr, body)
}
