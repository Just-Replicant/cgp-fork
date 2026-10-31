use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::sum::SumType;

/// `Sum!`: a type-level sum (`Either` chain).
pub struct SumTypeMacro;

impl Pipeline for SumTypeMacro {
    const NAME: &'static str = "Sum";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = SumType;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `Sum!` entry point.
#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> syn::Result<TokenStream> {
    run::<SumTypeMacro>(TokenStream::new(), body)
}
