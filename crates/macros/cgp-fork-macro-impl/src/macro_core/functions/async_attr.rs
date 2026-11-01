//! `#[async_trait]` is the old name of `#[cgp_async]`.
//!
//! `#[cgp_component]` and `#[cgp_fn]` run outside `#[async_trait]` and copy the attribute onto
//! every generated trait and impl. Leaving the old name there would warn once per copy.
//! A bare `#[async_trait]` is rewritten to `#[cgp_async]` and one deprecation marker, spanned
//! on the attribute, is emitted instead. `#[async_trait(...)]` is left alone so the deprecated
//! macro reports the argument itself.

use proc_macro2::Span;
use quote::quote_spanned;
use syn::{Attribute, Ident, Item, Meta, parse_quote};

/// Whether `attrs` carries `#[cgp_async(Send)]`.
///
/// A generated forwarding impl for that attribute must return the delegate's future instead of
/// wrapping it in `async { ….await }`. The wrapper captures `&Context`, which is not `Send`
/// unless `Context: Sync`, so a generic component could not implement the `Send` bound.
pub fn is_cgp_async_send(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cgp_async") {
            return false;
        }
        let Meta::List(list) = &attr.meta else {
            return false;
        };
        list.parse_args::<Ident>()
            .is_ok_and(|ident| ident == "Send")
    })
}

/// Rewrites a bare `#[async_trait]` to `#[cgp_async]` and returns the span of the old name.
///
/// An attribute with arguments is unchanged: `#[async_trait(?Send)]` still expands as the
/// deprecated macro and keeps its error.
pub fn rewrite_bare_async_trait(attr: &mut Attribute) -> Option<Span> {
    let Meta::Path(mut path) = attr.meta.clone() else {
        return None;
    };
    if path.segments.len() != 1 {
        return None;
    }
    let segment = path.segments.first_mut()?;
    if segment.ident != "async_trait" || !segment.arguments.is_empty() {
        return None;
    }
    let span = segment.ident.span();
    segment.ident = Ident::new("cgp_async", span);
    attr.meta = Meta::Path(path);
    Some(span)
}

/// An item that names `macro_prelude::ASYNC_TRAIT` at `span`, so the user is warned once, on
/// the `#[async_trait]` they wrote.
pub fn async_trait_deprecation(span: Span) -> Item {
    let root = crate::crate_path::crate_root();
    let marker = quote_spanned! { span => #root::macro_prelude::ASYNC_TRAIT };
    parse_quote! {
        const _: () = #marker;
    }
}
