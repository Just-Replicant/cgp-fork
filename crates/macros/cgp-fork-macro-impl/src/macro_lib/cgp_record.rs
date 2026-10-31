use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(CgpRecord)]`.
pub struct DeriveCgpRecordMacro;

impl Pipeline for DeriveCgpRecordMacro {
    const NAME: &'static str = "derive_cgp_record";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpRecord;

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

/// `derive_cgp_record` entry point.
pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpRecordMacro>(TokenStream::new(), body)
}
