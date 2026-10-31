use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpVariant;

/// `#[derive(FromVariant)]`: one `FromVariant` impl per enum variant.
pub struct DeriveFromVariantMacro;

impl Pipeline for DeriveFromVariantMacro {
    const NAME: &'static str = "derive_from_variant";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpVariant;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_enum: ItemEnum = parse2(body)?;

        Ok(ItemCgpVariant { item_enum })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_from_variant_impls()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `derive_from_variant` entry point.
pub fn derive_from_variant(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveFromVariantMacro>(TokenStream::new(), body)
}
