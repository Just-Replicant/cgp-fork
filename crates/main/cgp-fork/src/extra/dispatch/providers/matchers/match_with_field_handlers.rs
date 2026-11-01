use crate::core::prelude::*;
use crate::extra::dispatch::providers::matchers::to_field_handlers::{
    HasFieldHandlers, MapExtractFieldAndHandle,
};
use crate::extra::dispatch::{
    HandleFieldValue, MatchWithHandlers, MatchWithHandlersMut, MatchWithHandlersRef,
};
use crate::extra::handler::{
    AsyncComputerComponent, AsyncComputerRefComponent, ComputerComponent, ComputerRefComponent,
    HandlerComponent, HandlerRefComponent, PromoteRef, TryComputerComponent,
    TryComputerRefComponent, UseInputDelegate,
};

/// Dispatches an owned enum by running `Provider` on the matching field.
#[deprecated(since = "0.11.0", note = "use `cgp_fork::handler::MatchFields<P>`")]
pub type MatchWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<Provider>>;

/// [`MatchWithFieldHandlers`] that first unwraps the [`Field`](crate::core::field::types::Field) and passes the value to `Provider`.
#[deprecated(since = "0.11.0", note = "use `cgp_fork::handler::MatchValues<P>`")]
pub type MatchWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<HandleFieldValue<Provider>>>;

/// [`MatchWithFieldHandlers`] for a shared borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, All, Ref>` (`cgp_fork::handler::MatchFields`)"
)]
pub struct MatchWithFieldHandlersRef<Provider = UseContext>(pub PhantomData<Provider>);

/// [`MatchWithValueHandlers`] for a shared borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchValues<P, All, Ref>` (`cgp_fork::handler::MatchValues`)"
)]
pub struct MatchWithValueHandlersRef<Provider = UseContext>(pub PhantomData<Provider>);

/// [`MatchWithValueHandlers`] for a mutable borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchValues<P, All, Mut>` (`cgp_fork::handler::MatchValues`)"
)]
pub struct MatchWithValueHandlersMut<Provider = UseContext>(pub PhantomData<Provider>);

delegate_components! {
    <Provider>
    MatchWithFieldHandlersRef<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            AsyncComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsRef<Provider>>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<UseInputDelegate<MatchWithFieldHandlersInputsRef<PromoteRef<Provider>>>>,
    }
}

delegate_components! {
    <Provider>
    MatchWithValueHandlersRef<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            AsyncComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsRef<HandleFieldValue<Provider>>>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<UseInputDelegate<MatchWithFieldHandlersInputsRef<HandleFieldValue<PromoteRef<Provider>>>>>,
    }
}

delegate_components! {
    <Provider>
    MatchWithValueHandlersMut<Provider> {
        [
            ComputerComponent,
            TryComputerComponent,
            AsyncComputerComponent,
            HandlerComponent,
        ]:
            UseInputDelegate<MatchWithFieldHandlersInputsMut<HandleFieldValue<Provider>>>,
        [
            ComputerRefComponent,
            TryComputerRefComponent,
            AsyncComputerRefComponent,
            HandlerRefComponent,
        ]:
            PromoteRef<UseInputDelegate<MatchWithFieldHandlersInputsMut<HandleFieldValue<PromoteRef<Provider>>>>>,
    }
}

/// Inner table that sends an owned input to [`MatchWithHandlers`](crate::extra::dispatch::MatchWithHandlers) of that input's field handlers.
#[deprecated(since = "0.11.0", note = "use `cgp_fork::handler::MatchFields<P>`")]
pub struct MatchWithFieldHandlersInputs<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    MatchWithFieldHandlersInputs<Provider> {
        Input: MatchWithHandlers<Input::Handlers>
    }
}

/// [`MatchWithFieldHandlersInputs`] for a shared borrow.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, All, Ref>` (`cgp_fork::handler::MatchFields`)"
)]
pub struct MatchWithFieldHandlersInputsRef<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    MatchWithFieldHandlersInputsRef<Provider> {
        <'a> &'a Input:
            MatchWithHandlersRef<Input::Handlers>,
    }
}

/// [`MatchWithFieldHandlersInputs`] for a mutable borrow.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, All, Mut>` (`cgp_fork::handler::MatchFields`)"
)]
pub struct MatchWithFieldHandlersInputsMut<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFieldAndHandle<Provider>>, Provider>
    MatchWithFieldHandlersInputsMut<Provider> {
        <'a> &'a mut Input:
            MatchWithHandlersMut<Input::Handlers>
    }
}
