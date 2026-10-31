use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpData;

/// `#[derive(CgpData)]`: everything a struct or an enum supports.
pub struct DeriveCgpDataMacro;

impl Pipeline for DeriveCgpDataMacro {
    const NAME: &'static str = "derive_cgp_data";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpData>(body).map(DataShape::from)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_cgp_data` entry point.
pub fn derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpDataMacro>(TokenStream::new(), body)
}
