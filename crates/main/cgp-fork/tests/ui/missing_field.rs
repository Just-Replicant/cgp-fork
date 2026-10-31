// A `#[cgp_auto_getter]` trait is required of a context that lacks the field it reads.
// The message names the getter trait and the field instead of a type-level string.
use cgp_fork::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct App {
    pub title: String,
}

fn print_name<Context: HasName>(context: &Context) -> &str {
    context.name()
}

fn main() {
    let app = App {
        title: "app".to_owned(),
    };
    let _ = print_name(&app);
}
