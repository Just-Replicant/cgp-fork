//! The shared pipeline driver (`crate::ir`): stage order, error accumulation, and the
//! no-argument helper.

use std::cell::Cell;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Error;

use crate::ir::{Diagnostics, IrFamily, Pipeline, parse_body_without_args, run};
use crate::registry::EntrypointFn;

thread_local! {
    static EMITTED: Cell<bool> = const { Cell::new(false) };
}

/// Accepts an identifier and rejects the names `bad` and `worse` in validation.
struct Probe;

impl Pipeline for Probe {
    const NAME: &'static str = "probe";
    const FAMILY: IrFamily = IrFamily::DataShape;
    type Ir = Vec<syn::Ident>;

    fn parse(_attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let idents = syn::parse::Parser::parse2(
            syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
            body,
        )?;
        Ok(idents.into_iter().collect())
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        for ident in ir {
            if ident == "bad" || ident == "worse" {
                diagnostics.push(Error::new(ident.span(), format!("`{ident}` is rejected")));
            }
        }
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        EMITTED.with(|emitted| emitted.set(true));
        Ok(quote!( #( struct #ir; )* ))
    }
}

fn messages(error: Error) -> Vec<String> {
    error.into_iter().map(|e| e.to_string()).collect()
}

#[test]
fn run_emits_a_valid_ir() {
    EMITTED.with(|emitted| emitted.set(false));
    let out = run::<Probe>(quote!(), quote!(A, B)).expect("valid input");
    assert_eq!(
        out.to_string(),
        quote!(
            struct A;
            struct B;
        )
        .to_string()
    );
    assert!(EMITTED.with(Cell::get));
}

#[test]
fn run_reports_every_validation_error_and_does_not_emit() {
    EMITTED.with(|emitted| emitted.set(false));
    let error = run::<Probe>(quote!(), quote!(bad, ok, worse)).expect_err("rejected");
    assert_eq!(
        messages(error),
        ["`bad` is rejected", "`worse` is rejected"]
    );
    assert!(
        !EMITTED.with(Cell::get),
        "emit ran after a validation error"
    );
}

#[test]
fn run_stops_at_a_parse_error() {
    let error = run::<Probe>(quote!(), quote!(A B)).expect_err("grammar error");
    assert_eq!(messages(error), ["expected `,`"]);
}

#[test]
fn diagnostics_keep_push_order() {
    let mut diagnostics = Diagnostics::new();
    assert!(diagnostics.is_empty());
    assert_eq!(diagnostics.check(Ok::<_, Error>(1)), Some(1));
    diagnostics.push(Error::new(Span::call_site(), "first"));
    assert_eq!(
        diagnostics.check::<()>(Err(Error::new(Span::call_site(), "second"))),
        None
    );
    assert!(!diagnostics.is_empty());
    assert_eq!(
        messages(diagnostics.finish().expect_err("two errors")),
        ["first", "second"]
    );
}

#[test]
fn empty_diagnostics_finish_ok() {
    assert!(Diagnostics::new().finish().is_ok());
}

#[test]
fn no_args_helper_accepts_an_empty_attribute() {
    let item: syn::ItemTrait = parse_body_without_args(
        "x",
        quote!(),
        quote!(
            trait T {}
        ),
    )
    .expect("parses");
    assert_eq!(item.ident, "T");
}

#[test]
fn no_args_helper_rejects_arguments() {
    let error = parse_body_without_args::<syn::ItemTrait>(
        "x",
        quote!(y),
        quote!(
            trait T {}
        ),
    )
    .expect_err("has arguments");
    assert_eq!(
        messages(error),
        ["#[x] does not accept any attribute argument"]
    );
}

#[test]
fn no_args_helper_reports_both_errors() {
    let error = parse_body_without_args::<syn::ItemTrait>(
        "x",
        quote!(y),
        quote!(
            struct S;
        ),
    )
    .expect_err("two errors");
    let messages = messages(error);
    assert_eq!(messages.len(), 2, "{messages:?}");
    assert_eq!(messages[0], "#[x] does not accept any attribute argument");
}

#[test]
fn ir_families_are_listed_once() {
    let mut all = IrFamily::ALL.to_vec();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), IrFamily::ALL.len());
}

/// The component pipelines check the `name:` parameters in their validate stage and
/// report every undeclared one at once.
#[test]
fn component_pipelines_report_every_undeclared_name_parameter() {
    let expected = [
        "the component name's parameter `A` is not a generic parameter of the trait `CanFoo`",
        "the component name's parameter `B` is not a generic parameter of the trait `CanFoo`",
    ];

    let cases: [(&str, EntrypointFn, TokenStream); 3] = [
        (
            "cgp_component",
            crate::macro_lib::cgp_component,
            quote!(
                pub trait CanFoo<T> {
                    fn foo(&self, value: T);
                }
            ),
        ),
        (
            "cgp_getter",
            crate::macro_lib::cgp_getter,
            quote!(
                pub trait CanFoo<T> {
                    fn foo(&self) -> &T;
                }
            ),
        ),
        (
            "cgp_type",
            crate::macro_lib::cgp_type,
            quote!(
                pub trait CanFoo<T> {
                    type Foo;
                }
            ),
        ),
    ];

    for (name, entrypoint, body) in cases {
        let error = entrypoint(
            quote!(provider: FooProvider, name: FooComponent<T, A, B>),
            body,
        )
        .expect_err(name);
        assert_eq!(messages(error), expected, "{name}");
    }
}

#[test]
fn derive_provider_validates_the_inner_component() {
    let error = crate::macro_lib::derive_provider(
        quote!(WithProvider),
        quote! {
            #[cgp_component { provider: FooTypeProvider, name: FooComponent<X> }]
            pub trait CanFoo { type Foo; }
        },
    )
    .expect_err("undeclared parameter");
    assert_eq!(
        messages(error),
        ["the component name's parameter `X` is not a generic parameter of the trait `CanFoo`"]
    );
}
