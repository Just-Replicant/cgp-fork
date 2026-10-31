use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, run};

/// `Sum!`: a type-level sum (`Either` chain).
pub struct SumTypeMacro;

impl Pipeline for SumTypeMacro {
    const NAME: &'static str = "Sum";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Sum)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `Sum!` entry point.
#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> syn::Result<TokenStream> {
    run::<SumTypeMacro>(TokenStream::new(), body)
}
