use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, run};

/// `Symbol!`: a type-level string.
pub struct SymbolMacro;

impl Pipeline for SymbolMacro {
    const NAME: &'static str = "Symbol";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Symbol)
    }
}

/// `Symbol!` entry point.
#[allow(non_snake_case)]
pub fn Symbol(body: TokenStream) -> syn::Result<TokenStream> {
    run::<SymbolMacro>(TokenStream::new(), body)
}
