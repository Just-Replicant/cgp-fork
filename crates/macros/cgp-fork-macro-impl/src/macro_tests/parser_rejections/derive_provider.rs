//! `#[derive_provider]` only derives `WithProvider`, and only for a
//! `#[cgp_component]` trait that is an abstract type or a single-method getter.

use quote::quote;

use super::{assert_macro_rejects, assert_macro_rejects_with};

#[test]
fn rejects_unknown_provider() {
    assert_macro_rejects(
        "derive_provider with a provider other than WithProvider",
        || {
            crate::macro_lib::derive_provider(
                quote!(UseContext),
                quote!(
                    #[cgp_component(Greeter)]
                    pub trait CanGreet {
                        fn greet(&self);
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_trait_without_cgp_component() {
    assert_macro_rejects("derive_provider without #[cgp_component]", || {
        crate::macro_lib::derive_provider(
            quote!(WithProvider),
            quote!(
                pub trait HasNameType {
                    type Name;
                }
            ),
        )
    });
}

#[test]
fn rejects_component_that_is_not_a_type_or_getter() {
    assert_macro_rejects("derive_provider on a method that is not a getter", || {
        crate::macro_lib::derive_provider(
            quote!(WithProvider),
            quote!(
                #[cgp_component(Greeter)]
                pub trait CanGreet {
                    fn greet(&self);
                }
            ),
        )
    });
}

#[test]
fn rejects_getter_with_several_methods() {
    assert_macro_rejects_with(
        "derive_provider on a getter with two methods",
        "`#[derive_provider(WithProvider)]` supports a getter with a single method",
        || {
            crate::macro_lib::derive_provider(
                quote!(WithProvider),
                quote!(
                    #[cgp_component(NameGetter)]
                    pub trait HasNames {
                        fn first_name(&self) -> &str;
                        fn last_name(&self) -> &str;
                    }
                ),
            )
        },
    );
}
