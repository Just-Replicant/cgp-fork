//! Tokio as an [`AsyncRuntime`].
//!
//! Enable the `tokio` feature. Store a [`tokio::runtime::Handle`] as the context's runtime and
//! keep the [`tokio::runtime::Runtime`] alive for as long as its tasks run: dropping the runtime
//! shuts the driver down even if handles remain.
//!
//! [`AsyncRuntime::sleep_for`] and [`AsyncRuntime::timeout_after`] schedule their work on that
//! handle and return [`TokioSleep`] and [`TokioTimeout`], which are `Send + 'static`. The wait
//! starts when the future is first polled, not when it is created.
//! Poll them from any thread. Dropping a future that has been polled aborts the task. If the
//! runtime shuts down first, the waiting future panics. A panic inside a timed task is resumed
//! on the waiter.
//!
//! [`AsyncRuntime::spawn_task`] detaches the task. The join handle is dropped, so a panic inside
//! the task is discarded.
//!
//! ```
//! use core::time::Duration;
//!
//! use cgp_fork::extra::runtime::{
//!     CanSleep, CanSpawn, RuntimeGetterComponent, RuntimeTypeProviderComponent, SleepWithRuntime,
//!     SleeperComponent, SpawnWithRuntime, TaskSpawnerComponent,
//! };
//! use cgp_fork::prelude::*;
//!
//! #[derive(HasField)]
//! struct App {
//!     runtime: tokio::runtime::Handle,
//! }
//!
//! delegate_components! {
//!     App {
//!         RuntimeTypeProviderComponent: UseType<tokio::runtime::Handle>,
//!         RuntimeGetterComponent: UseField<Symbol!("runtime")>,
//!         SleeperComponent: SleepWithRuntime,
//!         TaskSpawnerComponent: SpawnWithRuntime,
//!     }
//! }
//!
//! let runtime = cgp_fork::tokio::current_thread().expect("current-thread runtime");
//! let app = App {
//!     runtime: runtime.handle().clone(),
//! };
//! runtime.block_on(async {
//!     app.spawn(async {});
//!     app.sleep(Duration::from_millis(1)).await;
//! });
//! ```

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use std::boxed::Box;

use crate::extra::runtime::{AsyncRuntime, TimeoutElapsed};

extern crate std;

/// A sleep scheduled on a Tokio runtime.
///
/// The timer starts on the first poll. Dropping the future before that does not spawn a task.
/// Dropping it afterwards aborts the sleep. Awaiting it panics if that runtime has already shut
/// down.
#[must_use = "sleep does nothing unless the future is awaited"]
pub struct TokioSleep {
    handle: tokio::runtime::Handle,
    duration: Duration,
    task: Option<AbortOnDrop<()>>,
}

/// A timeout scheduled on a Tokio runtime.
///
/// The timer starts on the first poll. Dropping the future before that drops `T`'s future
/// without spawning. Dropping it afterwards aborts the task. Awaiting it panics if that runtime
/// has already shut down, and resumes a panic raised by `T`'s future.
#[must_use = "the timeout does nothing unless the future is awaited"]
pub struct TokioTimeout<T> {
    handle: tokio::runtime::Handle,
    duration: Duration,
    work: Option<Pin<Box<dyn Future<Output = T> + Send>>>,
    task: Option<AbortOnDrop<Result<T, TimeoutElapsed>>>,
}

impl AsyncRuntime for tokio::runtime::Handle {
    type SleepFuture = TokioSleep;
    type TimeoutFuture<T: Send + 'static> = TokioTimeout<T>;

    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        // Detach. Dropping the `JoinHandle` does not cancel the task, and it drops
        // any panic the task stored.
        drop(self.spawn(task));
    }

    fn sleep_for(&self, duration: Duration) -> Self::SleepFuture {
        // The timer is armed on first poll, inside a task on this handle. Constructing
        // `tokio::time::sleep` off the runtime panics, and arming it at creation would
        // start the wait before the caller polls.
        TokioSleep {
            handle: self.clone(),
            duration,
            task: None,
        }
    }

    fn timeout_after<T: Send + 'static>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Self::TimeoutFuture<T> {
        TokioTimeout {
            handle: self.clone(),
            duration,
            work: Some(Box::pin(task)),
            task: None,
        }
    }
}

impl Future for TokioSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.task.is_none() {
            let duration = this.duration;
            this.task = Some(AbortOnDrop::new(this.handle.spawn(async move {
                tokio::time::sleep(duration).await;
            })));
        }
        let task = this.task.as_mut().expect("sleep task");
        match Pin::new(task).poll(context) {
            Poll::Ready(result) => {
                join_task(result);
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T: Send + 'static> Future for TokioTimeout<T> {
    type Output = Result<T, TimeoutElapsed>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.task.is_none() {
            let duration = this.duration;
            let work = this
                .work
                .take()
                .expect("timeout polled with no task and no work");
            this.task = Some(AbortOnDrop::new(this.handle.spawn(async move {
                match tokio::time::timeout(duration, work).await {
                    Ok(value) => Ok(value),
                    Err(_elapsed) => Err(TimeoutElapsed),
                }
            })));
        }
        let task = this.task.as_mut().expect("timeout task");
        match Pin::new(task).poll(context) {
            Poll::Ready(result) => Poll::Ready(join_task(result)),
            Poll::Pending => Poll::Pending,
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

/// Builds a multi-thread Tokio runtime with the time driver enabled.
///
/// The worker-thread count is Tokio's default. Sleep and timeout tasks run on those workers,
/// so the returned future can be polled from any thread.
///
/// # Errors
///
/// Returns the same I/O error `tokio::runtime::Builder::build` returns when the runtime
/// cannot be created.
pub fn multi_thread() -> std::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
}

/// Dropping a Tokio [`JoinHandle`](tokio::task::JoinHandle) detaches the task. Sleep and
/// timeout must cancel when the caller drops the future, so this aborts on drop.
struct AbortOnDrop<T> {
    handle: tokio::task::JoinHandle<T>,
}

impl<T> AbortOnDrop<T> {
    fn new(handle: tokio::task::JoinHandle<T>) -> Self {
        Self { handle }
    }
}

impl<T> Drop for AbortOnDrop<T> {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl<T> Future for AbortOnDrop<T> {
    type Output = Result<T, tokio::task::JoinError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.handle).poll(context)
    }
}

fn join_task<T>(result: Result<T, tokio::task::JoinError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) if error.is_panic() => std::panic::resume_unwind(error.into_panic()),
        Err(_cancelled) => panic!(
            "tokio runtime shut down while an AsyncRuntime sleep or timeout was still running"
        ),
    }
}
