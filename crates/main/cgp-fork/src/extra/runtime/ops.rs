//! Operations every runtime adapter implements: spawn, sleep, and timeout.
//!
//! A context whose [`HasRuntimeType::Runtime`](super::HasRuntimeType::Runtime) implements
//! [`AsyncRuntime`] gets the same operations through [`HasAsyncRuntime`], with no extra wiring.
//! [`SleepWithRuntime`](super::SleepWithRuntime), [`SpawnWithRuntime`](super::SpawnWithRuntime)
//! and [`TimeoutWithRuntime`](super::TimeoutWithRuntime) are the providers to wire when the
//! operation itself should be replaceable.

use core::future::Future;
use core::time::Duration;

use super::{HasRuntime, HasRuntimeType};

/// The future passed to [`AsyncRuntime::timeout_after`] did not finish in time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeoutElapsed;

impl core::fmt::Display for TimeoutElapsed {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("operation timed out")
    }
}

impl core::error::Error for TimeoutElapsed {}

/// Spawn, sleep, and timeout on one runtime value.
///
/// Implemented for `tokio::runtime::Handle` (feature `tokio`) and for `SmolRuntime` (feature
/// `smol`). The sleep and timeout futures must be polled on that runtime (`Runtime::block_on`
/// for Tokio, `SmolRuntime::block_on` for smol).
pub trait AsyncRuntime {
    /// Starts `task` on this runtime. The task runs after this call returns.
    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static);

    /// Waits for `duration`. Poll the returned future on this runtime.
    fn sleep_for(&self, duration: Duration) -> impl Future<Output = ()> + Send;

    /// Resolves to `Ok` with the task's output, or [`TimeoutElapsed`] if `duration` passes first.
    /// Poll the returned future on this runtime.
    fn timeout_after<T>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send,
    ) -> impl Future<Output = Result<T, TimeoutElapsed>> + Send;
}

/// [`AsyncRuntime`] through the context's [`HasRuntime::runtime`](super::HasRuntime::runtime).
///
/// Implemented for every context whose runtime type implements [`AsyncRuntime`].
pub trait HasAsyncRuntime: HasRuntime
where
    <Self as HasRuntimeType>::Runtime: AsyncRuntime,
{
    /// Starts `task` on [`HasRuntime::runtime`](super::HasRuntime::runtime).
    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        self.runtime().spawn_task(task);
    }

    /// Waits for `duration` on the context runtime.
    fn sleep_for(&self, duration: Duration) -> impl Future<Output = ()> + Send {
        self.runtime().sleep_for(duration)
    }

    /// Runs `task` on the context runtime, failing with [`TimeoutElapsed`] if it outlasts `duration`.
    fn timeout_after<T>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send,
    ) -> impl Future<Output = Result<T, TimeoutElapsed>> + Send {
        self.runtime().timeout_after(duration, task)
    }
}

impl<Context> HasAsyncRuntime for Context
where
    Context: HasRuntime,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
}
