//! A provider and a component default method can read a const they declare as `Self::LIMIT`.
//!
//! The `Self` rewrite turns a provider's `Self` into the context. An associated type the block
//! declares is skipped. A const is qualified to `<Provider as Trait<Context>>::LIMIT` first,
//! because leaving `Self::LIMIT` would be ambiguous with the consumer trait's blanket impl.
//! A default method on the component keeps `Self::LIMIT`: there `Self` is the provider trait.

use cgp_fork::prelude::*;

#[cgp_component(Limiter)]
pub trait CanLimit {
    const LIMIT: u32;

    fn below(&self, count: u32) -> bool {
        count < Self::LIMIT
    }
}

#[cgp_impl(new LimitTen)]
impl Limiter {
    const LIMIT: u32 = 10;

    fn below(&self, count: u32) -> bool {
        count < Self::LIMIT
    }
}

#[cgp_impl(new UseDefault)]
impl Limiter {
    const LIMIT: u32 = 4;
}

pub struct App;

delegate_components! {
    App {
        LimiterComponent: LimitTen,
    }
}

pub struct Small;

delegate_components! {
    Small {
        LimiterComponent: UseDefault,
    }
}

#[test]
fn provider_reads_its_own_const() {
    assert!(App.below(9));
    assert!(!App.below(10));
}

#[test]
fn default_method_reads_the_trait_const() {
    assert!(Small.below(3));
    assert!(!Small.below(4));
}
