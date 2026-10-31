use proc_macro2::TokenStream;
use syn::{ItemImpl, parse2};

use crate::ir::{Pipeline, ProviderImpl, run};
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};

/// `#[cgp_provider]`: a provider-trait impl gets its `IsProviderFor` impl.
pub struct CgpProviderMacro;

impl Pipeline for CgpProviderMacro {
    const NAME: &'static str = "cgp_provider";
    type Ir = ProviderImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse2(body)?;
        let args: ProviderArgs = parse2(attr)?;

        Ok(ProviderImpl::Provider(Box::new(ItemCgpProvider {
            args,
            item_impl,
        })))
    }
}

/// `cgp_provider` entry point.
pub fn cgp_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpProviderMacro>(attr, body)
}
