# `Struct!`

`Struct!` builds the type-level shape of a struct from the body of a struct declaration. The
result is exactly the type that `#[derive(HasFields)]` gives the same struct as its `Fields`, so a
shape can be written, compared, or used in a bound without declaring a struct.

## Syntax

The body is what would sit between a struct's braces or parentheses. A proc macro cannot see which
delimiter it was invoked with, so the form comes from the entries:

```rust
type Person = Struct! { name: String, age: u8 };   // named: `name: Type` entries
type Pair   = Struct!(u64, String);                 // positional: bare types
type Unit   = Struct! {};                           // empty body
```

All entries must use the same form. Field attributes, field visibility (`pub`), the name `_`,
a repeated name (`r#a` and `a` are the same name), a keyword used as a name, and a literal where a
type belongs are rejected with a targeted error.

## Expansion

| Body | Shape |
|---|---|
| named fields | `Product![Field<Symbol!("name"), T>, …]`, in declaration order |
| positional fields | `Product![Field<Index<0>, T>, Field<Index<1>, U>, …]` |
| one positional field | that field's type, bare |
| empty | `Nil` |

```rust
type Person = Product![Field<Symbol!("name"), String>, Field<Symbol!("age"), u8>];
type Pair   = Product![Field<Index<0>, u64>, Field<Index<1>, String>];
```

## See also

- [`Enum!`](enum.md), the sum counterpart.
- [`Product!`](product.md) and the [`HasFields` trait](../traits/has_fields.md).
- Implementation: [entrypoints/struct.md](../../implementation/entrypoints/struct.md).
- Tests: [`tests/shape_macros/struct_*.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/shape_macros) in cgp-fork.
