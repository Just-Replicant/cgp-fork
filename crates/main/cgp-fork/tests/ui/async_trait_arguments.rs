// `#[async_trait]` takes no arguments; `?Send` used to be silently ignored.
#![allow(deprecated)]

use cgp_fork::prelude::*;

#[async_trait(?Send)]
pub trait CanFetch {
    async fn fetch(&self) -> u32;
}

fn main() {}
