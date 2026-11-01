use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, run};

/// `Enum!`: the type-level form of an enum.
pub struct EnumTypeMacro;

impl Pipeline for EnumTypeMacro {
    const NAME: &'static str = "Enum";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Enum)
    }
}

/// `Enum!` entry point.
#[allow(non_snake_case)]
pub fn Enum(body: TokenStream) -> syn::Result<TokenStream> {
    run::<EnumTypeMacro>(TokenStream::new(), body)
}
