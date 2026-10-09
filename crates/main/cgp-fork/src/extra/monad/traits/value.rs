/// The type this monad carries inside `Output`.
///
/// For [`OkMonadic`](crate::extra::monad::monadic::ok::OkMonadic), `Output` is `Result<T, E>` and
/// `Value` is `E`: the pipe threads the error, not the success. [`ErrMonadic`](crate::extra::monad::monadic::err::ErrMonadic)
/// threads `T` instead.
pub trait ContainsValue<Output> {
    type Value;
}
