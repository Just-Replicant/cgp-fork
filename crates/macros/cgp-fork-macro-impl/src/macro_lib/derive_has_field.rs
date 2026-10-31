use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, RecordDerive, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(HasField)]`: one `HasField` impl per struct field.
pub struct DeriveHasFieldMacro;

impl Pipeline for DeriveHasFieldMacro {
    const NAME: &'static str = "derive_has_field";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2::<ItemCgpRecord>(body)
            .map(|record| DataShape::Record(record, RecordDerive::HasField))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_has_field` entry point.
pub fn derive_has_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveHasFieldMacro>(TokenStream::new(), body)
}
