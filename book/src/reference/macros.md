# Macros

The names below are exported from `cgp_fork::prelude` unless a row says otherwise.

| Macro | What it does |
|---|---|
| `#[cgp_component(Name)]` | Consumer trait, provider trait `Name`, and `NameComponent`. |
| `#[cgp_component(Name, mock)]` | Also `MockName` and `NameMock`, compiled with the `mock` feature. |
| `#[cgp_provider(NameComponent)]` | Implements the provider trait for one provider type. |
| `#[cgp_impl(new Provider)]` | Writes the provider against the consumer trait. `where Self:` bounds the context. |
| `delegate_components!` | Maps each component to a provider. A concrete context is checked. |
| `delegate_and_check_components!` | The same table, with an explicit check. Does not check twice. |
| `check_components!` | Names the parameters a generic component is checked with. |
| `#[unchecked]` | Skips the automatic check on that table. |
| `#[cfg_wiring(predicate)]` | `#[cfg(predicate)]` on the table or on one entry, including its check. |
| `cgp_preset!` | A reusable wiring bundle. `#[version(N)]` stores `VERSION`. `override` replaces a parent key. |
| `#[cgp_handler]` | A function becomes a `Handler`. An `async fn` becomes an `AsyncHandler`. |
| `#[derive_handler]` | Forwards a component method to `Handler`. |
| `#[cgp_async]` | `async fn` in a trait becomes `impl Future`. `#[cgp_async(Send)]` adds `+ Send`. |
| `#[cgp_auto_error]` | An inherent impl builds the error providers. An enum derives `Display`, `Error`, and `From`. |
| `#[cgp_auto_getter]` | A trait whose methods read fields. The message names those fields. |
| `#[derive(HasField)]` | `HasField` and `HasFieldMut` for each field, keyed by `Symbol!`. |
| `Symbol!("name")` | The type-level spelling of a field name. |

`#[cgp_computer]`, `#[cgp_producer]`, `#[derive_promote]`, and a bare `#[async_trait]`
still expand. They are deprecated. [`cargo xtask migrate`](../migration.md) rewrites them.
