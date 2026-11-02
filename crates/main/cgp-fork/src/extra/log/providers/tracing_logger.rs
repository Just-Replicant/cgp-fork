//! [`tracing`](https://docs.rs/tracing) backends for [`CanLog`](crate::extra::log::CanLog).

use core::fmt::{Debug, Display};

use crate::extra::log::{Logger, LoggerComponent};
use crate::prelude::*;

/// Logs `Detail` with [`tracing::info`] using [`Display`].
///
/// The application installs a `tracing` subscriber. This provider only emits the event.
pub struct TracingLogger;

#[cgp_provider(LoggerComponent)]
impl<Context, Detail> Logger<Context, Detail> for TracingLogger
where
    Detail: Display,
{
    fn log(_context: &Context, detail: Detail) {
        tracing::info!("{detail}");
    }
}

/// Logs `Detail` with [`tracing::info`] using [`Debug`].
///
/// Use this for a `#[cgp_auto_log]` detail struct that does not implement [`Display`].
pub struct TracingDebugLogger;

#[cgp_provider(LoggerComponent)]
impl<Context, Detail> Logger<Context, Detail> for TracingDebugLogger
where
    Detail: Debug,
{
    fn log(_context: &Context, detail: Detail) {
        tracing::info!("{detail:?}");
    }
}
