use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Pipeline, run};

/// `Product!`: a type-level product (`Cons` list).
pub struct ProductTypeMacro;

impl Pipeline for ProductTypeMacro {
    const NAME: &'static str = "Product";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::ProductType)
    }
}

/// `product!`: a value-level product (`Cons` list).
pub struct ProductExprMacro;

impl Pipeline for ProductExprMacro {
    const NAME: &'static str = "product";
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::ProductExpr)
    }
}

/// `product!` entry point.
pub fn product(body: TokenStream) -> syn::Result<TokenStream> {
    run::<ProductExprMacro>(TokenStream::new(), body)
}

/// `Product!` entry point.
#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> syn::Result<TokenStream> {
    run::<ProductTypeMacro>(TokenStream::new(), body)
}
