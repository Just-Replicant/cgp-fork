//! `#[derive_handler(Name)]` on `#[cgp_component]`, the replacement for the deprecated
//! `#[derive_promote(Name)]`: the generated provider forwards the component's single
//! method to a unified `Handler`, packing the method arguments into its input.

#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_component(AreaCalculator)]
#[derive_handler(HandleAreaCalculator)]
pub trait HasArea {
    fn area(&self) -> f64;
}

#[cgp_component(Resizer)]
#[derive_handler(HandleResizer)]
pub trait CanResize {
    fn resized_area(&self, width_factor: f64, height_factor: f64) -> f64;
}

#[cgp_handler]
fn rectangle_area(#[field] width: f64, #[field] height: f64) -> f64 {
    width * height
}

#[cgp_handler]
fn resized_rectangle_area(
    #[field] width: f64,
    #[field] height: f64,
    width_factor: f64,
    height_factor: f64,
) -> f64 {
    width * width_factor * height * height_factor
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_components! {
    Rectangle {
        AreaCalculatorComponent: HandleAreaCalculator<RectangleArea>,
        ResizerComponent: HandleResizer<ResizedRectangleArea>,
    }
}

#[test]
fn derived_handler_provider_forwards_the_method() {
    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };

    assert_eq!(rectangle.area(), 6.0);
    // (2 * 10) * (3 * 0.5) = 30
    assert_eq!(rectangle.resized_area(10.0, 0.5), 30.0);
}
