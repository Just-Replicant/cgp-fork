/*!
   The implementation of the `cgp-fork` procedural macros, as an ordinary library.

   `cgp-fork-macro` is a thin proc-macro shim: each `#[proc_macro*]` function there converts its
   `proc_macro::TokenStream` arguments and calls the function of the same name in `macro_lib`,
   `extra_macro_lib`, or `async_impl`. Everything that parses or generates code lives here, so
   it can be unit-tested, fuzzed with libFuzzer, and reused by tools.

   Every entrypoint implements [`ir::Pipeline`] (`parse → validate → emit`) and is listed in
   [`registry`]; see `docs/adr/0004-shared-ir-pipeline.md`.

   **This crate is an implementation detail of `cgp-fork`.** Its API has no stability guarantee
   and may change in any release. Depend on `cgp-fork` instead.

   See `docs/adr/0002-macro-impl-crate.md` in the repository for the rationale.
*/

#[doc(hidden)]
pub mod async_impl;
#[doc(hidden)]
pub mod crate_path;
#[doc(hidden)]
pub mod extra_macro_lib;
#[doc(hidden)]
pub mod ir;
#[doc(hidden)]
pub mod macro_lib;
#[doc(hidden)]
pub mod registry;

mod extra_macro_core;
mod macro_core;

/// The `snapshot_*` entrypoints, which print a macro's expansion for the behavior tests.
#[cfg(any(test, feature = "snapshot"))]
#[doc(hidden)]
pub mod test_util_lib;

#[cfg(test)]
#[path = "macro_tests/crate_path_tests.rs"]
mod crate_path_tests;
#[cfg(test)]
#[path = "macro_tests/diagnostic_attrs_tests.rs"]
mod diagnostic_attrs_tests;
#[cfg(test)]
#[path = "macro_tests/ident_with_type_params_tests.rs"]
mod ident_with_type_params_tests;
#[cfg(test)]
#[path = "macro_tests/invalid_expansion_tests.rs"]
mod invalid_expansion_tests;
#[cfg(test)]
#[path = "macro_tests/ir_tests.rs"]
mod ir_tests;
#[cfg(test)]
#[path = "macro_tests/parser_rejections_tests.rs"]
mod parser_rejections_tests;
#[cfg(test)]
#[path = "macro_tests/raw_ident_tests.rs"]
mod raw_ident_tests;
#[cfg(test)]
#[path = "macro_tests/registry_tests.rs"]
mod registry_tests;
#[cfg(test)]
#[path = "macro_tests/robustness_tests.rs"]
mod robustness_tests;
#[cfg(test)]
#[path = "macro_tests/shape_macro_parsing_tests.rs"]
mod shape_macro_parsing_tests;
#[cfg(test)]
#[path = "macro_tests/symbol_tests.rs"]
mod symbol_tests;
#[cfg(test)]
#[path = "macro_tests/token_rewrite_tests.rs"]
mod token_rewrite_tests;
