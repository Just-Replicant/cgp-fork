use proc_macro2::TokenStream;
use syn::ItemTrait;

use crate::ir::{Diagnostics, FamilyIr, Getter, IrFamily, Pipeline, parse_body_without_args, run};
use crate::macro_core::types::cgp_auto_getter::ItemCgpAutoGetter;

/// `#[cgp_auto_getter]`: a getter trait implemented for every context with the fields.
pub struct CgpAutoGetterMacro;

impl Pipeline for CgpAutoGetterMacro {
    const NAME: &'static str = "cgp_auto_getter";
    const FAMILY: IrFamily = Getter::FAMILY;
    type Ir = Getter;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse_body_without_args(Self::NAME, attr, body)?;

        ItemCgpAutoGetter::preprocess(&item_trait).map(Getter::Auto)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `cgp_auto_getter` entry point.
pub fn cgp_auto_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoGetterMacro>(attr, body)
}
