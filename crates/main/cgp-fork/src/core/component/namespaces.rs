/// Records which namespace entry a component redirects to.
///
/// `#[prefix(path in DefaultNamespace)]` implements this for the component. `Delegate` is a
/// [`RedirectLookup`](crate::core::component::RedirectLookup) along `path`.
pub trait DefaultNamespace<Components> {
    /// The provider a namespace table yields for this component.
    type Delegate;
}

/// Per-type default providers for a component with one extra type parameter.
///
/// `#[default_impl(Key in DefaultImpls1<Component>)]` registers `Key`. A `for <T, Provider> in
/// DefaultImpls1<Component>` loop reads `Delegate` back.
pub trait DefaultImpls1<T, Components> {
    /// The provider registered for `T`.
    type Delegate;
}

/// Per-type default providers for a component with two extra type parameters.
///
/// Same registration and lookup as [`DefaultImpls1`], with a second type in the key.
pub trait DefaultImpls2<T1, T2, Components> {
    type Delegate;
}
