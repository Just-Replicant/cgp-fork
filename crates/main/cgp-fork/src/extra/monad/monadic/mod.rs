/// Treats the `Err` side of `Result` as the value being piped.
pub mod err;
/// The identity monad: the output is the value.
pub mod ident;
pub mod ok;
