//! `#[derive_promote]` packs two or more method arguments into one tuple input.
//!
//! `resize` takes `width_factor` and `height_factor` besides `&self`, so the
//! promoted provider calls `compute` with `(width_factor, height_factor)`. A
//! computer whose own arguments are those two values receives them in order.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_computer.md.

use cgp_fork::prelude::*;

#[cgp_component(Resizer)]
#[derive_promote(PromoteResizer)]
pub trait CanResize {
    fn resized_area(&self, width_factor: f64, height_factor: f64) -> f64;
}

#[cgp_computer]
fn resized_rectangle_area(
    #[field] width: f64,
    #[field] height: f64,
    width_factor: f64,
    height_factor: f64,
) -> f64 {
    width * width_factor * height * height_factor
}

#[derive(HasField)]
struct Rectangle {
    width: f64,
    height: f64,
}

delegate_components! {
    Rectangle {
        ResizerComponent: PromoteResizer<ResizedRectangleArea>,
    }
}

#[test]
fn test_promoted_resize_with_two_inputs() {
    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };

    // (2 * 10) * (3 * 0.5) = 30
    assert_eq!(rectangle.resized_area(10.0, 0.5), 30.0);
}
