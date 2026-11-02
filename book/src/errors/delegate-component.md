# `DelegateComponent`

```text
`App` has no wiring entry for `GreeterComponent`
```

`App` is being used as a type-level map, and `GreeterComponent` is not a key.
Add the entry:

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

pub struct App;

delegate_components! {
    App {
        GreeterComponent: Hello,
    }
}

fn main() {
    assert_eq!(App.greet(), "hello");
}
```

`DelegateComponent` is also a lookup table for providers such as `UseDelegate`.
In that use the key is an ordinary type, and the message means the same thing:
the key was not written into the table.
