use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, RecordDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(BuildField)]`: the builder of a struct.
pub struct DeriveBuildFieldMacro;

impl Pipeline for DeriveBuildFieldMacro {
    const NAME: &'static str = "derive_build_field";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpRecord>(body)
            .map(|record| DataShape::Record(record, RecordDerive::BuildField))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_build_field` entry point.
pub fn derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveBuildFieldMacro>(TokenStream::new(), body)
}
