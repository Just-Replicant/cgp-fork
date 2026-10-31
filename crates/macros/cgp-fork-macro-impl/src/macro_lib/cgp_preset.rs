use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Pipeline, WiringTable, run};

/// `cgp_preset!`: a preset module whose `Provider` delegates each
/// entry, plus `with_components!` so another preset or `delegate_components!`
/// can replay those entries.
pub struct CgpPresetMacro;

impl Pipeline for CgpPresetMacro {
    const NAME: &'static str = "cgp_preset";
    type Ir = WiringTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(WiringTable::Preset)
    }
}

/// `cgp_preset!` entry point.
pub fn cgp_preset(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpPresetMacro>(TokenStream::new(), body)
}
