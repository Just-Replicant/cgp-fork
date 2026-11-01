use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, Ident};

use crate::ir::Diagnostics;
use crate::macro_core::types::check_components::{
    CheckComponentsTable, CheckEntries, derive_check_trait_ident,
};
use crate::macro_core::types::delegate_and_check_components::ToKeysWithCheckParams;
use crate::macro_core::types::delegate_component::DelegateTable;

pub struct ItemDelegateAndCheckComponents {
    pub table: DelegateTable,
}

impl ItemDelegateAndCheckComponents {
    /// Records every illegal attribute of the table and its keys. The table may carry one
    /// `#[check_trait(Name)]`; a checked key may carry one `#[check_params(..)]` or
    /// `#[skip_check]`; anything else is an error.
    pub fn validate(&self, diagnostics: &mut Diagnostics) {
        self.record_check_trait_ident(diagnostics);
        self.table.entries.to_keys_with_check_params(diagnostics);
    }

    pub fn to_check_components(&self) -> syn::Result<CheckComponentsTable> {
        let trait_name = self.check_trait_ident()?;
        let check_entries = self.to_check_entries()?;

        let check_table = CheckComponentsTable {
            check_providers: None,
            impl_generics: self.table.impl_generics.clone(),
            trait_name,
            context_type: self.table.table_type.clone(),
            where_clause: None,
            check_entries,
        };

        Ok(check_table)
    }

    pub fn to_check_entries(&self) -> syn::Result<CheckEntries> {
        let mut diagnostics = Diagnostics::new();
        let keys = self
            .table
            .entries
            .to_keys_with_check_params(&mut diagnostics);
        diagnostics.finish()?;

        let mut entries = CheckEntries::default();

        for key in keys {
            entries.entries.extend(key.to_check_entries().entries);
        }

        Ok(entries)
    }

    pub fn check_trait_ident(&self) -> syn::Result<Ident> {
        let mut diagnostics = Diagnostics::new();
        let ident = self.record_check_trait_ident(&mut diagnostics);
        diagnostics.finish()?;

        match ident {
            Some(ident) => Ok(ident),
            None => derive_check_trait_ident(&self.table.table_type, "__CanUse"),
        }
    }

    /// The name given with `#[check_trait(Name)]`, if any, recording an error for every
    /// table attribute that is not `#[check_trait]` and for every `#[check_trait]` after the
    /// first.
    fn record_check_trait_ident(&self, diagnostics: &mut Diagnostics) -> Option<Ident> {
        let mut ident = None;
        let mut seen_check_trait = false;

        for attribute in &self.table.attributes {
            if !attribute.path().is_ident("check_trait") {
                diagnostics.push(Error::new(
                    attribute.span(),
                    "Expected `#[check_trait]` attribute for specifying the check trait name",
                ));
            } else if seen_check_trait {
                diagnostics.push(Error::new(
                    attribute.span(),
                    "Expected exactly one attribute for the check trait name",
                ));
            } else {
                seen_check_trait = true;
                ident = diagnostics.check(attribute.parse_args());
            }
        }

        ident
    }
}

impl Parse for ItemDelegateAndCheckComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let table = input.parse()?;

        Ok(Self { table })
    }
}
