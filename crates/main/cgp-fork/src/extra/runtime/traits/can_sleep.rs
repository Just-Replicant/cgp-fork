use core::time::Duration;

use crate::core::prelude::*;

#[cgp_component(Sleeper)]
#[cgp_async]
/// Waits for a duration on the context's runtime.
///
/// Wire [`SleepWithRuntime`](crate::extra::runtime::SleepWithRuntime) when the runtime
/// implements [`AsyncRuntime`](crate::extra::runtime::AsyncRuntime). The future is not `Send`;
/// put `#[cgp_async(Send)]` on a component that must be spawned.
pub trait CanSleep {
    /// Suspends until `duration` has elapsed.
    async fn sleep(&self, duration: Duration);
}
