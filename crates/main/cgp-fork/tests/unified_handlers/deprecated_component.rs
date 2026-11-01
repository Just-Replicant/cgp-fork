//! A `#[deprecated]` consumer trait deprecates the whole component: the provider trait
//! and the component struct inherit it, while the generated impls do not warn the crate
//! that declares the component.

#![deny(deprecated)]

use cgp_fork::prelude::*;

#[cgp_component(OldGreeter)]
#[deprecated(since = "0.11.0", note = "use `CanGreet`")]
pub trait CanGreetOld {
    fn greet_old(&self) -> String;
}

#[allow(deprecated)]
mod uses {
    use super::*;

    #[cgp_new_provider]
    impl<Context> OldGreeter<Context> for GreetHello {
        fn greet_old(_context: &Context) -> String {
            "hello".to_owned()
        }
    }

    pub struct App;

    delegate_components! {
        App {
            OldGreeterComponent: GreetHello,
        }
    }

    #[test]
    fn deprecated_component_still_works() {
        assert_eq!(App.greet_old(), "hello");
    }
}
