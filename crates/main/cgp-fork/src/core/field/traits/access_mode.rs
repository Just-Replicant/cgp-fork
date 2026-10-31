//! One trait per field operation, generic over the access [`Mode`]: by value ([`Owned`]),
//! through a shared borrow ([`Ref`]) or through a mutable borrow ([`Mut`]).
//!
//! The per-mode traits ([`HasExtractor`], [`HasExtractorRef`], [`HasExtractorMut`],
//! [`ToFields`], [`ToFieldsRef`]) stay the primitives the derives implement. The traits here
//! put them behind one name, so a provider written once over `M: ModeExtractor<Input>` works
//! for an owned value, a `&T` and a `&mut T` alike (`docs/rewrite-plan.md` §7.3).

use crate::core::field::traits::{
    HasExtractor, HasExtractorMut, HasExtractorRef, ToFields, ToFieldsRef,
};
use crate::core::field::types::{Mut, Owned, Ref};

/// An access mode: [`Owned`], [`Ref`] or [`Mut`].
pub trait Mode: sealed::Sealed {}

impl Mode for Owned {}
impl Mode for Ref {}
impl Mode for Mut {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Owned {}
    impl Sealed for super::Ref {}
    impl Sealed for super::Mut {}
}

/// The extractor of `Input` in this mode: the [`Either`](crate::core::field::types::Either)
/// chain that dispatch walks.
///
/// | Mode | `Input` | Requires | `Extractor` |
/// |---|---|---|---|
/// | [`Owned`] | `T` | `T: HasExtractor` | `T::Extractor` |
/// | [`Ref`] | `&'a T` | `T: HasExtractorRef` | `T::ExtractorRef<'a>` |
/// | [`Mut`] | `&'a mut T` | `T: HasExtractorMut` | `T::ExtractorMut<'a>` |
pub trait ModeExtractor<Input>: Mode {
    /// The value or borrowed type whose variants are extracted (`T` for every row above).
    type Target: ?Sized;

    /// The sum of the target's variants, owned or borrowed as the mode says.
    type Extractor;

    /// Turns `input` into its extractor.
    fn to_extractor(input: Input) -> Self::Extractor;
}

impl<T> ModeExtractor<T> for Owned
where
    T: HasExtractor,
{
    type Target = T;
    type Extractor = T::Extractor;

    fn to_extractor(input: T) -> T::Extractor {
        input.to_extractor()
    }
}

impl<'a, T> ModeExtractor<&'a T> for Ref
where
    T: HasExtractorRef + ?Sized + 'a,
{
    type Target = T;
    type Extractor = T::ExtractorRef<'a>;

    fn to_extractor(input: &'a T) -> T::ExtractorRef<'a> {
        input.extractor_ref()
    }
}

impl<'a, T> ModeExtractor<&'a mut T> for Mut
where
    T: HasExtractorMut + ?Sized + 'a,
{
    type Target = T;
    type Extractor = T::ExtractorMut<'a>;

    fn to_extractor(input: &'a mut T) -> T::ExtractorMut<'a> {
        input.extractor_mut()
    }
}

/// The field list of `Input` in this mode.
///
/// | Mode | `Input` | Requires | `Fields` |
/// |---|---|---|---|
/// | [`Owned`] | `T` | `T: ToFields` | `T::Fields` |
/// | [`Ref`] | `&'a T` | `T: ToFieldsRef` | `T::FieldsRef<'a>` |
///
/// There is no mutable field list yet, so [`Mut`] has no implementation.
pub trait ModeFields<Input>: Mode {
    /// The product or sum of the fields, owned or borrowed as the mode says.
    type Fields;

    /// Turns `input` into its field list.
    fn to_fields(input: Input) -> Self::Fields;
}

impl<T> ModeFields<T> for Owned
where
    T: ToFields,
{
    type Fields = T::Fields;

    fn to_fields(input: T) -> T::Fields {
        input.to_fields()
    }
}

impl<'a, T> ModeFields<&'a T> for Ref
where
    T: ToFieldsRef + 'a,
{
    type Fields = T::FieldsRef<'a>;

    fn to_fields(input: &'a T) -> T::FieldsRef<'a> {
        input.to_fields_ref()
    }
}
