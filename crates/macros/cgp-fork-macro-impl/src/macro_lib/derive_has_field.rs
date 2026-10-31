use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::ItemCgpRecord;

/// `#[derive(HasField)]`: one `HasField` impl per struct field.
pub struct DeriveHasFieldMacro;

impl Pipeline for DeriveHasFieldMacro {
    const NAME: &'static str = "derive_has_field";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = ItemCgpRecord;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_struct: ItemStruct = parse2(body)?;

        Ok(ItemCgpRecord { item_struct })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_has_field_impls()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `derive_has_field` entry point.
pub fn derive_has_field(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveHasFieldMacro>(TokenStream::new(), body)
}
