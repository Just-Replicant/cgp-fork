//! A default method may bind a parameter `mut`. The forwarding impl drops that `mut`,
//! because it only passes the argument on, and the default body keeps it.

use cgp_fork::prelude::*;

#[cgp_component(Counter)]
pub trait CanCount {
    fn bump(&self, mut value: u32) -> u32 {
        value += 1;
        value
    }
}

#[cgp_impl(new UseDefault)]
impl Counter {}

pub struct App;

delegate_components! {
    App {
        CounterComponent: UseDefault,
    }
}

#[test]
fn default_body_mutates_its_parameter() {
    assert_eq!(App.bump(1), 2);
}
