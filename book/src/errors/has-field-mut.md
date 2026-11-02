# `HasFieldMut`

```text
`Person` does not have a mutable field that this code writes
```

The tag spells the field the same way as [`HasField`](has-field.md):
`Symbol<N, Chars<'a', Chars<'b', ...>>>` reads as `"ab..."`.

Add the field and derive `HasField` or `CgpData`. The derive implements both
the shared borrow and the mutable borrow. A field that is not `pub` is still
visible to the derive when the derive is on the struct.
