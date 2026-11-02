# A component

`#[cgp_component(Name)]` on a trait defines three things:

- the consumer trait you wrote (`CanGreet`), implemented for a context that wires the component;
- the provider trait (`Greeter<Context>`), implemented with `#[cgp_provider]` or `#[cgp_impl]`;
- the component type (`GreeterComponent`), the key in `delegate_components!`.

`#[cgp_impl(new Provider)]` writes the provider against the consumer trait. The
`where Self:` bounds become bounds on the context.

A table whose context has no generic parameters is checked. The program below
compiles only because `GreetHello` can greet a `Person`.

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

pub struct Person;

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    assert_eq!(Person.greet(), "hello");
}
```

`#[unchecked]` skips that check. Use it for a type-level map that is not a
context, or for a component whose parameters are not `()`. See
[checked wiring](checked-wiring.md).
