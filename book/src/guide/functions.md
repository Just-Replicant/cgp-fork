# Functions

`#[cgp_fn]` turns a function into a trait and a blanket impl. The trait name is
the function name in PascalCase (`greet` becomes `Greet`). `#[cgp_fn(CanGreet)]`
picks the trait name. A raw `fn r#type` becomes the trait `Type`.

Each `#[implicit]` parameter is removed from the method and read from a
same-named field. The [field table](fields.md) is the list of types. A shared
`&str` or `Option<&str>` implicit accepts any `AsRef<str>` value, including a
newtype. A getter of the same type still requires `String`.

```rust
use cgp_fork::prelude::*;

#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) -> String {
    format!("Hello, {name}!")
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

fn main() {
    let person = Person {
        name: "Ada".to_owned(),
    };
    assert_eq!(person.greet(), "Hello, Ada!");
}
```

No wiring table is required. The blanket impl's bound is the field.

`#[implicit]` on a `#[cgp_impl]` method is the same read, inside a provider you
wire yourself. `#[cgp_auto_impl]` lifts a trait's method bodies into one blanket
provider and removes the bodies from the trait. `#[blanket_trait]` is an alias:
an empty trait plus a blanket impl of its supertraits.

A `#[cgp_fn]` generic may appear only in the return type, because the generic
lives on the trait. A `#[cgp_handler]` or `#[cgp_computer]` parameter may not:
the generated impl cannot determine a type that no argument names. Name it from
an argument, or write the provider by hand.

The generated trait and method get a doc comment when you did not write one
(`The greet operation.`). A doc you wrote stays on the method.
