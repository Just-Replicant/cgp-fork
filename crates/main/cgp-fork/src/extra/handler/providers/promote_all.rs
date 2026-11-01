use crate::core::prelude::*;
use crate::extra::handler::{
    AsyncComputerComponent, AsyncComputerRefComponent, ComputerComponent, ComputerRefComponent,
    HandlerComponent, HandlerRefComponent, Promote, PromoteAsync, PromoteRef, TryComputerComponent,
    TryComputerRefComponent, TryPromote,
};

/// Promotes a `Computer` provider to the ref, try, async, and handler components.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::compat::Compat<P>`, or wire `AsyncHandlerComponent: cgp_fork::handler::Promote<P>`"
)]
pub struct PromoteComputer<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    PromoteComputer<Provider> {
        ComputerRefComponent: PromoteRef<Provider>,
        TryComputerComponent: Promote<Provider>,
        TryComputerRefComponent: PromoteRef<Provider>,
        AsyncComputerComponent: PromoteAsync<Provider>,
        AsyncComputerRefComponent: PromoteRef<Provider>,
        HandlerComponent: PromoteAsync<Provider>,
        HandlerRefComponent: PromoteRef<Provider>,
    }
}

/// Promotes a `TryComputer`, and forwards the other handler components through [`PromoteComputer`].
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::compat::Compat<P>`, or wire `AsyncHandlerComponent: cgp_fork::handler::Promote<P>`"
)]
pub struct PromoteTryComputer<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    PromoteTryComputer<Provider> {
        TryComputerComponent: TryPromote<Provider>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerComponent,
            AsyncComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ] ->
            PromoteComputer<Provider>,
    }
}

/// Promotes a `Producer` to `Computer`, then through [`PromoteComputer`].
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::compat::Compat<P>`, or wire `AsyncHandlerComponent: cgp_fork::handler::Promote<P>`"
)]
pub struct PromoteProducer<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    PromoteProducer<Provider> {
        ComputerComponent: Promote<Provider>,
        [
            ComputerRefComponent,
            TryComputerComponent,
            TryComputerRefComponent,
            AsyncComputerComponent,
            AsyncComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ] ->
            PromoteComputer<Provider>,
    }
}

/// Promotes an `AsyncComputer` to the ref and handler components.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::compat::Compat<P>`, or wire `AsyncHandlerComponent` to the unified `AsyncHandler` directly"
)]
pub struct PromoteAsyncComputer<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    PromoteAsyncComputer<Provider> {
        AsyncComputerRefComponent: PromoteRef<Provider>,
        HandlerComponent: Promote<Provider>,
        HandlerRefComponent: PromoteRef<Provider>,
    }
}

/// Promotes a `Handler`, and forwards the ref components through [`PromoteAsyncComputer`].
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::compat::Compat<P>`, or wire `AsyncHandlerComponent` to the unified `AsyncHandler` directly"
)]
pub struct PromoteHandler<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    PromoteHandler<Provider> {
        HandlerComponent: TryPromote<Provider>,
        [
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ] ->
            PromoteAsyncComputer<Provider>,
    }
}
