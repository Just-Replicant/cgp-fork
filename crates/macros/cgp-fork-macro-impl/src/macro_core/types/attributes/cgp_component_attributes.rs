use core::mem;

use proc_macro2::Span;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, ItemTrait, TypeParamBound};

use crate::macro_core::functions::rewrite_bare_async_trait;
use crate::macro_core::types::attributes::{
    DeriveDelegateAttribute, DeriveDelegateAttributes, DerivePromoteAttribute, PrefixAttribute,
    UseTypeAttribute, UseTypeAttributes,
};

#[derive(Default, Clone)]
pub struct CgpComponentAttributes {
    pub extend: Vec<TypeParamBound>,
    pub use_type: UseTypeAttributes,
    pub prefixes: Vec<PrefixAttribute>,
    pub derive_delegate_attributes: DeriveDelegateAttributes,
    pub derive_promote: Option<DerivePromoteAttribute>,
    /// Span of a bare `#[async_trait]` rewritten to `#[cgp_async]`, so the expansion can warn once.
    pub async_trait_span: Option<Span>,
}

impl CgpComponentAttributes {
    pub fn preprocess(item_trait: &ItemTrait) -> syn::Result<(Self, ItemTrait)> {
        let mut item_trait = item_trait.clone();

        let attributes = Self::parse(&mut item_trait.attrs)?;

        item_trait.supertraits.extend(attributes.extend.clone());

        attributes.use_type.transform_item_trait(&mut item_trait)?;

        Ok((attributes, item_trait))
    }

    pub fn parse(attributes: &mut Vec<Attribute>) -> syn::Result<Self> {
        let mut parsed_attributes = CgpComponentAttributes::default();

        let in_attributes = mem::take(attributes);

        for attribute in in_attributes.into_iter() {
            if let Some(ident) = attribute.path().get_ident() {
                if ident == "extend" {
                    let extend_bound = attribute
                        .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;
                    parsed_attributes.extend.extend(extend_bound);
                } else if ident == "use_type" {
                    let use_type_specs = attribute
                        .parse_args_with(Punctuated::<UseTypeAttribute, Comma>::parse_terminated)?;

                    for use_type_spec in &use_type_specs {
                        for type_ident in &use_type_spec.type_idents {
                            if let Some(equals) = &type_ident.equals {
                                return Err(syn::Error::new_spanned(
                                    equals,
                                    "Type equality constraints cannot be used in component trait definition",
                                ));
                            }
                        }
                    }

                    parsed_attributes.use_type.attributes.extend(use_type_specs);
                } else if ident == "prefix" {
                    let namespace_specs = attribute.parse_args_with(PrefixAttribute::parse)?;
                    parsed_attributes.prefixes.push(namespace_specs);
                } else if ident == "derive_delegate" {
                    let derive_delegate_attribute =
                        attribute.parse_args_with(DeriveDelegateAttribute::parse)?;
                    parsed_attributes
                        .derive_delegate_attributes
                        .attributes
                        .push(derive_delegate_attribute);
                } else if ident == "derive_promote" || ident == "derive_handler" {
                    if parsed_attributes.derive_promote.is_some() {
                        return Err(syn::Error::new_spanned(
                            &attribute,
                            format!(
                                "duplicate `#[{ident}]`; a component promotes through one provider (`#[derive_handler]` or `#[derive_promote]`)"
                            ),
                        ));
                    }

                    let unified = ident == "derive_handler";
                    parsed_attributes.derive_promote = Some(
                        attribute
                            .parse_args_with(DerivePromoteAttribute::parse_args(ident, unified))?,
                    );
                } else {
                    push_forwarded_attribute(&mut parsed_attributes, &mut *attributes, attribute);
                }
            } else {
                push_forwarded_attribute(&mut parsed_attributes, &mut *attributes, attribute);
            }
        }

        Ok(parsed_attributes)
    }
}

/// Keeps an attribute the component macro does not interpret, rewriting a bare
/// `#[async_trait]` to `#[cgp_async]` so generated items do not repeat the deprecation.
fn push_forwarded_attribute(
    parsed: &mut CgpComponentAttributes,
    attributes: &mut Vec<Attribute>,
    mut attribute: Attribute,
) {
    if let Some(span) = rewrite_bare_async_trait(&mut attribute)
        && parsed.async_trait_span.is_none()
    {
        parsed.async_trait_span = Some(span);
    }
    attributes.push(attribute);
}
