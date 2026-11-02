# `HasField`

```text
`Person` does not have a field that this code reads
```

The field name is the tag type. `Symbol<N, Chars<'n', Chars<'a', Chars<'m', Chars<'e', ...>>>>>`
reads as `"name"`.

Add the field and derive it:

```rust
use cgp_fork::prelude::*;

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

fn main() {
    let person = Person {
        name: "Ferris".to_owned(),
    };
    let tag: PhantomData<Symbol!("name")> = PhantomData;
    assert_eq!(HasField::get_field(&person, tag), "Ferris");
}
```

`#[derive(CgpData)]` derives the same field traits. A getter provider that does
not read a field is the other way to satisfy a `HasField` bound.
