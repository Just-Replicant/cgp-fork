use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpData;

/// `#[derive(CgpData)]`.
pub struct DeriveCgpDataMacro;

impl Pipeline for DeriveCgpDataMacro {
    const NAME: &'static str = "derive_cgp_data";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpData;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse2(body)
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `derive_cgp_data` entry point.
pub fn derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpDataMacro>(TokenStream::new(), body)
}
