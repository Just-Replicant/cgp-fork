//! `#[cgp_async(Send)]` makes the desugared future `Send`.
//!
//! `CanRun` stays unbound. The component that is handed to a `Send` executor is a
//! separate trait declared with the flag, implemented for the concrete context by
//! forwarding to `run`. The compiler proves `Send` there, so the abstract providers
//! never mention it.

use core::convert::Infallible;
use core::future::Future;

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::extra::run::{CanRun, Runner, RunnerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

fn dummy_spawn<F>(_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
}

#[cgp_component(SpawnRunner)]
#[cgp_async(Send)]
#[use_type(HasErrorType.Error)]
pub trait CanSpawnRun<Code> {
    /// Runs `Code` and is `Send`.
    async fn spawn_run(&self, _code: PhantomData<Code>) -> Result<(), Error>;
}

#[cgp_new_provider(RunnerComponent)]
impl<Context, Code> Runner<Context, Code> for Finish
where
    Context: HasErrorType,
{
    async fn run(_context: &Context, _code: PhantomData<Code>) -> Result<(), Context::Error> {
        Ok(())
    }
}

#[cgp_new_provider(RunnerComponent)]
impl<Context, Code, InCode> Runner<Context, Code> for SpawnAndRun<InCode>
where
    Context: 'static + Send + Clone + CanSpawnRun<InCode>,
{
    async fn run(context: &Context, _code: PhantomData<Code>) -> Result<(), Context::Error> {
        let context = context.clone();

        dummy_spawn(async move {
            let _ = context.spawn_run(PhantomData).await;
        });

        Ok(())
    }
}

#[derive(Clone)]
struct App;

struct ActionA;
struct ActionB;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<Infallible>,
        RunnerComponent: UseDelegate<new AppRunnerComponents {
            ActionA: Finish,
            ActionB: SpawnAndRun<ActionA>,
        }>,
    }
}

#[cgp_provider]
impl SpawnRunner<App, ActionA> for App {
    async fn spawn_run(context: &App, code: PhantomData<ActionA>) -> Result<(), Infallible> {
        context.run(code).await
    }
}

#[test]
fn cgp_async_send_can_be_spawned() {
    block_on(App.run(PhantomData::<ActionB>)).unwrap();
}
