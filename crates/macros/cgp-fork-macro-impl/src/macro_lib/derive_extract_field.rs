use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, VariantDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(ExtractField)]`: the extractor of an enum.
pub struct DeriveExtractFieldMacro;

impl Pipeline for DeriveExtractFieldMacro {
    const NAME: &'static str = "derive_extract_field";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpVariant>(body)
            .map(|variant| DataShape::Variants(variant, VariantDerive::ExtractField))
    }
}

/// `derive_extract_field` entry point.
pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveExtractFieldMacro>(TokenStream::new(), body)
}
