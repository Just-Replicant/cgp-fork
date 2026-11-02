/// Marks a component a preset wires, including components inherited from a parent preset.
///
/// `cgp_preset!` implements this for every component the preset exposes.
/// An `override(Parent)` entry checks `Parent::Provider: PresetContains<Component>`,
/// so an override that names a component the parent does not wire fails here.
#[diagnostic::on_unimplemented(
    message = "preset `{Self}` has no component `{Component}`",
    label = "`{Component}` is not part of this preset",
    note = "an `override` entry must name a component the parent preset already wires"
)]
pub trait PresetContains<Component> {}
