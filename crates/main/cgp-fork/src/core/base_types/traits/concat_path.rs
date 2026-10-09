use crate::core::base_types::types::{Nil, PathCons};

/// Appends `Other` to the end of a type-level [`PathCons`](crate::core::base_types::types::PathCons) list.
pub trait ConcatPath<Other: ?Sized> {
    type Output: ?Sized;
}

impl<Head: ?Sized, Tail: ?Sized, Other: ?Sized> ConcatPath<Other> for PathCons<Head, Tail>
where
    Tail: ConcatPath<Other>,
{
    type Output = PathCons<Head, <Tail as ConcatPath<Other>>::Output>;
}

impl<Other: ?Sized> ConcatPath<Other> for Nil {
    type Output = Other;
}
