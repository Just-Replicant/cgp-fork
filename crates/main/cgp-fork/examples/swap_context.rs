//! One interface, two implementations, each chosen by a context.
//!
//! `App` stores data in memory with a real clock, while `TestApp` uses a fixed clock,
//! so the code written against `CanStamp` is the same for both. The choice is made
//! in each context's `delegate_components!` table and compiles to a direct call.
//!
//! Run with `cargo run -p cgp-fork --example swap_context`.

use std::time::{SystemTime, UNIX_EPOCH};

use cgp_fork::prelude::*;

#[cgp_component(Clock)]
pub trait HasClock {
    fn now_secs(&self) -> u64;
}

#[cgp_component(Stamper)]
pub trait CanStamp {
    fn stamp(&self, message: &str) -> String;
}

#[cgp_impl(new SystemClock)]
impl Clock {
    fn now_secs(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or_default()
    }
}

#[cgp_impl(new FixedClock)]
impl Clock {
    fn now_secs(&self) -> u64 {
        1_700_000_000
    }
}

// Generic over any context that has a clock: written once, used by both contexts.
#[cgp_impl(new StampWithClock)]
impl Stamper
where
    Self: HasClock,
{
    fn stamp(&self, message: &str) -> String {
        format!("[{}] {message}", self.now_secs())
    }
}

pub struct App;

delegate_components! {
    App {
        ClockComponent: SystemClock,
        StamperComponent: StampWithClock,
    }
}

pub struct TestApp;

delegate_components! {
    TestApp {
        ClockComponent: FixedClock,
        StamperComponent: StampWithClock,
    }
}

fn main() {
    println!("{}", App.stamp("started"));

    let stamped = TestApp.stamp("started");
    println!("{stamped}");
    assert_eq!(stamped, "[1700000000] started");
}
