//! Log providers. Each one is behind the feature that supplies its backend.

#[cfg(feature = "log-defmt")]
mod defmt_logger;
#[cfg(feature = "log-tracing")]
mod tracing_logger;

#[cfg(feature = "log-defmt")]
pub use defmt_logger::DefmtLogger;
#[cfg(feature = "log-tracing")]
pub use tracing_logger::{TracingDebugLogger, TracingLogger};
