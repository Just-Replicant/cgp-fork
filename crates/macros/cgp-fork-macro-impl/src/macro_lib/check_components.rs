use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::check_components::CheckComponentsTables;

/// `check_components!`: compile-time checks that a context can use its components.
pub struct CheckComponentsMacro;

impl Pipeline for CheckComponentsMacro {
    const NAME: &'static str = "check_components";
    const FAMILY: IrFamily = IrFamily::WiringTable;
    type Ir = CheckComponentsTables;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `check_components` entry point.
pub fn check_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<CheckComponentsMacro>(TokenStream::new(), body)
}
