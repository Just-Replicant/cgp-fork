use core::future::Future;
use core::time::Duration;

use crate::core::prelude::*;

#[cgp_component(TimeoutRunner)]
#[cgp_async]
#[use_type(HasErrorType.Error)]
/// Runs a `Send` task and returns the context error if it outlasts a duration.
///
/// Wire [`TimeoutWithRuntime`](crate::extra::runtime::TimeoutWithRuntime). The provider maps
/// [`TimeoutElapsed`](crate::extra::runtime::TimeoutElapsed) through [`CanRaiseError`](crate::core::error::CanRaiseError).
pub trait CanTimeout {
    /// `Ok` is the task's output. `Err` is the context error, including a timeout.
    async fn timeout<T>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send,
    ) -> Result<T, Error>;
}
