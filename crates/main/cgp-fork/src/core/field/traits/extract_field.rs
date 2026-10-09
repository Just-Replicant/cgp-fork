use core::convert::Infallible;
use core::marker::PhantomData;

use crate::core::field::types::Void;

/// Converts `Self` to and from the sum type that dispatch and downcast walk.
///
/// The extractor is an [`Either`](crate::core::field::types::Either) chain of fields. Owned
/// extraction consumes `self`; the ref and mut traits borrow it instead.
pub trait HasExtractor {
    /// The sum of `Self`'s fields.
    type Extractor;

    /// Consumes `self` and returns its fields as a sum.
    fn to_extractor(self) -> Self::Extractor;

    /// Rebuilds `Self` from that sum.
    fn from_extractor(extractor: Self::Extractor) -> Self;
}

/// [`HasExtractor`] for a shared borrow.
pub trait HasExtractorRef {
    /// The sum of `Self`'s fields, borrowed for `'a`.
    type ExtractorRef<'a>
    where
        Self: 'a;

    /// Borrows `self`'s fields as a sum.
    fn extractor_ref(&self) -> Self::ExtractorRef<'_>;
}

/// [`HasExtractor`] for a mutable borrow.
pub trait HasExtractorMut {
    /// The sum of `Self`'s fields, mutably borrowed for `'a`.
    type ExtractorMut<'a>
    where
        Self: 'a;

    /// Mutably borrows `self`'s fields as a sum.
    fn extractor_mut(&mut self) -> Self::ExtractorMut<'_>;
}

/// Removes the `Tag` field from a product or sum, leaving the rest.
pub trait ExtractField<Tag> {
    /// The value stored under `Tag`.
    type Value;

    /// What remains after `Tag` is removed.
    type Remainder;

    /// Takes the `Tag` field, or returns the untouched remainder when this is a different variant.
    fn extract_field(self, _tag: PhantomData<Tag>) -> Result<Self::Value, Self::Remainder>;
}

/// Turns an uninhabited remainder into any type.
///
/// Implemented for [`Void`](crate::core::field::types::Void) and `Infallible`. Calling it means
/// every variant was consumed, so the remainder cannot exist.
pub trait FinalizeExtract {
    fn finalize_extract<T>(self) -> T;
}

impl FinalizeExtract for Void {
    fn finalize_extract<T>(self) -> T {
        match self {}
    }
}

impl FinalizeExtract for Infallible {
    fn finalize_extract<T>(self) -> T {
        match self {}
    }
}

pub trait FinalizeExtractResult {
    type Output;

    fn finalize_extract_result(self) -> Self::Output;
}

impl<T, E> FinalizeExtractResult for Result<T, E>
where
    E: FinalizeExtract,
{
    type Output = T;

    fn finalize_extract_result(self) -> T {
        match self {
            Ok(value) => value,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}
