/// A type constructor used by builders to say whether a field is present, absent, or optional.
///
/// [`IsPresent`](crate::core::field::impls::IsPresent), [`IsNothing`](crate::core::field::impls::IsNothing),
/// [`IsOptional`](crate::core::field::impls::IsOptional), and [`IsVoid`](crate::core::field::impls::IsVoid)
/// are the four maps a partial builder uses.
pub trait MapType {
    /// `T` as this map sees it. Present keeps `T`; nothing replaces it with `()`.
    type Map<T>;
}
