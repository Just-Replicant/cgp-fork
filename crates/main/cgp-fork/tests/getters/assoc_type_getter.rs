//! `#[cgp_getter]` on a getter trait with a local associated return type
//! (`type Scalar`): the full getter component is generated, including the
//! `UseContext`/`RedirectLookup`/`UseFields`/`UseField`/`WithProvider` provider
//! impls that carry the associated type through. The context then binds the
//! source field by wiring the getter to `UseField`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_getter.md and
//! cgp-knowledge-base-fork/cgp/reference/providers/use_field.md.

use core::ops::Mul;

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_getter;

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasScalar {
        type Scalar: Mul<Output = Self::Scalar> + Clone;

        fn scalar(&self) -> &Self::Scalar;
    }

    expand_has_scalar(output) {
        insta::assert_snapshot!(output, @r#"
        #[diagnostic::on_unimplemented(
            message = "`{Self}` does not implement `HasScalar`",
            label = "`HasScalar` is not implemented for `{Self}`",
            note = "`HasScalar` is implemented through the component `ScalarGetterComponent`: wire a provider for `ScalarGetterComponent` in the `delegate_components!` table of `{Self}`",
            note = "if a provider is wired, `check_components!` on `{Self}` with `ScalarGetterComponent` names the requirement it is missing"
        )]
        pub trait HasScalar {
            type Scalar: Mul<Output = Self::Scalar> + Clone;
            fn scalar(&self) -> &Self::Scalar;
        }
        impl<__Context__> HasScalar for __Context__
        where
            __Context__: ScalarGetter<__Context__>,
        {
            type Scalar = <__Context__ as ScalarGetter<__Context__>>::Scalar;
            fn scalar(&self) -> &Self::Scalar {
                __Context__::scalar(self)
            }
        }
        #[diagnostic::on_unimplemented(
            message = "`{Self}` does not provide `ScalarGetterComponent` for `{__Context__}`",
            label = "`{Self}` neither implements `ScalarGetter` nor wires `ScalarGetterComponent`",
            note = "if `{Self}` is a context or a wiring table, add `ScalarGetterComponent: SomeProvider` to its `delegate_components!` table",
            note = "if `{Self}` is a provider, implement `ScalarGetter` for it with `#[cgp_impl]` or `#[cgp_provider(ScalarGetterComponent)]`"
        )]
        pub trait ScalarGetter<
            __Context__,
        >: IsProviderFor<ScalarGetterComponent, __Context__, ()> {
            type Scalar: Mul<Output = Self::Scalar> + Clone;
            fn scalar(__context__: &__Context__) -> &Self::Scalar;
        }
        impl<__Provider__, __Context__> ScalarGetter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ScalarGetterComponent>
                + IsProviderFor<ScalarGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ScalarGetterComponent,
            >>::Delegate: ScalarGetter<__Context__>,
        {
            type Scalar = <<__Provider__ as DelegateComponent<
                ScalarGetterComponent,
            >>::Delegate as ScalarGetter<__Context__>>::Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                <__Provider__ as DelegateComponent<
                    ScalarGetterComponent,
                >>::Delegate::scalar(__context__)
            }
        }
        /// Component name for [`HasScalar`]. Choose its provider in `delegate_components!`.
        pub struct ScalarGetterComponent;
        impl<__Context__> ScalarGetter<__Context__> for UseContext
        where
            __Context__: HasScalar,
        {
            type Scalar = <__Context__ as HasScalar>::Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __Context__::scalar(__context__)
            }
        }
        impl<__Context__> IsProviderFor<ScalarGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasScalar,
        {}
        impl<__Context__, __Components__, __Path__> ScalarGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: ScalarGetter<__Context__>,
        {
            type Scalar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as ScalarGetter<__Context__>>::Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::scalar(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ScalarGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ScalarGetterComponent, __Context__, ()>
                + ScalarGetter<__Context__>,
        {}
        impl<__Context__, Scalar> ScalarGetter<__Context__> for UseFields
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'s', Chars<'c', Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>>>,
                >,
                Value = Scalar,
            >,
        {
            type Scalar = Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    's',
                                    Chars<
                                        'c',
                                        Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
            }
        }
        impl<__Context__, Scalar> IsProviderFor<ScalarGetterComponent, __Context__, ()>
        for UseFields
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'s', Chars<'c', Chars<'a', Chars<'l', Chars<'a', Chars<'r', Nil>>>>>>,
                >,
                Value = Scalar,
            >,
        {}
        impl<__Context__, Scalar, __Tag__> ScalarGetter<__Context__> for UseField<__Tag__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<__Tag__, Value = Scalar>,
        {
            type Scalar = Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, Scalar, __Tag__> IsProviderFor<ScalarGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Context__: HasField<__Tag__, Value = Scalar>,
        {}
        impl<__Context__, Scalar, __Provider__> ScalarGetter<__Context__>
        for WithProvider<__Provider__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Provider__: FieldGetter<__Context__, ScalarGetterComponent, Value = Scalar>,
        {
            type Scalar = Scalar;
            fn scalar(__context__: &__Context__) -> &Self::Scalar {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<ScalarGetterComponent>,
                )
            }
        }
        impl<
            __Context__,
            Scalar,
            __Provider__,
        > IsProviderFor<ScalarGetterComponent, __Context__, ()> for WithProvider<__Provider__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
            __Provider__: FieldGetter<__Context__, ScalarGetterComponent, Value = Scalar>,
        {}
        "#)
    }
}

#[derive(HasField)]
pub struct App {
    pub scalar: f64,
}

delegate_components! {
    App {
        ScalarGetterComponent:
            UseField<Symbol!("scalar")>,
    }
}

#[test]
fn test_auto_getter_scalar() {
    let app = App { scalar: 2.0 };

    assert_eq!(*app.scalar(), 2.0);
}
