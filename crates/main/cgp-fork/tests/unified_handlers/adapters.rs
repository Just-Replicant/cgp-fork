//! `PromoteDeref`, `IgnoreInput` and `ReturnInput` (the 0.10 `PromoteRef`, `Promote` of a
//! `Producer`, and `ReturnInput`; ported from `handlers/return_input_and_promote_ref.rs`).

use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, &str> for CountChars {
    type Output = usize;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: &str) -> usize {
        input.chars().count()
    }
}

#[cgp_new_provider]
impl<Context, Code> AsyncHandler<Context, Code, &str> for CountCharsAsync {
    type Output = usize;

    async fn handle_async(_context: &Context, _code: PhantomData<Code>, input: &str) -> usize {
        input.chars().count()
    }
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, ()> for Seven {
    type Output = u8;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: ()) -> u8 {
        7
    }
}

pub struct App;

#[test]
fn promote_deref_lends_an_owned_input() {
    let owned = String::from("héllo");

    assert_eq!(
        PromoteDeref::<CountChars>::handle(&App, NoCode, owned.clone()),
        5
    );
    assert_eq!(
        block_on(PromoteDeref::<CountCharsAsync>::handle_async(
            &App, NoCode, owned
        )),
        5
    );
    // `Box<str>` derefs to `str` too.
    assert_eq!(
        PromoteDeref::<CountChars>::handle(&App, NoCode, Box::<str>::from("ab")),
        2
    );
}

#[test]
fn ignore_input_runs_a_producer() {
    assert_eq!(IgnoreInput::<Seven>::handle(&App, NoCode, "ignored"), 7);
    assert_eq!(
        block_on(IgnoreInput::<Promote<Seven>>::handle_async(
            &App, NoCode, 1.5
        )),
        7
    );
}

#[test]
fn return_input_is_the_identity() {
    assert_eq!(ReturnInput::handle(&App, NoCode, vec![1, 2]), vec![1, 2]);
    assert_eq!(block_on(ReturnInput::handle_async(&App, NoCode, 'x')), 'x');
    // An empty pipe is the identity as well.
    assert_eq!(Pipe::<Nil>::handle(&App, NoCode, 3_u8), 3);
}
