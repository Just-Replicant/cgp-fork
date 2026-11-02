//! The check `delegate_components!` emits for a concrete context.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use super::cfg_wiring::read_table_flags;
use super::preset_invoke::invoke_preset_checks;
use crate::ir::Diagnostics;
use crate::macro_core::types::check_components::{
    CheckComponentsTable, CheckEntries, derive_check_trait_ident,
};
use crate::macro_core::types::delegate_and_check_components::ToKeysWithCheckParams;
use crate::macro_core::types::delegate_component::{
    DelegateEntries, DelegateMapping, DelegateTable,
};

/// The `__CanUse{Context}` check for a non-generic table, or nothing when the table
/// opts out, is generic, or has no component key to check.
///
/// Preset entries are checked through the preset's `with_components!`, which is where
/// those keys are known. Explicit keys are excluded from that expansion so a component
/// written in the table is checked once.
pub fn default_check(table: &DelegateTable) -> syn::Result<TokenStream> {
    let mut diagnostics = Diagnostics::new();
    let flags = read_table_flags(&table.attributes, &mut diagnostics);
    diagnostics.finish()?;

    if flags.unchecked || !table.impl_generics.generics.params.is_empty() {
        return Ok(TokenStream::new());
    }

    let mut diagnostics = Diagnostics::new();
    let keys = table.entries.to_keys_with_check_params(&mut diagnostics);
    diagnostics.finish()?;

    let mut entries = CheckEntries::default();
    let mut exclude: Vec<Type> = Vec::new();

    for key in &keys {
        let check_entries = key.to_check_entries();
        exclude.push(key.key_type.clone());
        entries.entries.extend(check_entries.entries);
    }

    entries
        .entries
        .extend(super::dispatch_check::dispatch_checks(table)?);

    let presets: Vec<_> = table
        .entries
        .entries
        .iter()
        .filter_map(|entry| match entry {
            DelegateMapping::Preset(preset) => Some(preset),
            _ => None,
        })
        .collect();

    if entries.entries.is_empty() && presets.is_empty() {
        return Ok(TokenStream::new());
    }

    let trait_name = derive_check_trait_ident(&table.table_type, "__CanUse")?;
    let check_table = CheckComponentsTable {
        check_providers: None,
        impl_generics: table.impl_generics.clone(),
        trait_name: trait_name.clone(),
        context_type: table.table_type.clone(),
        where_clause: None,
        check_entries: entries,
    };
    let items = check_table.to_items()?;
    let preset_checks = presets
        .iter()
        .map(|preset| invoke_preset_checks(&preset.path, &trait_name, &table.table_type, &exclude));

    Ok(quote! {
        #(#items)*
        #(#preset_checks)*
    })
}

/// `override` belongs to `cgp_preset!`. A wiring table replaces a preset entry by
/// writing the component key itself.
pub fn reject_preset_only_syntax(entries: &DelegateEntries, diagnostics: &mut Diagnostics) {
    reject_in_mappings(&entries.entries, diagnostics);
}

fn reject_in_mappings(entries: &Punctuated<DelegateMapping, Comma>, diagnostics: &mut Diagnostics) {
    for entry in entries {
        if let DelegateMapping::Override(override_entry) = entry {
            diagnostics.push(syn::Error::new(
                override_entry.span,
                "`override` is only valid in `cgp_preset!`. In a wiring table, write the component key again to replace the preset's provider",
            ));
        }
    }
}
