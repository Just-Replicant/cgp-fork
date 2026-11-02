# Checked wiring

`delegate_components!` on a context with no generic parameters also emits a check
trait, `__CanUse{Context}`. The trait is the same one `check_components!` and
`delegate_and_check_components!` emit. It is implemented only when the context can
use every component the table wires, including components that come from a preset.

```rust,ignore
delegate_components! {
    App {
        GreeterComponent: GreetHello,
    }
}
```

`#[unchecked]` skips the check. Use it for a type-level map that is not a context,
and for a component whose check parameters are not `()`. `CanLog<Detail>` is the
usual case: the automatic check would look for `CanLog<()>`. Keep the attribute
and add `check_components!` with the real detail when that check should still run.

```rust,ignore
delegate_components! {
    #[unchecked]
    App {
        LoggerComponent: TracingLogger,
    }
}
```

A generic table (`Context<T> { ... }`) is not checked. Write
`delegate_and_check_components!` with `#[check_params]` when that table should be
checked too.

An `open` component, and a `Key =>` redirect, are checked too. `@Component.A.B`
becomes `CanUseComponent<Component, (A, B)>` (one parameter stays a single type,
matching `(T)` in `IsProviderFor`). The path is checked only when this table
delegates that component. A namespace slot such as `@MyFooComponent: Provider`
names a lookup path, not a component, so it is left to an explicit
`check_components!`. A `for` loop's provider is an abstract type parameter; the
namespace bound does not imply `IsProviderFor`, so name the concrete parameters
in `check_components!` instead.

A component with lifetime parameters stores `Life<'a>` in `IsProviderFor`. The
`@` path does not carry that lifetime. `open` of such a component needs
`#[unchecked]` and an explicit `check_components!` that includes `Life<'a>`.

`delegate_and_check_components!` does not gain a second check. `#[unchecked]` is
not valid there; that macro's job is the check. `#[cfg_wiring]` on that table is
legal and is copied onto the generated items, including the inspect const.
`#[check_trait]` names the check trait. It is not a cfg.

The example is `examples/hello.rs`. The tests are `tests/capabilities/checked.rs`.
