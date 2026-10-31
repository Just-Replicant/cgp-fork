use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::path::UniPath;

/// `Path!`: a type-level namespace path.
pub struct PathMacro;

impl Pipeline for PathMacro {
    const NAME: &'static str = "Path";
    const FAMILY: IrFamily = IrFamily::Namespace;
    type Ir = UniPath;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.to_token_stream())
    }
}

/// `Path!` entry point.
#[allow(non_snake_case)]
pub fn Path(body: TokenStream) -> syn::Result<TokenStream> {
    run::<PathMacro>(TokenStream::new(), body)
}
