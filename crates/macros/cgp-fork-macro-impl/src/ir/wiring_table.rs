//! The `WiringTable` IR family: wiring tables, their checks, and presets.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::cgp_for_each::CgpForEach;
use crate::macro_core::types::cgp_preset::ItemCgpPreset;
use crate::macro_core::types::check_components::CheckComponentsTables;
use crate::macro_core::types::delegate_and_check_components::ItemDelegateAndCheckComponents;
use crate::macro_core::types::delegate_component::{DelegateTable, ValidateAttributes};

/// A wiring table or a check of one.
pub enum WiringTable {
    /// `delegate_components!`: the wiring table of a context.
    Delegate(DelegateTable),
    /// `delegate_and_check_components!`: a wiring table plus the checks of every entry.
    DelegateAndCheck(ItemDelegateAndCheckComponents),
    /// `check_components!`: compile-time checks that a context can use its components.
    Check(CheckComponentsTables),
    /// `cgp_for_each!`: a token body repeated once per component, used by presets.
    ForEach(CgpForEach),
    /// `cgp_preset!`: a reusable, replayable set of wiring entries.
    Preset(ItemCgpPreset),
}

impl FamilyIr for WiringTable {
    const FAMILY: IrFamily = IrFamily::WiringTable;

    /// Records every illegal attribute of a wiring table.
    ///
    /// `delegate_components!` and `cgp_preset!` support no attributes on the table or its
    /// keys, so every one is rejected instead of silently parsed and discarded.
    /// `delegate_and_check_components!` allows `#[check_trait]` on the table and
    /// `#[check_params]`/`#[skip_check]` on a checked key, and rejects everything else.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        match self {
            WiringTable::Delegate(table) => table.validate_attributes(diagnostics),
            WiringTable::DelegateAndCheck(item) => item.validate(diagnostics),
            WiringTable::Preset(item) => item.entries.validate_attributes(diagnostics),
            WiringTable::Check(_) | WiringTable::ForEach(_) => {}
        }
    }

    /// Generates the wiring impls and check traits.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            WiringTable::Delegate(table) => Ok(table.eval()?.to_token_stream()),
            WiringTable::DelegateAndCheck(item) => {
                let check_table = item.to_check_components()?;

                let evaluated_table = item.table.eval()?;

                let check_items = check_table.to_items()?;

                Ok(quote! {
                    #evaluated_table

                    #( #check_items )*
                })
            }
            WiringTable::Check(tables) => {
                let items = tables.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
            WiringTable::ForEach(item) => Ok(item.expand()),
            WiringTable::Preset(item) => item.expand(),
        }
    }
}
