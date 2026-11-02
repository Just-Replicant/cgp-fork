use core::future::Future;
use core::time::Duration;

use crate::core::prelude::*;

#[cgp_component(TimeoutRunner)]
#[use_type(HasErrorType.Error)]
/// Runs a `Send + 'static` task and returns the context error if it outlasts a duration.
///
/// Wire [`TimeoutWithRuntime`](crate::extra::runtime::TimeoutWithRuntime). The provider maps
/// [`TimeoutElapsed`](crate::extra::runtime::TimeoutElapsed) through
/// [`CanRaiseError`](crate::core::error::CanRaiseError). [`Timeout`](CanTimeout::Timeout) is
/// `Send + 'static` and does not borrow `self`. The signature is an associated type, rather
/// than `#[cgp_async(Send)]`, because that attribute's future still captures `&self`.
pub trait CanTimeout {
    /// The future [`timeout`](CanTimeout::timeout) returns. It does not borrow `Self`.
    type Timeout<T: Send + 'static>: Future<Output = Result<T, Error>> + Send + 'static;

    /// `Ok` is the task's output. `Err` is the context error, including a timeout.
    ///
    /// The wait starts when the future is first polled. Dropping the future cancels the task.
    #[must_use = "the timeout does nothing unless the future is awaited"]
    fn timeout<T: Send + 'static>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Self::Timeout<T>;
}
