//! `#[cgp_type]` where the abstract type is bounded by *another* abstract-type
//! component (`type Types: HasScalarType`).
//!
//! This variant shows the bound propagating onto the generated `UseType<Types>`
//! and `WithProvider` impls as `where Types: HasScalarType`, so a context may only
//! wire a concrete `Types` that itself implements `HasScalarType`. The prerequisite
//! `HasScalarType` is written with a plain `#[cgp_type]` (its simple expansion is
//! pinned in `cgp_type_macro`).
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_type.md and
//! cgp-knowledge-base-fork/cgp/concepts/abstract-types.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_type;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasTypes {
        type Types: HasScalarType;
    }

    expand_has_types(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::on_unimplemented(
            message = "`{Self}` does not implement `HasTypes`",
            label = "`HasTypes` is not implemented for `{Self}`",
            note = "`HasTypes` is implemented through the component `TypesTypeProviderComponent`: wire a provider for `TypesTypeProviderComponent` in the `delegate_components!` table of `{Self}`",
            note = "if a provider is wired, `check_components!` on `{Self}` with `TypesTypeProviderComponent` names the requirement it is missing"
        )]
        pub trait HasTypes {
            type Types: HasScalarType;
        }
        impl<__Context__> HasTypes for __Context__
        where
            __Context__: TypesTypeProvider<__Context__>,
        {
            type Types = <__Context__ as TypesTypeProvider<__Context__>>::Types;
        }
        #[diagnostic::on_unimplemented(
            message = "`{Self}` does not provide `TypesTypeProviderComponent` for `{__Context__}`",
            label = "`{Self}` neither implements `TypesTypeProvider` nor wires `TypesTypeProviderComponent`",
            note = "if `{Self}` is a context or a wiring table, add `TypesTypeProviderComponent: SomeProvider` to its `delegate_components!` table",
            note = "if `{Self}` is a provider, implement `TypesTypeProvider` for it with `#[cgp_impl]` or `#[cgp_provider(TypesTypeProviderComponent)]`"
        )]
        pub trait TypesTypeProvider<
            __Context__,
        >: IsProviderFor<TypesTypeProviderComponent, __Context__, ()> {
            type Types: HasScalarType;
        }
        impl<__Provider__, __Context__> TypesTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<TypesTypeProviderComponent>
                + IsProviderFor<TypesTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                TypesTypeProviderComponent,
            >>::Delegate: TypesTypeProvider<__Context__>,
        {
            type Types = <<__Provider__ as DelegateComponent<
                TypesTypeProviderComponent,
            >>::Delegate as TypesTypeProvider<__Context__>>::Types;
        }
        /// Component name for [`HasTypes`]. Choose its provider in `delegate_components!`.
        pub struct TypesTypeProviderComponent;
        impl<__Context__> TypesTypeProvider<__Context__> for UseContext
        where
            __Context__: HasTypes,
        {
            type Types = <__Context__ as HasTypes>::Types;
        }
        impl<__Context__> IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for UseContext
        where
            __Context__: HasTypes,
        {}
        impl<__Context__, __Components__, __Path__> TypesTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: TypesTypeProvider<__Context__>,
        {
            type Types = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as TypesTypeProvider<__Context__>>::Types;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
                + TypesTypeProvider<__Context__>,
        {}
        impl<Types, __Context__> TypesTypeProvider<__Context__> for UseType<Types>
        where
            Types: HasScalarType,
        {
            type Types = Types;
        }
        impl<Types, __Context__> IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for UseType<Types>
        where
            Types: HasScalarType,
        {}
        impl<__Provider__, Types, __Context__> TypesTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            Types: HasScalarType,
            __Provider__: TypeProvider<__Context__, TypesTypeProviderComponent, Type = Types>,
        {
            type Types = Types;
        }
        impl<
            __Provider__,
            Types,
            __Context__,
        > IsProviderFor<TypesTypeProviderComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            Types: HasScalarType,
            __Provider__: TypeProvider<__Context__, TypesTypeProviderComponent, Type = Types>,
        {}
        "#)
    }
}
