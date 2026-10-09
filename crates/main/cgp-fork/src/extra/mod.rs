/// Handler matchers, computers, and the producers programs usually import.
pub mod prelude;

/// Match an extensible enum or build an extensible record with a handler per field.
pub mod dispatch;
pub mod error;
pub mod field;
pub mod handler;
pub mod log;
pub mod monad;
pub mod run;
pub mod runtime;
