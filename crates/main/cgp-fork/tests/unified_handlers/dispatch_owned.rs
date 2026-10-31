//! `MatchWith` and `MatchValues` on an owned enum (ported from
//! `extensible_variants/shape_dispatch.rs`).

use cgp_fork::core::field::traits::StaticString;
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;
use futures::executor::block_on;

use super::shapes::{Area, Circle, Rectangle, Shape, ShapePlus, Triangle, circle_area};

#[cgp_new_provider]
impl<Context, Code, T: Area> Handler<Context, Code, T> for ComputeArea {
    type Output = f64;

    fn handle(_context: &Context, _code: PhantomData<Code>, shape: T) -> f64 {
        shape.area()
    }
}

#[test]
fn match_with_explicit_variant_handlers() {
    type ByHand = MatchWith<
        Product![
            OnField<Symbol!("Circle"), FieldValue<ComputeArea>>,
            OnField<Symbol!("Rectangle"), FieldValue<ComputeArea>>,
        ],
    >;

    let circle = Shape::Circle(Circle { radius: 1.0 });
    let rectangle = Shape::Rectangle(Rectangle {
        width: 2.0,
        height: 3.0,
    });

    assert_eq!(ByHand::handle(&(), NoCode, circle), circle_area(1.0));
    assert_eq!(ByHand::handle(&(), NoCode, rectangle), 6.0);
}

#[test]
fn match_values_derives_one_handler_per_variant() {
    let triangle = ShapePlus::Triangle(Triangle {
        base: 4.0,
        height: 2.0,
    });

    assert_eq!(
        MatchValues::<ComputeArea>::handle(&(), NoCode, triangle),
        4.0
    );
    assert_eq!(
        block_on(MatchValues::<Promote<ComputeArea>>::handle_async(
            &(),
            NoCode,
            ShapePlus::Circle(Circle { radius: 2.0 })
        )),
        circle_area(2.0)
    );
}

/// The handler of each variant comes back through the context: `MatchValues` defaults to
/// `UseContext`, and the context's table picks the provider by input type.
pub struct App;

delegate_components! {
    App {
        HandlerComponent: UseInputDelegate<new AreaHandlers {
            [
                Circle,
                Rectangle,
                Triangle,
            ]:
                ComputeArea,
            [
                Shape,
                ShapePlus,
            ]: MatchValues,
        }>
    }
}

check_components! {
    App {
        HandlerComponent: [
            ((), Shape),
            ((), ShapePlus),
        ],
    }
}

#[test]
fn match_values_through_the_context() {
    // The consumer trait is imported where the context calls it; next to the provider calls
    // above it would make `MatchValues::<P>::handle` ambiguous.
    use cgp_fork::handler::CanHandle;

    let app = App;

    assert_eq!(
        app.handle(
            NoCode,
            Shape::Rectangle(Rectangle {
                width: 2.0,
                height: 5.0
            })
        ),
        10.0
    );
    assert_eq!(
        app.handle(
            NoCode,
            ShapePlus::Triangle(Triangle {
                base: 3.0,
                height: 2.0
            })
        ),
        3.0
    );
}

/// `MatchFields` passes the whole `Field`, so the handler sees the variant tag.
#[cgp_new_provider]
impl<Context, Code, Tag, Value> Handler<Context, Code, Field<Tag, Value>> for VariantName
where
    Tag: StaticString,
{
    type Output = &'static str;

    fn handle(
        _context: &Context,
        _code: PhantomData<Code>,
        _field: Field<Tag, Value>,
    ) -> Self::Output {
        Tag::VALUE
    }
}

#[test]
fn match_fields_sees_the_tag() {
    let circle = Shape::Circle(Circle { radius: 1.0 });
    assert_eq!(
        MatchFields::<VariantName>::handle(&(), NoCode, circle),
        "Circle"
    );
}
