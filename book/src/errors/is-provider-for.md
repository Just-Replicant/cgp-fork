# `IsProviderFor`

```text
`Hello` is not a provider of `GreeterComponent` for `App`
```

The provider wired for `GreeterComponent` does not implement that component for
`App`, or a `where` requirement on its impl does not hold for `App`.

- Implement the provider with `#[cgp_impl]` or `#[cgp_provider(GreeterComponent)]`.
- Or wire a different provider.
- If the impl bounds the context (`Context: HasName`), give `App` that
  capability. The note under this message names the missing bound.
