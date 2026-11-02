//! An [`AsyncRuntime`] built from `async-executor` and `async-io`.
//!
//! Enable the `smol` feature and store a [`SmolRuntime`] on the context. Those two crates are
//! what this adapter uses. The feature does not link the `smol` meta-crate or its net,
//! filesystem, and process dependencies.
//!
//! [`AsyncRuntime::sleep_for`] and [`AsyncRuntime::timeout_after`] return
//! [`SmolSleep`] and [`SmolTimeout`], which are `Send + 'static` and are driven by the
//! `async-io` reactor. The wait starts when the future is first polled. They do not borrow
//! the runtime, and dropping them cancels the wait.
//! Spawned tasks are different: they run only while [`SmolRuntime::block_on`] is polling this
//! executor. [`AsyncRuntime::spawn_task`] detaches the task, so a panic inside it is not
//! delivered to the caller.

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use async_executor::Executor;
use async_io::Timer;

use crate::extra::runtime::{AsyncRuntime, TimeoutElapsed};

extern crate alloc;

/// An [`async_executor::Executor`] the context owns.
///
/// Cloning shares the executor. Dropping every clone does not by itself run pending tasks.
/// Sleep and timeout do not need the executor; the `async-io` reactor completes them while
/// their futures are polled. Tasks started with [`AsyncRuntime::spawn_task`] run only inside
/// [`SmolRuntime::block_on`].
#[derive(Clone)]
pub struct SmolRuntime {
    executor: Arc<Executor<'static>>,
}

impl SmolRuntime {
    /// An executor with no running tasks.
    #[must_use]
    pub fn new() -> Self {
        Self {
            executor: Arc::new(Executor::new()),
        }
    }

    /// Polls `future`, and every task spawned on this executor, until `future` completes.
    pub fn block_on<Output>(&self, future: impl Future<Output = Output>) -> Output {
        async_io::block_on(self.executor.run(future))
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

/// A sleep driven by the `async-io` reactor.
///
/// The deadline is set on the first poll, so creating the future does not start the wait.
/// Dropping it cancels the wait. The executor does not have to be polled.
#[must_use = "sleep does nothing unless the future is awaited"]
pub struct SmolSleep {
    duration: Duration,
    timer: Option<Timer>,
}

/// A timeout driven by the `async-io` reactor.
///
/// The deadline is set on the first poll. The task is preferred when it and the timer are both
/// ready. Dropping the future drops the task. The executor does not have to be polled unless
/// `task` itself spawns work onto it.
#[must_use = "the timeout does nothing unless the future is awaited"]
pub struct SmolTimeout<T> {
    task: Pin<Box<dyn Future<Output = T> + Send>>,
    duration: Duration,
    timer: Option<Timer>,
}

impl AsyncRuntime for SmolRuntime {
    type SleepFuture = SmolSleep;
    type TimeoutFuture<T: Send + 'static> = SmolTimeout<T>;

    fn spawn_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        // Detach, so the task outlives this call. A panic in the task is not
        // delivered to the caller.
        self.executor.spawn(task).detach();
    }

    fn sleep_for(&self, duration: Duration) -> Self::SleepFuture {
        SmolSleep {
            duration,
            timer: None,
        }
    }

    fn timeout_after<T: Send + 'static>(
        &self,
        duration: Duration,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Self::TimeoutFuture<T> {
        SmolTimeout {
            task: Box::pin(task),
            duration,
            timer: None,
        }
    }
}

impl Future for SmolSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.timer.is_none() {
            let duration = this.duration;
            this.timer = Some(Timer::after(duration));
        }
        let timer = this.timer.as_mut().expect("sleep timer");
        match Pin::new(timer).poll(context) {
            Poll::Ready(_instant) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> Future for SmolTimeout<T> {
    type Output = Result<T, TimeoutElapsed>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        // The task wins when both are ready, matching `futures_lite::future::or`.
        if let Poll::Ready(value) = this.task.as_mut().poll(context) {
            return Poll::Ready(Ok(value));
        }
        if this.timer.is_none() {
            let duration = this.duration;
            this.timer = Some(Timer::after(duration));
        }
        let timer = this.timer.as_mut().expect("timeout timer");
        match Pin::new(timer).poll(context) {
            Poll::Ready(_instant) => Poll::Ready(Err(TimeoutElapsed)),
            Poll::Pending => Poll::Pending,
        }
    }
}
