use crate::core::prelude::*;
use crate::extra::dispatch::{ExtractFieldAndHandle, ExtractFirstFieldAndHandle};

#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::VariantHandlers<P, S>`"
)]
pub trait HasFieldHandlers<M> {
    type Handlers;
}

#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::VariantHandlers<P, S>`"
)]
pub trait MapFieldHandler {
    type FieldHandler<Tag>;
}

impl<Context, Fields, M> HasFieldHandlers<M> for Context
where
    Context: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<M>,
{
    type Handlers = Fields::Handlers;
}

#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::VariantHandlers<P, S>`"
)]
pub trait ToFieldHandlers<M> {
    type Handlers;
}

impl<Tag, Value, RestFields, M> ToFieldHandlers<M> for Either<Field<Tag, Value>, RestFields>
where
    M: MapFieldHandler,
    RestFields: ToFieldHandlers<M>,
{
    type Handlers = Cons<M::FieldHandler<Tag>, RestFields::Handlers>;
}

impl<Provider> ToFieldHandlers<Provider> for Void {
    type Handlers = Nil;
}

#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::VariantHandlers<P, All>`"
)]
pub struct MapExtractFieldAndHandle<Provider>(pub PhantomData<Provider>);

impl<Provider> MapFieldHandler for MapExtractFieldAndHandle<Provider> {
    type FieldHandler<Tag> = ExtractFieldAndHandle<Tag, Provider>;
}

#[deprecated(
    since = "0.11.0",
    note = "use `cgp_fork::handler::VariantHandlers<P, First>`"
)]
pub struct MapExtractFirstFieldAndHandle<Provider>(pub PhantomData<Provider>);

impl<Provider> MapFieldHandler for MapExtractFirstFieldAndHandle<Provider> {
    type FieldHandler<Tag> = ExtractFirstFieldAndHandle<Tag, Provider>;
}
