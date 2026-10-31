use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpData;

/// `#[derive(CgpData)]`: everything a struct or an enum supports.
pub struct DeriveCgpDataMacro;

impl Pipeline for DeriveCgpDataMacro {
    const NAME: &'static str = "derive_cgp_data";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpData>(body).map(DataShape::from)
    }
}

/// `derive_cgp_data` entry point.
pub fn derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpDataMacro>(TokenStream::new(), body)
}
