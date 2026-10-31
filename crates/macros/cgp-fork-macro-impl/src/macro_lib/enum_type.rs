use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, run};

/// `Enum!`: the type-level form of an enum.
pub struct EnumTypeMacro;

impl Pipeline for EnumTypeMacro {
    const NAME: &'static str = "Enum";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Enum)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `Enum!` entry point.
#[allow(non_snake_case)]
pub fn Enum(body: TokenStream) -> syn::Result<TokenStream> {
    run::<EnumTypeMacro>(TokenStream::new(), body)
}
