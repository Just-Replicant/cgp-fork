use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemTrait, parse2};

use crate::ir::{ComponentDef, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentArgs, ItemCgpComponent};

/// `#[derive_provider(WithProvider)]` emits the `WithProvider` impl for a
/// `#[cgp_component]` trait. The `WithProvider` struct already lives in
/// `cgp_fork::core::component`; this macro only generates the impl (and its `IsProviderFor`).
pub struct DeriveProviderMacro;

impl Pipeline for DeriveProviderMacro {
    const NAME: &'static str = "derive_provider";
    type Ir = ComponentDef;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let kind: Ident = parse2(attr)?;

        if kind != "WithProvider" {
            return Err(Error::new(
                kind.span(),
                "`derive_provider` only supports `WithProvider`",
            ));
        }

        let item_trait: ItemTrait = parse2(body.clone())?;
        let component_attr = item_trait
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("cgp_component"))
            .ok_or_else(|| {
                Error::new(
                    item_trait.span(),
                    "`#[derive_provider(WithProvider)]` must sit on a `#[cgp_component]` trait",
                )
            })?;

        let args: CgpComponentArgs = component_attr.parse_args()?;

        let mut trait_for_pipeline = item_trait.clone();
        trait_for_pipeline
            .attrs
            .retain(|attr| !attr.path().is_ident("cgp_component"));

        Ok(ComponentDef::WithProvider {
            item: body,
            component: ItemCgpComponent {
                args,
                item_trait: trait_for_pipeline,
            },
        })
    }
}

/// `derive_provider` entry point.
pub fn derive_provider(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveProviderMacro>(attr, item)
}
