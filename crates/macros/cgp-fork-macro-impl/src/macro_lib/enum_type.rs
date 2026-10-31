use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::shape::EnumType;

/// `Enum!`: the type-level form of an enum.
pub struct EnumTypeMacro;

impl Pipeline for EnumTypeMacro {
    const NAME: &'static str = "Enum";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = EnumType;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `Enum!` entry point.
#[allow(non_snake_case)]
pub fn Enum(body: TokenStream) -> syn::Result<TokenStream> {
    run::<EnumTypeMacro>(TokenStream::new(), body)
}
