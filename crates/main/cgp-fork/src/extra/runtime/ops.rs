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
/// `smol`). [`SleepFuture`](AsyncRuntime::SleepFuture) and
/// [`TimeoutFuture`](AsyncRuntime::TimeoutFuture) are `Send + 'static`: they do not borrow the
/// runtime value, so the caller can drop that value or move the future onto another task.
///
/// [`spawn_task`](AsyncRuntime::spawn_task) detaches its task. The join handle is discarded, so
/// the caller does not observe completion, and a panic inside the task does not reach the caller
/// or the process panic hook.
pub trait AsyncRuntime {
    /// The `Send + 'static` future returned by [`sleep_for`](AsyncRuntime::sleep_for).
    type SleepFuture: Future<Output = ()> + Send + 'static;

    /// The `Send + 'static` future returned by [`timeout_after`](AsyncRuntime::timeout_after).
    type TimeoutFuture<T: Send + 'static>: Future<Output = Result<T, TimeoutElapsed>>
        + Send
        + Unpin
        + 'static;

    /// Starts `task` on this runtime. The task runs after this call returns.
    ///
    /// The task is detached. A panic inside `task` is not delivered to the caller.
    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static);

    /// Waits for `duration`.
    ///
    /// The wait starts when the future is first polled. The future does not borrow `self`.
    /// Dropping it cancels the wait.
    #[must_use = "sleep does nothing unless the future is awaited"]
    fn sleep_for(&self, duration: Duration) -> Self::SleepFuture;

    /// Resolves to `Ok` with the task's output, or [`TimeoutElapsed`] if `duration` passes first.
    ///
    /// `task` is `'static` because the timeout may run on the runtime's own threads. The wait
    /// starts when the future is first polled. The returned future does not borrow `self`.
    /// Dropping it cancels the timeout and `task`.
    #[must_use = "the timeout does nothing unless the future is awaited"]
    fn timeout_after<T: Send + 'static>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Self::TimeoutFuture<T>;
}

/// [`AsyncRuntime`] through the context's [`HasRuntime::runtime`](super::HasRuntime::runtime).
///
/// Implemented for every context whose runtime type implements [`AsyncRuntime`]. The futures do
/// not borrow the context.
pub trait HasAsyncRuntime: HasRuntime
where
    <Self as HasRuntimeType>::Runtime: AsyncRuntime,
{
    /// Starts `task` on [`HasRuntime::runtime`](super::HasRuntime::runtime).
    ///
    /// The task is detached. A panic inside `task` is not delivered to the caller.
    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        self.runtime().spawn_task(task);
    }

    /// Waits for `duration` on the context runtime.
    ///
    /// The future does not borrow `self`.
    #[must_use = "sleep does nothing unless the future is awaited"]
    fn sleep_for(
        &self,
        duration: Duration,
    ) -> <<Self as HasRuntimeType>::Runtime as AsyncRuntime>::SleepFuture {
        self.runtime().sleep_for(duration)
    }

    /// Runs `task` on the context runtime, failing with [`TimeoutElapsed`] if it outlasts
    /// `duration`.
    ///
    /// The future does not borrow `self`.
    #[must_use = "the timeout does nothing unless the future is awaited"]
    fn timeout_after<T: Send + 'static>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send + 'static,
    ) -> <<Self as HasRuntimeType>::Runtime as AsyncRuntime>::TimeoutFuture<T> {
        self.runtime().timeout_after(duration, task)
    }
}

impl<Context> HasAsyncRuntime for Context
where
    Context: HasRuntime,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
}
