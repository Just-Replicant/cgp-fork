use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Item, ItemEnum, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_data::{ItemCgpRecord, derive_has_fields_impls_from_enum};

/// `#[derive(HasFields)]`: the field list of a struct or the variant list of an enum.
pub struct DeriveHasFieldsMacro;

impl Pipeline for DeriveHasFieldsMacro {
    const NAME: &'static str = "derive_has_fields";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = HasFieldsIr;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        match parse2::<Item>(body)? {
            Item::Struct(item_struct) => Ok(HasFieldsIr::Record(ItemCgpRecord { item_struct })),
            Item::Enum(item_enum) => Ok(HasFieldsIr::Variants(item_enum)),
            item => Err(Error::new_spanned(
                item,
                "expect body to be either a struct or enum",
            )),
        }
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = match &ir {
            HasFieldsIr::Record(record) => record.to_has_fields_impls()?,
            HasFieldsIr::Variants(item_enum) => derive_has_fields_impls_from_enum(item_enum)?,
        };

        Ok(quote! {
            #( #items )*
        })
    }
}

/// The IR of `#[derive(HasFields)]`.
pub enum HasFieldsIr {
    /// A struct: its fields, in order.
    Record(ItemCgpRecord),
    /// An enum: its variants, in order.
    Variants(ItemEnum),
}

/// `derive_has_fields` entry point.
pub fn derive_has_fields(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveHasFieldsMacro>(TokenStream::new(), body)
}
