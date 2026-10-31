use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, TraitItem, parse2};

use crate::ir::{Diagnostics, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use crate::macro_core::types::cgp_getter::ItemCgpGetter;
use crate::macro_lib::CgpTypeMacro;

/// `#[cgp_getter]`: a trait of field getters becomes a getter component.
pub struct CgpGetterMacro;

/// The IR of `#[cgp_getter]`: a getter component, or an abstract type when the trait
/// only declares an associated type.
pub enum CgpGetterIr {
    /// Getter methods, emitted with the `UseField`/`UseFields`/`WithProvider` providers.
    Getter(ItemCgpComponent),
    /// A lone associated type, emitted through the `#[cgp_type]` provider path
    /// (`UseType`/`WithProvider`) rather than a field read.
    AbstractType(ItemCgpComponent),
}

impl Pipeline for CgpGetterMacro {
    const NAME: &'static str = "cgp_getter";
    const FAMILY: IrFamily = IrFamily::Getter;
    type Ir = CgpGetterIr;

    /// Parses the component, naming the provider `{Field}Getter` (from `Has{Field}`)
    /// by default.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_trait: ItemTrait = parse2(body.clone())?;

        if is_assoc_type_only(&item_trait) {
            return CgpTypeMacro::parse(attr, body).map(CgpGetterIr::AbstractType);
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

        Ok(CgpGetterIr::Getter(ItemCgpComponent {
            args: raw_args.try_into()?,
            item_trait,
        }))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        match ir {
            CgpGetterIr::Getter(item) | CgpGetterIr::AbstractType(item) => {
                item.validate(diagnostics);
            }
        }
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let item = match ir {
            CgpGetterIr::AbstractType(item) => return CgpTypeMacro::emit(item),
            CgpGetterIr::Getter(item) => item,
        };

        let item_getter = ItemCgpGetter::try_from(item.preprocess()?.eval()?)?;

        let items = item_getter.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `#[cgp_getter]` entry point.
pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpGetterMacro>(attr, body)
}

fn is_assoc_type_only(item_trait: &ItemTrait) -> bool {
    matches!(item_trait.items.as_slice(), [TraitItem::Type(_)])
}
