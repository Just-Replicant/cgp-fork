use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::field::Symbol;

/// `Symbol!`: a type-level string.
pub struct SymbolMacro;

impl Pipeline for SymbolMacro {
    const NAME: &'static str = "Symbol";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = Symbol;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.to_token_stream())
    }
}

/// `Symbol!` entry point.
#[allow(non_snake_case)]
pub fn Symbol(body: TokenStream) -> syn::Result<TokenStream> {
    run::<SymbolMacro>(TokenStream::new(), body)
}
