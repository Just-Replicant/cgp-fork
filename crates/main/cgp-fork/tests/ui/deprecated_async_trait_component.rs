//! Under `#[cgp_component]`, a bare `#[async_trait]` warns once, on that attribute.
#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_component(Fetcher)]
#[async_trait]
pub trait CanFetch {
    async fn fetch(&self) -> u32;
}

fn main() {}
