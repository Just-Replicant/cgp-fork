//! smol as an [`AsyncRuntime`].
//!
//! Enable the `smol` feature and store a [`SmolRuntime`] on the context. Sleep and timeout
//! futures, and tasks started with [`AsyncRuntime::spawn_task`],
//! run only while something polls this executor. [`SmolRuntime::block_on`] does that.

use alloc::sync::Arc;
use core::future::Future;
use core::time::Duration;

use crate::extra::runtime::{AsyncRuntime, TimeoutElapsed};

extern crate alloc;

/// A [`smol::Executor`] the context owns.
///
/// Cloning shares the executor. Dropping every clone does not by itself run pending tasks.
#[derive(Clone)]
pub struct SmolRuntime {
    executor: Arc<smol::Executor<'static>>,
}

impl SmolRuntime {
    /// An executor with no running tasks.
    #[must_use]
    pub fn new() -> Self {
        Self {
            executor: Arc::new(smol::Executor::new()),
        }
    }

    /// Polls `future`, and every task spawned on this executor, until `future` completes.
    pub fn block_on<Output>(&self, future: impl Future<Output = Output>) -> Output {
        smol::block_on(self.executor.run(future))
    }
}

impl Default for SmolRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for SmolRuntime {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("SmolRuntime")
    }
}

impl AsyncRuntime for SmolRuntime {
    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        self.executor.spawn(task).detach();
    }

    async fn sleep_for(&self, duration: Duration) {
        smol::Timer::after(duration).await;
    }

    async fn timeout_after<T>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send,
    ) -> Result<T, TimeoutElapsed> {
        let step = smol::future::or(async move { Step::Ready(task.await) }, async move {
            smol::Timer::after(duration).await;
            Step::TimedOut
        })
        .await;
        match step {
            Step::Ready(value) => Ok(value),
            Step::TimedOut => Err(TimeoutElapsed),
        }
    }
}

enum Step<T> {
    Ready(T),
    TimedOut,
}
