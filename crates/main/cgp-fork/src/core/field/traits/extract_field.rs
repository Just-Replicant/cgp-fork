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
    type ExtractorRef<'a>
    where
        Self: 'a;

    fn extractor_ref(&self) -> Self::ExtractorRef<'_>;
}

pub trait HasExtractorMut {
    type ExtractorMut<'a>
    where
        Self: 'a;

    fn extractor_mut(&mut self) -> Self::ExtractorMut<'_>;
}

pub trait ExtractField<Tag> {
    type Value;

    type Remainder;

    fn extract_field(self, _tag: PhantomData<Tag>) -> Result<Self::Value, Self::Remainder>;
}

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
