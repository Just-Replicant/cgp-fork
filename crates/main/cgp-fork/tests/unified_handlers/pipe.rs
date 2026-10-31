//! `Pipe` chaining handlers left to right (ported from `handlers/pipe_computers.rs`).

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{CanHandle, Handler, HandlerComponent};
use cgp_fork::prelude::*;

#[cgp_new_provider]
impl<Context, Code, Field> Handler<Context, Code, u64> for Multiply<Field>
where
    Context: HasField<Field, Value = u64>,
{
    type Output = u64;

    fn handle(context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input * context.get_field(PhantomData)
    }
}

#[cgp_new_provider]
impl<Context, Code, Field> Handler<Context, Code, u64> for Add<Field>
where
    Context: HasField<Field, Value = u64>,
{
    type Output = u64;

    fn handle(context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input + context.get_field(PhantomData)
    }
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, u64> for CheckedHalf
where
    Context: HasErrorType<Error = String>,
{
    type Output = Result<u64, String>;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: u64) -> Self::Output {
        if input.is_multiple_of(2) {
            Ok(input / 2)
        } else {
            Err(format!("{input} is odd"))
        }
    }
}

#[derive(HasField)]
pub struct MyContext {
    pub foo: u64,
    pub bar: u64,
    pub baz: u64,
}

delegate_components! {
    MyContext {
        ErrorTypeProviderComponent: UseType<String>,
        HandlerComponent:
            Pipe<
                Product! [
                    Multiply<Symbol!("foo")>,
                    Add<Symbol!("bar")>,
                    Multiply<Symbol!("baz")>,
                ]
            >,
    }
}

check_components! {
    MyContext {
        HandlerComponent: ((), u64),
    }
}

#[test]
fn pipe_runs_every_stage_in_order() {
    let context = MyContext {
        foo: 2,
        bar: 3,
        baz: 4,
    };

    assert_eq!(context.handle(PhantomData::<()>, 5), ((5 * 2) + 3) * 4);
}

#[test]
fn try_pipe_stops_at_the_first_error() {
    let context = MyContext {
        foo: 2,
        bar: 3,
        baz: 4,
    };

    type Halves = TryPipe<Product![CheckedHalf, CheckedHalf, PromoteOk<Add<Symbol!("bar")>>]>;

    assert_eq!(Halves::handle(&context, NoCode, 12), Ok(6));
    assert_eq!(
        Halves::handle(&context, NoCode, 6),
        Err("3 is odd".to_owned())
    );
    assert_eq!(TryPipe::<Nil>::handle(&context, NoCode, 1), Ok(1));
}
