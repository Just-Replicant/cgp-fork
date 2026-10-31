use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error};

use crate::ir::Diagnostics;

/// The "unsupported attribute" error for one attribute, spanned on it.
pub fn unsupported_attribute_error(attribute: &Attribute) -> Error {
    Error::new(
        attribute.span(),
        format!(
            "unsupported attribute: {}",
            attribute.path().to_token_stream()
        ),
    )
}

/// Records an "unsupported attribute" error for every attribute in `attributes`.
pub fn record_unsupported_attributes(attributes: &[Attribute], diagnostics: &mut Diagnostics) {
    for attribute in attributes {
        diagnostics.push(unsupported_attribute_error(attribute));
    }
}

/// Errors with a spanned "unsupported attribute" message for every attribute present,
/// combined into one [`syn::Error`] whose first message points at the first attribute.
pub fn reject_non_empty_attributes(attributes: &[Attribute]) -> syn::Result<()> {
    let mut diagnostics = Diagnostics::new();
    record_unsupported_attributes(attributes, &mut diagnostics);
    diagnostics.finish()
}
