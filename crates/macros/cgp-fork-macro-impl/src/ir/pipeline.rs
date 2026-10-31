//! The [`Pipeline`] trait and its driver.

use proc_macro2::{Span, TokenStream};
use syn::Error;
use syn::parse::Parse;

use crate::ir::{Diagnostics, FamilyIr};

/// One macro entrypoint: a parser into the IR of its family.
///
/// Implementors are zero-sized marker types; call them through [`run`]. Validation and code
/// generation belong to the family IR ([`FamilyIr`]), so every entrypoint of a family shares
/// them.
pub trait Pipeline {
    /// The macro's name as users write it (`"cgp_component"`, `"Symbol"`, …). Matches the
    /// name in [`crate::registry::entrypoints`].
    const NAME: &'static str;

    /// The family IR this entrypoint parses into.
    type Ir: FamilyIr;

    /// Parses the attribute arguments (empty for function-like and derive macros) and the
    /// annotated item or macro body into the IR. Grammar errors are returned here.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir>;
}

/// Runs `P` over one macro invocation: parse, then validate (reporting every error at
/// once), then emit.
pub fn run<P: Pipeline>(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let ir = P::parse(attr, body)?;

    let mut diagnostics = Diagnostics::new();
    ir.validate(&mut diagnostics);
    diagnostics.finish()?;

    ir.emit()
}

/// Parses `body` as `T` for an attribute macro that takes no arguments.
///
/// When `attr` is not empty the error says `#[name] does not accept any attribute
/// argument`. If the body fails to parse as well, both errors are reported, the argument
/// error first.
pub fn parse_body_without_args<T: Parse>(
    name: &str,
    attr: TokenStream,
    body: TokenStream,
) -> syn::Result<T> {
    let attr_error = (!attr.is_empty()).then(|| {
        Error::new(
            Span::call_site(),
            format!("#[{name}] does not accept any attribute argument"),
        )
    });

    match (attr_error, syn::parse2::<T>(body)) {
        (None, body) => body,
        (Some(error), Ok(_)) => Err(error),
        (Some(mut error), Err(body_error)) => {
            error.combine(body_error);
            Err(error)
        }
    }
}
