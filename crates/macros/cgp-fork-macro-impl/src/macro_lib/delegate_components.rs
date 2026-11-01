use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{Pipeline, WiringTable, run};

/// `delegate_components!`: the wiring table of a context.
pub struct DelegateComponentsMacro;

impl Pipeline for DelegateComponentsMacro {
    const NAME: &'static str = "delegate_components";
    type Ir = WiringTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(WiringTable::Delegate)
    }
}

/// `delegate_components!` entry point.
pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DelegateComponentsMacro>(TokenStream::new(), body)
}
