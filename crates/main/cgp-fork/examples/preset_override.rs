//! A preset version and an override of one parent component.
//!
//! `#[version(N)]` stores `VERSION` on the preset module. `override` replaces a
//! component the parent already wires. A bare `override` uses the only parent.
//!
//! Run with `cargo run -p cgp-fork --example preset_override`.

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

pub struct Loud;

#[cgp_provider(GreeterComponent)]
impl<Context> Greeter<Context> for Loud {
    fn greet(_context: &Context) -> &'static str {
        "hello!"
    }
}

cgp_preset! {
    #[version(1)]
    Base {
        GreeterComponent: Hello,
    }
}

cgp_preset! {
    #[version(2)]
    LoudPreset: Base {
        override GreeterComponent: Loud,
    }
}

pub struct App;

delegate_components! {
    App {
        preset LoudPreset,
    }
}

fn main() {
    assert_eq!(Base::VERSION, 1);
    assert_eq!(LoudPreset::VERSION, 2);
    assert_eq!(App.greet(), "hello!");
    println!("{}", App.greet());
}
