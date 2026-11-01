use crate::core::component::{DelegateComponent, IsProviderFor};

/**
    This is a convenient type alias that is used in the same way as [`IsProviderFor`],
    but with the `Self` type being the `Context` type rather than the `Provider` type
    that implements the provider trait.

    The `CanUseComponent` trait is automatically implemented for any CGP `Context` type
    that implements the `DelegateComponent<Component>` trait, and when `Contex::Delegate`
    implements `IsProviderFor<Component, Context, Params>`.

    This trait is used by `check_components!` to check whether a `Context` implements
    a given `Component` through its provider. When there are unsatisfied constraints,
    Rust would show the error messages from the `IsProviderFor` implementation.
*/
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot use the component `{Component}`",
    label = "`{Self}` cannot use `{Component}`",
    note = "`{Self}` must wire `{Component}` to a provider in `delegate_components!`, and that provider's requirements must hold for `{Self}`",
    note = "the notes below name the missing wiring entry or the unmet requirement"
)]
pub trait CanUseComponent<Component, Params: ?Sized = ()> {}

impl<Context, Component, Params: ?Sized> CanUseComponent<Component, Params> for Context
where
    Context: DelegateComponent<Component>,
    Context::Delegate: IsProviderFor<Component, Context, Params>,
{
}
