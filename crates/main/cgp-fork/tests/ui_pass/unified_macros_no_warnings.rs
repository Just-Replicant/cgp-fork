//! The replacements expand without any warning, including deprecation warnings.
#![deny(warnings)]

use cgp_fork::handler::prelude::*;
use cgp_fork::prelude::*;

#[cgp_handler]
fn double(input: u64) -> u64 {
    input * 2
}

#[cgp_handler]
async fn triple(input: u64) -> u64 {
    input * 3
}

#[cgp_component(Doubler)]
#[derive_handler(HandleDoubler)]
pub trait CanDouble {
    fn double_of(&self, input: u64) -> u64;
}

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
}

pub struct Circle;

#[cgp_auto_dispatch]
pub trait HasName {
    fn name(&self) -> &'static str;
}

impl HasName for Circle {
    fn name(&self) -> &'static str {
        "circle"
    }
}

pub struct App;

delegate_components! {
    App {
        DoublerComponent: HandleDoubler<Double>,
    }
}

fn main() {
    assert_eq!(App.double_of(2), 4);
    assert_eq!(Double::handle(&App, NoCode, 2), 4);
    let _ = Triple::handle_async(&App, NoCode, 1);
    assert_eq!(Shape::Circle(Circle).name(), "circle");
}
