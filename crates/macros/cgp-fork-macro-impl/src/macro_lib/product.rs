use proc_macro2::TokenStream;
use syn::parse2;

use crate::ir::{DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, run};

/// `Product!`: a type-level product (`Cons` list).
pub struct ProductTypeMacro;

impl Pipeline for ProductTypeMacro {
    const NAME: &'static str = "Product";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::ProductType)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `product!`: a value-level product (`Cons` list).
pub struct ProductExprMacro;

impl Pipeline for ProductExprMacro {
    const NAME: &'static str = "product";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body).map(DataShape::ProductExpr)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
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
