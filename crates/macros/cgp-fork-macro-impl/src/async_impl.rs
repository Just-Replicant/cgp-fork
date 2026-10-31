use proc_macro2::TokenStream;
use syn::ItemTrait;

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Pipeline, TraitRewrite, run};

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

impl Pipeline for AsyncTraitMacro {
    const NAME: &'static str = "async_trait";
    const FAMILY: IrFamily = TraitRewrite::FAMILY;
    type Ir = TraitRewrite;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        if !attr.is_empty() {
            return Err(syn::Error::new_spanned(
                attr,
                "`#[async_trait]` takes no arguments; the desugared future is not `Send`. \
                 Use `CanSendRun` or a `Send` bound at the call site when a `Send` future is needed",
            ));
        }

        Ok(match syn::parse2::<ItemTrait>(body.clone()) {
            Ok(item_trait) => TraitRewrite::AsyncTrait(Box::new(item_trait)),
            Err(_) => TraitRewrite::Passthrough(body),
        })
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `#[async_trait]` entry point.
pub fn impl_async(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<AsyncTraitMacro>(attr, item)
}
