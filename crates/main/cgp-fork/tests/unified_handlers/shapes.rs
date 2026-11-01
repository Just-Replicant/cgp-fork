//! Shared shapes for the dispatch tests.

use std::f64::consts::PI;

use cgp_fork::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Debug, PartialEq, CgpData)]
pub enum ShapePlus {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

pub trait Scale {
    fn scale(&mut self, factor: f64);
}

impl Scale for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl Scale for Rectangle {
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

impl Scale for Triangle {
    fn scale(&mut self, factor: f64) {
        self.base *= factor;
        self.height *= factor;
    }
}

pub fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}
