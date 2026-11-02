//! The `=>` redirect operator written directly in a `delegate_components!` table.
//!
//! `=>` sets an entry's `Delegate` to a `RedirectLookup` along an `@`-path instead
//! of naming a provider, so the provider is decided wherever the path lands. Two
//! uses appear here. `FooProviderComponent => @FooProviderComponent` roots a
//! component's route at its own name, which is **exactly** what `open
//! FooProviderComponent;` generates — the snapshots below are the equivalence,
//! since `App` and `OpenApp` differ only in which spelling they use and their
//! golden output is identical. And `[BarProviderComponent, BazProviderComponent]
//! => @shared` points two components at one slot, answered by a single
//! `@shared: DummyImpl` entry.
//!
//! This is the only place the suite exercises `=>` outside a `cgp_namespace!`
//! body. The two `delegate_components!` snapshots are the golden output this file
//! owns; the components and providers are incidental scaffolding.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/delegate_components.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

// Incidental: one generic component dispatched per type, and two plain ones
// sharing a redirect slot.
#[cgp_component(FooProvider)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

#[cgp_component(BarProvider)]
pub trait Bar {
    fn bar(&self);
}

#[cgp_component(BazProvider)]
pub trait Baz {
    fn baz(&self);
}

// Incidental: plain providers for the three components.
#[cgp_impl(new DummyFoo)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

#[cgp_impl(new DummyImpl)]
impl BarProvider {
    fn bar(&self) {}
}

#[cgp_impl(DummyImpl)]
impl BazProvider {
    fn baz(&self) {}
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            // The spelled-out form of `open FooProviderComponent;`.
            FooProviderComponent =>
                @FooProviderComponent,

            @FooProviderComponent.String:
                DummyFoo,

            // A list key redirecting two components onto one shared slot, which
            // the single entry below answers.
            [BarProviderComponent, BazProviderComponent] =>
                @shared,

            @shared:
                DummyImpl,
        }
    }

    expand_redirect_mapping_app(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<FooProviderComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>>
        for App {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<BarProviderComponent> for App {
            type Delegate = RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<BarProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >: IsProviderFor<BarProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<BazProviderComponent> for App {
            type Delegate = RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<BazProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >: IsProviderFor<BazProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<
                    6,
                    Chars<'s', Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>>,
                >,
                __Wildcard__,
            >,
        > for App {
            type Delegate = DummyImpl;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<
                Symbol<
                    6,
                    Chars<'s', Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>>,
                >,
                __Wildcard__,
            >,
            __Context__,
            __Params__,
        > for App
        where
            DummyImpl: IsProviderFor<
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    __Wildcard__,
                >,
                __Context__,
                __Params__,
            >,
        {}
        trait __CanUseApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseApp<FooProviderComponent, String> for App {}
        impl __CanUseApp<BarProviderComponent, ()> for App {}
        impl __CanUseApp<BazProviderComponent, ()> for App {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_App_a19fae0b : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"App\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"FooProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < App, :: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: Nil > >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < String, __Wildcard__ > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null},{\"component\":\"BarProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < App, :: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 6, :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 'h', :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'r', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 'd', :: cgp_fork :: macro_prelude :: Nil > > > > > > > , :: cgp_fork :: macro_prelude :: Nil > >\",\"cfgs\":[],\"override\":null},{\"component\":\"BazProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < App, :: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 6, :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 'h', :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'r', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 'd', :: cgp_fork :: macro_prelude :: Nil > > > > > > > , :: cgp_fork :: macro_prelude :: Nil > >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < :: cgp_fork :: macro_prelude :: Symbol < 6, :: cgp_fork :: macro_prelude :: Chars < 's', :: cgp_fork :: macro_prelude :: Chars < 'h', :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'r', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Chars < 'd', :: cgp_fork :: macro_prelude :: Nil > > > > > > > , __Wildcard__ >\",\"provider\":\"DummyImpl\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    App {
        FooProviderComponent: String,
        BarProviderComponent,
        BazProviderComponent,
    }
}

// The `open` spelling of the first entry above. Its `DelegateComponent` and
// `IsProviderFor` impls must match `App`'s `FooProviderComponent` pair exactly.
pub struct OpenApp;

snapshot_delegate_components! {
    delegate_components! {
        OpenApp {
            open FooProviderComponent;

            @FooProviderComponent.String:
                DummyFoo,
        }
    }

    expand_redirect_mapping_open_app(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<FooProviderComponent> for OpenApp {
            type Delegate = RedirectLookup<OpenApp, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for OpenApp
        where
            RedirectLookup<
                OpenApp,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>>
        for OpenApp {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            __Context__,
            __Params__,
        > for OpenApp
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        trait __CanUseOpenApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseOpenApp<FooProviderComponent, String> for OpenApp {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_OpenApp_c6d50956 : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"OpenApp\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"FooProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < OpenApp, :: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: Nil > , >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < String, __Wildcard__ > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    #[check_trait(__CheckOpenApp)]
    OpenApp {
        FooProviderComponent: String,
    }
}
