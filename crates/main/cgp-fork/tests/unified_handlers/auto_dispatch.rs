//! `#[cgp_auto_dispatch]` expands to the unified `Handler` family (`MatchValues` over the
//! receiver's access mode), so it works without the `compat-0.10` feature and names no
//! deprecated item: this file denies `deprecated`.

#![deny(deprecated)]

use cgp_fork::prelude::*;
use futures::executor::block_on;

#[derive(Debug, CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug)]
pub struct Square {
    pub side: f64,
}

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.0 * self.radius * self.radius
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

#[cgp_auto_dispatch]
pub trait CanScale {
    fn scale(&mut self, factor: f64);
}

impl CanScale for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl CanScale for Square {
    fn scale(&mut self, factor: f64) {
        self.side *= factor;
    }
}

#[cgp_auto_dispatch]
pub trait IntoName {
    fn into_name(self, prefix: &'static str) -> String;
}

impl IntoName for Circle {
    fn into_name(self, prefix: &'static str) -> String {
        format!("{prefix}circle")
    }
}

impl IntoName for Square {
    fn into_name(self, prefix: &'static str) -> String {
        format!("{prefix}square")
    }
}

#[cgp_auto_dispatch]
#[cgp_async]
pub trait CanDescribe {
    async fn describe(&self) -> String;
}

impl CanDescribe for Circle {
    async fn describe(&self) -> String {
        format!("circle of radius {}", self.radius)
    }
}

impl CanDescribe for Square {
    async fn describe(&self) -> String {
        format!("square of side {}", self.side)
    }
}

#[test]
fn dispatches_by_reference() {
    assert_eq!(Shape::Square(Square { side: 2.0 }).area(), 4.0);
    assert_eq!(Shape::Circle(Circle { radius: 1.0 }).area(), 3.0);
}

#[test]
fn dispatches_by_mutable_reference() {
    let mut shape = Shape::Square(Square { side: 2.0 });
    shape.scale(3.0);
    assert_eq!(shape.area(), 36.0);
}

#[test]
fn dispatches_by_value() {
    assert_eq!(
        Shape::Circle(Circle { radius: 1.0 }).into_name("a "),
        "a circle"
    );
}

#[test]
fn dispatches_async() {
    assert_eq!(
        block_on(Shape::Square(Square { side: 2.0 }).describe()),
        "square of side 2"
    );
}
