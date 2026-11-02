//! Wiring a context through per-type default impls and the `for <..> in ..` loop.
//!
//! Each context opens `DefaultNamespace` (or the inheriting `ExtendedNamespace`)
//! and populates the `ShowImpl` component from a default-impl source: `AppA` and
//! `AppC` iterate `DefaultImpls1<ShowImplComponent>` (the per-type registry filled
//! by `#[default_impl]` in the `default_impls` module) and override `u64`; `AppB`
//! iterates the `DefaultShowComponents` namespace table. This exercises the
//! namespace/`for … in`/`@`-path forms of `delegate_components!` (kept as
//! snapshots) against the reusable namespaces defined in the sibling
//! `default_impls` module.
//!
//! See cgp-knowledge-base-fork/cgp/reference/traits/default_namespace.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::core::component::DefaultImpls1;
use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

// The reusable namespaces and providers live in the sibling `default_impls`
// module (moved here from `cgp_fork_tests::namespaces::default_impls`).
use crate::namespaces::default_impls::{
    DefaultShowComponents, ExtendedNamespace, ShowImplComponent, ShowWithDisplay,
};

pub struct AppA;

snapshot_delegate_components! {
    delegate_components! {
        AppA {
            namespace DefaultNamespace;

            for <T, Provider> in DefaultImpls1<ShowImplComponent> {
                @test.ShowImplComponent.T: Provider,
            }

            @test.ShowImplComponent.u64:
                ShowWithDisplay,
        }
    }

    expand_default_impls_app_a(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppA
        where
            __Key__: DefaultNamespace<AppA, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppA
        where
            __Key__: DefaultNamespace<AppA, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for AppA
        where
            T: DefaultImpls1<ShowImplComponent, AppA, Delegate = Provider>,
        {
            type Delegate = Provider;
        }
        impl<
            __Wildcard__,
            T,
            Provider,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppA
        where
            T: DefaultImpls1<ShowImplComponent, AppA, Delegate = Provider>,
            Provider: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
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
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
        > for AppA {
            type Delegate = ShowWithDisplay;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppA
        where
            ShowWithDisplay: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_AppA_be4bc3b4 : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"AppA\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"__Key__\",\"provider\":\"__Value__\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 4, :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Nil > > > > > , :: cgp_fork :: macro_prelude :: PathCons < ShowImplComponent, :: cgp_fork :: macro_prelude :: PathCons < T, __Wildcard__ > > >\",\"provider\":\"Provider\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 4, :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Nil > > > > > , :: cgp_fork :: macro_prelude :: PathCons < ShowImplComponent, :: cgp_fork :: macro_prelude :: PathCons < u64, __Wildcard__ > > >\",\"provider\":\"ShowWithDisplay\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    AppA {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}

pub struct AppB;

snapshot_delegate_components! {
    delegate_components! {
        AppB {
            namespace DefaultNamespace;

            for <T, Provider> in DefaultShowComponents {
                @test.ShowImplComponent.T: Provider,
            }
        }
    }

    expand_default_impls_app_b(output) {
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
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for AppB
        where
            T: DefaultShowComponents<AppB, Delegate = Provider>,
        {
            type Delegate = Provider;
        }
        impl<
            __Wildcard__,
            T,
            Provider,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            T: DefaultShowComponents<AppB, Delegate = Provider>,
            Provider: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_AppB_1f986a13 : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"AppB\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"__Key__\",\"provider\":\"__Value__\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 4, :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Nil > > > > > , :: cgp_fork :: macro_prelude :: PathCons < ShowImplComponent, :: cgp_fork :: macro_prelude :: PathCons < T, __Wildcard__ > > >\",\"provider\":\"Provider\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    AppB {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}

pub struct AppC;

snapshot_delegate_components! {
    delegate_components! {
        AppC {
            namespace ExtendedNamespace;

            for <T, Provider> in DefaultImpls1<ShowImplComponent> {
                @test.ShowImplComponent.T: Provider,
            }

            @test.ShowImplComponent.u64:
                ShowWithDisplay,
        }
    }

    expand_default_impls_app_c(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppC
        where
            __Key__: ExtendedNamespace<AppC, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppC
        where
            __Key__: ExtendedNamespace<AppC, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for AppC
        where
            T: DefaultImpls1<ShowImplComponent, AppC, Delegate = Provider>,
        {
            type Delegate = Provider;
        }
        impl<
            __Wildcard__,
            T,
            Provider,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppC
        where
            T: DefaultImpls1<ShowImplComponent, AppC, Delegate = Provider>,
            Provider: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
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
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
        > for AppC {
            type Delegate = ShowWithDisplay;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppC
        where
            ShowWithDisplay: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_AppC_ab0bb906 : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"AppC\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"__Key__\",\"provider\":\"__Value__\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 4, :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Nil > > > > > , :: cgp_fork :: macro_prelude :: PathCons < ShowImplComponent, :: cgp_fork :: macro_prelude :: PathCons < T, __Wildcard__ > > >\",\"provider\":\"Provider\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 4, :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 't', :: cgp_fork :: macro_prelude :: Nil > > > > > , :: cgp_fork :: macro_prelude :: PathCons < ShowImplComponent, :: cgp_fork :: macro_prelude :: PathCons < u64, __Wildcard__ > > >\",\"provider\":\"ShowWithDisplay\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    AppC {
        ShowImplComponent: [
            String,
            u64,
            u32,
        ]
    }
}
