# Fields

`#[derive(HasField)]` implements `HasField` and `HasFieldMut` for each field.
A named field is keyed by `Symbol!("name")`. A positional field is keyed by
`Index<N>`.

`#[cgp_auto_getter]` implements the trait for any context that has those fields.
No wiring table is required. `#[cgp_getter]` is the same reads, as a component:
wire `UseFields` to use the method's name as the key, or `UseField<Tag>` to name
the key. `WithProvider<Provider>` reads through a `FieldGetter`.

```rust
use cgp_fork::prelude::*;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}

fn main() {
    let person = Person {
        name: "Ada".to_owned(),
    };
    assert_eq!(person.name(), "Ada");
}
```

The component key is the method name plus `GetterComponent` (`name` becomes
`NameGetterComponent`).

A getter that returns `&str` still requires a `String` field, because
`WithProvider` has to name that field type. The read is `AsRef::as_ref`, so the
`String` is borrowed. An `#[implicit]` argument uses the same read and a wider
bound. Both are in this table.

| Written | Getter field | `#[implicit]` field | Read |
|---|---|---|---|
| `&T` | `T` | `T` | `HasField::get_field` |
| `&mut T` | `T` | `T` | `HasFieldMut::get_field_mut` |
| `&str` | `String` | `AsRef<str>` | `AsRef::as_ref` |
| `&mut str` | `String` | `AsMut<str>` | `AsMut::as_mut` |
| `Option<&T>` | `Option<T>` | `Option<T>` | `Option::as_ref` |
| `Option<&mut T>` | `Option<T>` | `Option<T>` | `Option::as_mut` |
| `Option<&str>` | `Option<String>` | `OptionAsRef<str>` | `OptionAsRef::option_as_ref` |
| `Option<&mut str>` | `Option<String>` | `Option<String>` | `Option::as_deref_mut` |
| `&[T]` | `AsRef<[T]>` | `AsRef<[T]>` | `AsRef::as_ref` |
| `&mut [T]` | `AsMut<[T]>` | `AsMut<[T]>` | `AsMut::as_mut` |
| `T` by value | `T` | `T` | `Clone::clone` |

`OptionAsRef<str>` holds for `Option<String>`, `Option<Box<str>>`, and
`Option<U>` where `U: AsRef<str>`. A shared `&str` implicit is the same idea
without the `Option`. Neither read allocates.

`Option<&[T]>` and `Option<&dyn Trait>` are rejected. A slice and a trait object
are unsized, so `Option<[T]>` cannot be a field. Write `Option<&Vec<T>>` or
`Option<&Concrete>`.

Every read is a fully qualified call (`HasField::get_field`, `AsRef::as_ref`),
so an inherent method of the same name on the context is not selected.
