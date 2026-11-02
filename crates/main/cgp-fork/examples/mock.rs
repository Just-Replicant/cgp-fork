//! A test context whose greeter is a generated mock.
//!
//! `#[cgp_component(Greeter, mock)]` emits `MockGreeter` and `GreeterMock`. The
//! context stores `GreeterMock` in `greeter_mock`. A queued value is returned
//! until a closure is installed; the closure then wins, and every call is counted.
//!
//! Run with `cargo run -p cgp-fork --example mock --features mock`.

use cgp_fork::prelude::*;

#[cgp_component(Greeter, mock)]
pub trait CanGreet {
    fn greet(&self, name: &str) -> String;
}

#[derive(HasField)]
pub struct TestApp {
    pub greeter_mock: GreeterMock,
}

delegate_components! {
    TestApp {
        GreeterComponent: MockGreeter,
    }
}

fn main() {
    let app = TestApp {
        greeter_mock: GreeterMock::new(),
    };

    app.greeter_mock.greet_returns("hello".to_owned());
    assert_eq!(app.greet("unused"), "hello");

    app.greeter_mock.greet_with(|name| format!("hello, {name}"));
    assert_eq!(app.greet("Ferris"), "hello, Ferris");
    assert_eq!(app.greeter_mock.greet_call_count(), 2);
    println!("{}", app.greet("Ferris"));
}
