use crate::extra::monad::monadic::ok::OkMonadic;
use crate::extra::monad::providers::PipeMonadic;

/// Runs `Providers` in order through [`OkMonadic`](crate::extra::monad::monadic::ok::OkMonadic).
///
/// Each provider returns `Result`. The first `Ok` wins; an `Err` continues with the next provider.
pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
