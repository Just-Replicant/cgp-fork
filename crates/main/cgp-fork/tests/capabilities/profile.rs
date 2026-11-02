//! `#[cfg_wiring]` selects which provider a test build wires.

use cgp_fork::prelude::*;

#[cgp_component(Clock)]
pub trait HasClock {
    fn now_secs(&self) -> u64;
}

pub struct FixedClock;

#[cgp_provider(ClockComponent)]
impl<Context> Clock<Context> for FixedClock {
    fn now_secs(_context: &Context) -> u64 {
        42
    }
}

pub struct OtherClock;

#[cgp_provider(ClockComponent)]
impl<Context> Clock<Context> for OtherClock {
    fn now_secs(_context: &Context) -> u64 {
        7
    }
}

pub struct App;

delegate_components! {
    #[cfg_wiring(test)]
    App {
        #[cfg_wiring(not(test))]
        ClockComponent: OtherClock,
        #[cfg_wiring(test)]
        ClockComponent: FixedClock,
    }
}

#[test]
fn test_profile_uses_the_fixed_clock() {
    assert_eq!(App.now_secs(), 42);
}
