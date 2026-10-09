/// Handler matchers, computers, and the producers programs usually import.
pub mod prelude;

/// Match an extensible enum or build an extensible record with a handler per field.
pub mod dispatch;
/// Providers that raise or wrap the abstract error.
pub mod error;
/// Builders that fill missing fields from `Default` or from `Option`.
pub mod field;
/// The handler family: produce, compute, try, and handle.
pub mod handler;
pub mod log;
pub mod monad;
pub mod run;
pub mod runtime;
