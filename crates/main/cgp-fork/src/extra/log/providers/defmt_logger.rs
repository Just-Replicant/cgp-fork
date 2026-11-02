//! A [`defmt`](https://docs.rs/defmt) backend for [`CanLog`](crate::extra::log::CanLog).

use crate::extra::log::{Logger, LoggerComponent};
use crate::prelude::*;

/// Logs `Detail` with [`defmt::info`].
///
/// `Detail` implements [`defmt::Format`]. The binary supplies a global `defmt` logger.
pub struct DefmtLogger;

#[cgp_provider(LoggerComponent)]
impl<Context, Detail> Logger<Context, Detail> for DefmtLogger
where
    Detail: defmt::Format,
{
    fn log(_context: &Context, detail: Detail) {
        defmt::info!("{}", detail);
    }
}
