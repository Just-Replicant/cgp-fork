# Records and variants

`#[derive(CgpData)]` on a struct or enum derives the field traits, the shape
(`HasFields`, `HasFieldsRef`), the builder (`HasBuilder`, `BuildField`,
`FinalizeBuild`), and the extractor (`HasExtractor`, `HasExtractorRef`,
`HasExtractorMut`, `ExtractField`). The piece derives `HasField`, `HasFields`,
`BuildField`, `ExtractField`, and `FromVariant` exist on their own.

`Owned`, `Ref`, and `Mut` are the access modes. Dispatch and extractors take one
of them. `Product!` and `Sum!` are the type-level lists. `Struct!` and `Enum!`
are the shapes `HasFields` names.

```rust
use cgp_fork::prelude::*;

#[derive(CgpData, Debug, PartialEq, Eq)]
pub struct Person {
    pub name: &'static str,
}

#[derive(Debug, PartialEq, Eq, FromVariant)]
pub enum Tagged {
    Value(u32),
}

fn main() {
    let person = Person::builder()
        .build_field(PhantomData::<Symbol!("name")>, "Ada")
        .finalize_build();
    assert_eq!(person.name, "Ada");

    let tagged = Tagged::from_variant(PhantomData::<Symbol!("Value")>, 7);
    assert_eq!(tagged, Tagged::Value(7));
}
```

A variant may be named `Value`, `Remainder`, `Extractor`, `ExtractorRef`,
`ExtractorMut`, `Fields`, or `FieldsRef`. The derives refer to their associated
types as `<Self as FromVariant<Tag>>::Value` (and the same for the other
traits), so the variant is not ambiguous. `Self::Value(value)` in a constructor
stays a constructor.

`optional_builder` starts every field absent. `set` fills one. `finalize_optional`
returns the value once every field is present. `finalize_with_default` fills the
rest with `Default`. Those traits live in `cgp_fork::extra::field::impls`.
