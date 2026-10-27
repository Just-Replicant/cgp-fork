// `#[cgp_provider]` takes the component type, not the provider trait (the `dyn` hint below is misleading; see the plan).
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

pub struct GreetHello;

#[cgp_provider(Greeter)]
impl<Context> Greeter<Context> for GreetHello {
    fn greet(_context: &Context) -> String {
        "hello".to_owned()
    }
}

fn main() {}
