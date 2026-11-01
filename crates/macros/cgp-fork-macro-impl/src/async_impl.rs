use proc_macro2::TokenStream;
use syn::{Ident, ItemTrait};

use crate::ir::{Pipeline, TraitRewrite, run};

const LEGACY_ARGUMENTS: &str = "`#[async_trait]` takes no arguments; the desugared future is not `Send`. \
     Use `#[cgp_async]` instead, or `#[cgp_async(Send)]` when the future must be `Send`";

const CGP_ASYNC_ARGUMENTS: &str = "`#[cgp_async]` accepts no arguments, or `Send` \
     (`#[cgp_async(Send)]` adds `+ Send` to every desugared future)";

/// `#[async_trait]`: the old name of [`CgpAsyncMacro`].
///
/// Same desugaring, and it never adds `Send`. Arguments are rejected; `#[cgp_async(Send)]`
/// is how a future becomes `Send`.
pub struct AsyncTraitMacro;

impl Pipeline for AsyncTraitMacro {
    const NAME: &'static str = "async_trait";
    type Ir = TraitRewrite;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse_async(attr, body, false)
    }
}

/// `#[cgp_async]` and `#[cgp_async(Send)]`.
///
/// Desugars every `async fn` in a trait into a method returning `impl Future`.
/// `Send` adds `+ Send` to that future. A default body is wrapped in `async move { … }`.
/// Any item other than a trait is returned unchanged, because a native `async fn` in an
/// impl already matches the desugared trait method (and `#[cgp_component]` copies this
/// attribute onto those impls).
pub struct CgpAsyncMacro;

impl Pipeline for CgpAsyncMacro {
    const NAME: &'static str = "cgp_async";
    type Ir = TraitRewrite;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse_async(attr, body, true)
    }
}

/// `#[async_trait]` entry point.
pub fn impl_async(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<AsyncTraitMacro>(attr, item)
}

/// `#[cgp_async]` entry point.
pub fn impl_cgp_async(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAsyncMacro>(attr, item)
}

/// `allow_send` is `#[cgp_async]`. `#[async_trait]` rejects every argument.
fn parse_async(
    attr: TokenStream,
    body: TokenStream,
    allow_send: bool,
) -> syn::Result<TraitRewrite> {
    let send = if attr.is_empty() {
        false
    } else if allow_send && is_send_ident(&attr) {
        true
    } else {
        let message = if allow_send {
            CGP_ASYNC_ARGUMENTS
        } else {
            LEGACY_ARGUMENTS
        };
        return Err(syn::Error::new_spanned(attr, message));
    };

    Ok(match syn::parse2::<ItemTrait>(body.clone()) {
        Ok(item) => TraitRewrite::AsyncTrait {
            item: Box::new(item),
            send,
        },
        Err(_) => TraitRewrite::Passthrough(body),
    })
}

fn is_send_ident(attr: &TokenStream) -> bool {
    syn::parse2::<Ident>(attr.clone()).is_ok_and(|ident| ident == "Send")
}
