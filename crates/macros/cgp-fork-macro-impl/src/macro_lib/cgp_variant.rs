use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(CgpVariant)]`.
pub struct DeriveCgpVariantMacro;

impl Pipeline for DeriveCgpVariantMacro {
    const NAME: &'static str = "derive_cgp_variant";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpVariant;

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

/// `derive_cgp_variant` entry point.
pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveCgpVariantMacro>(TokenStream::new(), body)
}
