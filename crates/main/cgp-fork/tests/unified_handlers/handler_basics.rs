//! One `Handler` component for owned, borrowed, fallible and input-free handlers.

use cgp_fork::core::component::UseDelegate;
use cgp_fork::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{CanHandle, Handler, HandlerComponent};
use cgp_fork::prelude::*;

pub struct Double;
pub struct Length;
pub struct Parse;
pub struct Answer;

#[cgp_new_provider]
impl<Context> Handler<Context, Double, u64> for DoubleU64 {
    type Output = u64;

    fn handle(_context: &Context, _code: PhantomData<Double>, input: u64) -> u64 {
        input * 2
    }
}

// A borrowed input is `Input = &'a T`; no `HandlerRef` component is needed.
#[cgp_new_provider]
impl<Context> Handler<Context, Length, &str> for StrLength {
    type Output = usize;

    fn handle(_context: &Context, _code: PhantomData<Length>, input: &str) -> usize {
        input.len()
    }
}

// A fallible handler returns `Result`; no `TryComputer` component is needed.
#[cgp_new_provider]
impl<Context> Handler<Context, Parse, &str> for ParseU64
where
    Context: CanRaiseError<core::num::ParseIntError>,
{
    type Output = Result<u64, Context::Error>;

    fn handle(_context: &Context, _code: PhantomData<Parse>, input: &str) -> Self::Output {
        input.parse().map_err(Context::raise_error)
    }
}

// A producer has `Input = ()`; no `Producer` component is needed.
#[cgp_new_provider]
impl<Context> Handler<Context, Answer, ()> for ProduceAnswer {
    type Output = u64;

    fn handle(_context: &Context, _code: PhantomData<Answer>, _input: ()) -> u64 {
        42
    }
}

pub struct App;

pub struct RaiseParseError;

#[cgp_provider]
impl<Context> ErrorRaiser<Context, core::num::ParseIntError> for RaiseParseError
where
    Context: HasErrorType<Error = String>,
{
    fn raise_error(error: core::num::ParseIntError) -> String {
        error.to_string()
    }
}

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<String>,
        ErrorRaiserComponent: RaiseParseError,
        HandlerComponent: UseDelegate<new AppHandlers {
            Double: DoubleU64,
            Length: StrLength,
            Parse: ParseU64,
            Answer: ProduceAnswer,
        }>,
    }
}

check_components! {
    App {
        HandlerComponent: [
            (Double, u64),
            (Answer, ()),
        ],
    }
}

#[test]
fn one_component_covers_every_shape() {
    let app = App;

    assert_eq!(app.handle(PhantomData::<Double>, 21), 42);
    assert_eq!(app.handle(PhantomData::<Length>, "four"), 4);
    assert_eq!(app.handle(PhantomData::<Parse>, "7"), Ok(7));
    assert!(app.handle(PhantomData::<Parse>, "seven").is_err());
    assert_eq!(app.handle(PhantomData::<Answer>, ()), 42);
}

/// `UseField` runs the handler of a context field.
#[derive(HasField)]
pub struct Outer {
    pub inner: Inner,
}

pub struct Inner;

delegate_components! {
    Inner {
        HandlerComponent: DoubleU64,
    }
}

delegate_components! {
    Outer {
        HandlerComponent: UseField<Symbol!("inner")>,
    }
}

#[test]
fn use_field_delegates_to_the_field() {
    assert_eq!(Outer { inner: Inner }.handle(PhantomData::<Double>, 5), 10);
}

/// `UseInputDelegate` picks the provider by the input type.
pub struct ByInput;

delegate_components! {
    ByInput {
        HandlerComponent: UseInputDelegate<new ByInputHandlers {
            u64: DoubleU64,
        }>,
    }
}

#[test]
fn use_input_delegate_dispatches_on_the_input() {
    assert_eq!(ByInput.handle(PhantomData::<Double>, 4), 8);
}
