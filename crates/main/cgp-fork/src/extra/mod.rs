/// Handler matchers, computers, and the producers programs usually import.
///
/// The handler, dispatch and monad modules are the 0.10 handler family, kept with the
/// `compat-0.10` feature; [`crate::handler`] is the unified family.
pub mod prelude;

/// Match an extensible enum or build an extensible record with a handler per field.
#[cfg(feature = "compat-0.10")]
pub mod dispatch;
/// Providers that raise or wrap the abstract error.
pub mod error;
/// Builders that fill missing fields from `Default` or from `Option`.
pub mod field;
/// The handler family: produce, compute, try, and handle.
#[cfg(feature = "compat-0.10")]
pub mod handler;
/// Logging one detail value.
pub mod log;
/// Piping providers through `Result` and the identity monad.
#[cfg(feature = "compat-0.10")]
pub mod monad;
/// Running a context to completion.
pub mod run;
/// An abstract runtime value stored on the context.
pub mod runtime;
