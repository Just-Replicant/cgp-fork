# Checked wiring

`delegate_components!` on a context with no generic parameters also emits
`__CanUse{Context}`. The context implements it only when every wired provider
can be used. `#[unchecked]` skips the check. A generic table is not checked.

```rust
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> &'static str;
}

pub struct GreetHello;

#[cgp_provider(GreeterComponent)]
impl<Context> Greeter<Context> for GreetHello {
    fn greet(_context: &Context) -> &'static str {
        "hello"
    }
}

pub struct App;

delegate_components! {
    App {
        GreeterComponent: GreetHello,
    }
}

pub struct Map;

delegate_components! {
    #[unchecked]
    Map {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    assert_eq!(App.greet(), "hello");
}
```

`CanLog<Detail>` is generic, so the automatic check would look for `CanLog<()>`.
Keep `#[unchecked]` and add `check_components!` with the real detail. An `open`
entry and a `Key =>` redirect are checked when this table delegates that
component. A namespace lookup path is not that component, so it stays an
explicit `check_components!`.
