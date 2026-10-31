use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::delegate_and_check_components::ItemDelegateAndCheckComponents;

/// `delegate_and_check_components!`: a wiring table plus the checks of every entry.
pub struct DelegateAndCheckComponentsMacro;

impl Pipeline for DelegateAndCheckComponentsMacro {
    const NAME: &'static str = "delegate_and_check_components";
    const FAMILY: IrFamily = IrFamily::WiringTable;
    type Ir = ItemDelegateAndCheckComponents;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let check_table = ir.to_check_components()?;

        let evaluated_table = ir.table.eval()?;

        let check_items = check_table.to_items()?;

        Ok(quote! {
            #evaluated_table

            #( #check_items )*
        })
    }
}

/// `delegate_and_check_components` entry point.
pub fn delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DelegateAndCheckComponentsMacro>(TokenStream::new(), body)
}
