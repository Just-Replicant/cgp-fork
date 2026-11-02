//! `Compat` adapters between the 0.10 family and the unified one (`compat-0.10`).

// Exercises the deprecated 0.10 handler family.
#![allow(deprecated)]

use cgp_fork::compat::{Compat, CompatProducer, CompatRef, CompatTry};
use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::extra::handler::{
    AsyncComputer, CanCompute, CanComputeRef, CanProduce, CanTryCompute, Computer, ComputerRef,
    ComputerRefComponent, ProducerComponent, TryComputer,
};
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

// A 0.10 provider of each kind.

#[cgp_new_provider]
impl<Context, Code> Computer<Context, Code, u64> for LegacyDouble {
    type Output = u64;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input * 2
    }
}

#[cgp_new_provider]
impl<Context, Code> TryComputer<Context, Code, u64> for LegacyNonZero
where
    Context: HasErrorType<Error = String>,
{
    type Output = u64;

    fn try_compute(
        _context: &Context,
        _code: PhantomData<Code>,
        input: u64,
    ) -> Result<u64, String> {
        if input == 0 {
            Err("zero".to_owned())
        } else {
            Ok(input)
        }
    }
}

#[cgp_new_provider]
impl<Context, Code> ComputerRef<Context, Code, String> for LegacyLen {
    type Output = usize;

    fn compute_ref(_context: &Context, _code: PhantomData<Code>, input: &String) -> usize {
        input.len()
    }
}

#[cgp_new_provider]
impl<Context, Code> AsyncComputer<Context, Code, u64> for LegacyIncAsync {
    type Output = u64;

    async fn compute_async(_context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input + 1
    }
}

// A unified provider.

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, u64> for Triple {
    type Output = u64;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input * 3
    }
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, &String> for FirstChar {
    type Output = Option<char>;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: &String) -> Option<char> {
        input.chars().next()
    }
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, u64> for Halve
where
    Context: HasErrorType<Error = String>,
{
    type Output = Result<u64, String>;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: u64) -> Self::Output {
        if input.is_multiple_of(2) {
            Ok(input / 2)
        } else {
            Err("odd".to_owned())
        }
    }
}

pub struct App;

delegate_components! {
    #[unchecked]
    App {
        ErrorTypeProviderComponent: UseType<String>,
    }
}

#[test]
fn legacy_providers_as_unified_handlers() {
    type Mixed = Pipe<Product![Compat<LegacyDouble>, Triple]>;
    assert_eq!(Mixed::handle(&App, NoCode, 1), 6);

    type Checked = TryPipe<Product![CompatTry<LegacyNonZero>, Halve]>;
    assert_eq!(Checked::handle(&App, NoCode, 4), Ok(2));
    assert_eq!(Checked::handle(&App, NoCode, 0), Err("zero".to_owned()));

    assert_eq!(
        CompatRef::<LegacyLen>::handle(&App, NoCode, &"abc".to_owned()),
        3
    );
    assert_eq!(
        block_on(Compat::<LegacyIncAsync>::handle_async(&App, NoCode, 1)),
        2
    );
}

/// A context still consumed through the 0.10 traits, wired to unified providers.
pub struct Legacy;

delegate_components! {
    #[unchecked]
    Legacy {
        ErrorTypeProviderComponent: UseType<String>,
        ComputerComponent: Compat<Triple>,
        TryComputerComponent: Compat<Halve>,
        ComputerRefComponent: Compat<FirstChar>,
        ProducerComponent: Compat<IgnoreInput<ProduceSeven>>,
    }
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, ()> for ProduceSeven {
    type Output = u8;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: ()) -> u8 {
        7
    }
}

#[test]
fn unified_handlers_behind_legacy_components() {
    let legacy = Legacy;

    assert_eq!(legacy.compute(NoCode, 2_u64), 6);
    assert_eq!(legacy.try_compute(NoCode, 3_u64), Err("odd".to_owned()));
    assert_eq!(legacy.compute_ref(NoCode, &"xyz".to_owned()), Some('x'));
    assert_eq!(legacy.produce(NoCode), 7);
}

#[test]
fn compat_producer_ignores_the_input() {
    #[cgp_new_provider]
    impl<Context, Code> cgp_fork::extra::handler::Producer<Context, Code> for LegacyTen {
        type Output = u8;

        fn produce(_context: &Context, _code: PhantomData<Code>) -> u8 {
            10
        }
    }

    assert_eq!(
        CompatProducer::<LegacyTen>::handle(&App, NoCode, "anything"),
        10
    );
}
