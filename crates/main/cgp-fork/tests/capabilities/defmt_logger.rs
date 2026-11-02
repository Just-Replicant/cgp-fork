//! `DefmtLogger` logs a `defmt::Format` detail.
//!
//! A host build enables `defmt`'s `unstable-test` feature, so `defmt::info`
//! links without a device logger.

use cgp_fork::extra::log::DefmtLogger;
use cgp_fork::prelude::*;

pub struct App;

delegate_components! {
    #[unchecked]
    App {
        LoggerComponent: DefmtLogger,
    }
}

check_components! {
    App {
        LoggerComponent: &'static str,
    }
}

#[test]
fn defmt_logger_logs_the_detail() {
    App.log("booted");
}
