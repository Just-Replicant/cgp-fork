// A context without wiring for a component is passed where its consumer trait is required.
// The message names the component to wire.
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

pub struct App;

delegate_components! {
    App {}
}

fn greet_twice<Context: CanGreet>(context: &Context) -> String {
    context.greet() + &context.greet()
}

fn main() {
    let _ = greet_twice(&App);
}
