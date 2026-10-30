//! The list of every macro entrypoint, as plain functions over `proc_macro2`
//! token streams, and the seed inputs the fuzzers start from.
//!
//! Shared by the in-tree robustness fuzzer (`macro_tests/robustness_tests.rs`)
//! and the cargo-fuzz crate in `fuzz/`, so both cover the same macros. A new
//! entrypoint must be added here; `fuzz/fuzz_targets/` has one target per entry
//! (checked by `registry_tests`).

use proc_macro2::TokenStream;

/// A macro entrypoint by name, as an `(attr, body)` function.
pub type Entrypoint = (
    &'static str,
    fn(TokenStream, TokenStream) -> syn::Result<TokenStream>,
);

macro_rules! body_only {
    ($path:path) => {{
        fn call(_attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
            $path(body)
        }
        call as fn(TokenStream, TokenStream) -> syn::Result<TokenStream>
    }};
}

/// Every macro entrypoint, as `(attr, body)` functions.
pub fn entrypoints() -> Vec<Entrypoint> {
    use crate::{extra_macro_lib as extra, macro_lib as lib};
    vec![
        ("async_trait", crate::async_impl::impl_async),
        ("blanket_trait", lib::blanket_trait),
        ("cgp_auto_error", lib::cgp_auto_error),
        ("cgp_auto_getter", lib::cgp_auto_getter),
        ("cgp_auto_impl", lib::cgp_auto_impl),
        ("cgp_component", lib::cgp_component),
        ("cgp_fn", lib::cgp_fn),
        ("cgp_getter", lib::cgp_getter),
        ("cgp_impl", lib::cgp_impl),
        ("cgp_new_provider", lib::cgp_new_provider),
        ("cgp_provider", lib::cgp_provider),
        ("cgp_type", lib::cgp_type),
        ("derive_provider", lib::derive_provider),
        ("cgp_computer", extra::cgp_computer),
        ("cgp_producer", extra::cgp_producer),
        ("cgp_auto_dispatch", extra::cgp_auto_dispatch),
        ("cgp_auto_log", extra::cgp_auto_log),
        ("delegate_components", body_only!(lib::delegate_components)),
        (
            "delegate_and_check_components",
            body_only!(lib::delegate_and_check_components),
        ),
        ("check_components", body_only!(lib::check_components)),
        ("cgp_for_each", body_only!(lib::cgp_for_each)),
        ("cgp_preset", body_only!(lib::cgp_preset)),
        ("cgp_namespace", body_only!(lib::cgp_namespace)),
        ("Symbol", body_only!(lib::Symbol)),
        ("Path", body_only!(lib::Path)),
        ("Product", body_only!(lib::Product)),
        ("product", body_only!(lib::product)),
        ("Sum", body_only!(lib::Sum)),
        ("Struct", body_only!(lib::Struct)),
        ("Enum", body_only!(lib::Enum)),
        ("derive_has_field", body_only!(lib::derive_has_field)),
        ("derive_has_fields", body_only!(lib::derive_has_fields)),
        ("derive_build_field", body_only!(lib::derive_build_field)),
        (
            "derive_extract_field",
            body_only!(lib::derive_extract_field),
        ),
        ("derive_from_variant", body_only!(lib::derive_from_variant)),
        ("derive_cgp_data", body_only!(lib::derive_cgp_data)),
        ("derive_cgp_record", body_only!(lib::derive_cgp_record)),
        ("derive_cgp_variant", body_only!(lib::derive_cgp_variant)),
    ]
}

/// Valid (or nearly valid) inputs the mutator starts from: `(attr, body)`.
pub const SEEDS: &[(&str, &str)] = &[
    (
        "",
        "App { open ErrorRaiserComponent; GreeterComponent: GreetHello, @ErrorRaiserComponent.String: DisplayError, [FooComponent, BarComponent]: UseDelegate<new Table { u32: A, String: B }>, }",
    ),
    (
        "Greeter",
        "pub trait CanGreet<T>: HasName where T: Clone { type Out; fn greet(&self, value: &T) -> Self::Out; async fn run(&mut self) -> u32 { 1 } }",
    ),
    (
        "new GreetHello",
        "impl<T> Greeter<T> where Self: HasName { fn greet(&self, #[implicit] name: &str) -> String { format!(\"{name}\") } }",
    ),
    (
        "GreeterComponent",
        "impl<Context, T: Clone> Greeter<Context, T> for GreetHello where Context: HasName { fn greet(context: &Context) -> String { String::new() } }",
    ),
    ("", "\"hello world\""),
    ("", "u32, String, Vec<u8>, &'a str"),
    (
        "",
        "pub struct Person<'a, T: Clone> { pub name: &'a str, pub value: T, } ",
    ),
    (
        "",
        "pub enum Shape<T> { Circle(T), Square { side: T }, Empty }",
    ),
    (
        "",
        "@cgp.core.error in DefaultNamespace { ErrorRaiserComponent, FooComponent: Bar, }",
    ),
    (
        "",
        "App { GreeterComponent, [FooComponent, BarComponent]: (u32, String), }",
    ),
    ("", "MyPreset { GreeterComponent: GreetHello, } "),
    (
        "HasName",
        "pub trait HasName { fn name(&self) -> &str; fn age(&mut self) -> &mut u8; }",
    ),
    (
        "",
        "impl App { fn raise_error(e: String) -> Error { todo!() } }",
    ),
];

/// The cargo-fuzz target that fuzzes one entrypoint (`fuzz/fuzz_targets/<name>.rs`).
///
/// Type-level macros (`Symbol`, `Product`, …) get a `type_` prefix, so `Product` and
/// `product` stay distinct file names on case-insensitive file systems.
pub fn fuzz_target_name(entrypoint: &str) -> String {
    if entrypoint.starts_with(|c: char| c.is_ascii_uppercase()) {
        format!("type_{}", entrypoint.to_ascii_lowercase())
    } else {
        entrypoint.to_owned()
    }
}
