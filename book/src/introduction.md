# Introduction

cgp-fork is a Rust library for pluggable trait implementations. One interface (a
**component**) can have many implementations (**providers**). Each concrete type
(a **context**) picks its providers in one wiring table, and that choice compiles
to a direct call.

```rust
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let person = Person {
        name: "Ferris".to_owned(),
    };
    assert_eq!(person.greet(), "Hello, Ferris!");
}
```

The crate is `no_std`. Optional features add alloc, error backends, async
runtimes, mocks, and loggers. The 0.10 handler names stay available behind the
`compat-0.10` feature, which is on by default, and are deprecated. New code uses
[`Handler` and `AsyncHandler`](guide/handlers.md).

The rest of the guide is [fields](guide/fields.md), [functions](guide/functions.md),
[records and variants](guide/data.md), [abstract types](guide/types.md),
[namespaces](guide/namespaces.md), [async](guide/async.md), [errors](guide/errors.md),
[checked wiring](guide/checked-wiring.md), [profiles](guide/profiles.md),
[presets](guide/presets.md), [mocks](guide/mocks.md), [logging](guide/logging.md),
and the [wiring graph](guide/wiring-graph.md). The [macro reference](reference/macros.md)
is the full name list.
