use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, VariantDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(CgpVariant)]`: everything a variant type supports.
pub struct DeriveCgpVariantMacro;

impl Pipeline for DeriveCgpVariantMacro {
    const NAME: &'static str = "derive_cgp_variant";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpVariant>(body)
            .map(|variant| DataShape::Variants(variant, VariantDerive::All))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_cgp_variant` entry point.
pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpVariantMacro>(TokenStream::new(), body)
}
