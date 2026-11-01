# The `shape` AST stack

The `shape` stack ([`macro_core/types/shape`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_core/types/shape)) holds the two AST
types behind the type-level shape macros. Both store the input in the form `#[derive(HasFields)]`
receives, so their `eval` can call the derive's encoder unchanged.

## `StructType`

```rust
pub struct StructType {
    pub fields: Fields,
}
```

`fields` is the `syn::Fields` a struct declaration with the same body would have, built by
`parse_shape_fields`. `eval` returns `item_fields_to_product_type(&self.fields, …)`: a `Product!`
of `Field` entries keyed by `Symbol!` (named) or `Index<N>` (positional), a bare type for a
one-field tuple body, and `Nil` for an empty body.

## `EnumType`

```rust
pub struct EnumType {
    pub variants: Punctuated<Variant, Comma>,
}
```

`variants` is the `syn::Variant` list of the equivalent enum, with attributes and discriminants
always empty (both are rejected while parsing). `eval` returns `variants_to_sum_type(…)`: a `Sum!`
of `Field<Symbol!("Variant"), Payload>` entries ending in `Void`.

Entry points: [entrypoints/struct.md](../entrypoints/struct.md) and
[entrypoints/enum.md](../entrypoints/enum.md).
