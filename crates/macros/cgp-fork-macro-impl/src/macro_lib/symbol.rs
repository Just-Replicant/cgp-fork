use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, run};

/// `Symbol!`: a type-level string.
pub struct SymbolMacro;

impl Pipeline for SymbolMacro {
    const NAME: &'static str = "Symbol";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Symbol)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `Symbol!` entry point.
#[allow(non_snake_case)]
pub fn Symbol(body: TokenStream) -> syn::Result<TokenStream> {
    run::<SymbolMacro>(TokenStream::new(), body)
}
