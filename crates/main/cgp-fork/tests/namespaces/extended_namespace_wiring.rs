//! Wiring a context through the path-rewriting `ExtendedNamespace`.
//!
//! `App` opens the inheriting `ExtendedNamespace` (defined in the sibling
//! `extended` module), which rewrites the `cgp.core.error` prefix onto `app`, so
//! the error components can be wired under the shorter `@app.*` path while a
//! component outside that prefix (`@cgp.extra.handler.TryComputerComponent`) is
//! still reachable by its full path. Exercises the namespace/`@`-path form of
//! `delegate_components!` (kept as a snapshot) and confirms the wiring compiles
//! via the `CheckApp` supertrait bundle.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_namespace.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

// Exercises the deprecated 0.10 handler family.
#![allow(deprecated)]

use cgp_fork::core::error::{
    ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use cgp_fork::extra::error::RaiseFrom;
use cgp_fork::extra::handler::CanTryCompute;
use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

// The path-rewriting namespace lives in the sibling `extended` module (moved here
// from `cgp_fork_tests::namespaces::extended`).
use crate::namespaces::extended::ExtendedNamespace;

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace ExtendedNamespace;

            @app.ErrorTypeProviderComponent:
                UseType<String>,
            @app.{
                ErrorRaiserComponent.{&'static str, String},
                ErrorWrapperComponent,
            }:
                RaiseFrom,
            @cgp.extra.handler.TryComputerComponent:
                Foo,
        }
    }

    expand_extended_ns_app(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl<__Key__, __Value__> DelegateComponent<__Key__> for App
        where
            __Key__: ExtendedNamespace<App, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<__Key__, __Context__, __Params__> for App
        where
            __Key__: ExtendedNamespace<App, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorTypeProviderComponent, __Wildcard__>,
            >,
        > for App {
            type Delegate = UseType<String>;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorTypeProviderComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            UseType<
                String,
            >: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorTypeProviderComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorWrapperComponent, __Wildcard__>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorWrapperComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorWrapperComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<5, Chars<'e', Chars<'x', Chars<'t', Chars<'r', Chars<'a', Nil>>>>>>,
                    PathCons<
                        Symbol<
                            7,
                            Chars<
                                'h',
                                Chars<
                                    'a',
                                    Chars<
                                        'n',
                                        Chars<'d', Chars<'l', Chars<'e', Chars<'r', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                        PathCons<TryComputerComponent, __Wildcard__>,
                    >,
                >,
            >,
        > for App {
            type Delegate = Foo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<5, Chars<'e', Chars<'x', Chars<'t', Chars<'r', Chars<'a', Nil>>>>>>,
                    PathCons<
                        Symbol<
                            7,
                            Chars<
                                'h',
                                Chars<
                                    'a',
                                    Chars<
                                        'n',
                                        Chars<'d', Chars<'l', Chars<'e', Chars<'r', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                        PathCons<TryComputerComponent, __Wildcard__>,
                    >,
                >,
            >,
            __Context__,
            __Params__,
        > for App
        where
            Foo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                    PathCons<
                        Symbol<
                            5,
                            Chars<'e', Chars<'x', Chars<'t', Chars<'r', Chars<'a', Nil>>>>>,
                        >,
                        PathCons<
                            Symbol<
                                7,
                                Chars<
                                    'h',
                                    Chars<
                                        'a',
                                        Chars<
                                            'n',
                                            Chars<'d', Chars<'l', Chars<'e', Chars<'r', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                            PathCons<TryComputerComponent, __Wildcard__>,
                        >,
                    >,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_App_b2c8ae70 : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"App\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"__Key__\",\"provider\":\"__Value__\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < ErrorTypeProviderComponent, __Wildcard__ > >\",\"provider\":\"UseType < String >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < ErrorRaiserComponent, :: cgp_fork :: macro_prelude :: PathCons < & 'static str, __Wildcard__ > > >\",\"provider\":\"RaiseFrom\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < ErrorRaiserComponent, :: cgp_fork :: macro_prelude :: PathCons < String, __Wildcard__ > > >\",\"provider\":\"RaiseFrom\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < ErrorWrapperComponent, __Wildcard__ > >\",\"provider\":\"RaiseFrom\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 3, :: cgp_fork :: macro_prelude :: Chars < 'c', :: cgp_fork :: macro_prelude :: Chars < 'g', :: cgp_fork :: macro_prelude :: Chars < 'p', :: cgp_fork :: macro_prelude :: Nil > > > > , :: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 5, :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 'x', :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Chars < 'r', :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Nil > > > > > > , :: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 7, :: cgp_fork :: macro_prelude :: Chars < 'h', :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'n', :: cgp_fork :: macro_prelude :: Chars < 'd', :: cgp_fork :: macro_prelude :: Chars < 'l', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 'r', :: cgp_fork :: macro_prelude :: Nil > > > > > > > > , :: cgp_fork :: macro_prelude :: PathCons < TryComputerComponent, __Wildcard__ > > > >\",\"provider\":\"Foo\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

#[cgp_computer]
fn foo(x: u64) -> Result<u64, String> {
    Ok(x * 2)
}

pub trait CheckApp: HasErrorType + CanRaiseError<&'static str> + CanTryCompute<(), u64> {}

impl CheckApp for App {}
