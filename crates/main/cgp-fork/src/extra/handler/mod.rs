//! The deprecated 0.10 handler family, kept with the `compat-0.10` feature.
// Its items are deprecated in favour of `crate::handler`, and implement each other.
#![allow(deprecated)]

mod components;
mod providers;

pub use components::*;
pub use providers::*;

/// Shared with the unified family in [`crate::handler`].
pub use crate::handler::{NoCode, UseInputDelegate};
