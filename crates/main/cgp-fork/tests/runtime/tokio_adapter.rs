//! Tokio `Handle` as `AsyncRuntime`: sleep, spawn, and timeout, both through
//! `HasAsyncRuntime` and through the wired providers.
//!
//! `App` is `Send` and not `Sync`. Sleep and timeout futures are `Send + 'static`,
//! so they outlive `App` and do not need `App: Sync`.

#![cfg(feature = "tokio")]

use core::cell::Cell;
use core::future::{Future, pending};
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use std::time::Instant;

use cgp_fork::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp_fork::extra::error::RaiseFrom;
use cgp_fork::extra::runtime::{
    AsyncRuntime, CanSleep, CanSpawn, CanTimeout, HasAsyncRuntime, RuntimeGetterComponent,
    RuntimeTypeProviderComponent, SleepWithRuntime, SleeperComponent, SpawnWithRuntime,
    TaskSpawnerComponent, TimeoutElapsed, TimeoutRunnerComponent, TimeoutWithRuntime,
};
use cgp_fork::prelude::*;
use cgp_fork::tokio::{current_thread, multi_thread};

#[derive(Debug, PartialEq, Eq)]
struct AppError;

impl From<TimeoutElapsed> for AppError {
    fn from(_elapsed: TimeoutElapsed) -> Self {
        Self
    }
}

/// `Cell` makes this `!Sync`. The fields are read by the wiring, not by hand.
#[derive(HasField)]
struct App {
    runtime: tokio::runtime::Handle,
    #[allow(dead_code)]
    not_sync: PhantomData<Cell<()>>,
}

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<AppError>,
        ErrorRaiserComponent: RaiseFrom,
        RuntimeTypeProviderComponent: UseType<tokio::runtime::Handle>,
        RuntimeGetterComponent: UseField<Symbol!("runtime")>,
        SleeperComponent: SleepWithRuntime,
        TaskSpawnerComponent: SpawnWithRuntime,
        TimeoutRunnerComponent: TimeoutWithRuntime,
    }
}

fn app(runtime: &tokio::runtime::Runtime) -> App {
    App {
        runtime: runtime.handle().clone(),
        not_sync: PhantomData,
    }
}

/// Pending forever, and sends when dropped. Used to observe that a timeout future drops its task.
struct NotifyOnDrop {
    sender: Option<futures::channel::oneshot::Sender<()>>,
}

impl Drop for NotifyOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(());
        }
    }
}

impl Future for NotifyOnDrop {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn assert_slept(started: Instant) {
    let elapsed = started.elapsed();
    assert!(
        elapsed >= Duration::from_millis(70),
        "sleep returned after {elapsed:?}, under the 100ms request"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "sleep returned after {elapsed:?}"
    );
}

#[test]
fn tokio_handle_sleep_waits() {
    let runtime = current_thread().expect("current-thread runtime");
    let started = Instant::now();
    runtime.block_on(runtime.handle().sleep_for(Duration::from_millis(100)));
    assert_slept(started);
}

#[test]
fn tokio_sleep_spawn_and_timeout() {
    let runtime = current_thread().expect("current-thread runtime");
    let app = app(&runtime);

    // Created before `app` is dropped: the futures must not borrow it.
    let sleep = app.sleep(Duration::from_millis(100));
    let sleep_for = app.sleep_for(Duration::from_millis(100));
    let (sender, receiver) = futures::channel::oneshot::channel();
    app.spawn(async move {
        let _ = sender.send(());
    });
    let value = app.timeout(Duration::from_secs(2), async { 7u8 });
    let timed_out = app.timeout(Duration::from_millis(40), pending::<u8>());
    let timed_out_direct = app.timeout_after(Duration::from_millis(40), pending::<u8>());
    drop(app);

    runtime.block_on(async {
        let started = Instant::now();
        sleep.await;
        assert_slept(started);

        let started = Instant::now();
        sleep_for.await;
        assert_slept(started);

        assert_eq!(receiver.await, Ok(()));
        assert_eq!(value.await, Ok(7));
        assert_eq!(timed_out.await, Err(AppError));
        assert_eq!(timed_out_direct.await, Err(TimeoutElapsed));
    });
}

#[test]
fn dropping_a_polled_timeout_aborts_the_task() {
    let runtime = current_thread().expect("current-thread runtime");
    let (sender, receiver) = futures::channel::oneshot::channel();
    // One poll spawns the task. Abort is applied when that task is next polled, so the drop
    // and the following yield stay inside `block_on`.
    runtime.block_on(async {
        // `TokioTimeout` is `Unpin`, so this owns the future. `pin!` would keep it alive
        // until the end of the block, and `drop` would not abort the task.
        let mut timeout = runtime.handle().timeout_after(
            Duration::from_secs(30),
            NotifyOnDrop {
                sender: Some(sender),
            },
        );
        std::future::poll_fn(|context| {
            let _ = Pin::new(&mut timeout).poll(context);
            Poll::Ready(())
        })
        .await;
        drop(timeout);
        tokio::task::yield_now().await;
        assert_eq!(
            tokio::time::timeout(Duration::from_millis(200), receiver).await,
            Ok(Ok(()))
        );
    });
}

#[test]
fn tokio_multi_thread_runs_a_spawned_task() {
    let runtime = multi_thread().expect("multi-thread runtime");
    let app = app(&runtime);
    let (sender, receiver) = futures::channel::oneshot::channel();
    app.spawn(async move {
        let _ = sender.send(1u8);
    });
    let received = app.timeout(Duration::from_secs(2), receiver);
    drop(app);

    assert_eq!(runtime.block_on(received), Ok(Ok(1)));
}
