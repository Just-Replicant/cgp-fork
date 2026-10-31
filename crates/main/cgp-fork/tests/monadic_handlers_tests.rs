//! Entrypoint for the `monadic_handlers` concept.
//!
//! Covers CGP's monadic handler combinators: composing `Computer` handlers
//! through a monad with `PipeMonadic`, and the `BindOk`/`BindErr` binders that
//! sequence a handler within the ok/err monads. These are runtime tests — a
//! passing test is an actual assertion on the computed value, not just
//! successful compilation.
//!
//! See cgp-knowledge-base-fork/cgp/concepts/monadic-handlers.md and
//! cgp-knowledge-base-fork/cgp/reference/providers/monad_providers.md.
//!
//! The 0.10 handler family needs the `compat-0.10` feature (on by default).
#![cfg(feature = "compat-0.10")]
#![allow(dead_code)]

pub mod monadic_handlers;
