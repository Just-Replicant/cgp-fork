//! The [`Pipeline`] trait and its driver.

use proc_macro2::{Span, TokenStream};
use syn::Error;
use syn::parse::Parse;

use crate::ir::{Diagnostics, IrFamily};

/// One macro entrypoint, split into `parse → validate → emit`.
///
/// Implementors are zero-sized marker types; call them through [`run`].
pub trait Pipeline {
    /// The macro's name as users write it (`"cgp_component"`, `"Symbol"`, …). Matches the
    /// name in [`crate::registry::entrypoints`].
    const NAME: &'static str;

    /// The kind of IR this entrypoint produces.
    const FAMILY: IrFamily;

    /// The typed description of the macro input.
    type Ir;

    /// Parses the attribute arguments (empty for function-like and derive macros) and the
    /// annotated item or macro body into the IR. Grammar errors are returned here.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir>;

    /// Checks the IR and records every semantic error in `diagnostics`.
    ///
    /// The default accepts everything: most checks still run while emitting, inside the
    /// per-entrypoint types under `macro_core`, and move here as the IR is shared.
    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        let _ = (ir, diagnostics);
    }

    /// Generates the output tokens from a validated IR.
    fn emit(ir: Self::Ir) -> syn::Result<TokenStream>;
}

/// Runs `P` over one macro invocation: parse, then validate (reporting every error at
/// once), then emit.
pub fn run<P: Pipeline>(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let ir = P::parse(attr, body)?;

    let mut diagnostics = Diagnostics::new();
    P::validate(&ir, &mut diagnostics);
    diagnostics.finish()?;

    P::emit(ir)
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
