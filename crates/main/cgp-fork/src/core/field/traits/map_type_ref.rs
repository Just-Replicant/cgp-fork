/// A type constructor for a borrow: shared, mutable, or owned.
///
/// Every access [`Mode`] implements it, with the same `Map`, so code written against
/// `R: MapTypeRef` and `<R as MapTypeRef>::Map<'a, T>` keeps compiling with the markers the
/// derives now use ([`Ref`](crate::core::field::types::Ref),
/// [`Mut`](crate::core::field::types::Mut), [`Owned`](crate::core::field::types::Owned)).
#[deprecated(
    since = "0.11.0",
    note = "merged into the access-mode trait: use `Mode` (`cgp_fork::core::field::traits::Mode`) and `<M as Mode>::Map<'a, T>`"
)]
pub trait MapTypeRef {
    /// `T` borrowed (or owned) for `'a`.
    type Map<'a, T: 'a>: 'a;
}

#[allow(deprecated)]
impl<M: Mode> MapTypeRef for M {
    type Map<'a, T: 'a> = M::Map<'a, T>;
}

use crate::core::field::traits::Mode;
