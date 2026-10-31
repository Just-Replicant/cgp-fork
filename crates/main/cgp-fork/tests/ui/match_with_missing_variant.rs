// `MatchWith` with a handler list that misses a variant of the enum.
use cgp_fork::handler::prelude::*;
use cgp_fork::handler::{Handler, HandlerComponent};
use cgp_fork::prelude::*;

#[derive(CgpData)]
pub enum Shape {
    Circle(f64),
    Square(f64),
}

#[cgp_new_provider]
impl<Context, Code> Handler<Context, Code, f64> for Area {
    type Output = f64;

    fn handle(_context: &Context, _code: PhantomData<Code>, input: f64) -> f64 {
        input
    }
}

fn main() {
    type OnlyCircle = MatchWith<Product![OnField<Symbol!("Circle"), FieldValue<Area>>]>;

    let _ = OnlyCircle::handle(&(), NoCode, Shape::Square(1.0));
}
