use core::marker::PhantomData;

use crate::core::prelude::*;
use crate::extra::handler::ComposeHandlers;

/// Folds a [`Cons`](crate::core::field::types::Cons) list of providers into one [`ComposeHandlers`] chain.
///
/// A one-element list is that provider. A longer list composes from the head toward the tail.
pub struct PipeHandlers<Providers>(pub PhantomData<Providers>);

delegate_components! {
    <Component, Provider, Providers: ComposeProviders<Provider = Provider>>
    PipeHandlers<Providers> {
        Component: Provider,
    }
}

trait ComposeProviders {
    type Provider;
}

impl<ProviderA, ProviderB, RestProviders, OutProviders> ComposeProviders
    for Cons<ProviderA, Cons<ProviderB, RestProviders>>
where
    Cons<ProviderB, RestProviders>: ComposeProviders<Provider = OutProviders>,
{
    type Provider = ComposeHandlers<ProviderA, OutProviders>;
}

impl<Provider> ComposeProviders for Cons<Provider, Nil> {
    type Provider = Provider;
}
