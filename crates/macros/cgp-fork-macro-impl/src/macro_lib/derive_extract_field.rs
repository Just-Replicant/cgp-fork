use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, VariantDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(ExtractField)]`: the extractor of an enum.
pub struct DeriveExtractFieldMacro;

impl Pipeline for DeriveExtractFieldMacro {
    const NAME: &'static str = "derive_extract_field";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpVariant>(body)
            .map(|variant| DataShape::Variants(variant, VariantDerive::ExtractField))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_extract_field` entry point.
pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveExtractFieldMacro>(TokenStream::new(), body)
}
