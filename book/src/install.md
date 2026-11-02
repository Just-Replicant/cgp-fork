# Install

```sh
cargo add cgp-fork
```

Rust 1.89 or newer. The macros read `Cargo.toml`, so a renamed dependency works
without a `crate = ...` argument:

```toml
cgp = { package = "cgp-fork", version = "1" }
```

Generated paths then use `cgp::` instead of `cgp_fork::`.

```rust
use cgp_fork::prelude::*;

fn main() {
    let _symbol: PhantomData<Symbol!("app")> = PhantomData;
}
```

`anyhow`, `eyre`, and `std-error` select the context's error type. `tokio` and
`smol` are runtime adapters and are not part of the default build. `mock`,
`log-tracing`, `log-defmt`, and `inspect` are off until you ask for them. The
full list is the [features](reference/features.md) page.
