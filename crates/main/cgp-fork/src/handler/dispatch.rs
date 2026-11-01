//! Matching an extensible enum with one handler per variant, generic over the match
//! [`Strategy`](MatchStrategy) and the access [`Mode`].
//!
//! [`MatchWith<Handlers, Strategy, M>`](MatchWith) replaces the six 0.10 dispatchers
//! (`MatchWithHandlers`, `MatchFirstWithHandlers`, each with a `Ref` and a `Mut` form):
//!
//! | 0.10 | Unified |
//! |---|---|
//! | `MatchWithHandlers<H>` | `MatchWith<H>` (`All`, `Owned`) |
//! | `MatchWithHandlersRef<H>` | `MatchWith<H, All, Ref>` |
//! | `MatchWithHandlersMut<H>` | `MatchWith<H, All, Mut>` |
//! | `MatchFirstWithHandlers<H>` | `MatchWith<H, First>` |
//! | `MatchFirstWithHandlersRef<H>` / `Mut<H>` | `MatchWith<H, First, Ref>` / `MatchWith<H, First, Mut>` |
//! | `ExtractFieldAndHandle<Tag, P>` / `ExtractFirstFieldAndHandle<Tag, P>` | [`OnField<Tag, P, All>`](OnField) / `OnField<Tag, P, First>` |
//! | `HandleFieldValue<P>` | [`FieldValue<P, All>`](FieldValue) / `FieldValue<P, First>` |
//! | `MatchWith{Field,Value}Handlers{,Ref,Mut}` | [`MatchFields<P, S, M>`](MatchFields) / [`MatchValues<P, S, M>`](MatchValues) |
//!
//! [`BuildWith`] (with [`SetField`] and [`Merge`]) is the unified `BuildWithHandlers`.

use crate::core::field::impls::{CanBuildFrom, CanDowncastFields};
use crate::core::field::traits::{MapFields, Mode, ModeExtractor};
use crate::core::field::types::Owned;
use crate::core::prelude::*;
use crate::handler::{
    AsyncHandler, AsyncHandlerComponent, Handler, HandlerComponent, Pipe, TryPipe,
};

/// Match every variant: the input is the enum alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct All;

/// Match the first variant that applies, with extra arguments: the input is `(enum, args)`,
/// and every handler sees the arguments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct First;

/// A match strategy: [`All`] or [`First`].
pub trait MatchStrategy: sealed::Sealed {}

impl MatchStrategy for All {}
impl MatchStrategy for First {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::All {}
    impl Sealed for super::First {}
}

/// A non-empty [`Cons`] list of variant handlers tried in order.
///
/// Each handler returns `Result<Output, Remainder>`: `Ok` ends the match, `Err` is the input
/// with that variant removed, which the next handler gets. Used by [`MatchWith`]; implemented
/// for `Cons<H, Nil>` and `Cons<H, Cons<…>>`.
pub trait MatchHandlers<Context, Code, Input> {
    /// What a matching handler returns.
    type Output;

    /// What is left when no handler matched; uninhabited once every variant has a handler.
    type Remainder;

    /// Tries the handlers in order.
    fn match_handlers(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder>;
}

impl<Context, Code, Input, Head, Output, Remainder> MatchHandlers<Context, Code, Input>
    for Cons<Head, Nil>
where
    Head: Handler<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;
    type Remainder = Remainder;

    fn match_handlers(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        Head::handle(context, code, input)
    }
}

impl<Context, Code, Input, Head, Next, Rest, Output, Remainder> MatchHandlers<Context, Code, Input>
    for Cons<Head, Cons<Next, Rest>>
where
    Head: Handler<Context, Code, Input, Output = Result<Output, Remainder>>,
    Cons<Next, Rest>: MatchHandlers<Context, Code, Remainder, Output = Output>,
{
    type Output = Output;
    type Remainder = <Cons<Next, Rest> as MatchHandlers<Context, Code, Remainder>>::Remainder;

    fn match_handlers(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Self::Remainder> {
        match Head::handle(context, code, input) {
            Ok(output) => Ok(output),
            Err(remainder) => Cons::<Next, Rest>::match_handlers(context, code, remainder),
        }
    }
}

/// [`MatchHandlers`] for [`AsyncHandler`]s.
#[cgp_async]
pub trait AsyncMatchHandlers<Context, Code, Input> {
    /// What a matching handler returns.
    type Output;

    /// What is left when no handler matched.
    type Remainder;

    /// Tries the handlers in order.
    async fn match_handlers_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Self::Remainder>;
}

impl<Context, Code, Input, Head, Output, Remainder> AsyncMatchHandlers<Context, Code, Input>
    for Cons<Head, Nil>
where
    Head: AsyncHandler<Context, Code, Input, Output = Result<Output, Remainder>>,
{
    type Output = Output;
    type Remainder = Remainder;

    async fn match_handlers_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Remainder> {
        Head::handle_async(context, code, input).await
    }
}

impl<Context, Code, Input, Head, Next, Rest, Output, Remainder>
    AsyncMatchHandlers<Context, Code, Input> for Cons<Head, Cons<Next, Rest>>
where
    Head: AsyncHandler<Context, Code, Input, Output = Result<Output, Remainder>>,
    Cons<Next, Rest>: AsyncMatchHandlers<Context, Code, Remainder, Output = Output>,
{
    type Output = Output;
    type Remainder = <Cons<Next, Rest> as AsyncMatchHandlers<Context, Code, Remainder>>::Remainder;

    async fn match_handlers_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Output, Self::Remainder> {
        match Head::handle_async(context, code, input).await {
            Ok(output) => Ok(output),
            Err(remainder) => {
                Cons::<Next, Rest>::match_handlers_async(context, code, remainder).await
            }
        }
    }
}

/// Matches an extensible enum against `Handlers`, a [`Cons`] list of variant handlers such as
/// [`OnField`].
///
/// - `Strategy` is [`All`] (input: the enum) or [`First`] (input: `(enum, args)`).
/// - `M` is how the enum is reached: [`Owned`] (`T`), [`Ref`](crate::core::field::types::Ref)
///   (`&T`) or [`Mut`](crate::core::field::types::Mut) (`&mut T`), through
///   [`ModeExtractor`].
///
/// When every variant has a handler the remainder is uninhabited and the output is the
/// handlers' output; a missing variant is a compile error on `FinalizeExtract`.
pub struct MatchWith<Handlers, Strategy = All, M = Owned>(pub PhantomData<(Handlers, Strategy, M)>);

#[cgp_provider]
impl<Context, Code, Input, Handlers, M, Output, Remainder> Handler<Context, Code, Input>
    for MatchWith<Handlers, All, M>
where
    M: Mode + ModeExtractor<Input>,
    Handlers: MatchHandlers<Context, Code, M::Extractor, Output = Output, Remainder = Remainder>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        match Handlers::match_handlers(context, code, M::to_extractor(input)) {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Handlers, M, Output, Remainder>
    Handler<Context, Code, (Input, Args)> for MatchWith<Handlers, First, M>
where
    M: Mode + ModeExtractor<Input>,
    Handlers:
        MatchHandlers<Context, Code, (M::Extractor, Args), Output = Output, Remainder = Remainder>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    fn handle(context: &Context, code: PhantomData<Code>, (input, args): (Input, Args)) -> Output {
        match Handlers::match_handlers(context, code, (M::to_extractor(input), args)) {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Handlers, M, Output, Remainder> AsyncHandler<Context, Code, Input>
    for MatchWith<Handlers, All, M>
where
    M: Mode + ModeExtractor<Input>,
    Handlers:
        AsyncMatchHandlers<Context, Code, M::Extractor, Output = Output, Remainder = Remainder>,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle_async(context: &Context, code: PhantomData<Code>, input: Input) -> Output {
        match Handlers::match_handlers_async(context, code, M::to_extractor(input)).await {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Handlers, M, Output, Remainder>
    AsyncHandler<Context, Code, (Input, Args)> for MatchWith<Handlers, First, M>
where
    M: Mode + ModeExtractor<Input>,
    Handlers: AsyncMatchHandlers<
            Context,
            Code,
            (M::Extractor, Args),
            Output = Output,
            Remainder = Remainder,
        >,
    Remainder: FinalizeExtract,
{
    type Output = Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Output {
        match Handlers::match_handlers_async(context, code, (M::to_extractor(input), args)).await {
            Ok(output) => output,
            Err(remainder) => remainder.finalize_extract(),
        }
    }
}

/// The variant handler for the `Tag` variant: extracts it and runs `Provider` on the
/// [`Field<Tag, Value>`](Field), or returns the remainder when the input is another variant.
///
/// With [`All`] the input is the extractor and `Provider` gets the field; with [`First`] the
/// input is `(extractor, args)`, `Provider` gets `(field, args)`, and a miss keeps the args
/// beside the remainder.
pub struct OnField<Tag, Provider = UseContext, Strategy = All>(
    pub PhantomData<(Tag, Provider, Strategy)>,
);

#[cgp_provider]
impl<Context, Code, Input, Tag, Value, Remainder, Provider> Handler<Context, Code, Input>
    for OnField<Tag, Provider, All>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: Handler<Context, Code, Field<Tag, Value>>,
{
    type Output = Result<Provider::Output, Remainder>;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let value = input.extract_field(PhantomData::<Tag>)?;
        Ok(Provider::handle(context, code, value.into()))
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Tag, Value, Remainder, Provider>
    Handler<Context, Code, (Input, Args)> for OnField<Tag, Provider, First>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: Handler<Context, Code, (Field<Tag, Value>, Args)>,
{
    type Output = Result<Provider::Output, (Remainder, Args)>;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Self::Output {
        match input.extract_field(PhantomData::<Tag>) {
            Ok(value) => Ok(Provider::handle(context, code, (value.into(), args))),
            Err(remainder) => Err((remainder, args)),
        }
    }
}

#[cgp_provider]
impl<Context, Code, Input, Tag, Value, Remainder, Provider> AsyncHandler<Context, Code, Input>
    for OnField<Tag, Provider, All>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: AsyncHandler<Context, Code, Field<Tag, Value>>,
{
    type Output = Result<Provider::Output, Remainder>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        let value = input.extract_field(PhantomData::<Tag>)?;
        Ok(Provider::handle_async(context, code, value.into()).await)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Args, Tag, Value, Remainder, Provider>
    AsyncHandler<Context, Code, (Input, Args)> for OnField<Tag, Provider, First>
where
    Input: ExtractField<Tag, Value = Value, Remainder = Remainder>,
    Provider: AsyncHandler<Context, Code, (Field<Tag, Value>, Args)>,
{
    type Output = Result<Provider::Output, (Remainder, Args)>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Input, Args),
    ) -> Self::Output {
        match input.extract_field(PhantomData::<Tag>) {
            Ok(value) => Ok(Provider::handle_async(context, code, (value.into(), args)).await),
            Err(remainder) => Err((remainder, args)),
        }
    }
}

/// Unwraps a [`Field`] and runs `Provider` on its value, dropping the tag. With [`First`] the
/// input is `(field, args)` and `Provider` gets `(value, args)`.
pub struct FieldValue<Provider = UseContext, Strategy = All>(pub PhantomData<(Provider, Strategy)>);

#[cgp_provider]
impl<Context, Code, Tag, Value, Provider> Handler<Context, Code, Field<Tag, Value>>
    for FieldValue<Provider, All>
where
    Provider: Handler<Context, Code, Value>,
{
    type Output = Provider::Output;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Field<Tag, Value>,
    ) -> Self::Output {
        Provider::handle(context, code, input.value)
    }
}

#[cgp_provider]
impl<Context, Code, Tag, Value, Args, Provider> Handler<Context, Code, (Field<Tag, Value>, Args)>
    for FieldValue<Provider, First>
where
    Provider: Handler<Context, Code, (Value, Args)>,
{
    type Output = Provider::Output;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Field<Tag, Value>, Args),
    ) -> Self::Output {
        Provider::handle(context, code, (input.value, args))
    }
}

#[cgp_provider]
impl<Context, Code, Tag, Value, Provider> AsyncHandler<Context, Code, Field<Tag, Value>>
    for FieldValue<Provider, All>
where
    Provider: AsyncHandler<Context, Code, Value>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Field<Tag, Value>,
    ) -> Self::Output {
        Provider::handle_async(context, code, input.value).await
    }
}

#[cgp_provider]
impl<Context, Code, Tag, Value, Args, Provider>
    AsyncHandler<Context, Code, (Field<Tag, Value>, Args)> for FieldValue<Provider, First>
where
    Provider: AsyncHandler<Context, Code, (Value, Args)>,
{
    type Output = Provider::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        (input, args): (Field<Tag, Value>, Args),
    ) -> Self::Output {
        Provider::handle_async(context, code, (input.value, args)).await
    }
}

/// The variant handlers of a field list: one [`OnField<Tag, Provider, Strategy>`](OnField) per
/// [`Field`] of an [`Either`] chain, in order.
pub trait VariantHandlers<Provider, Strategy> {
    /// The [`Cons`] list of handlers.
    type Handlers;
}

impl<Tag, Value, Provider, Strategy> VariantHandlers<Provider, Strategy>
    for Either<Field<Tag, Value>, Void>
{
    type Handlers = Cons<OnField<Tag, Provider, Strategy>, Nil>;
}

impl<Tag, Value, NextTag, NextValue, Rest, Provider, Strategy> VariantHandlers<Provider, Strategy>
    for Either<Field<Tag, Value>, Either<Field<NextTag, NextValue>, Rest>>
where
    Either<Field<NextTag, NextValue>, Rest>: VariantHandlers<Provider, Strategy>,
{
    type Handlers = Cons<
        OnField<Tag, Provider, Strategy>,
        <Either<Field<NextTag, NextValue>, Rest> as VariantHandlers<Provider, Strategy>>::Handlers,
    >;
}

/// Matches an enum by running `Provider` on the [`Field`] of whichever variant it holds: a
/// [`MatchWith`] whose handlers are [`OnField`] for every variant of the enum's
/// [`HasFields`] list.
pub struct MatchFields<Provider = UseContext, Strategy = All, M = Owned>(
    pub PhantomData<(Provider, Strategy, M)>,
);

/// [`MatchFields`] that unwraps each [`Field`] first, so `Provider` gets the variant's value.
pub type MatchValues<Provider = UseContext, Strategy = All, M = Owned> =
    MatchFields<FieldValue<Provider, Strategy>, Strategy, M>;

/// The enum `MatchFields` matches: the input itself for [`All`], its first element for
/// [`First`].
pub trait MatchSubject<Strategy> {
    /// The enum, owned or borrowed.
    type Subject;
}

impl<Input> MatchSubject<All> for Input {
    type Subject = Input;
}

impl<Input, Args> MatchSubject<First> for (Input, Args) {
    type Subject = Input;
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Strategy, M, Fields> Handler<Context, Code, Input>
    for MatchFields<Provider, Strategy, M>
where
    Input: MatchSubject<Strategy>,
    M: ModeExtractor<Input::Subject>,
    M::Target: HasFields<Fields = Fields>,
    Fields: VariantHandlers<Provider, Strategy>,
    MatchWith<Fields::Handlers, Strategy, M>: Handler<Context, Code, Input>,
{
    type Output =
        <MatchWith<Fields::Handlers, Strategy, M> as Handler<Context, Code, Input>>::Output;

    fn handle(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        MatchWith::<Fields::Handlers, Strategy, M>::handle(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Provider, Strategy, M, Fields> AsyncHandler<Context, Code, Input>
    for MatchFields<Provider, Strategy, M>
where
    Input: MatchSubject<Strategy>,
    M: ModeExtractor<Input::Subject>,
    M::Target: HasFields<Fields = Fields>,
    Fields: VariantHandlers<Provider, Strategy>,
    MatchWith<Fields::Handlers, Strategy, M>: AsyncHandler<Context, Code, Input>,
{
    type Output =
        <MatchWith<Fields::Handlers, Strategy, M> as AsyncHandler<Context, Code, Input>>::Output;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Self::Output {
        MatchWith::<Fields::Handlers, Strategy, M>::handle_async(context, code, input).await
    }
}

/// Builds an `Output` record: starts from `Output::builder()`, runs `Handlers` over the
/// builder with [`Pipe`] (each typically a [`SetField`] or [`Merge`]), and finishes the build.
/// The input is ignored.
pub struct BuildWith<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers, Res> Handler<Context, Code, Input>
    for BuildWith<Output, Handlers>
where
    Output: HasBuilder<Builder = Builder>,
    Pipe<Handlers>: Handler<Context, Code, Builder, Output = Res>,
    Res: FinalizeBuild<Target = Output>,
{
    type Output = Output;

    fn handle(context: &Context, code: PhantomData<Code>, _input: Input) -> Output {
        Pipe::<Handlers>::handle(context, code, Output::builder()).finalize_build()
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers, Res> AsyncHandler<Context, Code, Input>
    for BuildWith<Output, Handlers>
where
    Output: HasBuilder<Builder = Builder>,
    Pipe<Handlers>: AsyncHandler<Context, Code, Builder, Output = Res>,
    Res: FinalizeBuild<Target = Output>,
{
    type Output = Output;

    async fn handle_async(context: &Context, code: PhantomData<Code>, _input: Input) -> Output {
        Pipe::<Handlers>::handle_async(context, code, Output::builder())
            .await
            .finalize_build()
    }
}

/// [`BuildWith`] for fallible handlers, piped with [`TryPipe`]: the first error is returned.
pub struct TryBuildWith<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers, Res> Handler<Context, Code, Input>
    for TryBuildWith<Output, Handlers>
where
    Context: HasErrorType,
    Output: HasBuilder<Builder = Builder>,
    TryPipe<Handlers>: Handler<Context, Code, Builder, Output = Result<Res, Context::Error>>,
    Res: FinalizeBuild<Target = Output>,
{
    type Output = Result<Output, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, _input: Input) -> Self::Output {
        Ok(TryPipe::<Handlers>::handle(context, code, Output::builder())?.finalize_build())
    }
}

#[cgp_provider]
impl<Context, Code, Input, Output, Builder, Handlers, Res> AsyncHandler<Context, Code, Input>
    for TryBuildWith<Output, Handlers>
where
    Context: HasErrorType,
    Output: HasBuilder<Builder = Builder>,
    TryPipe<Handlers>: AsyncHandler<Context, Code, Builder, Output = Result<Res, Context::Error>>,
    Res: FinalizeBuild<Target = Output>,
{
    type Output = Result<Output, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Self::Output {
        Ok(
            TryPipe::<Handlers>::handle_async(context, code, Output::builder())
                .await?
                .finalize_build(),
        )
    }
}

/// A builder step: runs `Provider` on `&builder` and stores its output in the `Tag` field.
/// The output is the builder with `Tag` set; [`TrySetField`] is the fallible form.
pub struct SetField<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

#[cgp_provider]
impl<Context, Code, Builder, Tag, Value, Built, Provider> Handler<Context, Code, Builder>
    for SetField<Tag, Provider>
where
    Provider: for<'a> Handler<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Built>,
{
    type Output = Built;

    fn handle(context: &Context, code: PhantomData<Code>, builder: Builder) -> Built {
        let value = Provider::handle(context, code, &builder);
        builder.build_field(PhantomData::<Tag>, value)
    }
}

#[cgp_provider]
impl<Context, Code, Builder, Tag, Value, Built, Provider> AsyncHandler<Context, Code, Builder>
    for SetField<Tag, Provider>
where
    Provider: for<'a> AsyncHandler<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Built>,
{
    type Output = Built;

    async fn handle_async(context: &Context, code: PhantomData<Code>, builder: Builder) -> Built {
        let value = Provider::handle_async(context, code, &builder).await;
        builder.build_field(PhantomData::<Tag>, value)
    }
}

/// [`SetField`] for a fallible `Provider` returning `Result<Value, Context::Error>`, for
/// [`TryBuildWith`].
pub struct TrySetField<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

#[cgp_provider]
impl<Context, Code, Builder, Tag, Value, Built, Provider> Handler<Context, Code, Builder>
    for TrySetField<Tag, Provider>
where
    Context: HasErrorType,
    Provider: for<'a> Handler<Context, Code, &'a Builder, Output = Result<Value, Context::Error>>,
    Builder: BuildField<Tag, Value = Value, Output = Built>,
{
    type Output = Result<Built, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let value = Provider::handle(context, code, &builder)?;
        Ok(builder.build_field(PhantomData::<Tag>, value))
    }
}

#[cgp_provider]
impl<Context, Code, Builder, Tag, Value, Built, Provider> AsyncHandler<Context, Code, Builder>
    for TrySetField<Tag, Provider>
where
    Context: HasErrorType,
    Provider:
        for<'a> AsyncHandler<Context, Code, &'a Builder, Output = Result<Value, Context::Error>>,
    Builder: BuildField<Tag, Value = Value, Output = Built>,
{
    type Output = Result<Built, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Self::Output {
        let value = Provider::handle_async(context, code, &builder).await?;
        Ok(builder.build_field(PhantomData::<Tag>, value))
    }
}

/// A builder step: runs `Provider` on `&builder` and merges every field of its output record
/// into the builder ([`CanBuildFrom`]).
pub struct Merge<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Builder, Provider, Res, Built> Handler<Context, Code, Builder>
    for Merge<Provider>
where
    Provider: for<'a> Handler<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Built>,
{
    type Output = Built;

    fn handle(context: &Context, code: PhantomData<Code>, builder: Builder) -> Built {
        let output = Provider::handle(context, code, &builder);
        builder.build_from(output)
    }
}

#[cgp_provider]
impl<Context, Code, Builder, Provider, Res, Built> AsyncHandler<Context, Code, Builder>
    for Merge<Provider>
where
    Provider: for<'a> AsyncHandler<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Built>,
{
    type Output = Built;

    async fn handle_async(context: &Context, code: PhantomData<Code>, builder: Builder) -> Built {
        let output = Provider::handle_async(context, code, &builder).await;
        builder.build_from(output)
    }
}

/// [`Merge`] for a fallible `Provider` returning `Result<Record, Context::Error>`, for
/// [`TryBuildWith`].
pub struct TryMerge<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_provider]
impl<Context, Code, Builder, Provider, Res, Built> Handler<Context, Code, Builder>
    for TryMerge<Provider>
where
    Context: HasErrorType,
    Provider: for<'a> Handler<Context, Code, &'a Builder, Output = Result<Res, Context::Error>>,
    Builder: CanBuildFrom<Res, Output = Built>,
{
    type Output = Result<Built, Context::Error>;

    fn handle(context: &Context, code: PhantomData<Code>, builder: Builder) -> Self::Output {
        let output = Provider::handle(context, code, &builder)?;
        Ok(builder.build_from(output))
    }
}

#[cgp_provider]
impl<Context, Code, Builder, Provider, Res, Built> AsyncHandler<Context, Code, Builder>
    for TryMerge<Provider>
where
    Context: HasErrorType,
    Provider:
        for<'a> AsyncHandler<Context, Code, &'a Builder, Output = Result<Res, Context::Error>>,
    Builder: CanBuildFrom<Res, Output = Built>,
{
    type Output = Result<Built, Context::Error>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        builder: Builder,
    ) -> Self::Output {
        let output = Provider::handle_async(context, code, &builder).await?;
        Ok(builder.build_from(output))
    }
}

/// Builds `Output` by merging the output record of every handler in `Handlers` (a [`Cons`]
/// list) into the builder: [`BuildWith`] with each handler wrapped in [`Merge`].
pub type MergeOutputs<Output, Handlers> =
    BuildWith<Output, <Handlers as MapFields<ToMerge>>::Mapped>;

/// [`MergeOutputs`] for fallible handlers: [`TryBuildWith`] with each handler wrapped in
/// [`TryMerge`].
pub type TryMergeOutputs<Output, Handlers> =
    TryBuildWith<Output, <Handlers as MapFields<ToTryMerge>>::Mapped>;

/// The [`MapType`] that wraps a handler in [`Merge`].
pub struct ToMerge;

impl MapType for ToMerge {
    type Map<Provider> = Merge<Provider>;
}

/// The [`MapType`] that wraps a handler in [`TryMerge`].
pub struct ToTryMerge;

impl MapType for ToTryMerge {
    type Map<Provider> = TryMerge<Provider>;
}

/// Tries `Handlers`, a [`Cons`] list of handlers that each return
/// `Result<Output, Remainder>`, in order: the first `Ok` wins, and each `Err` (the input
/// with one more variant removed) goes to the next handler.
///
/// The output is `Result<Output, Remainder>`, so a partial match can be chained further.
/// [`MatchWith`] is the same loop for a complete match, which unwraps the uninhabited
/// remainder. The unified `DispatchMatchers`.
pub struct DispatchHandlers<Handlers>(pub PhantomData<Handlers>);

#[cgp_provider]
impl<Context, Code, Input, Handlers> Handler<Context, Code, Input> for DispatchHandlers<Handlers>
where
    Handlers: MatchHandlers<Context, Code, Input>,
{
    type Output = Result<Handlers::Output, Handlers::Remainder>;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Handlers::Output, Handlers::Remainder> {
        Handlers::match_handlers(context, code, input)
    }
}

#[cgp_provider]
impl<Context, Code, Input, Handlers> AsyncHandler<Context, Code, Input>
    for DispatchHandlers<Handlers>
where
    Handlers: AsyncMatchHandlers<Context, Code, Input>,
{
    type Output = Result<Handlers::Output, Handlers::Remainder>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Handlers::Output, Handlers::Remainder> {
        Handlers::match_handlers_async(context, code, input).await
    }
}

/// Downcasts the input to the extensible enum `Inner` (a subset of its variants) and runs
/// `Provider` on it.
///
/// The output is `Result<Provider::Output, Remainder>`; `Err` is the input with `Inner`'s
/// variants removed. The unified `DowncastAndHandle`.
pub struct Downcast<Inner, Provider = UseContext>(pub PhantomData<(Inner, Provider)>);

#[cgp_provider]
impl<Context, Code, Input, Inner, Provider> Handler<Context, Code, Input>
    for Downcast<Inner, Provider>
where
    Input: CanDowncastFields<Inner>,
    Provider: Handler<Context, Code, Inner>,
{
    type Output = Result<Provider::Output, Input::Remainder>;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Provider::Output, Input::Remainder> {
        let inner = input.downcast_fields(PhantomData::<Inner>)?;
        Ok(Provider::handle(context, code, inner))
    }
}

#[cgp_provider]
impl<Context, Code, Input, Inner, Provider> AsyncHandler<Context, Code, Input>
    for Downcast<Inner, Provider>
where
    Input: CanDowncastFields<Inner>,
    Provider: AsyncHandler<Context, Code, Inner>,
{
    type Output = Result<Provider::Output, Input::Remainder>;

    async fn handle_async(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> Result<Provider::Output, Input::Remainder> {
        let inner = input.downcast_fields(PhantomData::<Inner>)?;
        Ok(Provider::handle_async(context, code, inner).await)
    }
}
