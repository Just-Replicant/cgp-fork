use proc_macro2::TokenStream;
use syn::{Ident, ItemTrait, TraitItem, parse2};

use crate::ir::{Diagnostics, FamilyIr, Getter, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use crate::macro_lib::cgp_type::parse_abstract_type;

/// `#[cgp_getter]`: a trait of field getters becomes a getter component.
pub struct CgpGetterMacro;

impl Pipeline for CgpGetterMacro {
    const NAME: &'static str = "cgp_getter";
    const FAMILY: IrFamily = Getter::FAMILY;
    type Ir = Getter;

    /// Parses the component, naming the provider `{Field}Getter` (from `Has{Field}`) by
    /// default. A trait that only declares an associated type is an abstract type.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse2(body.clone())?;

        if is_assoc_type_only(&item_trait) {
            return parse_abstract_type(attr, body).map(Getter::AbstractType);
        }

        let mut raw_args: CgpComponentRawArgs = parse2(attr)?;

        if raw_args.provider_ident.is_none()
            && let Some(field_name) = item_trait.ident.to_string().strip_prefix("Has")
            && !field_name.is_empty()
        {
            raw_args.provider_ident = Some(Ident::new(
                &format!("{field_name}Getter"),
                item_trait.ident.span(),
            ));
        }

        Ok(Getter::Component(ItemCgpComponent {
            args: raw_args.try_into()?,
            item_trait,
        }))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

fn is_assoc_type_only(item_trait: &ItemTrait) -> bool {
    matches!(item_trait.items.as_slice(), [TraitItem::Type(_)])
}

/// `cgp_getter` entry point.
pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpGetterMacro>(attr, body)
}
