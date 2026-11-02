//! A concrete `delegate_components!` checks its wiring. `#[unchecked]` opts out.

use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> &'static str;
}

pub struct Hello;

#[cgp_provider(GreeterComponent)]
impl<Context> Greeter<Context> for Hello {
    fn greet(_context: &Context) -> &'static str {
        "hello"
    }
}

pub struct NotAGreeter;

pub struct App;

delegate_components! {
    App {
        GreeterComponent: Hello,
    }
}

pub struct UncheckedApp;

delegate_components! {
    #[unchecked]
    UncheckedApp {
        GreeterComponent: NotAGreeter,
    }
}

#[test]
fn checked_context_greets() {
    assert_eq!(App.greet(), "hello");
}

#[test]
fn unchecked_table_is_still_a_type() {
    let _ = core::any::type_name::<UncheckedApp>();
}
