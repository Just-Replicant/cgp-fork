//! `delegate_components!` array-key form: several component keys mapping to one
//! provider in a single entry.
//!
//! An array key `[A, B]: Provider` expands to one `DelegateComponent` +
//! `IsProviderFor` impl pair per key, all pointing at the same provider.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::DelegateComponent;
use cgp_fork_macro::snapshot_delegate_components;

pub struct FooKey;
pub struct FooValue;
pub struct BarKey;
pub struct BarValue;
pub struct BazKey;

pub struct Components;

snapshot_delegate_components! {
    delegate_components! {
        #[unchecked]
        Components {
            FooKey: FooValue,
            [
                BarKey,
                BazKey,
            ]:
                BarValue,
        }
    }

    expand_components(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<FooKey> for Components {
            type Delegate = FooValue;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<FooKey, __Context__, __Params__>
        for Components
        where
            FooValue: IsProviderFor<FooKey, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<BarKey> for Components {
            type Delegate = BarValue;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<BarKey, __Context__, __Params__>
        for Components
        where
            BarValue: IsProviderFor<BarKey, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<BazKey> for Components {
            type Delegate = BarValue;
        }
        impl<__Context__, __Params__: ?Sized> IsProviderFor<BazKey, __Context__, __Params__>
        for Components
        where
            BarValue: IsProviderFor<BazKey, __Context__, __Params__>,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const __CGP_WIRING_Components_286b6974 :
            & 'static str =
            "{\"kind\":\"context\",\"context\":\"Components\",\"generics\":\"\",\"unchecked\":true,\"cfgs\":[],\"entries\":[{\"component\":\"FooKey\",\"provider\":\"FooValue\",\"cfgs\":[],\"override\":null},{\"component\":\"BarKey\",\"provider\":\"BarValue\",\"cfgs\":[],\"override\":null},{\"component\":\"BazKey\",\"provider\":\"BarValue\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#)
    }
}

pub trait CheckDelegates:
    DelegateComponent<FooKey, Delegate = FooValue>
    + DelegateComponent<BarKey, Delegate = BarValue>
    + DelegateComponent<BazKey, Delegate = BarValue>
{
}

impl CheckDelegates for Components {}
