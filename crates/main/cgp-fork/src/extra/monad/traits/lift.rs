/// Puts a plain value, or an already-monadic output, into this monad.
pub trait LiftValue<Value, Output> {
    type Output;

    fn lift_value(value: Value) -> Self::Output;

    fn lift_output(output: Output) -> Self::Output;
}
