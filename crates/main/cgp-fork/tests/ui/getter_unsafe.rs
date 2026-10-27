// Getter methods must be safe.
use cgp_fork::prelude::*;

#[cgp_getter(NameGetter)]
pub trait HasName {
    unsafe fn name(&self) -> &str;
}

fn main() {}
