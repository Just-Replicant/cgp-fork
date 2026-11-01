//! `#[cgp_component]` rejects inputs it cannot lower into a component: a
//! non-trait item, and a trait carrying a const generic parameter.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_component.md (Tests) for these failure
//! cases, and cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md for the user-facing
//! semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_non_trait_item() {
    // A struct is not a trait, so the consumer-trait parser rejects it.
    assert_macro_rejects("cgp_component on a struct", || {
        crate::macro_lib::cgp_component(
            quote!(FooProvider),
            quote!(
                pub struct NotATrait;
            ),
        )
    });
}

#[test]
fn rejects_const_generic_parameter() {
    // A const value has no place in the `IsProviderFor` params tuple (a tuple of
    // types) and cannot key CGP's type-based wiring, so a const generic parameter
    // on the component is rejected rather than lowered into non-compiling code.
    assert_macro_rejects("cgp_component with a const generic parameter", || {
        crate::macro_lib::cgp_component(
            quote!(Foo),
            quote!(
                pub trait CanFoo<const N: usize> {
                    fn foo(&self) -> usize;
                }
            ),
        )
    });
}

#[test]
fn rejects_derive_promote_with_two_methods() {
    assert_macro_rejects("derive_promote on a component with two methods", || {
        crate::macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteAreaCalculator)]
                pub trait HasArea {
                    fn area(&self) -> f64;
                    fn perimeter(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn rejects_duplicate_derive_promote() {
    assert_macro_rejects("duplicate derive_promote", || {
        crate::macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteArea)]
                #[derive_promote(PromoteOther)]
                pub trait HasArea {
                    fn area(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn rejects_derive_promote_with_associated_type() {
    assert_macro_rejects(
        "derive_promote on a component with an associated type",
        || {
            crate::macro_lib::cgp_component(
                quote!(AreaCalculator),
                quote!(
                    #[derive_promote(PromoteAreaCalculator)]
                    pub trait HasArea {
                        type Unit;
                        fn area(&self) -> f64;
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_component_name_parameter_the_trait_lacks() {
    assert_macro_rejects("cgp_component with an undeclared name parameter", || {
        crate::macro_lib::cgp_component(
            quote!(provider: AreaCalculator, name: AreaCalculatorComponent<T>),
            quote!(
                pub trait CanCalculateArea {
                    fn area(&self) -> f64;
                }
            ),
        )
    });
}

#[test]
fn accepts_component_name_parameter_the_trait_declares() {
    assert!(
        crate::macro_lib::cgp_component(
            quote!(provider: AreaCalculator, name: AreaCalculatorComponent<Shape>),
            quote!(
                pub trait CanCalculateArea<Shape> {
                    fn area(&self, shape: &Shape) -> f64;
                }
            ),
        )
        .is_ok()
    );
}

#[test]
fn rejects_derive_promote_on_async_method() {
    assert_macro_rejects("derive_promote on an async method", || {
        crate::macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteAreaCalculator)]
                pub trait HasArea {
                    async fn area(&self) -> f64;
                }
            ),
        )
    });
}

/// Each `#[derive_promote]` rule on the component's shape, pinned to its message.
///
/// The macro-item and non-identifier-argument checks in `derive_promote.rs` are
/// defensive: `#[cgp_component]` rejects those shapes earlier with its own
/// message, so they cannot be reached through the public entry point.
#[test]
fn rejects_each_unpromotable_component_with_its_message() {
    use proc_macro2::TokenStream;

    use super::assert_macro_rejects_with;

    let run = |attr: TokenStream, body: TokenStream| {
        move || {
            crate::macro_lib::cgp_component(
                quote!(AreaCalculator),
                quote!(
                    #[derive_promote(#attr)]
                    pub trait HasArea {
                        #body
                    }
                ),
            )
        }
    };

    let cases: Vec<(&str, &str, TokenStream, TokenStream)> = vec![
        (
            "extra attribute tokens",
            "`#[derive_promote]` takes only the provider name, as in `#[derive_promote(PromoteAreaCalculator)]`",
            quote!(PromoteAreaCalculator, Extra),
            quote!(
                fn area(&self) -> f64;
            ),
        ),
        (
            "associated const",
            "`#[derive_promote]` cannot promote a component that declares an associated const",
            quote!(PromoteAreaCalculator),
            quote!(
                const SIDES: u8;
                fn area(&self) -> f64;
            ),
        ),
        (
            "no method",
            "`#[derive_promote]` requires a component with exactly one method",
            quote!(PromoteAreaCalculator),
            quote!(),
        ),
        (
            "async method",
            "`#[derive_promote]` wraps a synchronous `Computer::compute`, which cannot implement an async method",
            quote!(PromoteAreaCalculator),
            quote!(
                async fn area(&self) -> f64;
            ),
        ),
        (
            "generic method",
            "`#[derive_promote]` does not support method-level generic parameters",
            quote!(PromoteAreaCalculator),
            quote!(
                fn area<T>(&self, scale: T) -> f64;
            ),
        ),
        (
            "no receiver and no arguments",
            "`#[derive_promote]` requires the method to take `&self`",
            quote!(PromoteAreaCalculator),
            quote!(
                fn area() -> f64;
            ),
        ),
        (
            "receiver by value",
            "`#[derive_promote]` requires the method to take `&self`",
            quote!(PromoteAreaCalculator),
            quote!(
                fn area(self) -> f64;
            ),
        ),
        (
            "mutable receiver",
            "`#[derive_promote]` wraps `Computer::compute`, which borrows the context immutably",
            quote!(PromoteAreaCalculator),
            quote!(
                fn area(&mut self) -> f64;
            ),
        ),
        (
            "impl Trait output",
            "`#[derive_promote]` cannot use an `impl Trait` return as a Computer `Output`",
            quote!(PromoteAreaCalculator),
            quote!(
                fn area(&self) -> impl Into<f64>;
            ),
        ),
    ];

    for (label, message, attr, body) in cases {
        assert_macro_rejects_with(label, message, run(attr, body));
    }
}

/// The arity rules of the promoted call: no argument passes `()`, one passes
/// the bare value, and several pass a tuple.
#[test]
fn derive_promote_packs_arguments_by_arity() {
    let expand = |body: proc_macro2::TokenStream| {
        crate::macro_lib::cgp_component(
            quote!(AreaCalculator),
            quote!(
                #[derive_promote(PromoteAreaCalculator)]
                pub trait HasArea {
                    #body
                }
            ),
        )
        .expect("a promotable component expands")
        .to_string()
    };

    // `compute(context, PhantomData::<()>, input,)` with the input packed by arity,
    // and the `Computer` bound's input parameter matching it.
    let none = expand(quote!(
        fn area(&self) -> f64;
    ));
    assert!(none.contains("PhantomData :: < () > , () ,)"), "{none}");
    assert!(
        none.contains("Computer < __Context__ , () , () , Output = f64 >"),
        "{none}"
    );

    let one = expand(quote!(
        fn area(&self, scale: f64) -> f64;
    ));
    assert!(one.contains("PhantomData :: < () > , scale ,)"), "{one}");
    assert!(
        one.contains("Computer < __Context__ , () , f64 , Output = f64 >"),
        "{one}"
    );

    let two = expand(quote!(
        fn area(&self, width: f64, height: f64);
    ));
    assert!(
        two.contains("PhantomData :: < () > , (width , height) ,)"),
        "{two}"
    );
    assert!(
        two.contains("Computer < __Context__ , () , (f64 , f64) , Output = () >"),
        "{two}"
    );
}
