//! `open` dispatch on a component's second type parameter alone.
//!
//! The redirect appends every type parameter to the lookup path, so a lookup of
//! `CanDescribe<Code, Input>` follows `@DescriberComponent.Code.Input`. A key whose
//! first segment is a per-entry generic, `@DescriberComponent.<Code> Code.u64`,
//! matches every `Code` and dispatches on the input, which is the `open` form of a
//! `UseInputDelegate` table. The test calls one context with two codes per input
//! and checks that the input alone selects the provider. The `delegate_components!`
//! snapshot pins the generic-first-segment expansion; the component and providers
//! are incidental scaffolding.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/delegate_components.md.

use core::marker::PhantomData;

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

// Incidental: a component with a code parameter and an input parameter.
#[cgp_component(Describer)]
pub trait CanDescribe<Code, Input> {
    fn describe(&self, code: PhantomData<Code>, input: Input) -> String;
}

// Incidental: one provider per input type, each generic over the code.
#[cgp_impl(new DescribeNumber)]
impl<Code> Describer<Code, u64> {
    fn describe(&self, _code: PhantomData<Code>, input: u64) -> String {
        format!("number {input}")
    }
}

#[cgp_impl(new DescribeText)]
impl<Code> Describer<Code, String> {
    fn describe(&self, _code: PhantomData<Code>, input: String) -> String {
        format!("text {input}")
    }
}

pub struct Short;

pub struct Long;

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            open DescriberComponent;

            @DescriberComponent.<Code> Code.u64:
                DescribeNumber,
            @DescriberComponent.<Code> Code.String:
                DescribeText,
        }
    }

    expand_open_later_param(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<DescriberComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<DescriberComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<DescriberComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<DescriberComponent, Nil>,
            >: IsProviderFor<DescriberComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            Code,
            __Wildcard__,
        > DelegateComponent<
            PathCons<DescriberComponent, PathCons<Code, PathCons<u64, __Wildcard__>>>,
        > for App {
            type Delegate = DescribeNumber;
        }
        impl<
            Code,
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<DescriberComponent, PathCons<Code, PathCons<u64, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for App
        where
            DescribeNumber: IsProviderFor<
                PathCons<DescriberComponent, PathCons<Code, PathCons<u64, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            Code,
            __Wildcard__,
        > DelegateComponent<
            PathCons<DescriberComponent, PathCons<Code, PathCons<String, __Wildcard__>>>,
        > for App {
            type Delegate = DescribeText;
        }
        impl<
            Code,
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<DescriberComponent, PathCons<Code, PathCons<String, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for App
        where
            DescribeText: IsProviderFor<
                PathCons<DescriberComponent, PathCons<Code, PathCons<String, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        trait __CanUseApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<Code> __CanUseApp<DescriberComponent, (Code, u64)> for App {}
        impl<Code> __CanUseApp<DescriberComponent, (Code, String)> for App {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_App_7329d0ab : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"App\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"DescriberComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < App, :: cgp_fork :: macro_prelude :: PathCons < DescriberComponent, :: cgp_fork :: macro_prelude :: Nil > , >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < DescriberComponent, :: cgp_fork :: macro_prelude :: PathCons < Code, :: cgp_fork :: macro_prelude :: PathCons < u64, __Wildcard__ > > >\",\"provider\":\"DescribeNumber\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < DescriberComponent, :: cgp_fork :: macro_prelude :: PathCons < Code, :: cgp_fork :: macro_prelude :: PathCons < String, __Wildcard__ > > >\",\"provider\":\"DescribeText\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    App {
        DescriberComponent: [
            (Short, u64),
            (Long, u64),
            (Short, String),
            (Long, String),
        ],
    }
}

#[test]
fn test_open_later_param() {
    let app = App;

    assert_eq!(app.describe(PhantomData::<Short>, 7), "number 7");
    assert_eq!(app.describe(PhantomData::<Long>, 7), "number 7");
    assert_eq!(
        app.describe(PhantomData::<Short>, "hi".to_owned()),
        "text hi"
    );
    assert_eq!(
        app.describe(PhantomData::<Long>, "hi".to_owned()),
        "text hi"
    );
}
