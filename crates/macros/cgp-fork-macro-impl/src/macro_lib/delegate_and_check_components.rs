use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Pipeline, WiringTable, run};

/// `delegate_and_check_components!`: a wiring table plus the checks of every entry.
pub struct DelegateAndCheckComponentsMacro;

impl Pipeline for DelegateAndCheckComponentsMacro {
    const NAME: &'static str = "delegate_and_check_components";
    type Ir = WiringTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(WiringTable::DelegateAndCheck)
    }
}

/// `delegate_and_check_components!` entry point.
pub fn delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DelegateAndCheckComponentsMacro>(TokenStream::new(), body)
}
