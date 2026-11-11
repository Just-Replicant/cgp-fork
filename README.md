# cgp-fork

[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/Just-Replicant/cgp-fork/blob/main/LICENSE)
[![Tests](https://github.com/Just-Replicant/cgp-fork/actions/workflows/tests.yml/badge.svg)](https://github.com/Just-Replicant/cgp-fork/actions/workflows/tests.yml)
![Rust 1.89+](https://img.shields.io/badge/rustc-1.89+-blue.svg)

**Pluggable trait implementations, resolved at compile time.**

This repository is **cgp-fork**, a fork of Context-Generic Programming (CGP). CGP lets one interface have many implementations, and lets each context — an application, a test, a deployment — choose the one it uses. The choice is written in one place and compiles down to a direct call.

The package name is `cgp-fork`. In Rust that crate is `cgp_fork`.

This fork has reviewed upstream [contextgeneric/cgp](https://github.com/contextgeneric/cgp) through commit [`3a235198`](https://github.com/contextgeneric/cgp/commit/3a23519885d00f07df7d9e26f80a619e928f5a51) (8 November 2026, pull request [#292](https://github.com/contextgeneric/cgp/pull/292)). The behavior changes in that range are ported here. Documentation and CI commits that only describe upstream's own crates are not carried over. The reviewed-through commit is recorded in [`UPSTREAM_COMMIT`](UPSTREAM_COMMIT). [`UPSTREAM_REVIEWED`](UPSTREAM_REVIEWED) lists anything newer that was reviewed and deliberately not merged; `bash scripts/upstream-sync.sh` lists the rest.

**[Website](https://contextgeneric.dev/) · [Changelog](CHANGELOG.md) · [Repository](https://github.com/Just-Replicant/cgp-fork)**

## Install

```sh
cargo add cgp-fork
```

`anyhow`, `eyre`, and `std-error` are optional features of that one crate. They select `anyhow::Error`, `eyre::Report`, or a boxed `core::error::Error` as the context error. `tokio` and `smol` are optional runtime adapters (spawn, sleep, timeout) and are not part of the default build. The `smol` feature is an `async-executor` runtime; it does not link the `smol` crate's net, filesystem, or process dependencies.

```rust
use cgp_fork::prelude::*;
```

Renaming the dependency (for example `cgp = { package = "cgp-fork", version = "0.10" }`) works:
the macros read your `Cargo.toml` and generate paths through the name you chose.

Rust **1.89+** on stable.

## Example

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

// One provider, generic over every context that has a name.
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

// The context chooses its provider.
delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let person = Person { name: "Ferris".to_owned() };
    assert_eq!(person.greet(), "Hello, Ferris!");
}
```

This example (and every Rust block in this README) is compiled as a doctest. More programs live in
[`crates/main/cgp-fork/examples`](crates/main/cgp-fork/examples): `hello`, `swap_context`,
`error_backends` (`--features anyhow,std-error`), `async_component`, `runtime`
(`--features tokio`), `preset_override`, `profile`, `mock` (`--features mock`),
`tracing` (`--features log-tracing`), `defmt` (`--features log-defmt`), and `error_enum`.

## Features

| Feature | Default | Needs | What it enables |
|---|---|---|---|
| `full` | yes | — | `alloc` and `compat-0.10` |
| `alloc` | via `full` | `alloc` | links `alloc`; the `DebugError` / `DisplayError` providers |
| `anyhow` | no | `alloc` | `anyhow::Error` as the context error |
| `eyre` | no | `std` | `eyre::Report` as the context error |
| `std-error` | no | `alloc` | a boxed `core::error::Error` as the context error |
| `tokio` | no | `std` | `tokio::runtime::Handle` as an `AsyncRuntime` (spawn, sleep, timeout) |
| `smol` | no | `std` | an owned `async-executor` runtime as an `AsyncRuntime` |
| `compat-0.10` | yes | — | the 0.10 handler names, deprecated |
| `mock` | no | `alloc` | `#[cgp_component(Name, mock)]` |
| `log-tracing` | no | — | `TracingLogger` and `TracingDebugLogger` |
| `log-defmt` | no | — | `DefmtLogger` |
| `inspect` | no | — | keeps the wiring JSON const |

With `default-features = false` the crate needs only `core`.

## Differences from upstream

- One facade crate (`cgp-fork`) and one macro crate (`cgp-fork-macro`) instead of upstream's
  per-concept crates; everything is reached through `cgp_fork::prelude`.
- The error backends are features of the facade rather than separate crates.
- Releases are versioned independently of upstream (fork 0.10 ≈ upstream 0.8.0 plus the fork's
  fixes); see the [Changelog](CHANGELOG.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the build, test, and release workflow, and for how
upstream changes are merged.

Wiring mistakes produce readable compiler errors that name the component to wire or the field a
getter reads. For a deeper analysis of a failed check, install
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) and run `cargo cgp check`.

## Docs

The guide, reference, error catalog, and migration page: [`book/`](book/src/SUMMARY.md).
`mdbook serve book` renders it. `cargo xtask migrate` rewrites 0.10 handler imports.

Upstream guides: [contextgeneric.dev](https://contextgeneric.dev/).

The rewrite plan: [docs/rewrite-plan.md](docs/rewrite-plan.md).

Construct reference and macro internals: [cgp-knowledge-base-fork](https://github.com/Just-Replicant/cgp-knowledge-base-fork). Related repositories: [sibling-projects.md](sibling-projects.md).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
