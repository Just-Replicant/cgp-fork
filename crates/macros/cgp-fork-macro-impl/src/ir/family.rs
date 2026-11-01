//! The kinds of IR the entrypoints produce, and the trait every family IR implements.

use proc_macro2::TokenStream;

use crate::ir::Diagnostics;

/// The kind of IR a [`Pipeline`](super::Pipeline) produces.
///
/// The first six are the IR groups named by the rewrite plan (`docs/rewrite-plan.md`,
/// §7.1); [`IrFamily::TraitRewrite`] covers the attribute macros that rewrite a trait in
/// place without defining a component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IrFamily {
    /// A component: consumer trait, provider trait, marker struct and standard providers
    /// (`#[cgp_component]`, `#[cgp_type]`, `#[cgp_fn]`, …).
    ComponentDef,
    /// A getter component or blanket getter that reads fields from the context.
    Getter,
    /// A provider implementation of a component (`#[cgp_impl]`, `#[cgp_provider]`, …).
    ProviderImpl,
    /// A wiring table or its checks (`delegate_components!`, `check_components!`, presets).
    WiringTable,
    /// A namespace or a namespace path (`cgp_namespace!`, `Path!`).
    Namespace,
    /// A data shape: field derives, records, variants and type-level constructors.
    DataShape,
    /// A trait rewritten in place (`#[async_trait]`, `#[blanket_trait]`, `#[cgp_auto_impl]`).
    TraitRewrite,
}

impl IrFamily {
    /// Every family, in declaration order.
    pub const ALL: [IrFamily; 7] = [
        IrFamily::ComponentDef,
        IrFamily::Getter,
        IrFamily::ProviderImpl,
        IrFamily::WiringTable,
        IrFamily::Namespace,
        IrFamily::DataShape,
        IrFamily::TraitRewrite,
    ];
}

/// One IR family: the shared IR type every entrypoint of the family parses into, with the
/// family's validation and code generation.
pub trait FamilyIr: Sized {
    /// The family this IR type represents.
    const FAMILY: IrFamily;

    /// Checks the IR and records every semantic error in `diagnostics`.
    fn validate(&self, diagnostics: &mut Diagnostics);

    /// Generates the output tokens from a validated IR.
    fn emit(self) -> syn::Result<TokenStream>;
}
