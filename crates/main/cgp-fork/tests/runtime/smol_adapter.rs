//! `SmolRuntime` as `AsyncRuntime`: sleep, spawn, and timeout, both through
//! `HasAsyncRuntime` and through the wired providers.
//!
//! `App` is `Send` and not `Sync`. Sleep and timeout futures are `Send + 'static`,
//! so they outlive `App` and do not need `App: Sync`.

#![cfg(feature = "smol")]

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
use cgp_fork::smol::SmolRuntime;

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
    runtime: SmolRuntime,
    #[allow(dead_code)]
    not_sync: PhantomData<Cell<()>>,
}

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<AppError>,
        ErrorRaiserComponent: RaiseFrom,
        RuntimeTypeProviderComponent: UseType<SmolRuntime>,
        RuntimeGetterComponent: UseField<Symbol!("runtime")>,
        SleeperComponent: SleepWithRuntime,
        TaskSpawnerComponent: SpawnWithRuntime,
        TimeoutRunnerComponent: TimeoutWithRuntime,
    }
}

/// Pending forever, and sends when dropped. Dropping a smol timeout drops this task with it.
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
fn smol_runtime_sleep_waits() {
    let runtime = SmolRuntime::new();
    let started = Instant::now();
    runtime.block_on(runtime.sleep_for(Duration::from_millis(100)));
    assert_slept(started);
}

#[test]
fn dropping_a_timeout_drops_the_task() {
    let runtime = SmolRuntime::new();
    let (sender, mut receiver) = futures::channel::oneshot::channel();
    let timeout = runtime.timeout_after(
        Duration::from_secs(30),
        NotifyOnDrop {
            sender: Some(sender),
        },
    );
    drop(timeout);
    assert_eq!(receiver.try_recv(), Ok(Some(())));
}

#[test]
fn smol_sleep_spawn_and_timeout() {
    let runtime = SmolRuntime::new();
    let app = App {
        runtime: runtime.clone(),
        not_sync: PhantomData,
    };

    // Created before `app` is dropped: the futures must not borrow it.
    // Spawn still needs `runtime`, which shares the executor, to be polled.
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
