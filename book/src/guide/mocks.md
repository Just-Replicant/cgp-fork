# Mocks

`#[cgp_component(Name, mock)]` also emits `MockName` and `NameMock`. The generated
items compile when the `mock` feature is on. The context stores the state in a
field named `{name}_mock` (`greeter_mock` for `Greeter`).

A queued return value is used until a closure is installed. The closure then
wins, and every call is counted.

```rust
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
}
```

`mock` does not support generics, associated types, associated constants, or a
where clause on the component trait. The trait is a list of methods.
