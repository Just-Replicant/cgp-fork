# Tracing and defmt

`CanLog` has three providers. They live in `cgp_fork::extra::log` and are not in the
prelude. Each feature is off by default.

| Provider | Feature | Detail bound |
|---|---|---|
| `TracingLogger` | `log-tracing` | `Display` |
| `TracingDebugLogger` | `log-tracing` | `Debug` |
| `DefmtLogger` | `log-defmt` | `defmt::Format` |

`tracing` is built with `default-features = false`, so the provider does not pull
`std` in by itself. The application installs a `tracing` subscriber. Without one,
the event is discarded. `DefmtLogger` calls `defmt::info`. Firmware supplies the global logger. A host
build of this crate enables `defmt`'s `unstable-test` feature so the same call
links without that logger; a `target_os = "none"` build does not.

`CanLog<Detail>` takes a type parameter. The automatic wiring check looks for `()`,
so a logger table writes `#[unchecked]` and a `check_components!` for the real
detail.

```rust,ignore
delegate_components! {
    #[unchecked]
    App {
        LoggerComponent: TracingLogger,
    }
}
```

The examples are `examples/tracing.rs` and `examples/defmt.rs`. The tests are
`tests/capabilities/tracing_logger.rs` and `tests/capabilities/defmt_logger.rs`.
