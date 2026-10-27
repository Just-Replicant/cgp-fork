//! `#[cgp_provider]` rejects inputs it cannot derive an `IsProviderFor` impl
//! from: an inherent `impl` with no trait, a provider trait carrying no type
//! argument (so there is no context), a const argument in the provider trait's
//! argument list, a `new`-keyword struct declaration (which belongs to
//! `#[cgp_new_provider]`), and a non-`impl` item.
//!
//! `#[cgp_new_provider]` shares this stack and rejects the same inputs.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_provider.md (Tests) for these failure
//! cases, and cgp-knowledge-base-fork/cgp/reference/macros/cgp_provider.md for the user-facing
//! semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_inherent_impl() {
    // An inherent `impl Foo { … }` has no provider trait to derive the component
    // name and `IsProviderFor` impl from.
    assert_macro_rejects("cgp_provider on an inherent impl", || {
        crate::macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl RectangleArea {
                    fn area(_context: &()) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_provider_trait_without_context() {
    // The provider trait must carry a leading type argument to serve as the
    // context; a bare `AreaCalculator` (the consumer-trait shape) has none.
    assert_macro_rejects("cgp_provider with no context type argument", || {
        crate::macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl AreaCalculator for RectangleArea {
                    fn area(_context: &()) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_const_argument_in_trait_args() {
    // A const value cannot sit in the `IsProviderFor` params tuple (a tuple of
    // types), so a const argument in the provider trait's own argument list is
    // rejected. A const generic on the provider *struct* is fine.
    assert_macro_rejects("cgp_provider with a const trait argument", || {
        crate::macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl<Context> AreaCalculator<Context, 3> for RectangleArea {
                    fn area(_context: &Context) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_new_struct_declaration() {
    // `#[cgp_provider]`'s argument is a component type only; declaring the
    // provider struct is `#[cgp_new_provider]`'s job. The `new Name` form is not
    // part of `#[cgp_provider]`'s grammar, so the trailing name is an unexpected
    // token rather than a silent struct declaration.
    assert_macro_rejects("cgp_provider(new Name)", || {
        crate::macro_lib::cgp_provider(
            quote!(new RectangleArea),
            quote!(
                impl<Context> AreaCalculator<Context> for RectangleArea {
                    fn area(_context: &Context) -> f64 {
                        0.0
                    }
                }
            ),
        )
    });
}

#[test]
fn rejects_non_impl_item() {
    // `#[cgp_provider]` only applies to an `impl` block, so a struct fails at the
    // `ItemImpl` parse.
    assert_macro_rejects("cgp_provider on a struct", || {
        crate::macro_lib::cgp_provider(
            quote!(),
            quote!(
                pub struct NotAnImpl;
            ),
        )
    });
}

#[test]
fn rejects_provider_trait_as_component() {
    super::assert_macro_rejects_with(
        "cgp_provider naming the provider trait",
        "`#[cgp_provider]` takes the component type, not the provider trait; \
         the component for `Greeter` is `GreeterComponent` unless `#[cgp_component]` named it",
        || {
            crate::macro_lib::cgp_provider(
                quote::quote!(Greeter),
                quote::quote!(
                    impl<Context> Greeter<Context> for GreetHello {
                        fn greet(_context: &Context) {}
                    }
                ),
            )
        },
    );
}

#[test]
fn accepts_component_type_with_trait_like_prefix() {
    // Only the exact trait name is rejected; a qualified or differently named
    // component type is the user's choice.
    for component in [
        quote::quote!(GreeterComponent),
        quote::quote!(other::Greeter),
    ] {
        crate::macro_lib::cgp_provider(
            component,
            quote::quote!(
                impl<Context> Greeter<Context> for GreetHello {
                    fn greet(_context: &Context) {}
                }
            ),
        )
        .expect("a component type is accepted");
    }
}
