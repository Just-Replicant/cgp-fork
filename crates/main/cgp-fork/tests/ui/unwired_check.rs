// `check_components!` lists a component that the context does not wire at all.
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

pub struct App;

delegate_components! {
    App {}
}

check_components! {
    App {
        GreeterComponent,
    }
}

fn main() {}
