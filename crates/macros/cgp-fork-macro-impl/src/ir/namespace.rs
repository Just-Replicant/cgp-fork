//! The `Namespace` IR family: namespaces and namespace paths.

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::delegate_component::ValidateAttributes;
use crate::macro_core::types::namespace::NamespaceTable;
use crate::macro_core::types::path::UniPath;

/// A namespace or a path into one.
pub enum Namespace {
    /// `cgp_namespace!`: a namespace and the components it redirects.
    Table(Box<NamespaceTable>),
    /// `Path!`: a type-level namespace path.
    Path(UniPath),
}

impl FamilyIr for Namespace {
    const FAMILY: IrFamily = IrFamily::Namespace;

    /// A namespace body reuses the `delegate_components!` entry grammar, which supports no
    /// attributes, so any on its entries are rejected, matching `delegate_components!`.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        if let Namespace::Table(table) = self {
            diagnostics.check(table.entries.validate_attributes());
        }
    }

    /// Generates the namespace impls or the path type.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            Namespace::Table(table) => Ok(table.eval()?.to_token_stream()),
            Namespace::Path(path) => Ok(path.to_token_stream()),
        }
    }
}
