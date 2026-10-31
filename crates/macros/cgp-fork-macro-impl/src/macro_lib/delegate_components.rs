use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Pipeline, WiringTable, run};

/// `delegate_components!`: the wiring table of a context.
pub struct DelegateComponentsMacro;

impl Pipeline for DelegateComponentsMacro {
    const NAME: &'static str = "delegate_components";
    const FAMILY: IrFamily = WiringTable::FAMILY;
    type Ir = WiringTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(WiringTable::Delegate)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `delegate_components!` entry point.
pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DelegateComponentsMacro>(TokenStream::new(), body)
}
