/// Stacks this monad transformer on `M`.
///
/// [`OkMonadic`](crate::extra::monad::monadic::ok::OkMonadic) stacked on `M` is `OkMonadicTrans<M>`.
/// [`IdentMonadic`](crate::extra::monad::monadic::ident::IdentMonadic) stacked on `M` is `M`.
pub trait MonadicTrans<M> {
    /// This transformer applied to `M`.
    type M;
}
