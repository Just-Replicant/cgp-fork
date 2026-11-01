//! `#[cgp_handler]`, the replacement for the deprecated `#[cgp_computer]` and
//! `#[cgp_producer]`: a function becomes a unified `Handler` provider (an `async`
//! function, an `AsyncHandler`), also wired as an `AsyncHandler` through `Promote`.
//!
//! `deny(deprecated)` proves the expansion names no deprecated item, with or without
//! the `compat-0.10` feature.

#![deny(deprecated)]

use cgp_fork::handler::prelude::*;
use cgp_fork::prelude::*;
use futures::executor::block_on;

#[cgp_handler]
fn rectangle_area(#[field] width: f64, #[field] height: f64, scale: f64) -> f64 {
    width * height * scale
}

#[cgp_handler]
async fn async_rectangle_area(#[field] width: f64, #[field] height: f64) -> f64 {
    width * height
}

#[cgp_handler]
fn label_length(label: &str) -> usize {
    label.len()
}

#[cgp_handler]
fn parse_number(input: String) -> Result<u32, String> {
    input.parse().map_err(|_| format!("not a number: {input}"))
}

#[cgp_handler]
fn forty_two() -> u32 {
    42
}

#[cgp_handler(Doubler)]
fn double_it(value: u32) -> u32 {
    value * 2
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[test]
fn sync_function_becomes_a_handler() {
    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };

    assert_eq!(RectangleArea::handle(&rectangle, NoCode, 2.0), 12.0);
}

#[test]
fn sync_handler_is_also_an_async_handler() {
    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };

    assert_eq!(
        block_on(RectangleArea::handle_async(&rectangle, NoCode, 0.5)),
        3.0
    );
}

#[test]
fn async_function_becomes_an_async_handler() {
    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };

    assert_eq!(
        block_on(AsyncRectangleArea::handle_async(&rectangle, NoCode, ())),
        6.0
    );
}

#[test]
fn borrowed_input() {
    assert_eq!(LabelLength::handle(&(), NoCode, "four"), 4);
}

#[test]
fn fallible_handler_returns_its_result() {
    assert_eq!(ParseNumber::handle(&(), NoCode, "12".to_owned()), Ok(12));
    assert_eq!(
        ParseNumber::handle(&(), NoCode, "x".to_owned()),
        Err("not a number: x".to_owned())
    );
}

#[test]
fn function_without_parameters_handles_unit() {
    assert_eq!(FortyTwo::handle(&(), NoCode, ()), 42);
    // `IgnoreInput` accepts any input, like a 0.10 `Producer` promoted to a `Computer`.
    assert_eq!(IgnoreInput::<FortyTwo>::handle(&(), NoCode, "ignored"), 42);
}

#[test]
fn explicit_provider_name() {
    assert_eq!(Doubler::handle(&(), NoCode, 21), 42);
    assert_eq!(
        Pipe::<Product![Doubler, Doubler]>::handle(&(), NoCode, 3),
        12
    );
}
