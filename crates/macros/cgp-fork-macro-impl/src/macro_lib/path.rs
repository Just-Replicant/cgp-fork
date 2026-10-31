use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Namespace, Pipeline, run};

/// `Path!`: a type-level namespace path.
pub struct PathMacro;

impl Pipeline for PathMacro {
    const NAME: &'static str = "Path";
    const FAMILY: IrFamily = Namespace::FAMILY;
    type Ir = Namespace;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(Namespace::Path)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `Path!` entry point.
#[allow(non_snake_case)]
pub fn Path(body: TokenStream) -> syn::Result<TokenStream> {
    run::<PathMacro>(TokenStream::new(), body)
}
