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

A const the `#[cgp_impl]` block declares is that provider's const. `Self::LIMIT`
inside the block reads it. A default method the provider does not replace reads
the const of whichever provider was wired.

```rust
use cgp_fork::prelude::*;

#[cgp_component(Limiter)]
pub trait CanLimit {
    const LIMIT: u32;

    fn below(&self, count: u32) -> bool {
        count < Self::LIMIT
    }
}

#[cgp_impl(new LimitTen)]
impl Limiter {
    const LIMIT: u32 = 10;

    fn below(&self, count: u32) -> bool {
        count < Self::LIMIT
    }
}

pub struct App;

delegate_components! {
    App {
        LimiterComponent: LimitTen,
    }
}

fn main() {
    assert!(App.below(9));
    assert!(!App.below(10));
}
```

A method parameter is a plain name. `mut count` is accepted. `(left, right)` is
not: destructure inside the body. The forwarding impl drops a `mut` it does not
use. `&mut self` stays.

`#[extend(HasName)]` adds a supertrait. `#[uses]` belongs on a provider, where
it names the provider's dependencies. `#[cgp_new_provider]` defines the provider
struct for you. `UseContext` forwards a provider back to the context's own
consumer impl. Do not wire a context to `UseContext` for that same component.

A public item the macro emits gets a doc comment when you wrote none. A doc you
wrote is left where you put it.
