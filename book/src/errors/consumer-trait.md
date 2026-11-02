# Consumer traits

```text
`App` does not implement `CanGreet`
```

`CanGreet` is implemented through `GreeterComponent`. Wire a provider for
`GreeterComponent` in the `delegate_components!` table of `App`.

If a provider is already wired, `check_components!` on `App` with
`GreeterComponent` names the requirement that provider is missing. That second
note is the [`CanUseComponent`](can-use-component.md) chain.

A `#[diagnostic::on_unimplemented]` written on the trait is kept. The generated
message is added only when the trait does not already have one.

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
