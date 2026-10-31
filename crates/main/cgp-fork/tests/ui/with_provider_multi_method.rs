// `#[derive_provider(WithProvider)]` reads one field, so a getter may have one method.
use cgp_fork::prelude::*;

#[derive_provider(WithProvider)]
#[cgp_component(NamesGetter)]
pub trait HasNames {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

fn main() {}
