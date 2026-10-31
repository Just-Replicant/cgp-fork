use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, VariantDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(CgpVariant)]`: everything a variant type supports.
pub struct DeriveCgpVariantMacro;

impl Pipeline for DeriveCgpVariantMacro {
    const NAME: &'static str = "derive_cgp_variant";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpVariant>(body)
            .map(|variant| DataShape::Variants(variant, VariantDerive::All))
    }
}

/// `derive_cgp_variant` entry point.
pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpVariantMacro>(TokenStream::new(), body)
}
