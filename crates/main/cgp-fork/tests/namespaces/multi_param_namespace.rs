//! Namespace `@`-path wiring for a component with a lifetime and many parameters.
//!
//! `Foo<'a, T, U>` attaches to `DefaultNamespace` via `#[prefix(@app in ..)]`, and
//! a context wires it through full paths that pin the leading path parameters,
//! including a generic key (`@app.FooProviderComponent.bool.<T> T: DummyFoo`). This
//! is the namespace counterpart to `multi_param_open`, dispatching the same
//! multi-parameter component through a joined namespace instead of `open`. The
//! namespace `delegate_components!` snapshot is the canonical golden output; the
//! component and provider are incidental scaffolding.
//!
//! See cgp-knowledge-base-fork/cgp/reference/traits/default_namespace.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

// Incidental component, attached to `DefaultNamespace` so it can be wired by path.
#[cgp_component(FooProvider)]
#[prefix(@app in DefaultNamespace)]
pub trait Foo<'a, T, U> {
    fn foo(&self, first: &'a T, second: U);
}

// Incidental: a plain per-value provider.
#[cgp_impl(new DummyFoo)]
impl<'a, T, U> FooProvider<'a, T, U> {
    fn foo(&self, _first: &'a T, _second: U) {}
}

pub struct AppB;

snapshot_delegate_components! {
    delegate_components! {
        AppB {
            namespace DefaultNamespace;

            @app.FooProviderComponent.String.u64:
                DummyFoo,
            @app.FooProviderComponent.bool.<T> T:
                DummyFoo,
        }
    }

    expand_multi_param_app_b(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppB
        where
            __Key__: DefaultNamespace<AppB, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppB
        where
            __Key__: DefaultNamespace<AppB, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
            >,
        > for AppB {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            T,
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            >,
        > for AppB {
            type Delegate = DummyFoo;
        }
        impl<
            T,
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_AppB_99620904 : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"AppB\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"__Key__\",\"provider\":\"__Value__\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < String, :: cgp_fork :: macro_prelude :: PathCons < u64, __Wildcard__ > > > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < bool, :: cgp_fork :: macro_prelude :: PathCons < T, __Wildcard__ > > > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    <'a> AppB {
        FooProviderComponent: [
            (Life<'a>, String, u64),
            (Life<'a>, bool, String),
        ],
    }
}
