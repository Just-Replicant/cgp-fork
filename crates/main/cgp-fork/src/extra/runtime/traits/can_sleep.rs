use core::future::Future;
use core::time::Duration;

use crate::core::prelude::*;

#[cgp_component(Sleeper)]
/// Waits for a duration on the context's runtime.
///
/// Wire [`SleepWithRuntime`](crate::extra::runtime::SleepWithRuntime) when the runtime
/// implements [`AsyncRuntime`](crate::extra::runtime::AsyncRuntime). [`Sleep`](CanSleep::Sleep)
/// is `Send + 'static` and does not borrow `self`, so the future can be spawned without
/// `Self: Sync`. The signature is an associated type, rather than `#[cgp_async(Send)]`, because
/// that attribute's future still captures `&self`.
pub trait CanSleep {
    /// The future [`sleep`](CanSleep::sleep) returns. It does not borrow `Self`.
    type Sleep: Future<Output = ()> + Send + 'static;

    /// Suspends until `duration` has elapsed.
    ///
    /// The wait starts when the future is first polled. Dropping the future cancels the wait.
    #[must_use = "sleep does nothing unless the future is awaited"]
    fn sleep(&self, duration: Duration) -> Self::Sleep;
}
