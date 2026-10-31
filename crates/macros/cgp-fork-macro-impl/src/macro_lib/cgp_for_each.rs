use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_for_each::CgpForEach;

/// Repeat a token body once per component type. Preset `with_components!`
/// macros expand through this so one preset entry becomes many impls.
pub struct CgpForEachMacro;

impl Pipeline for CgpForEachMacro {
    const NAME: &'static str = "cgp_for_each";
    const FAMILY: IrFamily = IrFamily::WiringTable;
    type Ir = CgpForEach;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.expand())
    }
}

/// `cgp_for_each` entry point.
pub fn cgp_for_each(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpForEachMacro>(TokenStream::new(), body)
}
