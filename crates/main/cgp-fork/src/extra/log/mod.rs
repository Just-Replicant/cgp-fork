//! `CanLog`, plus the `tracing` and `defmt` providers behind their features.

#[cfg(any(feature = "log-tracing", feature = "log-defmt"))]
mod providers;
mod traits;

#[cfg(any(feature = "log-tracing", feature = "log-defmt"))]
pub use providers::*;
pub use traits::{CanLog, Logger, LoggerComponent};
