use core::future::Future;

use crate::core::prelude::*;

#[cgp_component(TaskSpawner)]
/// Starts a `Send + 'static` task on the context's runtime.
///
/// Wire [`SpawnWithRuntime`](crate::extra::runtime::SpawnWithRuntime) when the runtime
/// implements [`AsyncRuntime`](crate::extra::runtime::AsyncRuntime).
pub trait CanSpawn {
    /// Starts `task`. The call returns before `task` finishes.
    fn spawn(&self, task: impl Future<Output = ()> + Send + 'static);
}
