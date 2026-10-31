use proc_macro2::TokenStream;
use syn::{Error, Item, parse2};

use crate::ir::{
    DataShape, Diagnostics, FamilyIr, IrFamily, Pipeline, RecordDerive, VariantDerive, run,
};
use crate::macro_core::types::cgp_data::{ItemCgpRecord, ItemCgpVariant};

/// `#[derive(HasFields)]`: the field list of a struct or the variant list of an enum.
pub struct DeriveHasFieldsMacro;

impl Pipeline for DeriveHasFieldsMacro {
    const NAME: &'static str = "derive_has_fields";
    const FAMILY: IrFamily = DataShape::FAMILY;
    type Ir = DataShape;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        match parse2::<Item>(body)? {
            Item::Struct(item_struct) => Ok(DataShape::Record(
                ItemCgpRecord { item_struct },
                RecordDerive::HasFields,
            )),
            Item::Enum(item_enum) => Ok(DataShape::Variants(
                ItemCgpVariant { item_enum },
                VariantDerive::HasFields,
            )),
            item => Err(Error::new_spanned(
                item,
                "expect body to be either a struct or enum",
            )),
        }
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `derive_has_fields` entry point.
pub fn derive_has_fields(body: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveHasFieldsMacro>(TokenStream::new(), body)
}
