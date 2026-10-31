use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::shape::StructType;

/// `Struct!`: the type-level form of a struct.
pub struct StructTypeMacro;

impl Pipeline for StructTypeMacro {
    const NAME: &'static str = "Struct";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = StructType;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `Struct!` entry point.
#[allow(non_snake_case)]
pub fn Struct(body: TokenStream) -> syn::Result<TokenStream> {
    run::<StructTypeMacro>(TokenStream::new(), body)
}
