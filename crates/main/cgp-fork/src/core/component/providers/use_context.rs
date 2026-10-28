use crate::core::component::WithProvider;

/**
    The `UseContext` pattern is used to define a trivial implementation of
    a provider trait, by forwarding the implementation to the consumer trait
    implementation of the context.

    This pattern is the _dual_ of the blanket implementation of a consumer trait,
    which forwards the implementation to its provider through the `CgpProvider` trait.

    The main use case for `UseContext` is to be used as a _higher-order provider_
    in the argument to a different provider implementation. This decouples the
    dependencies between different CGP traits, and allows non-default providers
    to be used within another provider implementation.

    For obvious reasons, `UseContext` should not be used as the delegation target
    from the context provider, as it would result in a cyclic dependency error.

    The `UseContext` provider is automatically implemented for all CPG traits that
    are generated from `#[cgp_component]`.

    ## Example

    Given the following component definition:

    ```rust
    # use cgp_fork::prelude::*;
    #[cgp_component(Greeter)]
    pub trait CanGreet {
        fn greet(&self) -> String;
    }

    #[cgp_impl(new GreetHello)]
    impl Greeter {
        fn greet(&self) -> String {
            "hello".to_owned()
        }
    }

    pub struct App;

    delegate_components! {
        App {
            GreeterComponent: GreetHello,
        }
    }

    // `UseContext` forwards the provider call back to `App: CanGreet`.
    assert_eq!(<UseContext as Greeter<App>>::greet(&App), "hello");
    ```

    The following `UseContext` implementation would be generated (written out by hand against stand-ins for the generated traits, and compiled):

    ```rust
    # use cgp_fork::prelude::*;
    # pub trait CanGreet {
    #     fn greet(&self) -> String;
    # }
    # // Stand-in for the provider trait `#[cgp_component]` generates.
    # pub trait Greeter<Context> {
    #     fn greet(context: &Context) -> String;
    # }
    impl<Context> Greeter<Context> for UseContext
    where
        Context: CanGreet,
    {
        fn greet(context: &Context) -> String {
            context.greet()
        }
    }
    ```
*/
pub struct UseContext;

/// [`WithProvider`]`<`[`UseContext`]`>`, so a consumer trait can be passed where a provider is expected.
pub type WithContext = WithProvider<UseContext>;
