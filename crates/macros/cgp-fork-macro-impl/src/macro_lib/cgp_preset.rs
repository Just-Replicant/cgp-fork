use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_preset::ItemCgpPreset;

/// `cgp_preset!`: a preset module whose `Provider` delegates each
/// entry, plus `with_components!` so another preset or `delegate_components!`
/// can replay those entries.
pub struct CgpPresetMacro;

impl Pipeline for CgpPresetMacro {
    const NAME: &'static str = "cgp_preset";
    const FAMILY: IrFamily = IrFamily::WiringTable;
    type Ir = ItemCgpPreset;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.expand()
    }
}

/// `cgp_preset` entry point.
pub fn cgp_preset(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpPresetMacro>(TokenStream::new(), body)
}
