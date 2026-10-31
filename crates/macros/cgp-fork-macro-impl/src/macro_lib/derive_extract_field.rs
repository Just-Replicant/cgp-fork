use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(ExtractField)]`: the extractor of an enum.
pub struct DeriveExtractFieldMacro;

impl Pipeline for DeriveExtractFieldMacro {
    const NAME: &'static str = "derive_extract_field";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpVariant;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_enum: ItemEnum = parse2(body)?;

        Ok(ItemCgpVariant { item_enum })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_extract_field_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `derive_extract_field` entry point.
pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveExtractFieldMacro>(TokenStream::new(), body)
}
