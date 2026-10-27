// A wiring entry needs `Key: Provider`.
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

pub struct App;
pub struct GreetHello;

delegate_components! {
    App {
        GreeterComponent GreetHello,
    }
}

fn main() {}
