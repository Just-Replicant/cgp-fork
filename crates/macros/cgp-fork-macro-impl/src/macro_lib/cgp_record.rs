use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, RecordDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(CgpRecord)]`: everything a record supports.
pub struct DeriveCgpRecordMacro;

impl Pipeline for DeriveCgpRecordMacro {
    const NAME: &'static str = "derive_cgp_record";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpRecord>(body).map(|record| DataShape::Record(record, RecordDerive::All))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_cgp_record` entry point.
pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpRecordMacro>(TokenStream::new(), body)
}
