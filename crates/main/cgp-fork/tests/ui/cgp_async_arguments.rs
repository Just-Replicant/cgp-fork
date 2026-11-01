// `#[cgp_async]` accepts `Send` and nothing else. `?Send` is not a way to opt out.
use cgp_fork::prelude::*;

#[cgp_async(?Send)]
pub trait CanFetch {
    async fn fetch(&self) -> u32;
}

fn main() {}
