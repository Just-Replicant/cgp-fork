//! `ReturnInput` and `PromoteRef` across the computation family.
//!
//! `ReturnInput` hands its input back unchanged from every family member
//! (`compute`, `try_compute`, `compute_async`, `handle`). `PromoteRef` bridges
//! owned and borrowed inputs in both directions: wrapping a `*Ref` provider, it
//! accepts an owned `Deref` input (here `Box<String>`) and passes `input.deref()`; wrapping an owned
//! provider over `&Input`, it implements the `*Ref` member for `Input`.
//!
//! The error type wiring on `App` is incidental scaffolding.
//!
//! See cgp-knowledge-base-fork/cgp/reference/components/computer.md.

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::extra::handler::{
    AsyncComputerRef, ComputerRef, HandlerRef, PromoteRef, ReturnInput, TryComputerRef,
};
use cgp_fork::prelude::*;
use futures::executor::block_on;

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseType<String>,
    }
}

/// A by-reference provider: the length of a borrowed `String`, in every `*Ref` member.
///
/// The `*Ref` traits take a sized `Input`, so the target is `String` (reached
/// through a `Box<String>`) rather than `str`.
pub struct StrLen;

#[cgp_provider]
impl<Context, Code> ComputerRef<Context, Code, String> for StrLen {
    type Output = usize;

    fn compute_ref(_context: &Context, _code: PhantomData<Code>, input: &String) -> usize {
        input.len()
    }
}

#[cgp_provider]
impl<Context, Code> TryComputerRef<Context, Code, String> for StrLen
where
    Context: HasErrorType,
{
    type Output = usize;

    fn try_compute_ref(
        _context: &Context,
        _code: PhantomData<Code>,
        input: &String,
    ) -> Result<usize, Context::Error> {
        Ok(input.len())
    }
}

#[cgp_provider]
impl<Context, Code> AsyncComputerRef<Context, Code, String> for StrLen {
    type Output = usize;

    async fn compute_async_ref(
        _context: &Context,
        _code: PhantomData<Code>,
        input: &String,
    ) -> usize {
        input.len()
    }
}

#[cgp_provider]
impl<Context, Code> HandlerRef<Context, Code, String> for StrLen
where
    Context: HasErrorType,
{
    type Output = usize;

    async fn handle_ref(
        _context: &Context,
        _code: PhantomData<Code>,
        input: &String,
    ) -> Result<usize, Context::Error> {
        Ok(input.len())
    }
}

/// An owned provider over `&Vec<u8>`, in every owned member.
pub struct VecSum;

#[cgp_provider]
impl<Context, Code> Computer<Context, Code, &Vec<u8>> for VecSum {
    type Output = u32;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: &Vec<u8>) -> u32 {
        input.iter().map(|b| u32::from(*b)).sum()
    }
}

#[cgp_provider]
impl<Context, Code> TryComputer<Context, Code, &Vec<u8>> for VecSum
where
    Context: HasErrorType,
{
    type Output = u32;

    fn try_compute(
        _context: &Context,
        _code: PhantomData<Code>,
        input: &Vec<u8>,
    ) -> Result<u32, Context::Error> {
        Ok(input.iter().map(|b| u32::from(*b)).sum())
    }
}

#[cgp_provider]
impl<Context, Code> AsyncComputer<Context, Code, &Vec<u8>> for VecSum {
    type Output = u32;

    async fn compute_async(_context: &Context, _code: PhantomData<Code>, input: &Vec<u8>) -> u32 {
        input.iter().map(|b| u32::from(*b)).sum()
    }
}

#[cgp_provider]
impl<Context, Code> Handler<Context, Code, &Vec<u8>> for VecSum
where
    Context: HasErrorType,
{
    type Output = u32;

    async fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        input: &Vec<u8>,
    ) -> Result<u32, Context::Error> {
        Ok(input.iter().map(|b| u32::from(*b)).sum())
    }
}

#[test]
fn test_return_input_in_every_member() {
    let app = App;
    let code = PhantomData::<()>;

    assert_eq!(ReturnInput::compute(&app, code, 7u8), 7);
    assert_eq!(ReturnInput::try_compute(&app, code, 7u8), Ok(7));
    assert_eq!(block_on(ReturnInput::compute_async(&app, code, 7u8)), 7);
    assert_eq!(block_on(ReturnInput::handle(&app, code, 7u8)), Ok(7));
}

#[test]
fn test_promote_ref_derefs_an_owned_input() {
    let app = App;
    let code = PhantomData::<()>;
    let owned = || Box::new(String::from("hello"));

    assert_eq!(PromoteRef::<StrLen>::compute(&app, code, owned()), 5);
    assert_eq!(
        PromoteRef::<StrLen>::try_compute(&app, code, owned()),
        Ok(5)
    );
    assert_eq!(
        block_on(PromoteRef::<StrLen>::compute_async(&app, code, owned())),
        5
    );
    assert_eq!(
        block_on(PromoteRef::<StrLen>::handle(&app, code, owned())),
        Ok(5)
    );
}

#[test]
fn test_promote_ref_borrows_for_an_owned_provider() {
    let app = App;
    let code = PhantomData::<()>;
    let bytes = vec![1u8, 2, 3];

    assert_eq!(PromoteRef::<VecSum>::compute_ref(&app, code, &bytes), 6);
    assert_eq!(
        PromoteRef::<VecSum>::try_compute_ref(&app, code, &bytes),
        Ok(6)
    );
    assert_eq!(
        block_on(PromoteRef::<VecSum>::compute_async_ref(&app, code, &bytes)),
        6
    );
    assert_eq!(
        block_on(PromoteRef::<VecSum>::handle_ref(&app, code, &bytes)),
        Ok(6)
    );
}
