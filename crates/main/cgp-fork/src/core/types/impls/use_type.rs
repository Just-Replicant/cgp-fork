use core::marker::PhantomData;

use cgp_fork_macro::cgp_provider;

use crate::core::component::WithProvider;
use crate::core::types::TypeProviderComponent;
use crate::core::types::traits::TypeProvider;

/**
    The `UseType` pattern is used to implement a CGP abstract type with the
    specified `Type`.

    When a CGP type component is defined using the `#[cgp_type]` macro, a
    provider implementation of `UseType` is automatically generated.
    With `UseType`, users can instantiate an abstract type with the specified
    `Type` without having to manually implement the given provider trait.

    ## Example

    Given the following type component definition:

    ```rust
    # use cgp_fork::prelude::*;
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    pub struct App;

    delegate_components! {
        App {
            NameTypeProviderComponent: UseType<String>,
        }
    }

    let name: <App as HasNameType>::Name = String::from("Alice");
    assert_eq!(name, "Alice");
    ```

    The following `UseType` implementation would be generated (illustrative, not compiled):

    ```rust,ignore
    impl<Context, Type> NameTypeProvider<Context> for UseType<Type> {
        type Name = Type;
    }
    ```
*/
pub struct UseType<Type>(pub PhantomData<Type>);

/// [`WithProvider`]`<`[`UseType`]`<Type>>`, for a consumer trait that should use `Type` directly.
pub type WithType<Type> = WithProvider<UseType<Type>>;

#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Type> TypeProvider<Context, Tag> for UseType<Type> {
    type Type = Type;
}
