use core::fmt::Debug;
use core::marker::PhantomData;

use crate::core::error::HasErrorType;

/// A context whose only job is to name the abstract error type `E`.
///
/// Useful as a stand-in context when a provider needs [`HasErrorType`](crate::core::error::HasErrorType)
/// and nothing else.
pub struct ErrorOnly<E>(pub PhantomData<E>);

impl<E> Default for ErrorOnly<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<E: Debug> HasErrorType for ErrorOnly<E> {
    type Error = E;
}
