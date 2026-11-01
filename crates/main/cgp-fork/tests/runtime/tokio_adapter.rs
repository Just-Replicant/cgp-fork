//! Tokio `Handle` as `AsyncRuntime`: sleep, spawn, and timeout, both through
//! `HasAsyncRuntime` and through the wired providers.

#![cfg(feature = "tokio")]

use core::future::pending;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use std::sync::Arc;
use std::time::Instant;

use cgp_fork::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp_fork::extra::error::ReturnError;
use cgp_fork::extra::runtime::{
    CanSleep, CanSpawn, CanTimeout, HasAsyncRuntime, RuntimeGetterComponent,
    RuntimeTypeProviderComponent, SleepWithRuntime, SleeperComponent, SpawnWithRuntime,
    TaskSpawnerComponent, TimeoutElapsed, TimeoutRunnerComponent, TimeoutWithRuntime,
};
use cgp_fork::prelude::*;
use cgp_fork::tokio::current_thread;

#[derive(HasField)]
struct App {
    runtime: tokio::runtime::Handle,
}

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<TimeoutElapsed>,
        ErrorRaiserComponent: ReturnError,
        RuntimeTypeProviderComponent: UseType<tokio::runtime::Handle>,
        RuntimeGetterComponent: UseField<Symbol!("runtime")>,
        SleeperComponent: SleepWithRuntime,
        TaskSpawnerComponent: SpawnWithRuntime,
        TimeoutRunnerComponent: TimeoutWithRuntime,
    }
}

#[test]
fn tokio_sleep_spawn_and_timeout() {
    let runtime = current_thread().expect("current-thread runtime");
    let app = App {
        runtime: runtime.handle().clone(),
    };

    runtime.block_on(async {
        let started = Instant::now();
        app.sleep(Duration::from_millis(40)).await;
        assert!(started.elapsed() >= Duration::from_millis(20));

        let started = Instant::now();
        app.sleep_for(Duration::from_millis(40)).await;
        assert!(started.elapsed() >= Duration::from_millis(20));

        let flag = Arc::new(AtomicBool::new(false));
        let spawned = Arc::clone(&flag);
        app.spawn(async move {
            spawned.store(true, Ordering::SeqCst);
        });
        app.sleep(Duration::from_millis(20)).await;
        assert!(flag.load(Ordering::SeqCst));

        assert_eq!(
            app.timeout(Duration::from_secs(2), async { 7 }).await,
            Ok(7)
        );
        assert_eq!(
            app.timeout(Duration::from_millis(20), pending::<u8>())
                .await,
            Err(TimeoutElapsed)
        );
        assert_eq!(
            app.timeout_after(Duration::from_millis(20), pending::<u8>())
                .await,
            Err(TimeoutElapsed)
        );
    });
}
