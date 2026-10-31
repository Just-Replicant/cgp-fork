//! `#[cgp_computer]` is deprecated in favour of `#[cgp_handler]`. The warning is on
//! the attribute; the code it generates does not warn again.
#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_computer]
fn double(input: u64) -> u64 {
    input * 2
}

fn main() {}
