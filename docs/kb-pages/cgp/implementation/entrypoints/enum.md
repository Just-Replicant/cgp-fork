# `Enum!`: implementation

`Enum!` parses an enum body into `syn::Variant`s and re-uses the `#[derive(HasFields)]` sum
encoder. For the accepted syntax and the expansion, read
[reference/macros/enum.md](../../reference/macros/enum.md).

## Entry point

[`macro_lib/enum_type.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_lib/enum_type.rs):

```rust
let enum_type: EnumType = parse2(body)?;
Ok(enum_type.eval()?.to_token_stream())
```

## Pipeline

1. **Parse.** `EnumType`'s `Parse` impl reads variants one at a time with its own
   `parse_variant` instead of `syn`'s `Variant` parser, because `syn` silently drops a visibility
   and accepts a discriminant; both are rejected here, as are variant attributes and repeated
   variant names. Braces and parentheses fix the field form, and the fields go through
   `parse_variant_fields`, which applies every `Struct!` field rule. `V {}`, `V()`, and `V` stay
   distinct after parsing, as in a Rust enum, though all three encode to `Nil`.
2. **Evaluate.** `EnumType::eval` calls `variants_to_sum_type`, the encoder the derive uses.

The AST type is described in [asts/shape.md](../asts/shape.md). Tests:
[`macro_tests/parser_rejections/enum_macro.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/parser_rejections/enum_macro.rs),
[`macro_tests/shape_macro_parsing`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/shape_macro_parsing), and
[`tests/shape_macros`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/shape_macros), including `enum_derive_parity.rs`.
