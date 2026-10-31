use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::product::{ProductExpr, ProductType};

/// `Product!`: a type-level product (`Cons` list).
pub struct ProductTypeMacro;

impl Pipeline for ProductTypeMacro {
    const NAME: &'static str = "Product";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ProductType;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `Product!` entry point.
#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> syn::Result<TokenStream> {
    run::<ProductTypeMacro>(TokenStream::new(), body)
}

/// `product!`: a value-level product (`Cons` list).
pub struct ProductExprMacro;

impl Pipeline for ProductExprMacro {
    const NAME: &'static str = "product";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ProductExpr;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `product!` entry point.
pub fn product(body: TokenStream) -> syn::Result<TokenStream> {
    run::<ProductExprMacro>(TokenStream::new(), body)
}
