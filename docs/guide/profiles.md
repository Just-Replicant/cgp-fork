# Profiles

`#[cfg_wiring(predicate)]` selects wiring the same way `#[cfg]` selects items. The
predicate uses the `cfg` grammar: `test`, `not(test)`, `feature = "mock"`.

On an entry, the predicate gates that entry and the check for that entry. Two
entries can name the same component when their predicates cannot both be true.

```rust,ignore
delegate_components! {
    App {
        #[cfg_wiring(not(test))]
        ClockComponent: SystemClock,
        #[cfg_wiring(test)]
        ClockComponent: FixedClock,
    }
}
```

On the table, the predicate is copied onto every generated item, including a
`delegate_and_check_components!` table. A Rust attribute applies only to the next
item, so the macro repeats it. `#[check_trait(Name)]` is the check trait's name,
not a profile.

```rust,ignore
delegate_components! {
    #[cfg_wiring(test)]
    TestApp {
        ClockComponent: FixedClock,
    }
}
```

The example is `examples/profile.rs`. The test is `tests/capabilities/profile.rs`.
