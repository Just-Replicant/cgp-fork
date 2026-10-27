// A context calls a component method without wiring a provider for it.
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

pub struct App;

delegate_components! {
    App {}
}

fn main() {
    let _ = App.greet();
}
