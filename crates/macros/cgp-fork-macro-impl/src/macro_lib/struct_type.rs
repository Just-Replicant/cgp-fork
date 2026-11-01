use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, run};

/// `Struct!`: the type-level form of a struct.
pub struct StructTypeMacro;

impl Pipeline for StructTypeMacro {
    const NAME: &'static str = "Struct";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Struct)
    }
}

/// `Struct!` entry point.
#[allow(non_snake_case)]
pub fn Struct(body: TokenStream) -> syn::Result<TokenStream> {
    run::<StructTypeMacro>(TokenStream::new(), body)
}
