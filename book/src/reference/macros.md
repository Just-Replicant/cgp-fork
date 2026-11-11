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
| `#[cgp_getter]` | A getter component. Wire `UseFields` or `UseField<Tag>`. |
| `#[cgp_auto_getter]` | A trait whose methods read fields. The message names those fields. |
| `#[cgp_fn]` | A function becomes a trait. `#[implicit]` reads a same-named field. `#[cgp_fn(Trait)]` names the trait. |
| `#[cgp_type]` | An abstract-type component. Wire `UseType<T>`. |
| `#[cgp_new_provider]` | `#[cgp_provider]`, and it defines the provider struct. |
| `#[cgp_auto_impl]` | Method bodies become one blanket provider. |
| `#[blanket_trait]` | An empty trait plus a blanket impl of its supertraits. Import it from `cgp_fork::core::macros`. |
| `#[derive_provider(WithProvider)]` | The `WithProvider` impl for a component, a getter, or an abstract type. |
| `#[derive(HasField)]` | `HasField` and `HasFieldMut` for each field, keyed by `Symbol!`. |
| `#[derive(CgpData)]` | Fields, shape, builder, and extractor. |
| `#[derive(HasFields)]` | The `Struct!` or `Enum!` shape. |
| `#[derive(BuildField)]` | `HasBuilder`, `BuildField`, and `FinalizeBuild`. |
| `#[derive(ExtractField)]` | `HasExtractor`, its ref and mut forms, and `ExtractField`. |
| `#[derive(FromVariant)]` | `FromVariant<Tag>` for each variant. |
| `cgp_namespace!` | A path-to-provider table. `namespace Name;` uses it. |
| `Symbol!("name")` | The type-level spelling of a field name. |
| `Path!(@a.b.Name)` | A type-level path. A lowercase segment that is not a primitive type is a `Symbol`. |
| `Product!`, `Sum!` | Type-level lists. |
| `Struct!`, `Enum!` | The shapes `HasFields` names. |
| `cgp_for_each!` | Repeats a body once per type. |
| `#[cgp_auto_dispatch]` | An enum forwards a trait to the same trait on each variant. |
| `#[cgp_auto_log]` | Logger methods become `CanLog` calls. |

The behavior of each row is in the guide: [fields](../guide/fields.md),
[functions](../guide/functions.md), [records and variants](../guide/data.md),
[abstract types](../guide/types.md), [namespaces](../guide/namespaces.md),
[a component](../guide/components.md).

`#[cgp_computer]`, `#[cgp_producer]`, `#[derive_promote]`, and a bare `#[async_trait]`
still expand. They are deprecated. [`cargo xtask migrate`](../migration.md) rewrites them.
