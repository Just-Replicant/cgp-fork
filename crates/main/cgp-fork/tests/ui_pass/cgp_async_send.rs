//! `#[cgp_async(Send)]` compiles cleanly and the desugared future is `Send`.
#![deny(warnings)]

use core::future::Future;

use cgp_fork::prelude::*;

#[cgp_async(Send)]
pub trait CanFetch {
    async fn fetch(&self) -> u32;
}

fn assert_send<Fut>(future: Fut) -> Fut
where
    Fut: Future<Output = u32> + Send,
{
    future
}

struct App;

impl CanFetch for App {
    async fn fetch(&self) -> u32 {
        1
    }
}

fn main() {
    let _ = assert_send(App.fetch());
}
