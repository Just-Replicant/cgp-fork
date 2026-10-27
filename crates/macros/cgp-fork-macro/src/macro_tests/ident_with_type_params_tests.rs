//! Entrypoint for parser corner-case tests of the `IdentWithTypeArgs` /
//! `IdentWithTypeGenerics` / `PathWithTypeArgs` grammars in `macro_core`.
//!
//! These call the `macro_core` parsers directly to pin what they accept,
//! reject, and round-trip.
#![allow(dead_code)]

pub mod ident_with_type_params;
