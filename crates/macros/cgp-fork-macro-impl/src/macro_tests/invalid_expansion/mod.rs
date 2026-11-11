//! Failure cases where a CGP macro emits invalid or incorrect Rust.
//!
//! See the entrypoint `invalid_expansion_tests.rs` for the pattern to follow when
//! adding a case.

// Pins the qualified associated-type paths the variant derives emit, so a variant named
// `Value` or `Fields` does not make `Self::Item` ambiguous.
pub mod reserved_variant_names;
