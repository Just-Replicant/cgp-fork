use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, run};

/// `Sum!`: a type-level sum (`Either` chain).
pub struct SumTypeMacro;

impl Pipeline for SumTypeMacro {
    const NAME: &'static str = "Sum";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::Sum)
    }
}

/// `Sum!` entry point.
#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> syn::Result<TokenStream> {
    run::<SumTypeMacro>(TokenStream::new(), body)
}
