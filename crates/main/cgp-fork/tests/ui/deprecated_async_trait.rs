//! `#[async_trait]` is deprecated in favour of `#[cgp_async]`.
#![deny(deprecated)]

use cgp_fork::prelude::*;

#[async_trait]
pub trait CanFetch {
    async fn fetch(&self) -> u32;
}

fn main() {}
