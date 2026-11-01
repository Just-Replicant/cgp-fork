use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, GenericParam, TypeParamBound, WherePredicate};

use crate::macro_core::functions::rewrite_bare_async_trait;
use crate::macro_core::types::attributes::{
    UseProviderAttribute, UseProviderAttributes, UseTypeAttribute, UseTypeAttributes,
};

#[derive(Default)]
pub struct FunctionAttributes {
    pub extend: Vec<TypeParamBound>,
    pub extend_where: Vec<WherePredicate>,
    pub uses: Vec<TypeParamBound>,
    pub use_type: UseTypeAttributes,
    pub use_provider: UseProviderAttributes,
    pub impl_generics: Vec<GenericParam>,
    pub raw_attributes: Vec<Attribute>,
    /// Span of a bare `#[async_trait]` rewritten to `#[cgp_async]`, so the expansion can warn once.
    pub async_trait_span: Option<Span>,
}

impl FunctionAttributes {
    pub fn parse(attributes: Vec<Attribute>) -> syn::Result<Self> {
        let mut parsed_attributes = FunctionAttributes::default();

        for attribute in attributes.into_iter() {
            if let Some(ident) = attribute.path().get_ident() {
                if ident == "extend" {
                    let extend_bound = attribute
                        .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;

                    parsed_attributes.extend.extend(extend_bound);
                } else if ident == "extend_where" {
                    let where_predicates = attribute
                        .parse_args_with(Punctuated::<WherePredicate, Comma>::parse_terminated)?;

                    parsed_attributes.extend_where.extend(where_predicates);
                } else if ident == "uses" {
                    let uses = attribute
                        .parse_args_with(Punctuated::<TypeParamBound, Comma>::parse_terminated)?;

                    parsed_attributes.uses.extend(uses);
                } else if ident == "use_type" {
                    let use_type = attribute
                        .parse_args_with(Punctuated::<UseTypeAttribute, Comma>::parse_terminated)?;

                    parsed_attributes.use_type.attributes.extend(use_type);
                } else if ident == "use_provider" {
                    let use_provider = attribute.parse_args_with(
                        Punctuated::<UseProviderAttribute, Comma>::parse_terminated,
                    )?;

                    parsed_attributes
                        .use_provider
                        .attributes
                        .extend(use_provider);
                } else if ident == "impl_generics" {
                    let impl_generics = attribute
                        .parse_args_with(Punctuated::<GenericParam, Comma>::parse_terminated)?;

                    parsed_attributes.impl_generics.extend(impl_generics);
                } else {
                    push_raw_attribute(&mut parsed_attributes, attribute);
                }
            } else {
                push_raw_attribute(&mut parsed_attributes, attribute);
            }
        }

        Ok(parsed_attributes)
    }
}

fn push_raw_attribute(parsed: &mut FunctionAttributes, mut attribute: Attribute) {
    if let Some(span) = rewrite_bare_async_trait(&mut attribute)
        && parsed.async_trait_span.is_none()
    {
        parsed.async_trait_span = Some(span);
    }
    parsed.raw_attributes.push(attribute);
}
