use crate::core::field::traits::MapType;
use crate::core::field::types::Void;

/// [`MapType`](crate::core::field::traits::MapType) that keeps the value: `Map<T> = T`.
pub struct IsPresent;

impl MapType for IsPresent {
    type Map<T> = T;
}

/// [`MapType`](crate::core::field::traits::MapType) that drops the value: `Map<T> = ()`.
pub struct IsNothing;

impl MapType for IsNothing {
    type Map<T> = ();
}

pub struct IsVoid;

impl MapType for IsVoid {
    type Map<T> = Void;
}

pub struct IsOptional;

impl MapType for IsOptional {
    type Map<T> = Option<T>;
}
