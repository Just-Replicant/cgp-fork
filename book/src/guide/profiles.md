# Profiles

`#[cfg_wiring(predicate)]` is `#[cfg(predicate)]`. On an entry it gates that
entry and its check. On the table it is copied onto every generated item. Two
entries may name the same component when their predicates cannot both be true.

```rust
use cgp_fork::prelude::*;

#[cgp_component(Clock)]
pub trait HasClock {
    fn now(&self) -> u64;
}

pub struct SystemClock;

#[cgp_provider(ClockComponent)]
impl<Context> Clock<Context> for SystemClock {
    fn now(_context: &Context) -> u64 {
        1
    }
}

pub struct FixedClock;

#[cgp_provider(ClockComponent)]
impl<Context> Clock<Context> for FixedClock {
    fn now(_context: &Context) -> u64 {
        0
    }
}

pub struct App;

delegate_components! {
    App {
        #[cfg_wiring(not(debug_assertions))]
        ClockComponent: SystemClock,
        #[cfg_wiring(debug_assertions)]
        ClockComponent: FixedClock,
    }
}

fn main() {
    let now = App.now();
    #[cfg(debug_assertions)]
    assert_eq!(now, 0);
    #[cfg(not(debug_assertions))]
    assert_eq!(now, 1);
}
```

A preset entry that is not an `override` is absent when the predicate is false.
An `override` forwards to the parent in that configuration, and the consumer
check still names the key.
