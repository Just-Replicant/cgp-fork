//! An async component: `#[async_trait]` desugars `async fn` so a provider can be async
//! and a default method body can await the others.
//!
//! Run with `cargo run -p cgp-fork --example async_component`.

use cgp_fork::prelude::*;
use futures::executor::block_on;

#[cgp_component(Fetcher)]
#[async_trait]
pub trait CanFetch {
    async fn fetch(&self, id: u32) -> String;
}

#[async_trait]
pub trait CanFetchTwo: CanFetch {
    // A provided async method, built on the component.
    async fn fetch_two(&self, first: u32, second: u32) -> (String, String) {
        (self.fetch(first).await, self.fetch(second).await)
    }
}

impl<Context: CanFetch> CanFetchTwo for Context {}

#[cgp_impl(new FetchFromMemory)]
impl Fetcher {
    async fn fetch(&self, id: u32) -> String {
        format!("item-{id}")
    }
}

pub struct App;

delegate_components! {
    App {
        FetcherComponent: FetchFromMemory,
    }
}

fn main() {
    let (first, second) = block_on(App.fetch_two(1, 2));
    println!("{first}, {second}");
    assert_eq!((first.as_str(), second.as_str()), ("item-1", "item-2"));
}
