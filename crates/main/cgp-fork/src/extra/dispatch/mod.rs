//! The deprecated 0.10 dispatch family, kept with the `compat-0.10` feature.
// Its items are deprecated in favour of `crate::handler`, and implement each other.
#![allow(deprecated)]

mod providers;

pub use providers::*;
