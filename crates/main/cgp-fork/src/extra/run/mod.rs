use core::marker::PhantomData;

use crate::core::prelude::*;

#[cgp_component(Runner)]
#[cgp_async]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
/// Runs `self` to completion, asynchronously.
///
/// `Code` selects which run this is. Failure is the context's abstract error.
/// The future is not `Send`. Declare the component that must be spawned with
/// `#[cgp_async(Send)]` so every desugared future is `Send`; that flag replaces a
/// second runner component.
pub trait CanRun<Code> {
    /// Runs until completion, or returns the context error.
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}

#[cgp_component(SendRunner)]
#[cgp_async(Send)]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
#[deprecated(
    since = "0.11.0",
    note = "put `#[cgp_async(Send)]` on the component you spawn. `CanRun` stays the runner; its future is `Send` only when that component says so"
)]
/// A `Send` view of a run, kept so existing spawn proxies still compile.
///
/// Implement it for a concrete context by forwarding to [`CanRun::run`]. The
/// compiler can prove `Send` only once that context is concrete. New code puts
/// `#[cgp_async(Send)]` on the component it spawns instead of wiring `SendRunner`.
pub trait CanSendRun<Code> {
    /// Returns a `Send` future that runs `self` to completion.
    async fn send_run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}
