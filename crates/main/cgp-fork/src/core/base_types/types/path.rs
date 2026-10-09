use core::marker::PhantomData;

/// One step of a type-level path, chained until [`Nil`](super::Nil).
///
/// [`Path!`](crate::core::macros::Path) builds this chain from a dotted path such as
/// `@app.error.ErrorRaiserComponent`.
pub struct PathCons<Head: ?Sized, Tail: ?Sized>(pub PhantomData<Head>, pub PhantomData<Tail>);
