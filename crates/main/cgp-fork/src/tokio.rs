//! Tokio as an [`AsyncRuntime`].
//!
//! Enable the `tokio` feature. Store a `tokio::runtime::Handle` as the context's runtime and
//! keep the `tokio::runtime::Runtime` alive for as long as its tasks run: dropping the runtime
//! shuts the driver down even if handles remain. Poll sleep and timeout futures with
//! `Runtime::block_on`, or from a task already running on that runtime.
//!
//! ```ignore
//! delegate_components! {
//!     App {
//!         RuntimeTypeProviderComponent: UseType<tokio::runtime::Handle>,
//!         RuntimeGetterComponent: UseField<Symbol!("runtime")>,
//!         SleeperComponent: SleepWithRuntime,
//!         TaskSpawnerComponent: SpawnWithRuntime,
//!         TimeoutRunnerComponent: TimeoutWithRuntime,
//!     }
//! }
//! ```

use core::future::Future;
use core::time::Duration;

use crate::extra::runtime::{AsyncRuntime, TimeoutElapsed};

extern crate std;

impl AsyncRuntime for tokio::runtime::Handle {
    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        drop(self.spawn(task));
    }

    async fn sleep_for(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    async fn timeout_after<T>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send,
    ) -> Result<T, TimeoutElapsed> {
        match tokio::time::timeout(duration, task).await {
            Ok(value) => Ok(value),
            Err(_elapsed) => Err(TimeoutElapsed),
        }
    }
}

/// Builds a current-thread Tokio runtime with the time driver enabled.
///
/// # Errors
///
/// Returns the same I/O error `tokio::runtime::Builder::build` returns when the runtime
/// cannot be created.
pub fn current_thread() -> std::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
}
