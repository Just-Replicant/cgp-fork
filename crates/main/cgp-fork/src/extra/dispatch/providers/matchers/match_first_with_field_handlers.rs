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
pub type MatchFirstWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<Provider>>;

/// [`MatchFirstWithFieldHandlers`] that unwraps the field and passes `(value, args)` to `Provider`.
pub type MatchFirstWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputs<HandleFirstFieldValue<Provider>>>;

/// [`MatchFirstWithFieldHandlers`] for a shared borrow of the enum.
pub type MatchFirstWithFieldHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<Provider>>;

/// [`MatchFirstWithValueHandlers`] for a shared borrow of the enum.
pub type MatchFirstWithValueHandlersRef<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsRef<HandleFirstFieldValue<Provider>>>;

/// [`MatchFirstWithFieldHandlers`] for a mutable borrow of the enum.
pub type MatchFirstWithFieldHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<Provider>>;

/// [`MatchFirstWithValueHandlers`] for a mutable borrow of the enum.
pub type MatchFirstWithValueHandlersMut<Provider = UseContext> =
    UseInputDelegate<MatchFirstWithFieldHandlersInputsMut<HandleFirstFieldValue<Provider>>>;

/// Inner table for an owned `(input, args)` pair.
pub struct MatchFirstWithFieldHandlersInputs<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    MatchFirstWithFieldHandlersInputs<Provider> {
        (Input, Args): MatchFirstWithHandlers<Input::Handlers>
    }
}

/// Inner table for a shared borrow of the input beside `args`.
pub struct MatchFirstWithFieldHandlersInputsRef<Provider>(pub PhantomData<Provider>);

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    MatchFirstWithFieldHandlersInputsRef<Provider> {
        <'a> (&'a Input, Args):
            MatchFirstWithHandlersRef<Input::Handlers>
    }
}

delegate_components! {
    <Input: HasFieldHandlers<MapExtractFirstFieldAndHandle<Provider>>, Args, Provider>
    new MatchFirstWithFieldHandlersInputsMut<Provider> {
        <'a> (&'a mut Input, Args):
            MatchFirstWithHandlersMut<Input::Handlers>
    }
}
