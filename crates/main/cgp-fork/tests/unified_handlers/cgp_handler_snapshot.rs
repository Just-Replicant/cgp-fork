//! The expansion of `#[cgp_handler]`: a unified `Handler` impl, its `IsProviderFor`
//! impl, the provider struct, and the `AsyncHandlerComponent` wiring through `Promote`.

use cgp_fork::handler::Handler;
use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_handler;

snapshot_cgp_handler! {
    #[cgp_handler]
    fn scaled_width(#[field] width: f64, scale: f64) -> f64 {
        width * scale
    }

    expand_cgp_handler(output) {
        insta::assert_snapshot!(output, @"
        fn scaled_width(width: f64, scale: f64) -> f64 {
            width * scale
        }
        impl<__Context__, __Code__> unified::Handler<__Context__, __Code__, (f64)>
        for ScaledWidth
        where
            __Context__: HasField<
                Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                Value = f64,
            >,
        {
            type Output = f64;
            fn handle(
                context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (f64),
            ) -> Self::Output {
                scaled_width(
                    context
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<
                                    5,
                                    Chars<
                                        'w',
                                        Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>,
                                    >,
                                >,
                            >,
                        )
                        .clone(),
                    arg_0,
                )
            }
        }
        impl<
            __Context__,
            __Code__,
        > IsProviderFor<unified::HandlerComponent, __Context__, (__Code__, (f64))>
        for ScaledWidth
        where
            __Context__: HasField<
                Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                Value = f64,
            >,
        {}
        pub struct ScaledWidth;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<unified::AsyncHandlerComponent> for ScaledWidth {
            type Delegate = unified::Promote<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<unified::AsyncHandlerComponent, __Context__, __Params__> for ScaledWidth
        where
            unified::Promote<
                Self,
            >: IsProviderFor<unified::AsyncHandlerComponent, __Context__, __Params__>,
        {}
        ");
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
}

#[test]
fn scaled_width_handles_its_input() {
    assert_eq!(
        ScaledWidth::handle(&Rectangle { width: 2.0 }, PhantomData::<()>, 3.0),
        6.0
    );
}
