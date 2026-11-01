//! The deprecated `MapTypeRef`, `IsRef` and `IsMut` still compile: `MapTypeRef` is
//! implemented for every access [`Mode`], and `IsRef`/`IsMut`/`IsOwned` are aliases of
//! `Ref`/`Mut`/`Owned`, so code written against either name agrees.

// Exercises the deprecated `MapTypeRef` names.
#![allow(deprecated)]

use cgp_fork::core::field::impls::{IsMut, IsOwned, IsRef};
use cgp_fork::core::field::traits::{MapTypeRef, Mode};
use cgp_fork::core::field::types::{Mut, Owned, Ref};

fn same_type<T>(_: T, _: T) {}

fn old_name<'a, M: MapTypeRef, T: 'a>(value: <M as MapTypeRef>::Map<'a, T>) -> M::Map<'a, T> {
    value
}

fn new_name<'a, M: Mode, T: 'a>(value: <M as Mode>::Map<'a, T>) -> <M as Mode>::Map<'a, T> {
    value
}

#[test]
fn map_type_ref_agrees_with_mode() {
    let mut number = 5u8;

    let by_ref: &u8 = old_name::<IsRef, u8>(&number);
    same_type(by_ref, new_name::<Ref, u8>(&number));

    let by_mut: &mut u8 = old_name::<IsMut, u8>(&mut number);
    *by_mut += 1;
    let by_mut: &mut u8 = new_name::<Mut, u8>(by_mut);
    *by_mut += 1;

    let owned: u8 = old_name::<IsOwned, u8>(number);
    assert_eq!(owned, new_name::<Owned, u8>(7));
}
