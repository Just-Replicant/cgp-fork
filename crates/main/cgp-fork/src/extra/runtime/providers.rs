//! Providers that perform [`AsyncRuntime`](super::AsyncRuntime) operations for whatever runtime
//! the context stores.

use core::future::Future;
use core::time::Duration;

use super::ops::{AsyncRuntime, TimeoutElapsed};
use super::traits::{
    HasRuntime, HasRuntimeType, Sleeper, SleeperComponent, TaskSpawner, TaskSpawnerComponent,
    TimeoutRunner, TimeoutRunnerComponent,
};
use crate::core::prelude::*;

/// [`CanSleep`](super::CanSleep) by calling [`AsyncRuntime::sleep_for`](super::AsyncRuntime::sleep_for).
#[cgp_new_provider(SleeperComponent)]
impl<Context> Sleeper<Context> for SleepWithRuntime
where
    Context: HasRuntime,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
    async fn sleep(context: &Context, duration: Duration) {
        context.runtime().sleep_for(duration).await;
    }
}

/// [`CanSpawn`](super::CanSpawn) by calling [`AsyncRuntime::spawn_task`](super::AsyncRuntime::spawn_task).
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
#[cgp_new_provider(TimeoutRunnerComponent)]
impl<Context> TimeoutRunner<Context> for TimeoutWithRuntime
where
    Context: HasRuntime + CanRaiseError<TimeoutElapsed>,
    <Context as HasRuntimeType>::Runtime: AsyncRuntime,
{
    async fn timeout<T>(
        context: &Context,
        duration: Duration,
        task: impl Future<Output = T> + Send,
    ) -> Result<T, Context::Error> {
        context
            .runtime()
            .timeout_after(duration, task)
            .await
            .map_err(Context::raise_error)
    }
}
