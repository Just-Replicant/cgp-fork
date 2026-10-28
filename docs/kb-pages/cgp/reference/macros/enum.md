# `Enum!`

`Enum!` builds the type-level shape of an enum from the body of an enum declaration: the same type
that `#[derive(HasFields)]` gives that enum as `Fields`.

## Syntax

The body is a comma-separated list of variants, written as in a Rust enum. A variant's delimiter is
visible to the macro, so braces hold named fields and parentheses positional ones:

```rust
type Shape = Enum! { Circle(f64), Rectangle { width: f64, height: f64 }, Empty };
```

Each variant's fields follow the [`Struct!`](struct.md) field rules. Variant attributes, a variant
visibility, a discriminant (`= …`), and a repeated variant name are rejected.

## Expansion

Each variant becomes `Field<Symbol!("Variant"), Payload>`, and the entries are chained into a
`Sum!` that ends in `Void`. The payload is encoded by the `Struct!` rules: a unit variant (and
`V {}` or `V()`) carries `Nil`, and a variant with one positional field carries that type. An
empty body is `Void`.

```rust
type Shape = Sum![
    Field<Symbol!("Circle"), f64>,
    Field<Symbol!("Rectangle"), Struct! { width: f64, height: f64 }>,
    Field<Symbol!("Empty"), Nil>,
];
```

## See also

- [`Struct!`](struct.md), [`Sum!`](sum.md), and the [`HasFields` trait](../traits/has_fields.md).
- Implementation: [entrypoints/enum.md](../../implementation/entrypoints/enum.md).
- Tests: [`tests/shape_macros/enum_*.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/shape_macros) in cgp-fork.
