use crate::core::field::types::{Cons, Nil};

/// Concatenates `Items` onto the end of a type-level [`Cons`](crate::core::field::types::Cons) list.
pub trait ConcatProduct<Items> {
    /// This list followed by `Items`. [`Nil`](crate::core::field::types::Nil) concatenated with `Items` is `Items`.
    type Output;
}

impl<Items> ConcatProduct<Items> for Nil {
    type Output = Items;
}

impl<Head, Tail, Items> ConcatProduct<Items> for Cons<Head, Tail>
where
    Tail: ConcatProduct<Items>,
{
    type Output = Cons<Head, Tail::Output>;
}
