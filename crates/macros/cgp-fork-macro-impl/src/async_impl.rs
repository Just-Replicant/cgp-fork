use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemTrait, ReturnType, TraitItem, Type, parse_quote};

use crate::ir::{IrFamily, Pipeline, run};

/// `#[async_trait]`: desugars every `async fn` in a trait into a method returning
/// `impl Future`.
///
/// A default body is wrapped in `async move { … }`, so a trait can keep provided
/// async methods. `#[async_trait]` takes no arguments: the desugared future is never
/// `Send`-bounded, and `CanSendRun` is the component that asks for a `Send` future.
///
/// Any item other than a trait (an `impl` block, for instance) is returned unchanged,
/// because native `async fn` in an impl already matches the desugared trait method.
pub struct AsyncTraitMacro;

/// The IR of `#[async_trait]`.
pub enum AsyncTraitIr {
    /// A trait whose `async fn`s are desugared.
    Trait(Box<ItemTrait>),
    /// Any other item, emitted unchanged.
    Passthrough(TokenStream),
}

impl Pipeline for AsyncTraitMacro {
    const NAME: &'static str = "async_trait";
    const FAMILY: IrFamily = IrFamily::TraitRewrite;
    type Ir = AsyncTraitIr;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        if !attr.is_empty() {
            return Err(syn::Error::new_spanned(
                attr,
                "`#[async_trait]` takes no arguments; the desugared future is not `Send`. \
                 Use `CanSendRun` or a `Send` bound at the call site when a `Send` future is needed",
            ));
        }

        Ok(match syn::parse2::<ItemTrait>(body.clone()) {
            Ok(item_trait) => AsyncTraitIr::Trait(Box::new(item_trait)),
            Err(_) => AsyncTraitIr::Passthrough(body),
        })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let mut target_trait = match ir {
            AsyncTraitIr::Trait(item_trait) => *item_trait,
            AsyncTraitIr::Passthrough(item) => return Ok(item),
        };

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

        Ok(target_trait.to_token_stream())
    }
}

/// `#[async_trait]` entry point.
pub fn impl_async(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<AsyncTraitMacro>(attr, item)
}
