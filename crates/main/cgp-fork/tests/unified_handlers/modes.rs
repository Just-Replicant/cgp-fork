//! The mode-generic field traits: one name for owned, shared and mutable access.

use cgp_fork::handler::prelude::*;
use cgp_fork::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub enum Value {
    Int(i64),
    Text(String),
}

#[derive(Debug, PartialEq, CgpData)]
pub struct Pair {
    pub left: u8,
    pub right: u8,
}

/// Written once over the mode, used with all three.
fn extract<M, Input>(input: Input) -> M::Extractor
where
    M: ModeExtractor<Input>,
{
    M::to_extractor(input)
}

#[test]
fn mode_extractor_matches_the_per_mode_traits() {
    let owned = Value::Int(3);
    let mut text = Value::Text("a".to_owned());

    let by_value = extract::<Owned, _>(Value::Int(3));
    assert!(matches!(
        by_value.extract_field(PhantomData::<Symbol!("Int")>),
        Ok(3)
    ));

    let by_ref = extract::<Ref, _>(&owned);
    assert!(matches!(
        by_ref.extract_field(PhantomData::<Symbol!("Int")>),
        Ok(&3)
    ));

    if let Err(rest) = extract::<Mut, _>(&mut text).extract_field(PhantomData::<Symbol!("Int")>) {
        let Ok(value) = rest.extract_field(PhantomData::<Symbol!("Text")>);
        value.push('b');
    }
    assert_eq!(text, Value::Text("ab".to_owned()));
}

#[test]
fn mode_fields_lists_owned_and_borrowed_fields() {
    let pair = Pair { left: 1, right: 2 };

    let Cons(left, Cons(right, Nil)) = <Ref as ModeFields<&Pair>>::to_fields(&pair);
    assert_eq!((*left.value, *right.value), (1, 2));

    let Cons(left, Cons(right, Nil)) = <Owned as ModeFields<Pair>>::to_fields(pair);
    assert_eq!((left.value, right.value), (1, 2));
}
