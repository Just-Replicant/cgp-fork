use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, RecordDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(BuildField)]`: the builder of a struct.
pub struct DeriveBuildFieldMacro;

impl Pipeline for DeriveBuildFieldMacro {
    const NAME: &'static str = "derive_build_field";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpRecord>(body)
            .map(|record| DataShape::Record(record, RecordDerive::BuildField))
    }
}

/// `derive_build_field` entry point.
pub fn derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveBuildFieldMacro>(TokenStream::new(), body)
}
