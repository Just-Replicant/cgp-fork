# Abstract types

`#[cgp_type]` is a component whose only item is an associated type. The provider
trait defaults to `{Type}TypeProvider` and the key to `{Type}TypeProviderComponent`.
`UseType<T>` sets the type to `T`. `{Type}Of<Context>` is an alias for it.
`WithProvider<Provider>` sets it from a `TypeProvider`.

```rust
use cgp_fork::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

pub struct App;

delegate_components! {
    App {
        ScalarTypeProviderComponent: UseType<u32>,
    }
}

fn main() {
    let value: <App as HasScalarType>::Scalar = 7;
    assert_eq!(value, 7);
}
```

`#[use_type(HasScalarType.Scalar)]` on a component or a `#[cgp_impl]` adds that
abstract type as a dependency and lets the body write `Scalar` for
`<Self as HasScalarType>::Scalar`. A bound on the associated type
(`type Scalar: Copy`) is required of `UseType<T>` as well.

`#[derive_provider(WithProvider)]` outside `#[cgp_component]` writes the
`WithProvider` impl. An abstract type bridges through `TypeProvider`. A
single-method getter bridges through `FieldGetter`.
