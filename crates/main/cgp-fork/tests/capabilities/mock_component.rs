//! `#[cgp_component(..., mock)]` records calls and returns queued values.

use std::rc::Rc;

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

#[test]
fn queued_value_then_closure() {
    let app = TestApp {
        greeter_mock: GreeterMock::new(),
    };

    app.greeter_mock.greet_returns("queued".to_owned());
    assert_eq!(app.greet("x"), "queued");

    app.greeter_mock.greet_with(|name| format!("hello, {name}"));
    assert_eq!(app.greet("Ferris"), "hello, Ferris");
    assert_eq!(app.greeter_mock.greet_call_count(), 2);
}

#[test]
fn closure_can_call_the_same_method() {
    let app = Rc::new(TestApp {
        greeter_mock: GreeterMock::new(),
    });
    let again = Rc::clone(&app);
    app.greeter_mock.greet_returns("inner".to_owned());
    app.greeter_mock
        .greet_with(move |_| again.greet("nested"));

    assert_eq!(app.greet("outer"), "inner");
    assert_eq!(app.greeter_mock.greet_call_count(), 2);
}

pub trait Named {
    fn label(&self) -> &'static str;
}

#[cgp_component(Namer, mock)]
pub trait CanName {
    fn name(&self) -> String
    where
        Self: Named;
}

#[derive(HasField)]
pub struct NamedApp {
    pub namer_mock: NamerMock,
}

impl Named for NamedApp {
    fn label(&self) -> &'static str {
        "app"
    }
}

delegate_components! {
    NamedApp {
        NamerComponent: MockNamer,
    }
}

#[test]
fn method_where_clause_binds_the_context() {
    let app = NamedApp {
        namer_mock: NamerMock::new(),
    };
    app.namer_mock.name_returns("ferris".to_owned());
    assert_eq!(app.name(), "ferris");
    assert_eq!(app.label(), "app");
}
