/// The provider that runs `Provider` inside this monad.
///
/// [`PipeMonadic`](crate::extra::monad::providers::PipeMonadic) asks the monad for this provider
/// so each step of a pipe can bind the previous output.
pub trait MonadicBind<Provider> {
    /// `Provider` wrapped so its output stays inside the monad.
    type Provider;
}
