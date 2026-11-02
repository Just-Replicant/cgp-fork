# Presets

`cgp_preset!` can carry a version and can replace a component a parent already wires.

`#[version(N)]` puts `pub const VERSION: u64` in the preset module. `N` is a `u64`.

```rust,ignore
cgp_preset! {
    #[version(1)]
    Base {
        GreeterComponent: Hello,
    }
}
```

`override Key: Provider` replaces `Key` on the single parent. With more than one
parent, name it: `override(Parent) Key: Provider`. `override` in a
`delegate_components!` table is an error; a table replaces a preset entry by writing
the key again.

The parent's keys are not known when the macro expands. The expansion emits a
`PresetContains` bound, and the compiler rejects an override of a component the
parent does not wire.

`#[cfg_wiring]` on a preset entry uses the same predicate for the provider impl,
the replayed key, and the check. A key that is not an `override` is absent when
the predicate is false, and it stays out of the parent replay in both
configurations. An `override` does not disappear: when the predicate is false the
preset forwards that component to the parent, and the consumer check still sees
the key. Stacked predicates are a conjunction, so the other configuration is
`not(predicate)` or `not(all(...))`.

```rust,ignore
cgp_preset! {
    #[version(2)]
    LoudPreset: Base {
        override GreeterComponent: Loud,
    }
}
```

The example is `examples/preset_override.rs`. The test is `tests/capabilities/preset.rs`.
