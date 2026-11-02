//! Checks for `open`, `@`-path, and `for` entries whose component this table delegates.
//!
//! A direct `Key: Provider` entry is checked by [`super::default_check`] as
//! `CanUseComponent<Key, ()>`. An `@` path is a component check only when this table
//! also delegates that component (`open`, or a `Key` / `Key =>` entry): leading
//! symbol segments are the namespace prefix, the first type segment is the
//! component, and the rest are its parameters. A lookup slot such as
//! `@MyFooComponent: Provider` names a path, not a component, so it is not checked
//! here — the component lives on the namespace trait, which this macro cannot see.
//!
//! A `for` loop's provider is an abstract type parameter. The namespace bound does
//! not imply `IsProviderFor`, so a blanket check over that loop cannot succeed;
//! name the concrete parameters in `check_components!` instead. A `Key =>`
//! redirect is checked as `CanUseComponent<Key, ()>` only when no parameterized
//! path already checks that key. A component with lifetime parameters stores
//! `Life<'a>` in `IsProviderFor`, which an `@` path does not carry; `open` of
//! such a component needs `#[unchecked]` and an explicit `check_components!`.

use std::collections::BTreeSet;

use quote::ToTokens;
use syn::parse_quote;
use syn::{Generics, Type};

use crate::macro_core::functions::merge_generics;
use crate::macro_core::traits::ToType;
use crate::macro_core::types::check_components::{
    CheckEntry, CheckKey, CheckValue, TypeWithGenerics,
};
use crate::macro_core::types::delegate_component::{
    DelegateEntries, DelegateKey, DelegateMapping, DelegateStatement, DelegateTable, cfg_attributes,
};
use crate::macro_core::types::generics::ImplGenerics;
use crate::macro_core::types::path::{PathElement, UniPath};

/// Check entries for every dispatch path in `table`.
pub fn dispatch_checks(table: &DelegateTable) -> syn::Result<Vec<CheckEntry>> {
    let direct = direct_components(&table.entries);
    let mut checks = Vec::new();
    let mut covered = BTreeSet::new();
    collect_paths(&table.entries, &direct, &mut covered, &mut checks)?;
    collect_redirect_fallbacks(&table.entries, &covered, &mut checks)?;
    Ok(checks)
}

fn direct_components(entries: &DelegateEntries) -> BTreeSet<String> {
    let mut direct = BTreeSet::new();

    for statement in &entries.statements {
        match statement {
            DelegateStatement::Open(statement) => {
                for component in &statement.components {
                    direct.insert(type_key(component));
                }
            }
            DelegateStatement::For(_) | DelegateStatement::Namespace(_) => {}
        }
    }

    for entry in &entries.entries {
        insert_mapping_keys(entry, &mut direct);
    }

    direct
}

fn insert_mapping_keys(mapping: &DelegateMapping, direct: &mut BTreeSet<String>) {
    match mapping {
        DelegateMapping::Normal(mapping) => insert_key_types(&mapping.key, direct),
        DelegateMapping::Direct(mapping) => insert_key_types(&mapping.key, direct),
        DelegateMapping::Redirect(mapping) => insert_key_types(&mapping.key, direct),
        DelegateMapping::Override(entry) => insert_mapping_keys(&entry.entry, direct),
        DelegateMapping::TypeShorthand(_)
        | DelegateMapping::GetterShorthand(_)
        | DelegateMapping::Preset(_) => {}
    }
}

fn insert_key_types(key: &DelegateKey, direct: &mut BTreeSet<String>) {
    match key {
        DelegateKey::Single(key) => {
            direct.insert(type_key(&key.ty));
        }
        DelegateKey::Multi(key) => {
            for inner in &key.keys {
                direct.insert(type_key(&inner.ty));
            }
        }
        DelegateKey::Path(_) => {}
    }
}

fn collect_paths(
    entries: &DelegateEntries,
    direct: &BTreeSet<String>,
    covered: &mut BTreeSet<String>,
    checks: &mut Vec<CheckEntry>,
) -> syn::Result<()> {
    for entry in &entries.entries {
        collect_path_mapping(entry, direct, covered, checks)?;
    }

    Ok(())
}

fn collect_path_mapping(
    mapping: &DelegateMapping,
    direct: &BTreeSet<String>,
    covered: &mut BTreeSet<String>,
    checks: &mut Vec<CheckEntry>,
) -> syn::Result<()> {
    match mapping {
        DelegateMapping::Normal(mapping) => {
            collect_path_key(&mapping.key, direct, covered, checks)?;
        }
        DelegateMapping::Direct(mapping) => {
            collect_path_key(&mapping.key, direct, covered, checks)?;
        }
        DelegateMapping::Override(entry) => {
            collect_path_mapping(&entry.entry, direct, covered, checks)?;
        }
        DelegateMapping::Redirect(_)
        | DelegateMapping::TypeShorthand(_)
        | DelegateMapping::GetterShorthand(_)
        | DelegateMapping::Preset(_) => {}
    }

    Ok(())
}

fn collect_path_key(
    key: &DelegateKey,
    direct: &BTreeSet<String>,
    covered: &mut BTreeSet<String>,
    checks: &mut Vec<CheckEntry>,
) -> syn::Result<()> {
    let DelegateKey::Path(path) = key else {
        return Ok(());
    };

    let cfgs = cfg_attributes(&path.attributes)?;

    for (path_generics, unipath) in path.path.paths() {
        let generics = merge_generics(&path.generics.generics, &path_generics.generics);

        if let Some((component, check)) = path_check(&unipath, generics, cfgs.clone())
            && direct.contains(&component)
        {
            covered.insert(component);
            checks.push(check);
        }
    }

    Ok(())
}

/// `()` checks for `Key =>` entries that no parameterized path already covers.
fn collect_redirect_fallbacks(
    entries: &DelegateEntries,
    covered: &BTreeSet<String>,
    checks: &mut Vec<CheckEntry>,
) -> syn::Result<()> {
    for entry in &entries.entries {
        collect_redirect_fallback(entry, covered, checks)?;
    }

    Ok(())
}

fn collect_redirect_fallback(
    mapping: &DelegateMapping,
    covered: &BTreeSet<String>,
    checks: &mut Vec<CheckEntry>,
) -> syn::Result<()> {
    match mapping {
        DelegateMapping::Redirect(mapping) => match &mapping.key {
            DelegateKey::Single(key) if !covered.contains(&type_key(&key.ty)) => {
                checks.push(plain_check(
                    key.ty.clone(),
                    key.generics.generics.clone(),
                    cfg_attributes(&key.attributes)?,
                ));
            }
            DelegateKey::Multi(key) => {
                let outer = cfg_attributes(&key.attributes)?;

                for inner in &key.keys {
                    if covered.contains(&type_key(&inner.ty)) {
                        continue;
                    }

                    let mut cfgs = outer.clone();
                    cfgs.extend(cfg_attributes(&inner.attributes)?);
                    checks.push(plain_check(
                        inner.ty.clone(),
                        inner.generics.generics.clone(),
                        cfgs,
                    ));
                }
            }
            DelegateKey::Single(_) | DelegateKey::Path(_) => {}
        },
        DelegateMapping::Override(entry) => {
            collect_redirect_fallback(&entry.entry, covered, checks)?;
        }
        _ => {}
    }

    Ok(())
}

fn type_key(ty: &Type) -> String {
    ty.to_token_stream()
        .to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// `@prefix.Component.A.B` checks `Component` with parameters `(A, B)`.
///
/// Leading symbol segments are the namespace prefix. The first type segment is the
/// component. The rest are the parameters of `CanUseComponent` (one parameter stays
/// a single type, matching `(T)` in `IsProviderFor`). The returned string is the
/// component's identity, so the caller can tell which key a parameterized path
/// already covered.
fn path_check(
    path: &UniPath,
    generics: Generics,
    cfgs: Vec<syn::Attribute>,
) -> Option<(String, CheckEntry)> {
    let mut elements = path.elements.iter();
    let component = loop {
        match elements.next()? {
            PathElement::Symbol(_) => continue,
            PathElement::Type(ty) => break ty.clone(),
        }
    };
    let params: Vec<Type> = elements.map(ToType::to_type).collect();
    let name = type_key(&component);
    Some((name, check_entry(component, params, generics, cfgs)))
}

fn plain_check(component: Type, generics: Generics, cfgs: Vec<syn::Attribute>) -> CheckEntry {
    check_entry(component, Vec::new(), generics, cfgs)
}

fn check_entry(
    component: Type,
    params: Vec<Type>,
    generics: Generics,
    cfgs: Vec<syn::Attribute>,
) -> CheckEntry {
    let keep_generics = !generics.params.is_empty() || generics.where_clause.is_some();
    let value = match params.as_slice() {
        [] if !keep_generics => None,
        [] => Some(unit_value(generics)),
        [one] if !keep_generics => Some(CheckValue::Single(Box::new(one.clone().into()))),
        [one] => Some(CheckValue::Single(Box::new(TypeWithGenerics {
            ty: one.clone(),
            generics: ImplGenerics { generics },
        }))),
        many => {
            let ty: Type = parse_quote!((#(#many),*));
            Some(CheckValue::Single(Box::new(TypeWithGenerics {
                ty,
                generics: ImplGenerics { generics },
            })))
        }
    };

    CheckEntry {
        key: CheckKey::Single(component),
        value,
        cfgs,
    }
}

fn unit_value(generics: Generics) -> CheckValue {
    CheckValue::Single(Box::new(TypeWithGenerics {
        ty: parse_quote!(()),
        generics: ImplGenerics { generics },
    }))
}
