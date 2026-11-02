//! `open` dispatch for a component with a lifetime and multiple type parameters.
//!
//! `Foo<'a, T, U>` is opened with `open { FooProviderComponent };` and dispatched
//! by its two leading path parameters, including a generic key
//! (`<T> @FooProviderComponent.bool.T: DummyFoo`). This shows `open` handling the
//! multi-parameter path (the lifetime is lifted into `Life<'a>` in the check keys,
//! not the delegate path). The `delegate_components!` snapshot is the canonical
//! `open` golden output; the component and provider are incidental scaffolding.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

// Incidental: a plain generic component dispatched via `open` below.
#[cgp_component(FooProvider)]
pub trait Foo<'a, T, U> {
    fn foo(&self, first: &'a T, second: U);
}

// Incidental: a plain per-value provider.
#[cgp_impl(new DummyFoo)]
impl<'a, T, U> FooProvider<'a, T, U> {
    fn foo(&self, _first: &'a T, _second: U) {}
}

pub struct AppA;

snapshot_delegate_components! {
    delegate_components! {
        // `Foo<'a, T, U>` stores `Life<'a>` in `IsProviderFor`. The `@` path only
        // names `T` and `U`, so the automatic check cannot name the real parameters.
        // The explicit `check_components!` below does.
        #[unchecked]
        AppA {
            open FooProviderComponent;

            @FooProviderComponent.String.u32:
                DummyFoo,
            <T> @FooProviderComponent.bool.T:
                DummyFoo,
        }
    }

    expand_multi_param_app_a(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<FooProviderComponent> for AppA {
            type Delegate = RedirectLookup<AppA, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for AppA
        where
            RedirectLookup<
                AppA,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<FooProviderComponent, PathCons<String, PathCons<u32, __Wildcard__>>>,
        > for AppA {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, PathCons<u32, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for AppA
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, PathCons<u32, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            T,
            __Wildcard__,
        > DelegateComponent<
            PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
        > for AppA {
            type Delegate = DummyFoo;
        }
        impl<
            T,
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for AppA
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_AppA_8de3091d : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"AppA\",\"generics\":\"\",\"unchecked\":true,\"cfgs\":[],\"entries\":[{\"component\":\"FooProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < AppA, :: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: Nil > , >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < String, :: cgp_fork :: macro_prelude :: PathCons < u32, __Wildcard__ > > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < bool, :: cgp_fork :: macro_prelude :: PathCons < T, __Wildcard__ > > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    AppA {
        FooProviderComponent: [
            <'a> (Life<'a>, String, u32),
            <'a> (Life<'a>, bool, String),
        ],
        FooProviderComponent:
            <'a> (Life<'a>, bool, bool),
    }
}
