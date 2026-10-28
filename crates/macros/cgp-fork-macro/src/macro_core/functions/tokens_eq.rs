use quote::ToTokens;

/// Whether two syntax nodes print as the same tokens.
///
/// The macros compare parsed types, paths, and generics against expected forms
/// (`&str`, `Self::Assoc`, a round-tripped generics list). `syn` only implements
/// `PartialEq` for its syntax tree with the `extra-traits` feature, which every
/// downstream build would then compile. Comparing the token text instead needs
/// no feature and, like `syn`'s `PartialEq`, ignores spans.
///
/// Both sides have the same type, so the expected side can be written as an
/// inferred `parse_internal!` / `parse_quote!`.
pub fn tokens_eq<T>(a: &T, b: &T) -> bool
where
    T: ToTokens + ?Sized,
{
    a.to_token_stream().to_string() == b.to_token_stream().to_string()
}
