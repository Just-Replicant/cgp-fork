//! `#[cgp_auto_error]` rejects an attribute argument, a trait impl, a missing
//! piece of the error definition, and a method that is not a provider function.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_error.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_argument() {
    assert_macro_rejects("cgp_auto_error with an attribute argument", || {
        crate::macro_lib::cgp_auto_error(
            quote!(Context),
            quote!(
                impl ProvideAppError {
                    type Error = String;
                    fn raise_error<Source>(source: Source) -> Error
                    where
                        Source: core::fmt::Display,
                    {
                        source.to_string()
                    }
                    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error
                    where
                        Detail: core::fmt::Display,
                    {
                        format!("{detail}: {error}")
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_trait_impl() {
    assert_macro_rejects("cgp_auto_error on a trait impl", || {
        crate::macro_lib::cgp_auto_error(
            quote!(),
            quote!(
                impl ProvideAppError for App {
                    type Error = String;
                }
            ),
        )
    });
}

#[test]
fn rejects_missing_raise_error() {
    assert_macro_rejects("cgp_auto_error without raise_error", || {
        crate::macro_lib::cgp_auto_error(
            quote!(),
            quote!(
                impl ProvideAppError {
                    type Error = String;
                    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error {
                        format!("{detail}: {error}")
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_receiver() {
    assert_macro_rejects("cgp_auto_error method with a receiver", || {
        crate::macro_lib::cgp_auto_error(
            quote!(),
            quote!(
                impl ProvideAppError {
                    type Error = String;
                    fn raise_error<Source>(&self, source: Source) -> Error {
                        source.to_string()
                    }
                    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error {
                        format!("{detail}: {error}")
                    }
                }
            ),
        )
    });
}

/// One rejection per rule of the error definition, each pinned to its message.
/// `RAISE` and `WRAP` are valid bodies, so a case only differs in what it breaks.
#[test]
fn rejects_each_malformed_definition_with_its_message() {
    use proc_macro2::TokenStream;

    use super::assert_macro_rejects_with;

    let raise = quote! {
        fn raise_error<Source>(source: Source) -> Error
        where
            Source: core::fmt::Display,
        {
            source.to_string()
        }
    };
    let wrap = quote! {
        fn wrap_error<Detail>(error: Error, detail: Detail) -> Error
        where
            Detail: core::fmt::Display,
        {
            format!("{detail}: {error}")
        }
    };
    let run = |item: TokenStream| move || crate::macro_lib::cgp_auto_error(quote!(), item);

    let cases: Vec<(&str, &str, TokenStream)> = vec![
        (
            "unsafe impl",
            "#[cgp_auto_error] does not support unsafe or default impls",
            quote!(unsafe impl ProvideAppError { type Error = String; #raise #wrap }),
        ),
        (
            "generic impl",
            "put generic parameters on raise_error and wrap_error, not on the impl",
            quote!(impl<T> ProvideAppError { type Error = String; #raise #wrap }),
        ),
        (
            "helper attribute",
            "#[cgp_auto_error] does not accept helper attributes",
            quote!(#[inline] impl ProvideAppError { type Error = String; #raise #wrap }),
        ),
        (
            "path provider name",
            "the provider name must be a single identifier",
            quote!(impl errors::ProvideAppError { type Error = String; #raise #wrap }),
        ),
        (
            "tuple provider name",
            "the provider name must be a single identifier",
            quote!(impl (A, B) { type Error = String; #raise #wrap }),
        ),
        (
            "generic provider name",
            "the provider name cannot take generic arguments",
            quote!(impl ProvideAppError<u8> { type Error = String; #raise #wrap }),
        ),
        (
            "other associated type",
            "the only associated type in #[cgp_auto_error] is `Error`",
            quote!(impl ProvideAppError { type Message = String; #raise #wrap }),
        ),
        (
            "duplicate Error",
            "`Error` is already defined",
            quote!(impl ProvideAppError { type Error = String; type Error = u8; #raise #wrap }),
        ),
        (
            "generic Error",
            "`Error` cannot take generic parameters",
            quote!(impl ProvideAppError { type Error<T> = String; #raise #wrap }),
        ),
        (
            "duplicate raise_error",
            "`raise_error` is already defined",
            quote!(impl ProvideAppError { type Error = String; #raise #raise #wrap }),
        ),
        (
            "duplicate wrap_error",
            "`wrap_error` is already defined",
            quote!(impl ProvideAppError { type Error = String; #raise #wrap #wrap }),
        ),
        (
            "unknown method",
            "#[cgp_auto_error] accepts only `raise_error` and `wrap_error`",
            quote!(impl ProvideAppError { type Error = String; #raise #wrap fn other() {} }),
        ),
        (
            "const item",
            "#[cgp_auto_error] accepts an `Error` type, `raise_error`, and `wrap_error`",
            quote!(impl ProvideAppError { type Error = String; const N: u8 = 1; #raise #wrap }),
        ),
        (
            "missing Error",
            "define `type Error = ...`",
            quote!(impl ProvideAppError { #raise #wrap }),
        ),
        (
            "missing wrap_error",
            "define `wrap_error`, the body of `CanWrapError`",
            quote!(impl ProvideAppError { type Error = String; #raise }),
        ),
        (
            "async raise_error",
            "raise_error and wrap_error must be synchronous safe functions",
            quote!(impl ProvideAppError {
                type Error = String;
                async fn raise_error<Source>(source: Source) -> Error { todo!() }
                #wrap
            }),
        ),
        (
            "raise_error arity",
            "expected 1 argument(s), found 2",
            quote!(impl ProvideAppError {
                type Error = String;
                fn raise_error<Source>(source: Source, extra: u8) -> Error { todo!() }
                #wrap
            }),
        ),
        (
            "raise_error returning a concrete type",
            "raise_error must return `Error`",
            quote!(impl ProvideAppError {
                type Error = String;
                fn raise_error<Source>(source: Source) -> String { todo!() }
                #wrap
            }),
        ),
        (
            "raise_error without a return type",
            "raise_error must return `Error`",
            quote!(impl ProvideAppError {
                type Error = String;
                fn raise_error<Source>(source: Source) { todo!() }
                #wrap
            }),
        ),
        (
            "wrap_error taking a concrete error",
            "wrap_error's first argument must be `Error`",
            quote!(impl ProvideAppError {
                type Error = String;
                #raise
                fn wrap_error<Detail>(error: String, detail: Detail) -> Error { todo!() }
            }),
        ),
        (
            "wrap_error returning a concrete type",
            "wrap_error must return `Error`",
            quote!(impl ProvideAppError {
                type Error = String;
                #raise
                fn wrap_error<Detail>(error: Error, detail: Detail) -> String { todo!() }
            }),
        ),
        (
            "const generic on a method",
            "const generics are not supported on #[cgp_auto_error] methods",
            quote!(impl ProvideAppError {
                type Error = String;
                fn raise_error<const N: usize>(source: [u8; N]) -> Error { todo!() }
                #wrap
            }),
        ),
        (
            "method generic named Error",
            "`Error` names the abstract error type in this definition",
            quote!(impl ProvideAppError {
                type Error = String;
                fn raise_error<Error>(source: Error) -> Error { todo!() }
                #wrap
            }),
        ),
    ];

    for (label, message, item) in cases {
        assert_macro_rejects_with(label, message, run(item));
    }
}
