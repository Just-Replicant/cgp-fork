//! `#[version]` and `override` on a preset.

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
        "loud"
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

#[test]
fn override_replaces_the_parent_provider() {
    assert_eq!(Base::VERSION, 1);
    assert_eq!(LoudPreset::VERSION, 2);
    assert_eq!(App.greet(), "loud");
}

// `not(test)` is false while this crate's tests run. The override falls back to
// `Base`, and the new key is absent, so it does not have to name a real type.
cgp_preset! {
    Child: Base {
        #[cfg_wiring(not(test))]
        override GreeterComponent: Loud,

        #[cfg_wiring(not(test))]
        ClockComponent: FixedClock,
    }
}

pub struct ChildApp;

delegate_components! {
    ChildApp {
        preset Child,
    }
}

#[test]
fn cfg_off_override_keeps_the_parent_and_drops_the_new_key() {
    assert_eq!(ChildApp.greet(), "hello");
}
