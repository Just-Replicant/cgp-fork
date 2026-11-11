# Auto getters

```text
`Person` does not implement `HasName`
```

`#[cgp_auto_getter]` names the fields the trait reads. `HasName` that returns
`&str` from `name` needs a `name` field of that type, and `HasField` on the
context:

```rust
use cgp_fork::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

fn main() {
    let person = Person {
        name: "Ferris".to_owned(),
    };
    assert_eq!(person.name(), "Ferris");
}
```

The label says "the field `name`" for one field and "the fields `a`, `b`" for
several. A `String` field satisfies a getter that returns `&str`. An
`#[implicit]` argument of type `&str` also accepts any `AsRef<str>`. The
[fields](../guide/fields.md) page is the full table.
