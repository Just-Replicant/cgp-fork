//! `CanLog` with the `tracing` backend.
//!
//! `TracingLogger` records `Detail` with `tracing::info` and [`Display`].
//! `TracingDebugLogger` records it with [`Debug`]. The application installs a
//! subscriber; without one the event is discarded and the call still returns.
//!
//! `CanLog<Detail>` is generic, so the automatic `()` check is skipped and
//! `check_components!` names the real detail.
//!
//! Run with `cargo run -p cgp-fork --example tracing --features log-tracing`.

use std::fmt::{self, Display};

use cgp_fork::extra::log::{TracingDebugLogger, TracingLogger};
use cgp_fork::prelude::*;

pub struct Started {
    pub name: &'static str,
}

impl Display for Started {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "started {}", self.name)
    }
}

pub struct App;

delegate_components! {
    #[unchecked]
    App {
        LoggerComponent: TracingLogger,
    }
}

check_components! {
    App {
        LoggerComponent: Started,
    }
}

#[derive(Debug)]
pub struct Tick {
    pub n: u32,
}

pub struct DebugApp;

delegate_components! {
    #[unchecked]
    DebugApp {
        LoggerComponent: TracingDebugLogger,
    }
}

check_components! {
    DebugApp {
        LoggerComponent: Tick,
    }
}

fn main() {
    App.log(Started { name: "Ferris" });
    DebugApp.log(Tick { n: 1 });
    println!("logged");
}
