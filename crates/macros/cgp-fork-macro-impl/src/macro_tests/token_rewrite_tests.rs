//! Token-level `Self` / `self` rewriting inside macro bodies, which `VisitMut`
//! cannot see into. These pin the look-ahead rules: `Self::Local` stays when
//! `Local` is one of the trait's own associated types, and a `self::` module
//! path stays a module path.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::macro_core::visitors::{
    replace_self_type_in_token_stream, replace_self_value_in_token_stream,
};

fn rewrite_type(tokens: TokenStream, local: &[&str]) -> String {
    let local = local.iter().map(|name| format_ident!("{name}")).collect();
    replace_self_type_in_token_stream(tokens, quote!(__Context__), &local).to_string()
}

fn rewrite_value(tokens: TokenStream) -> String {
    replace_self_value_in_token_stream(tokens, &format_ident!("__context__")).to_string()
}

#[test]
fn self_type_is_replaced() {
    assert_eq!(
        rewrite_type(quote!(Vec<Self>), &[]),
        quote!(Vec<__Context__>).to_string()
    );
}

#[test]
fn self_path_to_a_foreign_associated_type_is_replaced() {
    assert_eq!(
        rewrite_type(quote!(Self::Error), &["Output"]),
        quote!(__Context__::Error).to_string()
    );
}

#[test]
fn self_path_to_a_local_associated_type_is_kept() {
    assert_eq!(
        rewrite_type(quote!(Self::Output), &["Output"]),
        quote!(Self::Output).to_string()
    );
}

#[test]
fn self_type_rewrite_recurses_into_groups() {
    assert_eq!(
        rewrite_type(quote!((Self, [Self::Output; 2], { Self })), &["Output"]),
        quote!((__Context__, [Self::Output; 2], { __Context__ })).to_string()
    );
}

#[test]
fn self_followed_by_a_single_colon_is_replaced() {
    // `Self: Trait` in a bound is a type, not a path; only `::` is a path.
    assert_eq!(
        rewrite_type(quote!(where Self: Clone), &["Clone"]),
        quote!(where __Context__: Clone).to_string()
    );
}

#[test]
fn self_at_the_end_of_the_stream_is_replaced() {
    assert_eq!(
        rewrite_type(quote!(Self), &["Output"]),
        quote!(__Context__).to_string()
    );
}

#[test]
fn self_value_is_replaced() {
    assert_eq!(
        rewrite_value(quote!(format!("{}", self.name()))),
        quote!(format!("{}", __context__.name())).to_string()
    );
}

#[test]
fn self_module_path_is_kept() {
    assert_eq!(
        rewrite_value(quote!(self::helper(self))),
        quote!(self::helper(__context__)).to_string()
    );
}

#[test]
fn self_value_followed_by_a_single_colon_is_replaced() {
    assert_eq!(
        rewrite_value(quote!(Struct { field: self })),
        quote!(Struct { field: __context__ }).to_string()
    );
}
