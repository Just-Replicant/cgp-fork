//! `#[cfg_wiring(...)]` and `#[unchecked]` on wiring tables.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Item, Meta};

use crate::ir::Diagnostics;
use crate::macro_core::functions::unsupported_attribute_error;

/// Whether `attribute` is `#[cfg_wiring(...)]`.
pub fn is_cfg_wiring(attribute: &Attribute) -> bool {
    attribute.path().is_ident("cfg_wiring")
}

/// Whether `attribute` is `#[unchecked]`.
pub fn is_unchecked(attribute: &Attribute) -> bool {
    attribute.path().is_ident("unchecked")
}

/// `#[cfg_wiring(predicate)]` as the `#[cfg(predicate)]` it expands to.
///
/// The predicate uses the same grammar as `cfg`: `test`, `not(test)`,
/// `feature = "mock"`, `any(...)`.
pub fn cfg_attribute(attribute: &Attribute) -> syn::Result<Attribute> {
    let list = attribute.meta.require_list().map_err(|_| {
        Error::new(
            attribute.span(),
            "`#[cfg_wiring(...)]` expects a cfg predicate, for example `#[cfg_wiring(test)]`",
        )
    })?;

    if list.tokens.is_empty() {
        return Err(Error::new(
            attribute.span(),
            "`#[cfg_wiring(...)]` expects a cfg predicate, for example `#[cfg_wiring(test)]`",
        ));
    }

    let tokens = &list.tokens;
    Ok(syn::parse_quote!(#[cfg(#tokens)]))
}

/// The `#[cfg(...)]` attributes corresponding to every `#[cfg_wiring(...)]`.
pub fn cfg_attributes(attributes: &[Attribute]) -> syn::Result<Vec<Attribute>> {
    let mut cfgs = Vec::new();

    for attribute in attributes {
        if is_cfg_wiring(attribute) {
            cfgs.push(cfg_attribute(attribute)?);
        }
    }

    Ok(cfgs)
}

/// Records an error for every key attribute other than `#[cfg_wiring(...)]`, and for a
/// `#[cfg_wiring]` whose predicate is missing.
pub fn validate_key_attributes(attributes: &[Attribute], diagnostics: &mut Diagnostics) {
    for attribute in attributes {
        if is_cfg_wiring(attribute) {
            if let Err(error) = cfg_attribute(attribute) {
                diagnostics.push(error);
            }
        } else {
            diagnostics.push(unsupported_attribute_error(attribute));
        }
    }
}

/// Table-level flags of a `delegate_components!` invocation.
pub struct DelegateTableFlags {
    /// `#[unchecked]` opts the concrete context out of the default wiring check.
    pub unchecked: bool,
    /// `#[cfg(...)]` attributes the whole expansion is gated on.
    pub cfgs: Vec<Attribute>,
}

/// Records every illegal table attribute and returns the flags it did accept.
///
/// `#[unchecked]` takes no arguments. `#[cfg_wiring(...)]` takes a cfg predicate.
/// Anything else is rejected. A duplicate `#[unchecked]` is an error; several
/// `#[cfg_wiring]` attributes all apply.
pub fn read_table_flags(
    attributes: &[Attribute],
    diagnostics: &mut Diagnostics,
) -> DelegateTableFlags {
    let mut unchecked = false;
    let mut saw_unchecked = false;
    let mut cfgs = Vec::new();

    for attribute in attributes {
        if is_unchecked(attribute) {
            if attribute.meta.require_path_only().is_err() {
                diagnostics.push(Error::new(
                    attribute.span(),
                    "`#[unchecked]` takes no arguments",
                ));
            }

            if saw_unchecked {
                diagnostics.push(Error::new(
                    attribute.span(),
                    "duplicate `#[unchecked]` on one wiring table",
                ));
            }

            saw_unchecked = true;
            unchecked = true;
        } else if is_cfg_wiring(attribute) {
            match cfg_attribute(attribute) {
                Ok(cfg) => cfgs.push(cfg),
                Err(error) => diagnostics.push(error),
            }
        } else {
            diagnostics.push(unsupported_attribute_error(attribute));
        }
    }

    DelegateTableFlags { unchecked, cfgs }
}

/// Copies `cfgs` onto `item`.
pub fn apply_cfgs(item: &mut syn::ItemImpl, cfgs: &[Attribute]) {
    item.attrs.extend(cfgs.iter().cloned());
}

/// The `#[cfg]` attributes on `attributes`, ignoring every other attribute.
pub fn table_cfgs(attributes: &[Attribute]) -> syn::Result<Vec<Attribute>> {
    let mut cfgs = Vec::new();

    for attribute in attributes {
        if is_cfg_wiring(attribute) {
            cfgs.push(cfg_attribute(attribute)?);
        }
    }

    Ok(cfgs)
}

/// `#[cfg(not(...))]` that is true when every attribute in `cfgs` is false.
///
/// One predicate becomes `not(predicate)`. Several become `not(all(...))`, because
/// stacked `#[cfg]` attributes are a conjunction.
pub fn negate_cfgs(cfgs: &[Attribute]) -> Attribute {
    let predicates: Vec<TokenStream> = cfgs
        .iter()
        .filter_map(|attribute| match &attribute.meta {
            Meta::List(list) if attribute.path().is_ident("cfg") => Some(list.tokens.clone()),
            _ => None,
        })
        .collect();

    match predicates.as_slice() {
        [predicate] => syn::parse_quote!(#[cfg(not(#predicate))]),
        predicates => syn::parse_quote!(#[cfg(not(all(#(#predicates),*)))]),
    }
}

/// Puts `cfgs` on every item in `tokens`.
///
/// A Rust attribute applies to the next item only, so a table-level
/// `#[cfg_wiring]` has to be repeated on each generated struct, impl, trait, and
/// macro invocation.
pub fn prefix_item_cfgs(cfgs: &[Attribute], tokens: TokenStream) -> syn::Result<TokenStream> {
    if cfgs.is_empty() {
        return Ok(tokens);
    }

    let items = Parser::parse2(
        |input: syn::parse::ParseStream| {
            let mut items = Vec::new();

            while !input.is_empty() {
                items.push(input.parse::<Item>()?);
            }

            Ok(items)
        },
        tokens,
    )?;

    let mut output = TokenStream::new();

    for item in items {
        output.extend(quote! {
            #(#cfgs)*
            #item
        });
    }

    Ok(output)
}
