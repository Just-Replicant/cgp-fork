use proc_macro2::TokenStream;
use syn::Error;

use crate::ir::{Pipeline, ProviderImpl, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_error::{ItemCgpAutoError, ItemCgpErrorEnum};

/// `#[cgp_auto_error]`: an inherent impl becomes error providers, or an enum gains
/// `Display` and a source chain.
pub struct CgpAutoErrorMacro;

impl Pipeline for CgpAutoErrorMacro {
    const NAME: &'static str = "cgp_auto_error";
    type Ir = ProviderImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item: syn::Item = parse_body_without_args(Self::NAME, attr, body)?;
        match item {
            syn::Item::Impl(item_impl) => ItemCgpAutoError::parse(&item_impl)
                .map(|item| ProviderImpl::AutoError(Box::new(item))),
            syn::Item::Enum(item_enum) => Ok(ProviderImpl::ErrorEnum(Box::new(
                ItemCgpErrorEnum::from_enum(item_enum),
            ))),
            other => Err(Error::new_spanned(
                other,
                "apply #[cgp_auto_error] to an inherent impl that names the provider, or to an error enum",
            )),
        }
    }
}

/// `cgp_auto_error` entry point.
pub fn cgp_auto_error(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoErrorMacro>(attr, body)
}
