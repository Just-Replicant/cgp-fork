//! The `ComponentDef` IR family: a component and the items derived from it.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, ItemImpl};

use crate::extra_macro_core::types::cgp_auto_dispatch::ItemCgpAutoDispatch;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;
use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::cgp_component::ItemCgpComponent;
use crate::macro_core::types::cgp_fn::ItemCgpFn;
use crate::macro_core::types::cgp_getter::ItemCgpGetter;
use crate::macro_core::types::cgp_type::{ItemCgpType, extract_item_type_from_trait};

/// A component definition, as written with one of the component macros.
pub enum ComponentDef {
    /// `#[cgp_component]`: a consumer trait with its provider trait and standard providers.
    Component(ItemCgpComponent),
    /// `#[cgp_type]` (and `#[cgp_getter]` on a lone associated type): an abstract type.
    AbstractType(ItemCgpComponent),
    /// `#[derive_provider(WithProvider)]`: the `WithProvider` impl of a component, emitted
    /// after the original item, which still carries its `#[cgp_component]`.
    WithProvider {
        /// The annotated item, emitted unchanged.
        item: TokenStream,
        /// The component, without its `#[cgp_component]` attribute.
        component: ItemCgpComponent,
    },
    /// `#[cgp_fn]`: a function lifted into a trait with one blanket implementation.
    Function(ItemCgpFn),
    /// `#[cgp_auto_dispatch]`: a trait implemented for enums by dispatching per variant.
    AutoDispatch(ItemCgpAutoDispatch),
}

impl FamilyIr for ComponentDef {
    const FAMILY: IrFamily = IrFamily::ComponentDef;

    /// The component-name parameter checks of every component variant.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        match self {
            ComponentDef::Component(component)
            | ComponentDef::AbstractType(component)
            | ComponentDef::WithProvider { component, .. } => component.validate(diagnostics),
            ComponentDef::Function(_) | ComponentDef::AutoDispatch(_) => {}
        }
    }

    /// Generates the items of the component.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            ComponentDef::Component(component) => {
                let derived = component.preprocess()?.eval()?.to_items()?;

                Ok(quote! {
                    #( #derived )*
                })
            }
            ComponentDef::AbstractType(component) => {
                let item_cgp_type = ItemCgpType {
                    item_component: component.preprocess()?.eval()?,
                };

                let items = item_cgp_type.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
            ComponentDef::WithProvider { item, component } => emit_with_provider(item, component),
            ComponentDef::Function(item_cgp_fn) => {
                let items = item_cgp_fn.preprocess()?.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
            ComponentDef::AutoDispatch(item) => {
                let evaluated = item.preprocess()?.eval()?;

                let item_trait = &evaluated.item_trait;
                let blanket_impl = &evaluated.blanket_impl;

                let mut output = quote! {
                    #item_trait
                    #blanket_impl
                };

                for computer in &evaluated.computers {
                    output.extend(lower_handler_fn(&computer.preprocess()?.eval()?)?);
                }

                Ok(output)
            }
        }
    }
}

fn emit_with_provider(item: TokenStream, component: ItemCgpComponent) -> syn::Result<TokenStream> {
    let trait_span = component.item_trait.span();
    let evaluated = component.preprocess()?.eval()?;

    let provider_impls: Vec<ItemImpl> =
        if extract_item_type_from_trait(&evaluated.consumer_trait).is_ok() {
            let item = ItemCgpType {
                item_component: evaluated,
            };
            item.to_with_provider_impl()?.to_item_impls()?
        } else {
            let item = ItemCgpGetter::try_from(evaluated)?;
            let provider = item.to_with_provider_impl()?.ok_or_else(|| {
                Error::new(
                    trait_span,
                    "`#[derive_provider(WithProvider)]` supports a getter with a single method",
                )
            })?;
            provider.to_item_impls()?
        };

    // Re-emit the original trait so the inner `#[cgp_component]` still expands.
    // This attribute has to be the outer one.
    Ok(quote! {
        #item
        #(#provider_impls)*
    })
}
