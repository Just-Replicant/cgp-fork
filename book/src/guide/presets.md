# Presets

`cgp_preset!` bundles wiring. `#[version(N)]` stores `pub const VERSION: u64` on
the preset module. `override` replaces a component a parent already wires. The
parent's keys are not known while the macro runs, so the expansion emits a
`PresetContains` bound and the compiler checks it.

```rust
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
}
```

A bare `override` uses the only parent. With several parents, write
`override(Parent) Key: Provider`. `override` inside `delegate_components!` is an
error; a table replaces a preset entry by writing the key again.
