//! Variant names that match the derives' associated types still compile.
//!
//! The derives name those types as `<Self as Trait<Tag>>::Item`, so a variant called
//! `Value`, `Remainder`, `Extractor`, or `Fields` is not an ambiguous `Self::Item`.

use cgp_fork::prelude::*;

#[derive(Debug, Eq, PartialEq, FromVariant, ExtractField, HasFields)]
pub enum Tagged {
    Value(u32),
    Remainder(u8),
    Extractor(u8),
    ExtractorRef(u8),
    ExtractorMut(u8),
    Fields(u8),
    FieldsRef(u8),
}

#[test]
fn colliding_variant_names_construct() {
    let tagged = Tagged::from_variant(core::marker::PhantomData::<Symbol!("Value")>, 7u32);
    assert_eq!(tagged, Tagged::Value(7));
}
