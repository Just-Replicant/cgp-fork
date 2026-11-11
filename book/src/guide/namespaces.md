# Namespaces

`cgp_namespace!` is a type-level map from a component to a path. `new Name`
declares the table. `Name: Parent` inherits the parent's entries, and a child
entry replaces the parent's. A namespace cannot inherit itself (`Name: Name` or
`Name: self::Name`).

A context wires the table with `namespace Name;` and maps each path to a
provider. `#[prefix(@Path in Name)]` on a component registers that component
under the path.

```rust
use cgp_fork::prelude::*;

#[cgp_component(FooProvider)]
pub trait Foo {
    fn foo(&self) -> u8;
}

pub struct AppFoo;

cgp_namespace! {
    new AppNamespace {
        FooProviderComponent => @AppFoo,
    }
}

#[cgp_impl(new ReturnOne)]
impl FooProvider {
    fn foo(&self) -> u8 {
        1
    }
}

pub struct App;

delegate_components! {
    App {
        namespace AppNamespace;

        @AppFoo: ReturnOne,
    }
}

fn main() {
    assert_eq!(App.foo(), 1);
}
```

`Path!(@app.error.ErrorRaiserComponent)` builds the same kind of path. A segment
that starts with a lowercase letter and is not a primitive type name becomes a
`Symbol`. The primitive names are `u8`, `u16`, `u32`, `u64`, `u128`, `usize`,
`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `f32`, `f64`, `char`, `bool`, and
`str`. `f` and `u2` are symbols. A segment that starts with an uppercase letter
is a type.
