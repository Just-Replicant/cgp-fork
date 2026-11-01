use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Error, Type};

use crate::ir::Diagnostics;

#[derive(Clone)]
pub enum CheckParamsAttribute {
    Default,
    Skip,
    Multi(Punctuated<Type, Comma>),
}

impl CheckParamsAttribute {
    pub fn merge(&self, other: &Self) -> syn::Result<Self> {
        let res = match (self, other) {
            (Self::Default, other) => other.clone(),
            (other, Self::Default) => other.clone(),
            (Self::Skip, Self::Skip) => Self::Skip,
            (Self::Multi(params_a), Self::Multi(params_b)) => Self::Multi(Punctuated::from_iter(
                params_a.iter().chain(params_b.iter()).cloned(),
            )),
            (Self::Skip, Self::Multi(params)) | (Self::Multi(params), Self::Skip) => {
                return Err(Error::new(
                    params.span(),
                    "cannot combine #[skip_check] with #[check_params]",
                ));
            }
        };

        Ok(res)
    }

    /// Parses the check control of one key and records an error for every illegal
    /// attribute instead of stopping at the first. Only `#[check_params(..)]` and
    /// `#[skip_check]` are legal, and at most one of them; every attribute after the
    /// first is reported.
    pub fn record_attributes(attributes: &[Attribute], diagnostics: &mut Diagnostics) -> Self {
        let mut result = Self::Default;

        for (index, attribute) in attributes.iter().enumerate() {
            let parsed = Self::parse_attribute(attribute);

            if index == 0 {
                if let Some(parsed) = diagnostics.check(parsed) {
                    result = parsed;
                }
            } else if parsed.is_ok() {
                diagnostics.push(Error::new(
                    attribute.span(),
                    "Expected at most one `#[check_params]` or `#[skip_check]` attribute",
                ));
            } else {
                diagnostics.check(parsed);
            }
        }

        result
    }

    fn parse_attribute(attribute: &Attribute) -> syn::Result<Self> {
        if attribute.path().is_ident("check_params") {
            let params = attribute.parse_args_with(Punctuated::parse_terminated)?;
            Ok(CheckParamsAttribute::Multi(params))
        } else if attribute.path().is_ident("skip_check") {
            attribute.meta.require_path_only().map_err(|_| {
                Error::new(
                    attribute.span(),
                    "`#[skip_check]` does not take any arguments",
                )
            })?;

            Ok(CheckParamsAttribute::Skip)
        } else {
            Err(Error::new(
                attribute.span(),
                "Expected either `#[skip_check]` or `#[check_params]` attribute for specifying the check generics",
            ))
        }
    }
}
