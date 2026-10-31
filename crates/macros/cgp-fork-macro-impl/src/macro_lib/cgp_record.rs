use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, RecordDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(CgpRecord)]`: everything a record supports.
pub struct DeriveCgpRecordMacro;

impl Pipeline for DeriveCgpRecordMacro {
    const NAME: &'static str = "derive_cgp_record";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpRecord>(body).map(|record| DataShape::Record(record, RecordDerive::All))
    }
}

/// `derive_cgp_record` entry point.
pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpRecordMacro>(TokenStream::new(), body)
}
