# `PresetContains`

```text
preset `Base` has no component `GreeterComponent`
```

An `override` entry must name a component the parent preset already wires.
`cgp_preset!` cannot see the parent's keys while it expands, so the expansion
emits `PresetContains` and the compiler checks it.

Write the key on the parent, or drop the `override` and add the key as a new
entry. A bare `override` uses the only parent. With several parents, write
`override(Parent) GreeterComponent: Provider`.
