//! Monadic piping, built on the 0.10 handler family. It is not deprecated: the unified
//! family has no monadic replacement yet.
// It implements the deprecated 0.10 traits.
#![allow(deprecated)]

/// The identity monad and the `Ok` and `Err` sides of `Result`.
pub mod monadic;
/// Piping a list of providers through a monad.
pub mod providers;
/// Binding, lifting, and reading the value inside a monadic output.
pub mod traits;
