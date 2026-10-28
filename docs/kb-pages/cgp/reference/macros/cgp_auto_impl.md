# `#[cgp_auto_impl]`

`#[cgp_auto_impl]` turns a trait whose methods have bodies into a trait plus a blanket
implementation, so every context that meets the trait's supertraits gets the methods for free.

## Example

```rust
#[cgp_auto_impl]
pub trait CanGreet: HasName {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}
```

## Expansion

- The trait is emitted with its method (and const) bodies removed.
- A blanket `impl<__Context__> CanGreet for __Context__` carries those bodies.
- The trait's supertraits become a `where` bound on `__Context__` (here `__Context__: HasName`).
- Associated types on the trait are lifted to extra generic parameters of the impl, with their
  bounds alongside.

When the context does not implement a supertrait, the call fails with `E0599`; see
[errors/checks/unsatisfied-blanket-supertrait.md](../../errors/checks/unsatisfied-blanket-supertrait.md).
The macro takes no arguments and is not allowed on a trait impl.

## See also

- [`#[blanket_trait]`](blanket_trait.md), which writes the same kind of blanket impl by hand.
- Source: [`macro_core/types/cgp_auto_impl`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_core/types/cgp_auto_impl) in cgp-fork.
- Tests: [`tests/blanket_traits/auto_impl_*.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/blanket_traits) and
  [`macro_tests/parser_rejections/cgp_auto_impl.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/parser_rejections/cgp_auto_impl.rs).
