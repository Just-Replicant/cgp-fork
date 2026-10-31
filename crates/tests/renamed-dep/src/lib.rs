//! Compile-time check that the `cgp-fork` macros work under a renamed dependency.
//!
//! `Cargo.toml` imports the package `cgp-fork` as `cgp`, so there is no
//! `cgp_fork` crate in scope here. The integration tests in `tests/` are copies
//! of representative behavior tests of `cgp-fork` with every `cgp_fork` path
//! renamed to `cgp`; each macro must therefore generate paths through `::cgp`.
//! Before ADR 0003 every one of them failed with
//! `error[E0433]: cannot find cgp_fork in the list of imported crates`.
//!
//! The library itself exercises the core macros once more, so a plain
//! `cargo build` already catches a regression.
#![no_std]

use cgp::prelude::*;

/// A component used only to prove the expansion compiles.
#[cgp_component(Greeter)]
pub trait CanGreet {
    /// The greeting.
    fn greet(&self) -> &'static str;
}

/// A provider for [`CanGreet`].
#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> &'static str {
        "hello"
    }
}

/// A context wired to [`GreetHello`].
#[derive(HasField)]
pub struct App {
    /// A field, so `HasField` has something to derive.
    pub name: &'static str,
}

delegate_and_check_components! {
    App {
        GreeterComponent: GreetHello,
    }
}

cgp_preset! {
    GreeterPreset {
        GreeterComponent: GreetHello,
    }
}
