//! The list of every macro entrypoint, as plain functions over `proc_macro2`
//! token streams, and the seed inputs the fuzzers start from.
//!
//! Shared by the in-tree robustness fuzzer (`macro_tests/robustness_tests.rs`)
//! and the cargo-fuzz crate in `fuzz/`, so both cover the same macros. A new
//! entrypoint must be added here; `fuzz/fuzz_targets/` has one target per entry
//! (checked by `registry_tests`).

use proc_macro2::TokenStream;

use crate::ir::{FamilyIr, IrFamily, Pipeline, run};

/// A macro expansion function over `(attr, body)` token streams.
pub type EntrypointFn = fn(TokenStream, TokenStream) -> syn::Result<TokenStream>;

/// A macro entrypoint by name, as an `(attr, body)` function.
pub type Entrypoint = (&'static str, EntrypointFn);

/// Lists pipelines as `(name, family, run::<P>)` triples, taking each name from
/// [`Pipeline::NAME`] so the registry cannot drift from the pipelines.
macro_rules! pipelines {
    ($($pipeline:ty),* $(,)?) => {
        vec![$(
            (
                <$pipeline as Pipeline>::NAME,
                <<$pipeline as Pipeline>::Ir as FamilyIr>::FAMILY,
                run::<$pipeline> as EntrypointFn,
            ),
        )*]
    };
}

/// Every macro entrypoint with its IR family, in registry order.
pub fn pipelines() -> Vec<(&'static str, IrFamily, EntrypointFn)> {
    use crate::{async_impl as asy, extra_macro_lib as extra, macro_lib as lib};
    pipelines![
        asy::AsyncTraitMacro,
        lib::BlanketTraitMacro,
        lib::CgpAutoErrorMacro,
        lib::CgpAutoGetterMacro,
        lib::CgpAutoImplMacro,
        lib::CgpComponentMacro,
        lib::CgpFnMacro,
        lib::CgpGetterMacro,
        lib::CgpImplMacro,
        lib::CgpNewProviderMacro,
        lib::CgpProviderMacro,
        lib::CgpTypeMacro,
        lib::DeriveProviderMacro,
        extra::CgpComputerMacro,
        extra::CgpProducerMacro,
        extra::CgpAutoDispatchMacro,
        extra::CgpAutoLogMacro,
        lib::DelegateComponentsMacro,
        lib::DelegateAndCheckComponentsMacro,
        lib::CheckComponentsMacro,
        lib::CgpForEachMacro,
        lib::CgpPresetMacro,
        lib::CgpNamespaceMacro,
        lib::SymbolMacro,
        lib::PathMacro,
        lib::ProductTypeMacro,
        lib::ProductExprMacro,
        lib::SumTypeMacro,
        lib::StructTypeMacro,
        lib::EnumTypeMacro,
        lib::DeriveHasFieldMacro,
        lib::DeriveHasFieldsMacro,
        lib::DeriveBuildFieldMacro,
        lib::DeriveExtractFieldMacro,
        lib::DeriveFromVariantMacro,
        lib::DeriveCgpDataMacro,
        lib::DeriveCgpRecordMacro,
        lib::DeriveCgpVariantMacro,
    ]
}

/// Every macro entrypoint, as `(attr, body)` functions. Function-like and derive
/// macros ignore `attr`.
pub fn entrypoints() -> Vec<Entrypoint> {
    pipelines()
        .into_iter()
        .map(|(name, _family, entrypoint)| (name, entrypoint))
        .collect()
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
