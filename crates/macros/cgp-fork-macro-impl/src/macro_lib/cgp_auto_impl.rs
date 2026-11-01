use proc_macro2::TokenStream;
use syn::ItemTrait;

use crate::ir::{Pipeline, TraitRewrite, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_impl::ItemCgpAutoImpl;

/// `#[cgp_auto_impl]`: a trait with default methods gets a blanket impl.
pub struct CgpAutoImplMacro;

impl Pipeline for CgpAutoImplMacro {
    const NAME: &'static str = "cgp_auto_impl";
    type Ir = TraitRewrite;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoImpl::preprocess(&item_trait).map(TraitRewrite::AutoImpl)
    }
}

/// `cgp_auto_impl` entry point.
pub fn cgp_auto_impl(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoImplMacro>(attr, body)
}
