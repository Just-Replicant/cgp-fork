//! `delegate_components!` shorthand for the obvious type and getter providers.
//!
//! `type Name = Type` is `NameTypeProviderComponent: UseType<Type>`.
//! `getter field` is `FieldGetterComponent: UseField` of that field's symbol.
//! The explicit `Key: Provider` form is unchanged.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_delegate_components;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

pub struct GreetHello;

snapshot_delegate_components! {
    delegate_components! {
        #[unchecked]
        new PersonComponents {
            type Name = String,
            getter name,
            GreeterComponent: GreetHello,
        }
    }

    expand_shorthand(output) {
        insta::assert_snapshot!(output, @r#"
        /// Wiring table `PersonComponents`.
        pub struct PersonComponents;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<NameTypeProviderComponent> for PersonComponents {
            type Delegate = UseType<String>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for PersonComponents
        where
            UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<NameGetterComponent> for PersonComponents {
            type Delegate = UseField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
            >;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<NameGetterComponent, __Context__, __Params__> for PersonComponents
        where
            UseField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<GreeterComponent> for PersonComponents {
            type Delegate = GreetHello;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<GreeterComponent, __Context__, __Params__> for PersonComponents
        where
            GreetHello: IsProviderFor<GreeterComponent, __Context__, __Params__>,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const
            __CGP_WIRING_PersonComponents_f9176d8e : & 'static str =
            "{\"kind\":\"context\",\"context\":\"PersonComponents\",\"generics\":\"\",\"unchecked\":true,\"cfgs\":[],\"entries\":[{\"component\":\"NameTypeProviderComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: UseType < String >\",\"cfgs\":[],\"override\":null},{\"component\":\"NameGetterComponent\",\"provider\":\":: cgp_fork :: macro_prelude :: UseField < :: cgp_fork :: macro_prelude :: Symbol < 4, :: cgp_fork :: macro_prelude :: Chars < 'n', :: cgp_fork :: macro_prelude :: Chars < 'a', :: cgp_fork :: macro_prelude :: Chars < 'm', :: cgp_fork :: macro_prelude :: Chars < 'e', :: cgp_fork :: macro_prelude :: Nil > > > > > >\",\"cfgs\":[],\"override\":null},{\"component\":\"GreeterComponent\",\"provider\":\"GreetHello\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#);
    }
}

snapshot_delegate_components! {
    delegate_components! {
        #[unchecked]
        new ExplicitComponents {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<Symbol!("name")>,
            GreeterComponent: GreetHello,
        }
    }

    expand_explicit(output) {
        insta::assert_snapshot!(output, @r#"
        /// Wiring table `ExplicitComponents`.
        pub struct ExplicitComponents;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<NameTypeProviderComponent> for ExplicitComponents {
            type Delegate = UseType<String>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>
        for ExplicitComponents
        where
            UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<NameGetterComponent> for ExplicitComponents {
            type Delegate = UseField<Symbol!("name")>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<NameGetterComponent, __Context__, __Params__> for ExplicitComponents
        where
            UseField<
                Symbol!("name"),
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<GreeterComponent> for ExplicitComponents {
            type Delegate = GreetHello;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<GreeterComponent, __Context__, __Params__> for ExplicitComponents
        where
            GreetHello: IsProviderFor<GreeterComponent, __Context__, __Params__>,
        {}
        ::cgp_fork::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)] const
            __CGP_WIRING_ExplicitComponents_b5d6bd54 : & 'static str =
            "{\"kind\":\"context\",\"context\":\"ExplicitComponents\",\"generics\":\"\",\"unchecked\":true,\"cfgs\":[],\"entries\":[{\"component\":\"NameTypeProviderComponent\",\"provider\":\"UseType < String >\",\"cfgs\":[],\"override\":null},{\"component\":\"NameGetterComponent\",\"provider\":\"UseField < Symbol! (\\\"name\\\") >\",\"cfgs\":[],\"override\":null},{\"component\":\"GreeterComponent\",\"provider\":\"GreetHello\",\"cfgs\":[],\"override\":null}],\"presets\":[]}";
        }
        "#);
    }
}
