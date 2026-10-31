// The context wires one component but not the one `check_components!` asks for. The other
// wiring entries are not offered as near matches.
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_component(Namer)]
pub trait CanName {
    fn name(&self) -> String;
}

#[cgp_new_provider]
impl<Context> Namer<Context> for NameApp {
    fn name(_context: &Context) -> String {
        "app".to_owned()
    }
}

pub struct App;

delegate_components! {
    App {
        NamerComponent: NameApp,
    }
}

check_components! {
    App {
        NamerComponent,
        GreeterComponent,
    }
}

fn main() {}
