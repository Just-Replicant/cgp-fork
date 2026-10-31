//! The `TraitRewrite` IR family: attribute macros that rewrite a trait in place and add
//! at most a blanket impl, without defining a component or a provider.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemTrait, ReturnType, TraitItem, Type, parse_quote};

use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::blanket_trait::ItemBlanketTrait;
use crate::macro_core::types::cgp_auto_impl::ItemCgpAutoImpl;
use crate::macro_core::types::cgp_auto_log::ItemCgpAutoLog;

/// A trait rewritten in place.
pub enum TraitRewrite {
    /// `#[async_trait]` on a trait: every `async fn` desugared to `-> impl Future`.
    AsyncTrait(Box<ItemTrait>),
    /// `#[async_trait]` on any other item: emitted unchanged.
    Passthrough(TokenStream),
    /// `#[blanket_trait]`: a trait implied by its supertraits gets a blanket impl.
    Blanket(ItemBlanketTrait),
    /// `#[cgp_auto_impl]`: a trait with default methods gets a blanket impl.
    AutoImpl(ItemCgpAutoImpl),
    /// `#[cgp_auto_log]`: a logging trait gets its logging providers.
    AutoLog(ItemCgpAutoLog),
}

impl FamilyIr for TraitRewrite {
    const FAMILY: IrFamily = IrFamily::TraitRewrite;

    /// No family-wide checks yet.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        let _ = diagnostics;
    }

    /// Generates the rewritten trait and its impls.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            TraitRewrite::AsyncTrait(item_trait) => Ok(desugar_async_fns(*item_trait)),
            TraitRewrite::Passthrough(item) => Ok(item),
            TraitRewrite::Blanket(item) => items(item.to_items()?),
            TraitRewrite::AutoImpl(item) => items(item.to_items()?),
            TraitRewrite::AutoLog(item) => items(item.to_items()?),
        }
    }
}

fn items<T: ToTokens>(items: Vec<T>) -> syn::Result<TokenStream> {
    Ok(quote! {
        #( #items )*
    })
}

/// A default body is wrapped in `async move { … }`, so a trait keeps provided async methods.
fn desugar_async_fns(mut target_trait: ItemTrait) -> TokenStream {
    for trait_item in &mut target_trait.items {
        if let TraitItem::Fn(trait_fn) = trait_item
            && trait_fn.sig.asyncness.is_some()
        {
            let return_type: Type = match &trait_fn.sig.output {
                ReturnType::Default => parse_quote!(()),
                ReturnType::Type(_, return_type) => return_type.as_ref().clone(),
            };

            trait_fn.sig.output = parse_quote! {
                -> impl ::core::future::Future<Output = #return_type>
            };
            trait_fn.sig.asyncness = None;

            // A provided method keeps its body, now evaluated lazily as a future.
            if let Some(body) = trait_fn.default.take() {
                trait_fn.default = Some(parse_quote!({ async move #body }));
            }
        }
    }

    target_trait.to_token_stream()
}
