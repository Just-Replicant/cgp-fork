//! `MatchWith<_, First>`: the input is `(enum, args)` and every handler sees the args
//! (ported from `extensible_variants/shape_dispatch.rs`, `Container`).

use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

use super::shapes::{Circle, Rectangle, Shape, ShapePlus, Triangle};

pub trait Contains {
    fn contains(&self, x: f64, y: f64) -> bool;
}

impl Contains for Circle {
    fn contains(&self, x: f64, y: f64) -> bool {
        x * x + y * y <= self.radius * self.radius
    }
}

impl Contains for Rectangle {
    fn contains(&self, x: f64, y: f64) -> bool {
        (0.0..=self.width).contains(&x) && (0.0..=self.height).contains(&y)
    }
}

impl Contains for Triangle {
    fn contains(&self, x: f64, y: f64) -> bool {
        x >= 0.0 && y >= 0.0 && x / self.base + y / self.height <= 1.0
    }
}

#[cgp_new_provider]
impl<Context, Code, T: Contains> Handler<Context, Code, (T, (f64, f64))> for ContainsPoint {
    type Output = bool;

    fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        (shape, (x, y)): (T, (f64, f64)),
    ) -> bool {
        shape.contains(x, y)
    }
}

impl Contains for Shape {
    fn contains(&self, x: f64, y: f64) -> bool {
        // Borrowed dispatch: `Ref` mode, so the handlers get `&Circle` and `&Rectangle`.
        MatchValues::<ContainsRef, First, Ref>::handle(&(), NoCode, (self, (x, y)))
    }
}

impl Contains for ShapePlus {
    fn contains(&self, x: f64, y: f64) -> bool {
        MatchValues::<ContainsRef, First, Ref>::handle(&(), NoCode, (self, (x, y)))
    }
}

#[cgp_new_provider]
impl<Context, Code, T: Contains> Handler<Context, Code, (&T, (f64, f64))> for ContainsRef {
    type Output = bool;

    fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        (shape, (x, y)): (&T, (f64, f64)),
    ) -> bool {
        shape.contains(x, y)
    }
}

#[test]
fn first_strategy_threads_the_args() {
    type ByHand = MatchWith<
        Product![
            OnField<Symbol!("Circle"), FieldValue<ContainsPoint, First>, First>,
            OnField<Symbol!("Rectangle"), FieldValue<ContainsPoint, First>, First>,
        ],
        First,
    >;

    let circle = Shape::Circle(Circle { radius: 1.0 });
    assert!(ByHand::handle(&(), NoCode, (circle, (0.5, 0.5))));

    let rectangle = Shape::Rectangle(Rectangle {
        width: 1.0,
        height: 1.0,
    });
    assert!(!ByHand::handle(&(), NoCode, (rectangle, (2.0, 0.5))));
}

#[test]
fn first_strategy_with_derived_handlers_and_borrowed_input() {
    let shape = ShapePlus::Triangle(Triangle {
        base: 2.0,
        height: 2.0,
    });
    assert!(shape.contains(0.5, 0.5));
    assert!(!shape.contains(1.5, 1.5));

    assert!(MatchValues::<ContainsPoint, First>::handle(
        &(),
        NoCode,
        (Shape::Circle(Circle { radius: 2.0 }), (1.0, 1.0))
    ));
    assert!(!block_on(
        MatchValues::<Promote<ContainsPoint>, First>::handle_async(
            &(),
            NoCode,
            (Shape::Circle(Circle { radius: 2.0 }), (3.0, 0.0))
        )
    ));
}
