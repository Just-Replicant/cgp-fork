/// A type constructor for a borrow: shared, mutable, or owned.
///
/// [`IsRef`](crate::core::field::impls::IsRef), [`IsMut`](crate::core::field::impls::IsMut), and
/// [`IsOwned`](crate::core::field::impls::IsOwned) are the three maps.
pub trait MapTypeRef {
    type Map<'a, T: 'a>: 'a;
}
