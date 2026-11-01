# `cgp-fork` — Context-Generic Programming in Rust

## Overview

Context-Generic Programming (CGP) is a language extension for Rust, with pluggable trait implementations at compile-time. In ordinary Rust a trait has one implementation per type; CGP lets one trait have many interchangeable implementations and lets each *context* choose which one it uses, through a small wiring table the compiler resolves statically — so the flexibility costs nothing at runtime. The `cgp-fork` crate is the library users depend on, and it re-exports everything through `cgp_fork::prelude`. The `anyhow`, `eyre`, and `std-error` features select the context error type.

To learn more, see the website [contextgeneric.dev](https://contextgeneric.dev/) and the book [Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/). The [CGP knowledge base](https://github.com/Just-Replicant/cgp-knowledge-base-fork) is the exhaustive per-construct reference.

## Example

A component is a consumer trait plus a generated provider trait. A provider implements it once,
generically, and each context picks its provider in `delegate_components!`:

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

let person = Person { name: "Ferris".to_owned() };
assert_eq!(person.greet(), "Hello, Ferris!");
```

More complete programs are in the repository's
[`examples/`](https://github.com/Just-Replicant/cgp-fork/tree/main/crates/main/cgp-fork/examples)
directory: swapping implementations per context, the error backends, and async components.

## Features

| Feature | Default | Needs | What it enables |
|---|---|---|---|
| `full` | yes | — | everything below that does not pull in a dependency (currently `alloc`) |
| `alloc` | via `full` | `alloc` | links `alloc`; the `String`-building error providers `DebugError` and `DisplayError` |
| `anyhow` | no | `alloc` | `cgp_fork::anyhow`: `anyhow::Error` as the context error |
| `eyre` | no | `std` | `cgp_fork::eyre`: `eyre::Report` as the context error (installs eyre's default hook on first use) |
| `std-error` | no | `alloc` | `cgp_fork::std_error`: `Box<dyn core::error::Error + Send + Sync>` as the context error |

With `default-features = false` the crate needs only `core`.
