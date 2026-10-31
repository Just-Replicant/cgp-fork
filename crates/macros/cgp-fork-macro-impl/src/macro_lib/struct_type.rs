use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, run};

/// `Struct!`: the type-level form of a struct.
pub struct StructTypeMacro;

impl Pipeline for StructTypeMacro {
    const NAME: &'static str = "Struct";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Struct)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `Struct!` entry point.
#[allow(non_snake_case)]
pub fn Struct(body: TokenStream) -> syn::Result<TokenStream> {
    run::<StructTypeMacro>(TokenStream::new(), body)
}
