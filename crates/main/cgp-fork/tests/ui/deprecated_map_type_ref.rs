//! `MapTypeRef` was merged into the access-mode trait `Mode`, and `IsRef` into `Ref`.
#![deny(deprecated)]

use cgp_fork::core::field::impls::IsRef;
use cgp_fork::core::field::traits::MapTypeRef;

fn lend<'a, M: MapTypeRef>(value: M::Map<'a, u8>) -> M::Map<'a, u8> {
    value
}

fn main() {
    let number = 1u8;
    let _ = lend::<IsRef>(&number);
}
