use crate::core::field::traits::MapTypeRef;

/// [`MapTypeRef`](crate::core::field::traits::MapTypeRef) for a shared borrow: `Map<'a, T> = &'a T`.
pub struct IsRef;

impl MapTypeRef for IsRef {
    type Map<'a, T: 'a> = &'a T;
}

/// [`MapTypeRef`](crate::core::field::traits::MapTypeRef) for a mutable borrow: `Map<'a, T> = &'a mut T`.
pub struct IsMut;

impl MapTypeRef for IsMut {
    type Map<'a, T: 'a> = &'a mut T;
}

pub struct IsOwned;

impl MapTypeRef for IsOwned {
    type Map<'a, T: 'a> = T;
}
