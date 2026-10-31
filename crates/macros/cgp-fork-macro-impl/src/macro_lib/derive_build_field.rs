use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(BuildField)]`: the builder of a struct.
pub struct DeriveBuildFieldMacro;

impl Pipeline for DeriveBuildFieldMacro {
    const NAME: &'static str = "derive_build_field";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpRecord;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_struct: ItemStruct = parse2(body)?;

        Ok(ItemCgpRecord { item_struct })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_build_field_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `derive_build_field` entry point.
pub fn derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveBuildFieldMacro>(TokenStream::new(), body)
}
