//! A dispatch trait whose method has a raw identifier as its name, `r#type`. The
//! generated computer and helper are named from the unrawed name (`ComputeType` and
//! `__compute_type__`), since `r#` cannot appear inside a longer identifier.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_auto_dispatch;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

pub struct Circle;

pub struct Square;

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait HasKind {
        fn r#type(&self) -> &'static str;
    }

    expand_raw_method_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasKind {
            fn r#type(&self) -> &'static str;
        }
        impl<__Variants__> HasKind for __Variants__
        where
            unified::MatchValues<
                ComputeType,
                unified::All,
                Ref,
            >: for<'__a__> unified::Handler<(), (), &'__a__ __Variants__, Output = &'static str>,
            __Variants__: HasExtractor,
        {
            fn r#type(&self) -> &'static str {
                <unified::MatchValues<
                    ComputeType,
                    unified::All,
                    Ref,
                > as unified::Handler<
                    _,
                    _,
                    _,
                >>::handle(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        fn __compute_type__<'__a__, __Variants__: HasKind>(
            __Variants__: &'__a__ __Variants__,
        ) -> &'static str {
            __Variants__.r#type()
        }
        impl<
            '__a__,
            __Variants__: HasKind,
            __Context__,
            __Code__,
        > unified::Handler<__Context__, __Code__, (&'__a__ __Variants__)> for ComputeType {
            type Output = &'static str;
            fn handle(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (&'__a__ __Variants__),
            ) -> Self::Output {
                __compute_type__(arg_0)
            }
        }
        impl<
            '__a__,
            __Variants__: HasKind,
            __Context__,
            __Code__,
        > IsProviderFor<
            unified::HandlerComponent,
            __Context__,
            (__Code__, (&'__a__ __Variants__)),
        > for ComputeType {}
        /// Provider `ComputeType` for [`Handler`].
        pub struct ComputeType;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<unified::AsyncHandlerComponent> for ComputeType {
            type Delegate = unified::Promote<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<unified::AsyncHandlerComponent, __Context__, __Params__> for ComputeType
        where
            unified::Promote<
                Self,
            >: IsProviderFor<unified::AsyncHandlerComponent, __Context__, __Params__>,
        {}
        __cgp_compat_0_10! {
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            ComputerComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            ComputerComponent, __Context__, __Params__ > for ComputeType where Compat < Self > :
            IsProviderFor < ComputerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            ComputerRefComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            ComputerRefComponent, __Context__, __Params__ > for ComputeType where Compat < Self >
            : IsProviderFor < ComputerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            TryComputerComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            TryComputerComponent, __Context__, __Params__ > for ComputeType where Compat < Self >
            : IsProviderFor < TryComputerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            TryComputerRefComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            TryComputerRefComponent, __Context__, __Params__ > for ComputeType where Compat <
            Self > : IsProviderFor < TryComputerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            AsyncComputerComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            AsyncComputerComponent, __Context__, __Params__ > for ComputeType where Compat < Self
            > : IsProviderFor < AsyncComputerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            AsyncComputerRefComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            AsyncComputerRefComponent, __Context__, __Params__ > for ComputeType where Compat <
            Self > : IsProviderFor < AsyncComputerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            HandlerComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            HandlerComponent, __Context__, __Params__ > for ComputeType where Compat < Self > :
            IsProviderFor < HandlerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            HandlerRefComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            HandlerRefComponent, __Context__, __Params__ > for ComputeType where Compat < Self >
            : IsProviderFor < HandlerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            ProducerComponent > for ComputeType { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            ProducerComponent, __Context__, __Params__ > for ComputeType where Compat < Self > :
            IsProviderFor < ProducerComponent, __Context__, __Params__ > {}
        }
        ")
    }
}

impl HasKind for Circle {
    fn r#type(&self) -> &'static str {
        "circle"
    }
}

impl HasKind for Square {
    fn r#type(&self) -> &'static str {
        "square"
    }
}

#[test]
fn test_raw_method_name_dispatches() {
    assert_eq!(Shape::Circle(Circle).r#type(), "circle");
    assert_eq!(Shape::Square(Square).r#type(), "square");
}
