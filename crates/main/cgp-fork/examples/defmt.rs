//! `CanLog` with the `defmt` backend.
//!
//! `DefmtLogger` records a `defmt::Format` detail with `defmt::info`. Firmware
//! supplies the global logger. A host build of this crate enables `defmt`'s
//! `unstable-test` feature, so the same call links here.
//!
//! `CanLog<Detail>` is generic, so the automatic `()` check is skipped and
//! `check_components!` names the real detail.
//!
//! Build with `cargo build -p cgp-fork --example defmt --features log-defmt`.

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

fn main() {
    App.log("booted");
    println!("logged");
}
