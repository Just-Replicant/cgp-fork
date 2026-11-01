//! `MatchValues` through a shared and a mutable borrow (ported from
//! `extensible_variants/shape_dispatch_ref.rs`).

use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

use super::shapes::{Area, Circle, Rectangle, Scale, Shape, ShapePlus, Triangle, circle_area};

#[cgp_new_provider]
impl<Context, Code, T: Area> Handler<Context, Code, &T> for AreaRef {
    type Output = f64;

    fn handle(_context: &Context, _code: PhantomData<Code>, shape: &T) -> f64 {
        shape.area()
    }
}

#[cgp_new_provider]
impl<Context, Code, T: Scale> Handler<Context, Code, (&mut T, f64)> for ScaleBy {
    type Output = ();

    fn handle(_context: &Context, _code: PhantomData<Code>, (shape, factor): (&mut T, f64)) {
        shape.scale(factor);
    }
}

#[test]
fn ref_mode_borrows_the_variant() {
    let shape = Shape::Rectangle(Rectangle {
        width: 2.0,
        height: 3.0,
    });

    assert_eq!(
        MatchValues::<AreaRef, All, Ref>::handle(&(), NoCode, &shape),
        6.0
    );
    // The enum is still owned by the caller.
    assert_eq!(
        shape,
        Shape::Rectangle(Rectangle {
            width: 2.0,
            height: 3.0
        })
    );

    let circle = ShapePlus::Circle(Circle { radius: 1.0 });
    assert_eq!(
        block_on(MatchValues::<Promote<AreaRef>, All, Ref>::handle_async(
            &(),
            NoCode,
            &circle
        )),
        circle_area(1.0)
    );
}

#[test]
fn mut_mode_changes_the_variant_in_place() {
    let mut shape = ShapePlus::Triangle(Triangle {
        base: 1.0,
        height: 2.0,
    });

    MatchValues::<ScaleBy, First, Mut>::handle(&(), NoCode, (&mut shape, 3.0));

    assert_eq!(
        shape,
        ShapePlus::Triangle(Triangle {
            base: 3.0,
            height: 6.0
        })
    );

    let mut async_shape = Shape::Circle(Circle { radius: 2.0 });
    block_on(MatchValues::<Promote<ScaleBy>, First, Mut>::handle_async(
        &(),
        NoCode,
        (&mut async_shape, 0.5),
    ));
    assert_eq!(async_shape, Shape::Circle(Circle { radius: 1.0 }));
}

/// A blanket impl drives dispatch from a trait method, for every enum whose variants scale.
pub trait CanScale {
    fn scale_all(&mut self, factor: f64);
}

impl<Context> CanScale for Context
where
    MatchValues<ScaleBy, First, Mut>: for<'a> Handler<(), (), (&'a mut Context, f64), Output = ()>,
{
    fn scale_all(&mut self, factor: f64) {
        MatchValues::<ScaleBy, First, Mut>::handle(&(), NoCode, (self, factor));
    }
}

#[test]
fn blanket_impl_over_mut_dispatch() {
    let mut shape = Shape::Circle(Circle { radius: 2.0 });
    shape.scale_all(0.5);
    assert_eq!(shape, Shape::Circle(Circle { radius: 1.0 }));
}
