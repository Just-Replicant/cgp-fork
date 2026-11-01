use proc_macro2::Span;
use syn::{Ident, Item, ItemImpl, ItemTrait};

use crate::macro_core::functions::{
    add_on_unimplemented, auto_getter_on_unimplemented, parse_getter_fields,
};
use crate::macro_core::types::attributes::CgpComponentAttributes;
use crate::macro_core::types::cgp_auto_getter::derive_blanket_impl;

pub struct ItemCgpAutoGetter {
    pub item_trait: ItemTrait,
}

impl ItemCgpAutoGetter {
    pub fn preprocess(item_trait: &ItemTrait) -> syn::Result<Self> {
        let (_attributes, item_trait) = CgpComponentAttributes::preprocess(item_trait)?;
        Ok(Self { item_trait })
    }

    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let item_impl = Item::Impl(self.to_blanket_impl()?);

        Ok(vec![Item::Trait(self.to_item_trait()?), item_impl])
    }

    /// The trait as written, plus an `on_unimplemented` message naming the fields it reads.
    pub fn to_item_trait(&self) -> syn::Result<ItemTrait> {
        let context_type = Ident::new("__Context__", Span::call_site());
        let (fields, _) = parse_getter_fields(&context_type, &self.item_trait)?;
        let field_names: Vec<&Ident> = fields.iter().map(|field| &field.field_name).collect();

        let mut item_trait = self.item_trait.clone();
        if !field_names.is_empty() {
            add_on_unimplemented(
                &mut item_trait,
                auto_getter_on_unimplemented(&self.item_trait.ident, &field_names),
            );
        }

        Ok(item_trait)
    }

    pub fn to_blanket_impl(&self) -> syn::Result<ItemImpl> {
        let context_type = Ident::new("__Context__", Span::call_site());

        let (fields, field_type) = parse_getter_fields(&context_type, &self.item_trait)?;

        let blanket_impl = derive_blanket_impl(
            &context_type,
            &self.item_trait,
            &fields,
            field_type.as_ref(),
        )?;

        Ok(blanket_impl)
    }
}
