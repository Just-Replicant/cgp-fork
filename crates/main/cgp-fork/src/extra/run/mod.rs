use core::future::Future;
use core::marker::PhantomData;

use crate::core::prelude::*;

#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
/// Runs `self` to completion, asynchronously.
///
/// `Code` selects which run this is. Failure is the context's abstract error.
pub trait CanRun<Code> {
    /// Runs until completion, or returns the context error.
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}

#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
#[use_type(HasErrorType.Error)]
/// [`CanRun`] whose future is `Send`, so it can be spawned on a work-stealing runtime.
pub trait CanSendRun<Code> {
    fn send_run(&self, _code: PhantomData<Code>) -> impl Future<Output = Result<(), Error>> + Send;
}
