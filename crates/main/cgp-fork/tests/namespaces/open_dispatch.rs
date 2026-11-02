//! The `open` statement: per-value dispatch of a generic-parameter component.
//!
//! `open { FooProviderComponent, BarProviderComponent };` opens two generic
//! components for per-value wiring, then `@Component.Key: Provider` entries assign
//! a provider per value of the dispatch parameter (a brace group,
//! `@BarProviderComponent.{u32, u64, ..}: DummyBar`, shares one provider across
//! several values). `open` is a lightweight special case of `RedirectLookup`, so
//! each opened component gets a `DelegateComponent` pointing at
//! `RedirectLookup<App, PathCons<Component, Nil>>`. The `delegate_components!`
//! snapshot is the canonical `open` golden output this concept owns; the two
//! components and their providers are incidental scaffolding written plainly.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md and
//! cgp-knowledge-base-fork/cgp/reference/providers/redirect_lookup.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

pub struct App;

// Incidental: two plain generic components dispatched via `open` below.
#[cgp_component(FooProvider)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

#[cgp_component(BarProvider)]
pub trait Bar<T> {
    fn bar(&self, value: &T);
}

// Incidental: plain per-value providers for the two components.
#[cgp_impl(new DummyFoo)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

#[cgp_impl(new DummyBar)]
impl<T> BarProvider<T> {
    fn bar(&self, _value: &T) {}
}

snapshot_delegate_components! {
    delegate_components! {
        App {
            open {FooProviderComponent, BarProviderComponent};

            // FooProviderComponent =>
            //     @FooProviderComponent,
            // BarProviderComponent =>
            //     @BarProviderComponent,

            @FooProviderComponent.String:
                DummyFoo,
            @BarProviderComponent.{u32, u64, bool, usize, isize}:
                DummyBar,
        }
    }

    expand_open_app(output) {
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
        impl DelegateComponent<BarProviderComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<BarProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<BarProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<BarProviderComponent, Nil>,
            >: IsProviderFor<BarProviderComponent, __Context__, __Params__>,
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
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<u32, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<u32, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<u32, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<u64, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<bool, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<bool, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<bool, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<usize, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<usize, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<usize, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        #[diagnostic::do_not_recommend]
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<BarProviderComponent, PathCons<isize, __Wildcard__>>>
        for App {
            type Delegate = DummyBar;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<
            PathCons<BarProviderComponent, PathCons<isize, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyBar: IsProviderFor<
                PathCons<BarProviderComponent, PathCons<isize, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        trait __CanUseApp<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseApp<FooProviderComponent, String> for App {}
        impl __CanUseApp<BarProviderComponent, u32> for App {}
        impl __CanUseApp<BarProviderComponent, u64> for App {}
        impl __CanUseApp<BarProviderComponent, bool> for App {}
        impl __CanUseApp<BarProviderComponent, usize> for App {}
        impl __CanUseApp<BarProviderComponent, isize> for App {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_App_6ff1b36e : &
            'static str =
            "{\"kind\":\"context\",\"context\":\"App\",\"generics\":\"\",\"unchecked\":false,\"cfgs\":[],\"entries\":[{\"component\":\"FooProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < App, :: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: Nil > , >\",\"cfgs\":[],\"override\":null},{\"component\":\"BarProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: RedirectLookup < App, :: cgp_fork :: macro_prelude :: PathCons < BarProviderComponent, :: cgp_fork :: macro_prelude :: Nil > , >\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < FooProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < String, __Wildcard__ > >\",\"provider\":\"DummyFoo\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < BarProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < u32, __Wildcard__ > >\",\"provider\":\"DummyBar\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < BarProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < u64, __Wildcard__ > >\",\"provider\":\"DummyBar\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < BarProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < bool, __Wildcard__ > >\",\"provider\":\"DummyBar\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < BarProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < usize, __Wildcard__ > >\",\"provider\":\"DummyBar\",\"cfgs\":[],\"override\":null},{\"component\":\":: cgp_fork :: macro_prelude :: PathCons < BarProviderComponent, :: cgp_fork :: macro_prelude :: PathCons < isize, __Wildcard__ > >\",\"provider\":\"DummyBar\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

check_components! {
    App {
        FooProviderComponent:
            String,
        BarProviderComponent: [
            u32,
            u64,
            bool,
            usize,
            isize,
        ],
    }
}
