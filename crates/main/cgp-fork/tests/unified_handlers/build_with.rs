//! `BuildWith` with `SetField` and `Merge`, and their fallible forms (ported from
//! `extensible_records/record_build_with_handlers.rs`).

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct FooBarBaz {
    pub foo: u64,
    pub bar: String,
    pub baz: bool,
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct FooBar {
    pub foo: u64,
    pub bar: String,
}

// A builder step's provider reads the partial builder; these ignore it.
#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for BuildFooBar {
    type Output = FooBar;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: Input) -> FooBar {
        FooBar {
            foo: 1,
            bar: "bar".to_owned(),
        }
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for BuildFoo {
    type Output = u64;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: Input) -> u64 {
        1
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for BuildBar {
    type Output = String;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: Input) -> String {
        "bar".to_owned()
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for BuildBaz {
    type Output = bool;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: Input) -> bool {
        true
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for FailBaz
where
    Context: HasErrorType<Error = String>,
{
    type Output = Result<bool, String>;

    fn handle(_context: &Context, _code: PhantomData<Code>, _input: Input) -> Self::Output {
        Err("no baz".to_owned())
    }
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<String>,
    }
}

fn expected() -> FooBarBaz {
    FooBarBaz {
        foo: 1,
        bar: "bar".to_owned(),
        baz: true,
    }
}

#[test]
fn build_with_merge_and_set_field() {
    type Handlers = Product![Merge<BuildFooBar>, SetField<Symbol!("baz"), BuildBaz>];

    assert_eq!(
        BuildWith::<FooBarBaz, Handlers>::handle(&App, NoCode, ()),
        expected()
    );

    type AsyncHandlers =
        Product![Merge<Promote<BuildFooBar>>, SetField<Symbol!("baz"), Promote<BuildBaz>>];

    assert_eq!(
        block_on(BuildWith::<FooBarBaz, AsyncHandlers>::handle_async(
            &App,
            NoCode,
            ()
        )),
        expected()
    );
}

#[test]
fn build_with_fields_in_any_order() {
    type Handlers = Product![
        SetField<Symbol!("baz"), BuildBaz>,
        SetField<Symbol!("bar"), BuildBar>,
        SetField<Symbol!("foo"), BuildFoo>,
    ];

    assert_eq!(
        BuildWith::<FooBarBaz, Handlers>::handle(&App, NoCode, ()),
        expected()
    );
}

#[test]
fn merge_outputs_wraps_each_handler() {
    // `MergeOutputs` wraps every handler in `Merge`.
    #[derive(Debug, Eq, PartialEq, CgpData)]
    pub struct Baz {
        pub baz: bool,
    }

    #[cgp_new_provider]
    impl<Context, Code, Input> Handler<Context, Code, Input> for BuildBazRecord {
        type Output = Baz;

        fn handle(_context: &Context, _code: PhantomData<Code>, _input: Input) -> Baz {
            Baz { baz: true }
        }
    }

    type Merged = MergeOutputs<FooBarBaz, Product![BuildFooBar, BuildBazRecord]>;

    assert_eq!(Merged::handle(&App, NoCode, ()), expected());

    // `TryMergeOutputs` is the same wrap, with each handler returning `Result`.
    type TryMerged =
        TryMergeOutputs<FooBarBaz, Product![PromoteOk<BuildFooBar>, PromoteOk<BuildBazRecord>]>;

    assert_eq!(TryMerged::handle(&App, NoCode, ()), Ok(expected()));

    type AsyncTryMerged = TryMergeOutputs<
        FooBarBaz,
        Product![
            Promote<PromoteOk<BuildFooBar>>,
            Promote<PromoteOk<BuildBazRecord>>,
        ],
    >;

    assert_eq!(
        block_on(AsyncTryMerged::handle_async(&App, NoCode, ())),
        Ok(expected())
    );
}

#[test]
fn try_build_with_returns_the_first_error() {
    type Ok3 = Product![
        TrySetField<Symbol!("baz"), PromoteOk<BuildBaz>>,
        TryMerge<PromoteOk<BuildFooBar>>,
    ];

    assert_eq!(
        TryBuildWith::<FooBarBaz, Ok3>::handle(&App, NoCode, ()),
        Ok(expected())
    );

    type Fails = Product![TryMerge<PromoteOk<BuildFooBar>>, TrySetField<Symbol!("baz"), FailBaz>];

    assert_eq!(
        TryBuildWith::<FooBarBaz, Fails>::handle(&App, NoCode, ()),
        Err("no baz".to_owned())
    );

    type AsyncOk = Product![
        TrySetField<Symbol!("baz"), Promote<PromoteOk<BuildBaz>>>,
        TryMerge<Promote<PromoteOk<BuildFooBar>>>,
    ];

    assert_eq!(
        block_on(TryBuildWith::<FooBarBaz, AsyncOk>::handle_async(
            &App,
            NoCode,
            ()
        )),
        Ok(expected())
    );

    type AsyncFails = Product![
        TryMerge<Promote<PromoteOk<BuildFooBar>>>,
        TrySetField<Symbol!("baz"), Promote<FailBaz>>,
    ];

    assert_eq!(
        block_on(TryBuildWith::<FooBarBaz, AsyncFails>::handle_async(
            &App,
            NoCode,
            ()
        )),
        Err("no baz".to_owned())
    );
}
