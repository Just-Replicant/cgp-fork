use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Pipeline, WiringTable, run};

/// `check_components!`: compile-time checks that a context can use its components.
pub struct CheckComponentsMacro;

impl Pipeline for CheckComponentsMacro {
    const NAME: &'static str = "check_components";
    const FAMILY: IrFamily = WiringTable::FAMILY;
    type Ir = WiringTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(WiringTable::Check)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `check_components!` entry point.
pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CheckComponentsMacro>(TokenStream::new(), body)
}
