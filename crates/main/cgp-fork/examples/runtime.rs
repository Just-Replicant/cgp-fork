//! A context whose runtime is a Tokio handle: sleep, then spawn a task.
//!
//! Run with `cargo run -p cgp-fork --example runtime --features tokio`.

use core::time::Duration;

use cgp_fork::extra::runtime::{
    CanSleep, CanSpawn, RuntimeGetterComponent, RuntimeTypeProviderComponent, SleepWithRuntime,
    SleeperComponent, SpawnWithRuntime, TaskSpawnerComponent,
};
use cgp_fork::prelude::*;

#[derive(HasField)]
struct App {
    runtime: tokio::runtime::Handle,
}

delegate_components! {
    App {
        RuntimeTypeProviderComponent: UseType<tokio::runtime::Handle>,
        RuntimeGetterComponent: UseField<Symbol!("runtime")>,
        SleeperComponent: SleepWithRuntime,
        TaskSpawnerComponent: SpawnWithRuntime,
    }
}

fn main() {
    let runtime = cgp_fork::tokio::current_thread().expect("current-thread runtime");
    let app = App {
        runtime: runtime.handle().clone(),
    };

    runtime.block_on(async {
        app.spawn(async {
            println!("spawned");
        });
        app.sleep(Duration::from_millis(20)).await;
        println!("slept");
    });
}
