//! The smallest complete CGP program: one component, one provider, one context.
//!
//! Run with `cargo run -p cgp-fork --example hello`.

use cgp_fork::prelude::*;

// A component: the consumer trait `CanGreet` plus a generated provider trait `Greeter`
// and a `GreeterComponent` key.
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

// A getter component, so the provider can read the name from any context.
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

// A provider: implements `Greeter` for every context that has a name.
#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

// A context, wired to the provider it wants.
#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let person = Person {
        name: "Ferris".to_owned(),
    };
    println!("{}", person.greet());
    assert_eq!(person.greet(), "Hello, Ferris!");
}
