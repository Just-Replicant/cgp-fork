//! Two providers for one component, selected with `#[cfg_wiring]`.
//!
//! The predicate is the same grammar as `#[cfg]`. A test build wires `FixedClock`.
//! A non-test build wires `SystemClock`. Entry-level attributes apply to that entry's
//! wiring and to the matching check.
//!
//! Run with `cargo run -p cgp-fork --example profile`.

use std::time::{SystemTime, UNIX_EPOCH};

use cgp_fork::prelude::*;

#[cgp_component(Clock)]
pub trait HasClock {
    fn now_secs(&self) -> u64;
}

pub struct SystemClock;

#[cgp_provider(ClockComponent)]
impl<Context> Clock<Context> for SystemClock {
    fn now_secs(_context: &Context) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or_default()
    }
}

pub struct FixedClock;

#[cgp_provider(ClockComponent)]
impl<Context> Clock<Context> for FixedClock {
    fn now_secs(_context: &Context) -> u64 {
        1_700_000_000
    }
}

pub struct App;

delegate_components! {
    App {
        #[cfg_wiring(not(test))]
        ClockComponent: SystemClock,
        #[cfg_wiring(test)]
        ClockComponent: FixedClock,
    }
}

fn main() {
    let now = App.now_secs();
    println!("{now}");
    assert!(now > 1_600_000_000);
}
