// The attribute macros that take no arguments point the error at the argument, not the item.
use cgp_fork::prelude::*;

#[cgp_auto_getter(Name)]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_auto_impl(Blanket)]
pub trait CanGreet {
    fn greet(&self) {}
}

#[cgp_auto_log(Level)]
pub trait CanLogCount {
    fn log_count(&self, count: u64);
}

pub struct ProvideAppError;

#[cgp_auto_error(Backend)]
impl ProvideAppError {
    type Error = String;
}

fn main() {}
