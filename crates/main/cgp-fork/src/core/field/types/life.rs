use core::marker::PhantomData;

/// A type that is invariant in `'a`.
///
/// Used where a lifetime has to be named in a `PhantomData` without accidentally becoming
/// covariant or contravariant. The inner `*mut` is what makes the lifetime invariant.
pub struct Life<'a>(pub PhantomData<*mut &'a ()>);
