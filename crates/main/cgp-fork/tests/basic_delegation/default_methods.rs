//! A `#[cgp_component]` trait may carry a default method body and a supertrait.
//!
//! The default body is copied into the generated provider trait, so a provider
//! declared with an empty `#[cgp_impl]` (here `UseDefault`) inherits it. This is
//! the distinct component-expansion variant with a supertrait bound and a default
//! method, kept alongside the plain snapshot in `component_macro`.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_component.md (Snapshots) for this
//! supertrait-plus-default-method variant, and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md and
//! cgp-knowledge-base-fork/cgp/reference/attributes/extend.md for the user-facing semantics.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_component;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str {
        "John"
    }
}

snapshot_cgp_component! {
    #[cgp_component(Greeter)]
    pub trait CanGreet: HasName {
        fn greet(&self) -> String {
            format!("Hello, {}!", self.name())
        }
    }

    expand_can_greet(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::on_unimplemented(
            message = "`{Self}` does not implement `CanGreet`",
            label = "`CanGreet` is not implemented for `{Self}`",
            note = "`CanGreet` is implemented through the component `GreeterComponent`: wire a provider for `GreeterComponent` in the `delegate_components!` table of `{Self}`",
            note = "if a provider is wired, `check_components!` on `{Self}` with `GreeterComponent` names the requirement it is missing"
        )]
        pub trait CanGreet: HasName {
            fn greet(&self) -> String {
                format!("Hello, {}!", self.name())
            }
        }
        impl<__Context__> CanGreet for __Context__
        where
            __Context__: HasName,
            __Context__: Greeter<__Context__>,
        {
            fn greet(&self) -> String {
                __Context__::greet(self)
            }
        }
        #[diagnostic::on_unimplemented(
            message = "`{Self}` does not provide `GreeterComponent` for `{__Context__}`",
            label = "`{Self}` neither implements `Greeter` nor wires `GreeterComponent`",
            note = "if `{Self}` is a context or a wiring table, add `GreeterComponent: SomeProvider` to its `delegate_components!` table",
            note = "if `{Self}` is a provider, implement `Greeter` for it with `#[cgp_impl]` or `#[cgp_provider(GreeterComponent)]`"
        )]
        pub trait Greeter<__Context__>: IsProviderFor<GreeterComponent, __Context__, ()>
        where
            __Context__: HasName,
        {
            fn greet(__context__: &__Context__) -> String {
                format!("Hello, {}!", __context__.name())
            }
        }
        impl<__Provider__, __Context__> Greeter<__Context__> for __Provider__
        where
            __Context__: HasName,
            __Provider__: DelegateComponent<GreeterComponent>
                + IsProviderFor<GreeterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                GreeterComponent,
            >>::Delegate: Greeter<__Context__>,
        {
            fn greet(__context__: &__Context__) -> String {
                <__Provider__ as DelegateComponent<
                    GreeterComponent,
                >>::Delegate::greet(__context__)
            }
        }
        /// Component name for [`CanGreet`]. Choose its provider in `delegate_components!`.
        pub struct GreeterComponent;
        impl<__Context__> Greeter<__Context__> for UseContext
        where
            __Context__: HasName,
            __Context__: CanGreet,
        {
            fn greet(__context__: &__Context__) -> String {
                __Context__::greet(__context__)
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasName,
            __Context__: CanGreet,
        {}
        impl<__Context__, __Components__, __Path__> Greeter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasName,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: Greeter<__Context__>,
        {
            fn greet(__context__: &__Context__) -> String {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::greet(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<GreeterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasName,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<GreeterComponent, __Context__, ()>
                + Greeter<__Context__>,
        {}
        "#)
    }
}

pub struct UseDefault;

// Empty provider impls pick up the default method bodies from the provider trait.
#[cgp_impl(UseDefault)]
impl<Context> NameGetter for Context {}

#[cgp_impl(UseDefault)]
impl<Context: HasName> Greeter for Context {}

#[test]
fn test_default_method_impl() {
    pub struct App;

    delegate_components! {
        App {
            [
                NameGetterComponent,
                GreeterComponent,
            ]:
                UseDefault,
        }
    }

    assert_eq!(App.greet(), "Hello, John!");
}
