//! The `Getter` IR family: traits whose methods read fields of the context.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::{ComponentDef, Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::cgp_auto_getter::ItemCgpAutoGetter;
use crate::macro_core::types::cgp_component::ItemCgpComponent;
use crate::macro_core::types::cgp_getter::ItemCgpGetter;

/// A getter trait.
pub enum Getter {
    /// `#[cgp_getter]`: a getter component, with the `UseField`/`UseFields`/`WithProvider`
    /// providers.
    Component(ItemCgpComponent),
    /// `#[cgp_getter]` on a trait that only declares an associated type: emitted through
    /// the abstract-type path (`UseType`/`WithProvider`) rather than a field read.
    AbstractType(ItemCgpComponent),
    /// `#[cgp_auto_getter]`: a getter trait implemented for every context with the fields.
    Auto(ItemCgpAutoGetter),
}

impl FamilyIr for Getter {
    const FAMILY: IrFamily = IrFamily::Getter;

    /// The component-name parameter checks of the component variants.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        match self {
            Getter::Component(component) | Getter::AbstractType(component) => {
                component.validate(diagnostics);
            }
            Getter::Auto(_) => {}
        }
    }

    /// Generates the getter trait's items.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            Getter::Component(component) => {
                let item_getter = ItemCgpGetter::try_from(component.preprocess()?.eval()?)?;

                let items = item_getter.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
            Getter::AbstractType(component) => ComponentDef::AbstractType(component).emit(),
            Getter::Auto(item) => {
                let items = item.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
        }
    }
}
