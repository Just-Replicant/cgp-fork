use core::marker::PhantomData;

/// Looks up a provider in `Components` at `Key`, then appends the component's type parameters.
///
/// Namespace wiring and `open` entries delegate here. The key is a type-level path; the
/// component's own generics are concatenated onto that path before the lookup.
pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);
