//! `#[deprecated]` on the items the macros generate.
//!
//! A deprecated component (a `#[deprecated]` consumer trait under `#[cgp_component]`) passes
//! the attribute on to its provider trait and component struct, so code that names any of
//! the three is warned. The impls the macro generates for it name those items too; they must
//! not warn the crate that only declares the component, and an impl cannot itself carry
//! `#[deprecated]`, so they get `#[allow(deprecated)]` instead.

use syn::{Attribute, Item, parse_quote};

/// Whether `attr` is `#[deprecated]`, with or without arguments.
pub fn is_deprecated_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("deprecated")
}

/// The `#[deprecated]` attributes among `attrs`.
pub fn deprecated_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| is_deprecated_attr(attr))
        .cloned()
        .collect()
}

/// `#[allow(deprecated)]`.
pub fn allow_deprecated_attr() -> Attribute {
    parse_quote!(#[allow(deprecated)])
}

/// Prepares generated `items` that name deprecated items: every impl loses any
/// `#[deprecated]` copied from the trait (rustc rejects it on an impl) and gains
/// `#[allow(deprecated)]`, and every trait gains `#[allow(deprecated)]` for its
/// supertraits (a deprecated trait still warns its users). Other items are left alone,
/// so a deliberate deprecation marker still warns.
pub fn allow_deprecated_in(items: &mut [Item]) {
    for item in items {
        match item {
            Item::Impl(item_impl) => {
                item_impl.attrs.retain(|attr| !is_deprecated_attr(attr));
                item_impl.attrs.push(allow_deprecated_attr());
            }
            Item::Trait(item_trait) => item_trait.attrs.push(allow_deprecated_attr()),
            _ => {}
        }
    }
}
