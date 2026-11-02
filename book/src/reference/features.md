# Features

`default` is `full`, which is `alloc` and `compat-0.10`. A build with
`--no-default-features` uses only `core`.

| Feature | What it adds |
|---|---|
| `alloc` | The `alloc` crate, and the `String` error providers `DebugError` and `DisplayError`. |
| `compat-0.10` | The 0.10 handler family, `#[cgp_computer]`, `#[cgp_producer]`, and the bridges in `cgp_fork::compat`. On by default. The names are deprecated. |
| `anyhow` | `anyhow::Error` as a context error. Needs `alloc`. |
| `eyre` | `eyre::Report`. Needs `std`. |
| `std-error` | A boxed `core::error::Error`. Needs `alloc`. |
| `tokio` | `tokio::runtime::Handle` implements `AsyncRuntime`. Not part of `full`. |
| `smol` | An `async-executor` runtime with `async-io` timers. Does not link the `smol` meta-crate. |
| `inspect` | Keeps the wiring JSON const that `delegate_components!` and `cgp_preset!` emit. |
| `mock` | Compiles the types from `#[cgp_component(Name, mock)]`. Needs `alloc`. |
| `log-tracing` | `TracingLogger` and `TracingDebugLogger`. `tracing` is built without its default features. |
| `log-defmt` | `DefmtLogger`. Host builds enable `defmt`'s `unstable-test` so the call links. `target_os = "none"` does not. |

The macros read the caller's `Cargo.toml`, so `cgp = { package = "cgp-fork", version = "1" }`
generates `cgp::` paths.
