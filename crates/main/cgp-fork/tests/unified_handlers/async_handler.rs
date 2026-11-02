//! `AsyncHandler`, and `Promote`/`PromoteOk` lifting synchronous and infallible handlers.

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{CanHandleAsync, Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

#[cgp_new_provider]
impl<Context, Code> AsyncHandler<Context, Code, u64> for AddOneAsync {
    type Output = u64;

    async fn handle_async(_context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input + 1
    }
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, u64> for Square {
    type Output = u64;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input * input
    }
}

// An async fallible handler: the 0.10 `Handler` is `AsyncHandler` returning `Result`.
#[cgp_new_provider]
impl<Context, Code> AsyncHandler<Context, Code, u64> for NonZero
where
    Context: HasErrorType<Error = String>,
{
    type Output = Result<u64, String>;

    async fn handle_async(
        _context: &Context,
        _code: PhantomData<Code>,
        input: u64,
    ) -> Self::Output {
        if input == 0 {
            Err("zero".to_owned())
        } else {
            Ok(input)
        }
    }
}

pub struct App;

delegate_components! {
    #[unchecked]
    App {
        ErrorTypeProviderComponent: UseType<String>,
        AsyncHandlerComponent: Pipe<Product![AddOneAsync, Promote<Square>]>,
    }
}

#[test]
fn async_pipe_mixes_async_and_promoted_handlers() {
    assert_eq!(block_on(App.handle_async(NoCode, 2)), 9);
}

#[test]
fn promote_ok_and_try_pipe_short_circuit() {
    type Checked = TryPipe<Product![PromoteOk<AddOneAsync>, NonZero, PromoteOk<Promote<Square>>]>;

    assert_eq!(block_on(Checked::handle_async(&App, NoCode, 2)), Ok(9));

    type FailsFirst = TryPipe<Product![NonZero, PromoteOk<AddOneAsync>]>;

    assert_eq!(
        block_on(FailsFirst::handle_async(&App, NoCode, 0)),
        Err("zero".to_owned())
    );
}

#[test]
fn promote_ok_wraps_a_sync_handler() {
    assert_eq!(PromoteOk::<Square>::handle(&App, NoCode, 3), Ok(9));
}
