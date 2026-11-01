use proc_macro2::TokenStream;
use syn::ItemTrait;

use crate::ir::{Pipeline, TraitRewrite, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_log::ItemCgpAutoLog;

/// `#[cgp_auto_log]`: a logging trait gets its logging providers.
pub struct CgpAutoLogMacro;

impl Pipeline for CgpAutoLogMacro {
    const NAME: &'static str = "cgp_auto_log";
    type Ir = TraitRewrite;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoLog::parse(&item_trait).map(TraitRewrite::AutoLog)
    }
}

/// `cgp_auto_log` entry point.
pub fn cgp_auto_log(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoLogMacro>(attr, body)
}
