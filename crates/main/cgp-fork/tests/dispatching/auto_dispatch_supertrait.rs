//! A dispatch trait with a supertrait.
//!
//! The blanket impl requires the trait's supertraits of the enum, so the enum
//! must provide them. Here the supertrait is itself a dispatch trait, so the enum
//! gets it through that trait's own blanket impl, and the subtrait's per-payload
//! impls can call the supertrait's method.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_auto_dispatch;

pub struct Circle;

pub struct Square;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[cgp_auto_dispatch]
pub trait HasName {
    fn name(&self) -> &'static str;
}

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanGreet: HasName {
        fn greet(&self) -> String;
    }

    expand_supertrait(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanGreet: HasName {
            fn greet(&self) -> String;
        }
        impl<__Variants__> CanGreet for __Variants__
        where
            unified::MatchValues<
                ComputeGreet,
                unified::All,
                Ref,
            >: for<'__a__> unified::Handler<(), (), &'__a__ __Variants__, Output = String>,
            __Variants__: HasExtractor,
            __Variants__: HasName,
        {
            fn greet(&self) -> String {
                <unified::MatchValues<
                    ComputeGreet,
                    unified::All,
                    Ref,
                > as unified::Handler<
                    _,
                    _,
                    _,
                >>::handle(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        fn __compute_greet__<'__a__, __Variants__: CanGreet>(
            __Variants__: &'__a__ __Variants__,
        ) -> String {
            __Variants__.greet()
        }
        impl<
            '__a__,
            __Variants__: CanGreet,
            __Context__,
            __Code__,
        > unified::Handler<__Context__, __Code__, (&'__a__ __Variants__)> for ComputeGreet {
            type Output = String;
            fn handle(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0): (&'__a__ __Variants__),
            ) -> Self::Output {
                __compute_greet__(arg_0)
            }
        }
        impl<
            '__a__,
            __Variants__: CanGreet,
            __Context__,
            __Code__,
        > IsProviderFor<
            unified::HandlerComponent,
            __Context__,
            (__Code__, (&'__a__ __Variants__)),
        > for ComputeGreet {}
        /// Provider `ComputeGreet` for [`Handler`].
        pub struct ComputeGreet;
        #[diagnostic::do_not_recommend]
        impl DelegateComponent<unified::AsyncHandlerComponent> for ComputeGreet {
            type Delegate = unified::Promote<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<unified::AsyncHandlerComponent, __Context__, __Params__> for ComputeGreet
        where
            unified::Promote<
                Self,
            >: IsProviderFor<unified::AsyncHandlerComponent, __Context__, __Params__>,
        {}
        __cgp_compat_0_10! {
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            ComputerComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            ComputerComponent, __Context__, __Params__ > for ComputeGreet where Compat < Self > :
            IsProviderFor < ComputerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            ComputerRefComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            ComputerRefComponent, __Context__, __Params__ > for ComputeGreet where Compat < Self
            > : IsProviderFor < ComputerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            TryComputerComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            TryComputerComponent, __Context__, __Params__ > for ComputeGreet where Compat < Self
            > : IsProviderFor < TryComputerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            TryComputerRefComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            TryComputerRefComponent, __Context__, __Params__ > for ComputeGreet where Compat <
            Self > : IsProviderFor < TryComputerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            AsyncComputerComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            AsyncComputerComponent, __Context__, __Params__ > for ComputeGreet where Compat <
            Self > : IsProviderFor < AsyncComputerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            AsyncComputerRefComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            AsyncComputerRefComponent, __Context__, __Params__ > for ComputeGreet where Compat <
            Self > : IsProviderFor < AsyncComputerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            HandlerComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            HandlerComponent, __Context__, __Params__ > for ComputeGreet where Compat < Self > :
            IsProviderFor < HandlerComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            HandlerRefComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            HandlerRefComponent, __Context__, __Params__ > for ComputeGreet where Compat < Self >
            : IsProviderFor < HandlerRefComponent, __Context__, __Params__ > {}
            #[diagnostic::do_not_recommend] #[allow(deprecated)] impl DelegateComponent <
            ProducerComponent > for ComputeGreet { type Delegate = Compat < Self >; }
            #[allow(deprecated)] impl < __Context__, __Params__ : ? Sized > IsProviderFor <
            ProducerComponent, __Context__, __Params__ > for ComputeGreet where Compat < Self > :
            IsProviderFor < ProducerComponent, __Context__, __Params__ > {}
        }
        ")
    }
}

impl HasName for Circle {
    fn name(&self) -> &'static str {
        "circle"
    }
}

impl HasName for Square {
    fn name(&self) -> &'static str {
        "square"
    }
}

impl CanGreet for Circle {
    fn greet(&self) -> String {
        format!("hello, {}", self.name())
    }
}

impl CanGreet for Square {
    fn greet(&self) -> String {
        format!("hi, {}", self.name())
    }
}

#[test]
fn test_supertrait() {
    assert_eq!(Shape::Square(Square).greet(), "hi, square");
}
