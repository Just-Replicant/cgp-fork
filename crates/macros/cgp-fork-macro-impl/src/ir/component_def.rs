//! The `ComponentDef` IR family: a component and the items derived from it.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, ItemImpl, TraitItem};

use crate::extra_macro_core::types::cgp_auto_dispatch::ItemCgpAutoDispatch;
use crate::extra_macro_core::types::cgp_handler::lower_handler;
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

    /// The component-name parameter checks of every component variant, and for
    /// `#[derive_provider(WithProvider)]` the check that a getter has a single method.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        match self {
            ComponentDef::Component(component) | ComponentDef::AbstractType(component) => {
                component.validate(diagnostics);
            }
            ComponentDef::WithProvider { component, .. } => {
                component.validate(diagnostics);
                validate_with_provider(component, diagnostics);
            }
            ComponentDef::Function(_) | ComponentDef::AutoDispatch(_) => {}
        }
    }

    /// Generates the items of the component.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            ComponentDef::Component(component) => {
                let mock = crate::macro_core::types::cgp_component::mock_items(&component)?;
                let derived = component.preprocess()?.eval()?.to_items()?;

                Ok(quote! {
                    #( #derived )*
                    #mock
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

                for handler in &evaluated.handlers {
                    output.extend(lower_handler(&handler.eval()?)?);
                }

                Ok(output)
            }
        }
    }
}

/// The error for a `#[derive_provider(WithProvider)]` getter with more than one method.
fn multi_method_getter_error(component: &ItemCgpComponent) -> Error {
    Error::new(
        component.item_trait.span(),
        "`#[derive_provider(WithProvider)]` supports a getter with a single method",
    )
}

/// `WithProvider` reads one field, so a getter it derives for may declare one method.
/// An abstract type (a lone associated type) has no method and is always accepted; a
/// trait with other kinds of items is left to the getter parser, which names them.
fn validate_with_provider(component: &ItemCgpComponent, diagnostics: &mut Diagnostics) {
    let methods = component
        .item_trait
        .items
        .iter()
        .filter(|item| matches!(item, TraitItem::Fn(_)))
        .count();

    if methods > 1 {
        diagnostics.push(multi_method_getter_error(component));
    }
}

fn emit_with_provider(item: TokenStream, component: ItemCgpComponent) -> syn::Result<TokenStream> {
    // Kept for the error below; validation has already rejected a multi-method getter.
    let multi_method_error = multi_method_getter_error(&component);
    let evaluated = component.preprocess()?.eval()?;

    let provider_impls: Vec<ItemImpl> =
        if extract_item_type_from_trait(&evaluated.consumer_trait).is_ok() {
            let item = ItemCgpType {
                item_component: evaluated,
            };
            item.to_with_provider_impl()?.to_item_impls()?
        } else {
            let item = ItemCgpGetter::try_from(evaluated)?;
            let provider = item.to_with_provider_impl()?.ok_or(multi_method_error)?;
            provider.to_item_impls()?
        };

    // Re-emit the original trait so the inner `#[cgp_component]` still expands.
    // This attribute has to be the outer one.
    Ok(quote! {
        #item
        #(#provider_impls)*
    })
}
