use proc_macro2::TokenStream;
use syn::ItemImpl;

use crate::ir::{Pipeline, ProviderImpl, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_error::ItemCgpAutoError;

/// `#[cgp_auto_error]`: error-raising methods on a context become error providers.
pub struct CgpAutoErrorMacro;

impl Pipeline for CgpAutoErrorMacro {
    const NAME: &'static str = "cgp_auto_error";
    type Ir = ProviderImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoError::parse(&item_impl).map(|item| ProviderImpl::AutoError(Box::new(item)))
    }
}

/// `cgp_auto_error` entry point.
pub fn cgp_auto_error(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoErrorMacro>(attr, body)
}
