use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemImpl, ItemTrait, parse2};

use crate::ir::{Diagnostics, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentArgs, ItemCgpComponent};
use crate::macro_core::types::cgp_getter::ItemCgpGetter;
use crate::macro_core::types::cgp_type::{ItemCgpType, extract_item_type_from_trait};

/// `#[derive_provider(WithProvider)]` emits the `WithProvider` impl for a
/// `#[cgp_component]` trait. The `WithProvider` struct already lives in
/// `cgp_fork::core::component`; this macro only generates the impl (and its `IsProviderFor`).
pub struct DeriveProviderMacro;

/// The IR of `#[derive_provider]`: the original item, re-emitted unchanged, and the
/// component it describes.
pub struct DeriveProviderIr {
    /// The annotated trait with its inner `#[cgp_component]`, emitted as written so the
    /// component still expands.
    pub item: TokenStream,
    /// The component, without its `#[cgp_component]` attribute.
    pub component: ItemCgpComponent,
}

impl Pipeline for DeriveProviderMacro {
    const NAME: &'static str = "derive_provider";
    const FAMILY: IrFamily = IrFamily::ComponentDef;
    type Ir = DeriveProviderIr;

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

        Ok(DeriveProviderIr {
            item: body,
            component: ItemCgpComponent {
                args,
                item_trait: trait_for_pipeline,
            },
        })
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.component.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let DeriveProviderIr { item, component } = ir;
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
}

/// `#[derive_provider]` entry point.
pub fn derive_provider(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<DeriveProviderMacro>(attr, item)
}
