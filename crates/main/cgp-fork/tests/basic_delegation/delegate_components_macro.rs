//! Canonical expansion of `delegate_components!` for a standalone provider table.
//!
//! `delegate_components!` builds the type-level wiring table: for each entry it
//! emits a `DelegateComponent` impl (mapping a component key to its provider) and
//! an `IsProviderFor` impl that forwards the provider's dependencies. This file
//! owns the reference snapshot, including the `->` forwarding form that delegates
//! to another table's entry.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

snapshot_delegate_components! {
    delegate_components! {
        #[unchecked]
        new FooComponents {
            Index<0>: u64,
            Index<1>: String,
        }
    }

    expand_foo_component(output) {
        insta::assert_snapshot!(output, @r#"
        /// Wiring table `FooComponents`.
        pub struct FooComponents;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<Index<0>> for FooComponents {
            type Delegate = u64;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<Index<0>, __Context__, __Params__>
        for FooComponents
        where
            u64: IsProviderFor<Index<0>, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<Index<1>> for FooComponents {
            type Delegate = String;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<Index<1>, __Context__, __Params__>
        for FooComponents
        where
            String: IsProviderFor<Index<1>, __Context__, __Params__>,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_FooComponents_0415bc1d
            : & 'static str =
            "{\"kind\":\"context\",\"context\":\"FooComponents\",\"generics\":\"\",\"unchecked\":true,\"cfgs\":[],\"entries\":[{\"component\":\"Index < 0 >\",\"provider\":\"u64\",\"cfgs\":[],\"override\":null},{\"component\":\"Index < 1 >\",\"provider\":\"String\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

// The `->` form forwards a key to another table's delegate instead of assigning
// a provider directly.
snapshot_delegate_components! {
    delegate_components! {
        #[unchecked]
        new BarComponents {
            Index<0>:
                FooComponents,
            Index<1> ->
                FooComponents,
        }
    }

    expand_bar_component(output) {
        insta::assert_snapshot!(output, @r#"
        /// Wiring table `BarComponents`.
        pub struct BarComponents;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<Index<0>> for BarComponents {
            type Delegate = FooComponents;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<Index<0>, __Context__, __Params__>
        for BarComponents
        where
            FooComponents: IsProviderFor<Index<0>, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<Index<1>> for BarComponents
        where
            FooComponents: DelegateComponent<Index<1>>,
        {
            type Delegate = <FooComponents as DelegateComponent<Index<1>>>::Delegate;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<Index<1>, __Context__, __Params__>
        for BarComponents
        where
            FooComponents: DelegateComponent<Index<1>>,
            <FooComponents as DelegateComponent<
                Index<1>,
            >>::Delegate: IsProviderFor<Index<1>, __Context__, __Params__>,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_BarComponents_84042bb9
            : & 'static str =
            "{\"kind\":\"context\",\"context\":\"BarComponents\",\"generics\":\"\",\"unchecked\":true,\"cfgs\":[],\"entries\":[{\"component\":\"Index < 0 >\",\"provider\":\"FooComponents\",\"cfgs\":[],\"override\":null},{\"component\":\"Index < 1 >\",\"provider\":\"< FooComponents as :: cgp_fork :: macro_prelude :: DelegateComponent < Index < 1 > > > :: Delegate\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

// Compile-time proof that the table resolves to the expected delegates.
pub trait CheckBarDelegates:
    DelegateComponent<Index<0>, Delegate = FooComponents>
    + DelegateComponent<Index<1>, Delegate = String>
{
}

impl CheckBarDelegates for BarComponents {}
