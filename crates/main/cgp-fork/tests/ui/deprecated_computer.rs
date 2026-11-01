//! The 0.10 `Computer` component is deprecated; the warning names the unified `Handler`.
#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_new_provider]
impl<Context, Code> Computer<Context, Code, u64> for Double {
    type Output = u64;

    fn compute(_context: &Context, _code: PhantomData<Code>, input: u64) -> u64 {
        input * 2
    }
}

fn main() {}
