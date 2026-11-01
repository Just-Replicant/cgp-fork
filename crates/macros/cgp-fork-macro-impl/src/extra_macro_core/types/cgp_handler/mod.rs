//! `#[cgp_handler]`: a function as a provider of the unified handler family
//! (`cgp_fork::handler`), the replacement for `#[cgp_computer]` and `#[cgp_producer]`.

mod item;
mod lower;

pub use item::*;
pub use lower::*;
