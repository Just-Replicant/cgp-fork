use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Namespace, Pipeline, run};

/// `cgp_namespace!`: a namespace and the components it redirects.
pub struct CgpNamespaceMacro;

impl Pipeline for CgpNamespaceMacro {
    const NAME: &'static str = "cgp_namespace";
    const FAMILY: IrFamily = Namespace::FAMILY;
    type Ir = Namespace;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(|table| Namespace::Table(Box::new(table)))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `cgp_namespace!` entry point.
pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpNamespaceMacro>(TokenStream::new(), body)
}
