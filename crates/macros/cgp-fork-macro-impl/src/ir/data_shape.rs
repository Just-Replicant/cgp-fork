//! The `DataShape` IR family: records, variants, and type-level data constructors.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::cgp_data::{ItemCgpData, ItemCgpRecord, ItemCgpVariant};
use crate::macro_core::types::field::Symbol;
use crate::macro_core::types::product::{ProductExpr, ProductType};
use crate::macro_core::types::shape::{EnumType, StructType};
use crate::macro_core::types::sum::SumType;

/// What to derive for a struct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordDerive {
    /// `#[derive(HasField)]`: one `HasField` impl per field.
    HasField,
    /// `#[derive(HasFields)]`: the field list.
    HasFields,
    /// `#[derive(BuildField)]`: the builder.
    BuildField,
    /// `#[derive(CgpRecord)]`, or `#[derive(CgpData)]` on a struct: everything a record
    /// supports.
    All,
}

/// What to derive for an enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariantDerive {
    /// `#[derive(HasFields)]`: the variant list.
    HasFields,
    /// `#[derive(ExtractField)]`: the extractor.
    ExtractField,
    /// `#[derive(FromVariant)]`: one `FromVariant` impl per variant.
    FromVariant,
    /// `#[derive(CgpVariant)]`, or `#[derive(CgpData)]` on an enum: everything a variant
    /// type supports.
    All,
}

/// A data shape.
pub enum DataShape {
    /// A struct and what to derive for it.
    Record(ItemCgpRecord, RecordDerive),
    /// An enum and what to derive for it.
    Variants(ItemCgpVariant, VariantDerive),
    /// `Symbol!`: a type-level string.
    Symbol(Symbol),
    /// `Product!`: a type-level product.
    ProductType(ProductType),
    /// `product!`: a value-level product.
    ProductExpr(ProductExpr),
    /// `Sum!`: a type-level sum.
    Sum(SumType),
    /// `Struct!`: the type-level form of a struct.
    Struct(StructType),
    /// `Enum!`: the type-level form of an enum.
    Enum(EnumType),
}

impl From<ItemCgpData> for DataShape {
    /// `#[derive(CgpData)]`: everything the item's kind supports.
    fn from(data: ItemCgpData) -> Self {
        match data {
            ItemCgpData::Record(record) => DataShape::Record(record, RecordDerive::All),
            ItemCgpData::Variant(variant) => DataShape::Variants(variant, VariantDerive::All),
        }
    }
}

impl FamilyIr for DataShape {
    const FAMILY: IrFamily = IrFamily::DataShape;

    /// No family-wide checks yet; each shape still checks while generating.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        let _ = diagnostics;
    }

    /// Generates the derived impls or the type-level expression.
    fn emit(self) -> syn::Result<TokenStream> {
        let items: Vec<TokenStream> = match self {
            DataShape::Record(record, derive) => match derive {
                RecordDerive::HasField => tokens(record.to_has_field_impls()?),
                RecordDerive::HasFields => tokens(record.to_has_fields_impls()?),
                RecordDerive::BuildField => tokens(record.to_build_field_items()?),
                RecordDerive::All => tokens(record.to_items()?),
            },
            DataShape::Variants(variant, derive) => match derive {
                VariantDerive::HasFields => tokens(variant.to_has_fields_impls()?),
                VariantDerive::ExtractField => tokens(variant.to_extract_field_items()?),
                VariantDerive::FromVariant => tokens(variant.to_from_variant_impls()?),
                VariantDerive::All => tokens(variant.to_items()?),
            },
            DataShape::Symbol(symbol) => return Ok(symbol.to_token_stream()),
            DataShape::ProductType(product) => return Ok(product.eval()?.to_token_stream()),
            DataShape::ProductExpr(product) => return Ok(product.eval()?.to_token_stream()),
            DataShape::Sum(sum) => return Ok(sum.eval()?.to_token_stream()),
            DataShape::Struct(shape) => return Ok(shape.eval()?.to_token_stream()),
            DataShape::Enum(shape) => return Ok(shape.eval()?.to_token_stream()),
        };

        Ok(quote! {
            #( #items )*
        })
    }
}

fn tokens<T: ToTokens>(items: Vec<T>) -> Vec<TokenStream> {
    items.iter().map(ToTokens::to_token_stream).collect()
}
