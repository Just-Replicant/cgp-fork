/// Puts a plain value, or an already-monadic output, into this monad.
pub trait LiftValue<Value, Output> {
    /// The monadic type. For [`OkMonadic`](crate::extra::monad::monadic::ok::OkMonadic) this is `Result<T, Value>`.
    type Output;

    /// Wraps `value` as the monadic context (the `Err` of `Ok`, the `Ok` of `Err`).
    fn lift_value(value: Value) -> Self::Output;

    fn lift_output(output: Output) -> Self::Output;
}
