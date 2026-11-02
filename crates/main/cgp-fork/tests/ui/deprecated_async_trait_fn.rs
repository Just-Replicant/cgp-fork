//! Under `#[cgp_fn]`, a bare `#[async_trait]` warns once, on that attribute.
#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_fn]
#[async_trait]
pub async fn fetch(&self) -> u32 {
    1
}

fn main() {}
