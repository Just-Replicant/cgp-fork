# `#[cgp_auto_log]`

`#[cgp_auto_log]` blanket-implements a logger trait. Each method packs its arguments into a detail
struct and passes it to `CanLog`.

## Example

```rust
#[cgp_auto_log]
pub trait CanLogHello {
    fn log_hello(&self, name: &str);
}
```

## Expansion

- Every method takes `&self` and has no body.
- For each method, a detail struct named after it is generated (`log_hello` → `__LogHello`), with
  one field per argument (borrowed arguments stay borrowed).
- The blanket impl's method calls `self.log(__LogHello { name })`.
- A context implements `CanLogHello` when it implements `CanLog` for every detail struct.

A context without a `LoggerComponent` entry for a detail struct fails to compile; see
[errors/checks/unsatisfied-logger-detail.md](../../errors/checks/unsatisfied-logger-detail.md).
A method with a body, or without a `&self` receiver, is rejected.

## See also

- Source: [`macro_core/types/cgp_auto_log`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_core/types/cgp_auto_log) in cgp-fork.
- Tests: [`tests/logging`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/main/cgp-fork/tests/logging) and
  [`macro_tests/parser_rejections/cgp_auto_log.rs`](https://github.com/Just-Replicant/cgp-fork/blob/main/crates/macros/cgp-fork-macro/src/macro_tests/parser_rejections/cgp_auto_log.rs).
