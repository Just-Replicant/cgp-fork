//! `CanSendRun` is deprecated. The future is `Send` when the component uses `#[cgp_async(Send)]`.
#![deny(deprecated)]

use cgp_fork::extra::run::CanSendRun;

#[allow(dead_code)]
fn needs_send<Context: CanSendRun<()>>() {}

fn main() {}
