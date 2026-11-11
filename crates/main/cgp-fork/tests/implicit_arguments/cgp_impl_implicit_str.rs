//! A shared `#[implicit] name: &str` reads any `AsRef<str>` field, so the provider
//! does not require `String` and does not need an allocator.

use core::convert::AsRef;

use cgp_fork::prelude::*;

pub struct Label(pub &'static str);

impl AsRef<str> for Label {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetName)]
impl Greeter {
    fn greet(&self, #[implicit] name: &str) -> String {
        format!("hello {name}")
    }
}

#[derive(HasField)]
pub struct ByString {
    pub name: String,
}

#[derive(HasField)]
pub struct ByLabel {
    pub name: Label,
}

delegate_components! {
    ByString {
        GreeterComponent: GreetName,
    }
}

delegate_components! {
    ByLabel {
        GreeterComponent: GreetName,
    }
}

#[test]
fn string_and_as_ref_fields_both_greet() {
    let by_string = ByString {
        name: "ada".to_owned(),
    };
    assert_eq!(by_string.greet(), "hello ada");

    let by_label = ByLabel { name: Label("ada") };
    assert_eq!(by_label.greet(), "hello ada");
}
