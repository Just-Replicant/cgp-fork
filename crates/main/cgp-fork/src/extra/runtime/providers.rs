//! Providers that perform [`AsyncRuntime`](super::AsyncRuntime) operations for whatever runtime
//! the context stores.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context as TaskContext, Poll};
use core::time::Duration;

use super::ops::{AsyncRuntime, TimeoutElapsed};
use super::traits::{
    HasRuntime, HasRuntimeType, Sleeper, SleeperComponent, TaskSpawner, TaskSpawnerComponent,
    TimeoutRunner, TimeoutRunnerComponent,
};
use crate::core::prelude::*;

/// The future [`TimeoutWithRuntime`] returns.
///
/// It owns the runtime's timeout future and maps [`TimeoutElapsed`] through the function stored
/// at construction, which is the context's [`CanRaiseError::raise_error`](crate::core::error::CanRaiseError::raise_error).
/// The context itself is not stored, so the future does not borrow it and does not require the
/// context to be `'static`.
#[must_use = "the timeout does nothing unless the future is awaited"]
pub struct RaisedTimeout<Error, Fut> {
    future: Fut,
    raise: fn(TimeoutElapsed) -> Error,
}

impl<Error, Fut> RaisedTimeout<Error, Fut> {
    fn new(future: Fut, raise: fn(TimeoutElapsed) -> Error) -> Self {
        Self { future, raise }
    }
}

impl<Error, Fut, T> Future for RaisedTimeout<Error, Fut>
where
    Fut: Future<Output = Result<T, TimeoutElapsed>> + Unpin,
{
    type Output = Result<T, Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut TaskContext<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.future).poll(context) {
            Poll::Ready(Ok(value)) => Poll::Ready(Ok(value)),
            Poll::Ready(Err(elapsed)) => Poll::Ready(Err((self.raise)(elapsed))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// [`CanSleep`](super::CanSleep) by calling [`AsyncRuntime::sleep_for`](super::AsyncRuntime::sleep_for).
///
/// The future is the runtime's own sleep future. It does not borrow `context`, so `Context`
/// does not need to be `Sync`.
#[cgp_new_provider(SleeperComponent)]
impl<Context> Sleeper<Context> for SleepWithRuntime
where
    Context: HasRuntime,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
    type Sleep = <<Context as HasRuntimeType>::Runtime as AsyncRuntime>::SleepFuture;

    fn sleep(context: &Context, duration: Duration) -> Self::Sleep {
        context.runtime().sleep_for(duration)
    }
}

/// [`CanSpawn`](super::CanSpawn) by calling [`AsyncRuntime::spawn_task`](super::AsyncRuntime::spawn_task).
///
/// The task is detached. A panic inside it is not delivered to the caller.
#[cgp_new_provider(TaskSpawnerComponent)]
impl<Context> TaskSpawner<Context> for SpawnWithRuntime
where
    Context: HasRuntime,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
    fn spawn(context: &Context, task: impl Future<Output = ()> + Send + 'static) {
        context.runtime().spawn_task(task);
    }
}

/// [`CanTimeout`](super::CanTimeout) by calling [`AsyncRuntime::timeout_after`](super::AsyncRuntime::timeout_after)
/// and raising [`TimeoutElapsed`] through [`CanRaiseError`](crate::core::error::CanRaiseError).
///
/// The future does not borrow `context`. The context error must be `'static`, because the
/// future stores the `raise_error` function.
#[cgp_new_provider(TimeoutRunnerComponent)]
impl<Context> TimeoutRunner<Context> for TimeoutWithRuntime
where
    Context: HasRuntime + CanRaiseError<TimeoutElapsed>,
    Context::Error: 'static,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
    type Timeout<T: Send + 'static> = RaisedTimeout<
        Context::Error,
        <<Context as HasRuntimeType>::Runtime as AsyncRuntime>::TimeoutFuture<T>,
    >;

    fn timeout<T: Send + 'static>(
        context: &Context,
        duration: Duration,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Self::Timeout<T> {
        RaisedTimeout::new(
            context.runtime().timeout_after(duration, task),
            Context::raise_error,
        )
    }
}
