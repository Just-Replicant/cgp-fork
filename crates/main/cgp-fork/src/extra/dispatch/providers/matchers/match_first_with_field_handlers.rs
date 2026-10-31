use crate::core::prelude::*;
use crate::extra::dispatch::providers::matchers::to_field_handlers::{
    HasFieldHandlers, MapExtractFirstFieldAndHandle,
};
use crate::extra::dispatch::{
    HandleFirstFieldValue, MatchFirstWithHandlers, MatchFirstWithHandlersMut,
    MatchFirstWithHandlersRef,
};
use crate::extra::handler::UseInputDelegate;

/// Dispatches `(enum, args)` by running `Provider` on the first matching field, keeping `args`.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, First>` (`cgp_fork::handler::MatchFields`)"
)]
pub type MatchFirstWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<Provider>>;

/// [`MatchFirstWithFieldHandlers`] that unwraps the field and passes `(value, args)` to `Provider`.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchValues<P, First>` (`cgp_fork::handler::MatchValues`)"
)]
pub type MatchFirstWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<HandleFirstFieldValue<Provider>>>;

/// [`MatchFirstWithFieldHandlers`] for a shared borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, First, Ref>` (`cgp_fork::handler::MatchFields`)"
)]
pub type MatchFirstWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<Provider>>;

/// [`MatchFirstWithValueHandlers`] for a shared borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchValues<P, First, Ref>` (`cgp_fork::handler::MatchValues`)"
)]
pub type MatchFirstWithValueHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<HandleFirstFieldValue<Provider>>>;

/// [`MatchFirstWithFieldHandlers`] for a mutable borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, First, Mut>` (`cgp_fork::handler::MatchFields`)"
)]
pub type MatchFirstWithFieldHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<Provider>>;

/// [`MatchFirstWithValueHandlers`] for a mutable borrow of the enum.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchValues<P, First, Mut>` (`cgp_fork::handler::MatchValues`)"
)]
pub type MatchFirstWithValueHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<HandleFirstFieldValue<Provider>>>;

/// Inner table for an owned `(input, args)` pair.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, First>` (`cgp_fork::handler::MatchFields`)"
)]
pub struct MatchFirstWithFieldHandlersInputs<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    MatchFirstWithFieldHandlersInputs<Provider> {
        (Input, Args): MatchFirstWithHandlers<Input::Handlers>
    }
}

/// Inner table for a shared borrow of the input beside `args`.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, First, Ref>` (`cgp_fork::handler::MatchFields`)"
)]
pub struct MatchFirstWithFieldHandlersInputsRef<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    MatchFirstWithFieldHandlersInputsRef<Provider> {
        <'a> (&'a Input, Args):
            MatchFirstWithHandlersRef<Input::Handlers>
    }
}

/// Inner table for a mutable borrow of the input beside `args`.
#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::MatchFields<P, First, Mut>` (`cgp_fork::handler::MatchFields`)"
)]
pub struct MatchFirstWithFieldHandlersInputsMut<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    MatchFirstWithFieldHandlersInputsMut<Provider> {
        <'a> (&'a mut Input, Args):
            MatchFirstWithHandlersMut<Input::Handlers>
    }
}
