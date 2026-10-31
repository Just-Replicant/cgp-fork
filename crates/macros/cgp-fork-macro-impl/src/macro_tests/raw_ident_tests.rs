//! Raw identifiers (`r#Person`, `r#type`) must never make a macro panic.
//!
//! Several macros derive new names from a user's identifier (`__CheckPerson`,
//! `PersonComponent`, `__PartialPerson`). Formatting a raw identifier keeps its
//! `r#`, and `Ident::new("__Checkr#Person")` panics, which surfaces as an
//! opaque "proc macro panicked". The cargo-fuzz target found the first case
//! (`check_components! { r#r {} }`); this file pins every derived-name site.
//! Each case must expand, and the derived name must use the unrawed spelling.

use std::panic::{AssertUnwindSafe, catch_unwind};

use proc_macro2::TokenStream;
use quote::quote;

#[track_caller]
fn expands_without_panic(
    label: &str,
    expected: &str,
    run: impl FnOnce() -> syn::Result<TokenStream>,
) {
    match catch_unwind(AssertUnwindSafe(run)) {
        Err(_) => panic!("`{label}` panicked on a raw identifier"),
        Ok(Err(error)) => panic!("`{label}` rejected a raw identifier: {error}"),
        Ok(Ok(tokens)) => {
            let tokens = tokens.to_string();
            assert!(
                tokens.contains(expected),
                "`{label}` should derive `{expected}`:\n{tokens}"
            );
        }
    }
}

#[test]
fn check_components_with_raw_context() {
    expands_without_panic("check_components", "__CheckPerson", || {
        crate::macro_lib::check_components(quote!(r#Person { GreeterComponent }))
    });
}

#[test]
fn delegate_and_check_components_with_raw_context() {
    expands_without_panic("delegate_and_check_components", "__CanUsePerson", || {
        crate::macro_lib::delegate_and_check_components(quote!(r#Person {
            GreeterComponent: GreetHello,
        }))
    });
}

#[test]
fn build_field_with_raw_struct() {
    expands_without_panic("derive_build_field", "__PartialPerson", || {
        crate::macro_lib::derive_build_field(quote!(
            pub struct r#Person {
                pub name: String,
            }
        ))
    });
}

#[test]
fn extract_field_with_raw_enum() {
    expands_without_panic("derive_extract_field", "__PartialRefShape", || {
        crate::macro_lib::derive_extract_field(quote!(
            pub enum r#Shape {
                Circle(u8),
            }
        ))
    });
}

#[test]
fn cgp_component_with_raw_provider() {
    expands_without_panic("cgp_component", "GreeterComponent", || {
        crate::macro_lib::cgp_component(
            quote!(r#Greeter),
            quote!(
                pub trait CanGreet {
                    fn greet(&self);
                }
            ),
        )
    });
}

#[test]
fn cgp_impl_with_raw_provider() {
    expands_without_panic("cgp_impl", "GreetHello", || {
        crate::macro_lib::cgp_impl(
            quote!(new r#GreetHello),
            quote!(impl Greeter { fn greet(&self) {} }),
        )
    });
}

#[test]
fn cgp_impl_with_raw_provider_trait() {
    // Found by cargo-fuzz (cgp_impl): the component name derived from a raw provider trait
    // kept its `r#` and `Ident::new("r#GreeterComponent")` panicked.
    expands_without_panic("cgp_impl", "GreeterComponent", || {
        crate::macro_lib::cgp_impl(
            quote!(new GreetHello),
            quote!(impl r#Greeter { fn greet(&self) {} }),
        )
    });
}

#[test]
fn cgp_provider_with_raw_provider_trait() {
    expands_without_panic("cgp_provider", "GreeterComponent", || {
        crate::macro_lib::cgp_provider(
            quote!(),
            quote!(
                impl<Context> r#Greeter<Context> for GreetHello {
                    fn greet(_context: &Context) {}
                }
            ),
        )
    });
}

#[test]
fn cgp_fn_with_raw_name() {
    expands_without_panic("cgp_fn", "Greet", || {
        crate::macro_lib::cgp_fn(
            quote!(),
            quote!(
                fn r#greet(&self) {}
            ),
        )
    });
}

#[test]
fn cgp_auto_log_with_raw_method() {
    expands_without_panic("cgp_auto_log", "__Type", || {
        crate::extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLog {
                    fn r#type(&self, name: &str);
                }
            ),
        )
    });
}

#[test]
fn cgp_preset_with_raw_name() {
    expands_without_panic("cgp_preset", "__cgp_with_", || {
        crate::macro_lib::cgp_preset(quote!(r#Types { type Name = String, }))
    });
}

#[test]
fn type_shorthand_with_raw_name() {
    expands_without_panic(
        "delegate_components type shorthand",
        "NameTypeProviderComponent",
        || crate::macro_lib::delegate_components(quote!(App { type r#Name = String, })),
    );
}

#[test]
fn cgp_impl_helper_with_raw_provider() {
    expands_without_panic("cgp_impl helper trait", "__Impl_GreetHello__", || {
        crate::macro_lib::cgp_impl(
            quote!(r#GreetHello),
            quote!(impl Greeter { fn greet(&self) { self.helper() } #[helper] fn helper(&self) {} }),
        )
    });
}
