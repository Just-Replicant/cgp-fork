//! `#[derive_promote]` is deprecated in favour of `#[derive_handler]`; the warning
//! points at the attribute.
#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_component(AreaCalculator)]
#[derive_promote(PromoteAreaCalculator)]
pub trait HasArea {
    fn area(&self) -> f64;
}

fn main() {}
