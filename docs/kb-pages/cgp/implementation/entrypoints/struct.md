# `Struct!`: implementation

`Struct!` is a function-like macro that parses a struct body and re-uses the `#[derive(HasFields)]`
encoder on it. For the accepted syntax and the expansion, read
[reference/macros/struct.md](../../reference/macros/struct.md).

## Entry point

[`macro_lib/struct_type.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_lib/struct_type.rs):

```rust
let struct_type: StructType = parse2(body)?;
Ok(struct_type.eval()?.to_token_stream())
```

## Pipeline

1. **Parse.** `StructType`'s `Parse` impl calls `parse_shape_fields`
   ([`functions/shape/parse_fields.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_core/functions/shape/parse_fields.rs)). The macro
   never sees its delimiter, so the first entry decides the form (`name: Type` is named, a bare
   type is positional) and every later entry must match. The entries become the
   `syn::Fields` a struct declaration would hold; an empty body is `Fields::Unit`. A keyword used
   as a field name and a literal where a type belongs are reported before `syn`'s type parser
   would give a generic error, and a path type such as `core::marker::PhantomData<u8>` is not
   mistaken for `name: Type`.
2. **Validate.** `validate_shape_fields`
   ([`functions/shape/validate_fields.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_core/functions/shape/validate_fields.rs))
   rejects field attributes, visibility, `_`, and repeated names (compared without `r#`).
3. **Evaluate.** `StructType::eval` calls `item_fields_to_product_type`, the encoder
   `#[derive(HasFields)]` uses, so `Struct!` agrees with the derive by construction.

The AST type is described in [asts/shape.md](../asts/shape.md). Tests:
[`macro_tests/parser_rejections/struct_macro.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/parser_rejections/struct_macro.rs),
[`macro_tests/shape_macro_parsing`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/shape_macro_parsing), and
[`tests/shape_macros`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/shape_macros), including `struct_derive_parity.rs`.
