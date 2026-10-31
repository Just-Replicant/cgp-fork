use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Pipeline, WiringTable, run};

/// `cgp_preset!`: a preset module whose `Provider` delegates each
/// entry, plus `with_components!` so another preset or `delegate_components!`
/// can replay those entries.
pub struct CgpPresetMacro;

impl Pipeline for CgpPresetMacro {
    const NAME: &'static str = "cgp_preset";
    const FAMILY: IrFamily = WiringTable::FAMILY;
    type Ir = WiringTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(WiringTable::Preset)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `cgp_preset!` entry point.
pub fn cgp_preset(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpPresetMacro>(TokenStream::new(), body)
}
