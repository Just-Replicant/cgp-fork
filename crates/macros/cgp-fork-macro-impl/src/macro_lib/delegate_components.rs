use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::ir::{Diagnostics, IrFamily, Pipeline, run};
use crate::macro_core::types::delegate_component::{DelegateTable, ValidateAttributes};

/// `delegate_components!`: the wiring table of a context.
pub struct DelegateComponentsMacro;

impl Pipeline for DelegateComponentsMacro {
    const NAME: &'static str = "delegate_components";
    const FAMILY: IrFamily = IrFamily::WiringTable;
    type Ir = DelegateTable;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    /// `delegate_components!` does not support any attributes on the table or its
    /// keys, so reject them instead of silently parsing and discarding them.
    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        diagnostics.check(ir.validate_attributes());
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        Ok(ir.eval()?.to_token_stream())
    }
}

/// `delegate_components!` entry point.
pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DelegateComponentsMacro>(TokenStream::new(), body)
}
