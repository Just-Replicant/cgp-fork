# `#[cgp_auto_error]`

`#[cgp_auto_error]` turns a small error definition, written as an inherent `impl` block, into the
providers for `HasErrorType`, `CanRaiseError`, and `CanWrapError`.

## Example

```rust
#[cgp_auto_error]
impl ProvideAppError {
    type Error = AppError;

    fn raise_error<Source>(source: Source) -> Error
    where
        Source: core::fmt::Display,
    {
        AppError { message: source.to_string() }
    }

    fn wrap_error<Detail>(error: Error, detail: Detail) -> Error
    where
        Detail: core::fmt::Display,
    {
        AppError { message: format!("{detail}: {}", error.message) }
    }
}
```

## Rules

- The impl's self type names the provider (`ProvideAppError`).
- `type Error` selects the abstract error type.
- `raise_error` and `wrap_error` are the provider bodies. A bare `Error` in type position means
  that abstract error type.
- Wire the provider on `ErrorTypeProviderComponent`, `ErrorRaiserComponent`, and
  `ErrorWrapperComponent`.
- An attribute argument, a trait impl, a missing piece of the definition, or a method that is not
  one of the provider functions is rejected.

## See also

- The error components: [has_error_type](../components/has_error_type.md) and
  [can_raise_error](../components/can_raise_error.md) (which also covers `CanWrapError`).
- Source: [`macro_core/types/cgp_auto_error`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_core/types/cgp_auto_error) in cgp-fork.
- Tests: [`tests/error_definition`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/error_definition) and
  [`macro_tests/parser_rejections/cgp_auto_error.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/parser_rejections/cgp_auto_error.rs).
