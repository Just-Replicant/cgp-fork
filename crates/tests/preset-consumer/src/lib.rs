//! A preset defined where `cgp-fork` is imported as `cgp`, used where it is `cgp_fork`.
//!
//! `GreeterPreset` lives in `cgp-fork-renamed-dep`. `with_components!` has to name the facade
//! as this crate sees it (`::cgp_fork`). Baking in `::cgp` fails here with
//! `error[E0433]: cannot find crate cgp`.

use cgp_fork::prelude::*;
use cgp_fork_renamed_dep::{CanGreet, GreeterComponent, GreeterPreset};

/// A context in this crate, wired with the other crate's preset.
pub struct Host;

delegate_and_check_components! {
    Host {
        preset GreeterPreset,
    }
}

fn assert_wired<T: CanGreet>() {}

#[allow(dead_code)]
fn host_can_greet() {
    assert_wired::<Host>();
}
