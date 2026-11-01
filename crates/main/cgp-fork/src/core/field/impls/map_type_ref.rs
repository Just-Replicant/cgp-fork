//! The 0.10 names of the access modes.

/// A shared borrow: `Map<'a, T> = &'a T`.
#[deprecated(
    since = "0.11.0",
    note = "use the access mode `Ref` (`cgp_fork::core::field::types::Ref`)"
)]
pub type IsRef = crate::core::field::types::Ref;

/// A mutable borrow: `Map<'a, T> = &'a mut T`.
#[deprecated(
    since = "0.11.0",
    note = "use the access mode `Mut` (`cgp_fork::core::field::types::Mut`)"
)]
pub type IsMut = crate::core::field::types::Mut;

/// The owned value: `Map<'a, T> = T`.
#[deprecated(
    since = "0.11.0",
    note = "use the access mode `Owned` (`cgp_fork::core::field::types::Owned`)"
)]
pub type IsOwned = crate::core::field::types::Owned;
