use core::future::Future;

use crate::core::prelude::*;

#[cgp_component(TaskSpawner)]
/// Starts a `Send + 'static` task on the context's runtime.
///
/// Wire [`SpawnWithRuntime`](crate::extra::runtime::SpawnWithRuntime) when the runtime
/// implements [`AsyncRuntime`](crate::extra::runtime::AsyncRuntime). The task is detached:
/// the call does not return a join handle, so a panic inside `task` is not delivered here.
pub trait CanSpawn {
    /// Starts `task`. The call returns before `task` finishes.
    ///
    /// A panic inside `task` is discarded with the join handle.
    fn spawn(&self, task: impl Future<Output = ()> + Send + 'static);
}
