/// Access by value: the input is consumed.
///
/// One of the three access modes ([`Owned`], [`Ref`], [`Mut`]) that the mode-generic field
/// traits ([`ModeExtractor`](crate::core::field::traits::ModeExtractor),
/// [`ModeFields`](crate::core::field::traits::ModeFields)) and the unified dispatchers in
/// [`crate::handler`] are generic over.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Owned;

/// Access through a shared borrow: the input is `&'a T`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref;

/// Access through a mutable borrow: the input is `&'a mut T`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mut;
