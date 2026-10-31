use proc_macro2::TokenStream;
use syn::{ItemImpl, parse2};

use crate::ir::{Pipeline, ProviderImpl, run};
use crate::macro_core::types::cgp_impl::{ImplArgs, ItemCgpImpl};

/// `#[cgp_impl]`: an impl written against the consumer trait becomes a provider impl.
pub struct CgpImplMacro;

impl Pipeline for CgpImplMacro {
    const NAME: &'static str = "cgp_impl";
    type Ir = ProviderImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let args: ImplArgs = parse2(attr)?;
        let item_impl: ItemImpl = parse2(body)?;

        Ok(ProviderImpl::Impl(Box::new(ItemCgpImpl {
            args,
            item_impl,
        })))
    }
}

/// `cgp_impl` entry point.
pub fn cgp_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpImplMacro>(attr, body)
}
