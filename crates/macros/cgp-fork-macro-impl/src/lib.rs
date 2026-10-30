/*!
   The implementation of the `cgp-fork` procedural macros, as an ordinary library.

   `cgp-fork-macro` is a thin proc-macro shim: each `#[proc_macro*]` function there converts its
   `proc_macro::TokenStream` arguments and calls the function of the same name in [`macro_lib`],
   [`extra_macro_lib`] or [`async_impl`]. Everything that parses or generates code lives here, so
   it can be unit-tested, fuzzed with libFuzzer, and reused by tools.

   **This crate is an implementation detail of `cgp-fork`.** Its API has no stability guarantee
   and may change in any release. Depend on `cgp-fork` instead.

   See `docs/adr/0002-macro-impl-crate.md` in the repository for the rationale.
*/

pub mod async_impl;
pub mod extra_macro_lib;
pub mod macro_lib;
pub mod registry;

mod extra_macro_core;
mod macro_core;

/// The `snapshot_*` entrypoints, which print a macro's expansion for the behavior tests.
#[cfg(any(test, feature = "snapshot"))]
pub mod test_util_lib;

#[cfg(test)]
#[path = "macro_tests/ident_with_type_params_tests.rs"]
mod ident_with_type_params_tests;
#[cfg(test)]
#[path = "macro_tests/invalid_expansion_tests.rs"]
mod invalid_expansion_tests;
#[cfg(test)]
#[path = "macro_tests/parser_rejections_tests.rs"]
mod parser_rejections_tests;
#[cfg(test)]
#[path = "macro_tests/raw_ident_tests.rs"]
mod raw_ident_tests;
#[cfg(test)]
#[path = "macro_tests/robustness_tests.rs"]
mod robustness_tests;
#[cfg(test)]
#[path = "macro_tests/shape_macro_parsing_tests.rs"]
mod shape_macro_parsing_tests;
#[cfg(test)]
#[path = "macro_tests/token_rewrite_tests.rs"]
mod token_rewrite_tests;
