//! Variant derives name their associated types with a fully qualified projection, so a variant
//! whose name matches one of those types does not make `Self::Item` ambiguous.
//!
//! `#[derive(FromVariant)]` writes `<Self as FromVariant<Tag>>::Value`. `#[derive(ExtractField)]`
//! writes the same form for `Value`, `Remainder`, `Extractor`, `ExtractorRef`, and `ExtractorMut`.
//! `#[derive(HasFields)]` writes `<Self as HasFields>::Fields` and `<Self as HasFieldsRef>::FieldsRef`.
//! The value-namespace constructor `Self::Value(value)` stays a variant constructor.

use quote::quote;

use crate::test_util_lib::functions::pretty_format;

/// `#[derive(FromVariant)]` on an enum with a variant named `Value`.
#[test]
fn test_from_variant_qualifies_value() {
    let output = crate::macro_lib::derive_from_variant(quote! {
        #[derive(FromVariant)]
        pub enum Tagged {
            Value(u32),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        formatted.contains("<Self as")
            && formatted.contains("FromVariant")
            && formatted.contains(">::Value"),
        "expected a qualified `FromVariant::Value` parameter, got:\n{formatted}"
    );
    assert!(
        formatted.contains("Self::Value(value)"),
        "the variant constructor must stay `Self::Value`, got:\n{formatted}"
    );

    insta::assert_snapshot!(formatted, @r"
    impl FromVariant<
        Symbol<5, Chars<'V', Chars<'a', Chars<'l', Chars<'u', Chars<'e', Nil>>>>>>,
    > for Tagged {
        type Value = u32;
        fn from_variant(
            _tag: ::core::marker::PhantomData<
                Symbol<5, Chars<'V', Chars<'a', Chars<'l', Chars<'u', Chars<'e', Nil>>>>>>,
            >,
            value: <Self as FromVariant<
                Symbol<5, Chars<'V', Chars<'a', Chars<'l', Chars<'u', Chars<'e', Nil>>>>>>,
            >>::Value,
        ) -> Self {
            Self::Value(value)
        }
    }
    ");
}

/// `#[derive(ExtractField)]` on an enum with a variant named `Remainder`.
#[test]
fn test_extract_field_qualifies_remainder() {
    let output = crate::macro_lib::derive_extract_field(quote! {
        #[derive(ExtractField)]
        pub enum Tagged {
            Remainder(u32),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        formatted.contains("ExtractField") && formatted.contains(">::Remainder"),
        "expected a qualified `ExtractField::Remainder`, got:\n{formatted}"
    );
    assert!(
        formatted.contains("HasExtractor") && formatted.contains(">::Extractor"),
        "expected a qualified `HasExtractor::Extractor`, got:\n{formatted}"
    );
}

/// `#[derive(HasFields)]` on an enum with a variant named `Fields`.
#[test]
fn test_has_fields_qualifies_fields() {
    let output = crate::macro_lib::derive_has_fields(quote! {
        #[derive(HasFields)]
        pub enum Tagged {
            Fields(u32),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        formatted.contains("HasFields") && formatted.contains(">::Fields"),
        "expected a qualified `HasFields::Fields`, got:\n{formatted}"
    );
    assert!(
        formatted.contains("HasFieldsRef") && formatted.contains(">::FieldsRef"),
        "expected a qualified `HasFieldsRef::FieldsRef`, got:\n{formatted}"
    );
}
