# Provider traits

```text
`Hello` does not provide `GreeterComponent` for `App`
```

The provider trait's `Self` is either a provider or a context that forwards the
component. The notes cover both:

- If `Self` is a context or a wiring table, add `GreeterComponent: SomeProvider`
  to its `delegate_components!` table.
- If `Self` is a provider, implement the provider trait with `#[cgp_impl]` or
  `#[cgp_provider(GreeterComponent)]`.

A method call on an unwired context is often reported here, because the
consumer method forwards to the provider trait. Wiring the component is the
fix in that case. See [a component](../guide/components.md).
