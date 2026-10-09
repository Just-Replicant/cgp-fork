use crate::core::field::types::{Cons, Nil};

/// Appends `Item` to the end of a type-level [`Cons`](crate::core::field::types::Cons) list.
pub trait AppendProduct<Item: ?Sized> {
    /// The list with `Item` as its last element. [`Nil`](crate::core::field::types::Nil) becomes `Cons<Item, Nil>`.
    type Output;
}

impl<Item> AppendProduct<Item> for Nil {
    type Output = Cons<Item, Nil>;
}

impl<Head, Tail, Item> AppendProduct<Item> for Cons<Head, Tail>
where
    Tail: AppendProduct<Item>,
{
    type Output = Cons<Head, Tail::Output>;
}
