use proc_macro2::TokenStream;
use syn::ItemTrait;

use crate::ir::{
    Diagnostics, FamilyIr, IrFamily, Pipeline, TraitRewrite, parse_body_without_args, run,
};
use crate::macro_core::types::cgp_auto_log::ItemCgpAutoLog;

/// `#[cgp_auto_log]`: a logging trait gets its logging providers.
pub struct CgpAutoLogMacro;

impl Pipeline for CgpAutoLogMacro {
    const NAME: &'static str = "cgp_auto_log";
    const FAMILY: IrFamily = TraitRewrite::FAMILY;
    type Ir = TraitRewrite;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoLog::parse(&item_trait).map(TraitRewrite::AutoLog)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `cgp_auto_log` entry point.
pub fn cgp_auto_log(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoLogMacro>(attr, body)
}
