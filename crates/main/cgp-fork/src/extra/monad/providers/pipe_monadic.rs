use crate::core::field::traits::MapFields;
use crate::core::prelude::*;
use crate::extra::handler::{
    AsyncComputerComponent, ComposeHandlers, ComputerComponent, HandlerComponent,
    TryComputerComponent, TryPromote,
};
use crate::extra::monad::monadic::err::ErrMonadic;
use crate::extra::monad::traits::{MonadicBind, MonadicTrans};

/// Pipes `Providers` through the monad `M`, binding each step's output into the next.
///
/// `Computer` and `AsyncComputer` bind directly. `TryComputer` and `Handler` are demoted with
/// [`TryPromote`](crate::extra::handler::TryPromote), bound under [`ErrMonadic`](crate::extra::monad::monadic::err::ErrMonadic),
/// and promoted back.
pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);

delegate_components! {
    <Provider, M, Providers: BindProviders<M, Provider = Provider>>
    PipeMonadic<M, Providers> {
        [
            ComputerComponent,
            AsyncComputerComponent,
        ]: Provider,
    }
}

// Support monadic piping of TryComputer by first demoting them to Computer,
// together with a monad transformer application of ErrMonadic to the base monad,
// compose them, and then TryPromote them again back into TryComputer.

delegate_components! {
    <
        Provider,
        M1: MonadicTrans<ErrMonadic, M = M2>,
        M2,
        ProvidersA: MapFields<TryPromoteProviders, Mapped = ProvidersB>,
        ProvidersB: BindProviders<M2, Provider = Provider>,
    >
    PipeMonadic<M1, ProvidersA> {
        TryComputerComponent: TryPromote<Provider>,
        HandlerComponent: TryPromote<Provider>,
    }
}

/// [`MapType`](crate::core::field::traits::MapType) that wraps each provider in [`TryPromote`](crate::extra::handler::TryPromote).
pub struct TryPromoteProviders;

impl MapType for TryPromoteProviders {
    type Map<Provider> = TryPromote<Provider>;
}

trait BindProviders<M> {
    type Provider;
}

impl<M, ProviderA, ProviderB, RestProviders, OutProviders> BindProviders<M>
    for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    Cons<ProviderB, RestProviders>: BindProviders<M, Provider = OutProviders>,
    M: MonadicBind<OutProviders>,
{
    type Provider = ComposeHandlers<ProviderA, M::Provider>;
}

impl<M, Provider> BindProviders<M> for Cons<Provider, Nil> {
    type Provider = Provider;
}
