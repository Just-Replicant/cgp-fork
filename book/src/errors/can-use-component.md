# `CanUseComponent`

```text
`App` cannot use the component `GreeterComponent`
```

`check_components!` and the check that `delegate_components!` emits for a
concrete context ask whether `App` implements `CanUseComponent<GreeterComponent, Params>`.
That holds when `App` wires the component and the provider's requirements hold
for `App`.

The notes under this message name the missing entry (`DelegateComponent`) or
the unmet requirement (`IsProviderFor`). Read those before changing the table.

A generic component is checked with `()`. If the real parameters are not `()`,
write `#[unchecked]` and a `check_components!` that names them. See
[checked wiring](../guide/checked-wiring.md).
