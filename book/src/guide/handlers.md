# Handlers

The unified handler family has two components.

- `Handler<Context, Code, Input>` is synchronous. `Output` is an associated type.
- `AsyncHandler<Context, Code, Input>` returns a future of `Output`.

Borrowing is the input type (`&T`, `&mut T`). A fallible handler uses
`Output = Result<T, Context::Error>`. A producer takes `()`.

Import them with `use cgp_fork::handler::prelude::*;`. They are not in
`cgp_fork::prelude`, which still exports the deprecated 0.10 `Handler`.

```rust
use cgp_fork::handler::prelude::*;
use cgp_fork::prelude::*;

#[cgp_handler(Doubler)]
fn double_it(value: u32) -> u32 {
    value * 2
}

fn main() {
    assert_eq!(Doubler::handle(&(), NoCode, 21), 42);
}
```

`#[cgp_handler]` replaces `#[cgp_computer]` and `#[cgp_producer]`.
`#[derive_handler(Name)]` on a component replaces `#[derive_promote]`.
`Pipe` and `TryPipe` compose handlers. `MatchWith<Handlers, All | First, Owned | Ref | Mut>`
dispatches on an extensible enum. The [handler reference](../reference/handlers.md)
maps every 0.10 name.
