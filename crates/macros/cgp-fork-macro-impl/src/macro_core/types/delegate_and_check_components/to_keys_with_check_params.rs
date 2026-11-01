use crate::ir::Diagnostics;
use crate::macro_core::types::delegate_and_check_components::{
    CheckParamsAttribute, KeyWithCheckParams,
};
use crate::macro_core::types::delegate_component::{
    DelegateEntries, DelegateKey, DelegateMapping, MultiDelegateKey, SingleDelegateKey,
    ValidateAttributes,
};
use crate::macro_core::types::generics::ImplGenerics;

/// The keys `delegate_and_check_components!` checks, with the check control of each.
///
/// The walk records every illegal attribute in `diagnostics` instead of stopping at the
/// first: `#[check_params]`/`#[skip_check]` are legal on a checked key, any attribute is
/// illegal on a key that is not checked (path keys, redirects, statement forms, keys of a
/// nested inner table). The validate stage runs it to collect the errors; emit runs it
/// again on a table that passed validation to build the check entries.
pub trait ToKeysWithCheckParams {
    fn to_keys_with_check_params(&self, diagnostics: &mut Diagnostics) -> Vec<KeyWithCheckParams>;
}

impl ToKeysWithCheckParams for SingleDelegateKey {
    fn to_keys_with_check_params(&self, diagnostics: &mut Diagnostics) -> Vec<KeyWithCheckParams> {
        let check_params = CheckParamsAttribute::record_attributes(&self.attributes, diagnostics);

        // Carry the key's own generics (`<I>` in `<I> FooKey<I>`) into the check
        // entry so the derived check impl binds them; `KeyWithCheckParams` threads
        // them onto each check value.
        vec![KeyWithCheckParams {
            check_params,
            generics: self.generics.clone(),
            key_type: self.ty.clone(),
        }]
    }
}

impl ToKeysWithCheckParams for MultiDelegateKey {
    fn to_keys_with_check_params(&self, diagnostics: &mut Diagnostics) -> Vec<KeyWithCheckParams> {
        let check_params = CheckParamsAttribute::record_attributes(&self.attributes, diagnostics);

        let mut out = Vec::new();

        for key in &self.keys {
            for inner in key.to_keys_with_check_params(diagnostics) {
                let inner_params = diagnostics
                    .check(check_params.merge(&inner.check_params))
                    .unwrap_or(CheckParamsAttribute::Default);
                out.push(KeyWithCheckParams {
                    key_type: inner.key_type,
                    generics: inner.generics,
                    check_params: inner_params,
                });
            }
        }

        out
    }
}

impl ToKeysWithCheckParams for DelegateKey {
    fn to_keys_with_check_params(&self, diagnostics: &mut Diagnostics) -> Vec<KeyWithCheckParams> {
        match self {
            DelegateKey::Single(key) => key.to_keys_with_check_params(diagnostics),
            DelegateKey::Multi(key) => key.to_keys_with_check_params(diagnostics),
            DelegateKey::Path(key) => {
                // Path keys are not checked, so they take no check attributes.
                key.validate_attributes(diagnostics);
                Vec::new()
            }
        }
    }
}

impl ToKeysWithCheckParams for DelegateMapping {
    fn to_keys_with_check_params(&self, diagnostics: &mut Diagnostics) -> Vec<KeyWithCheckParams> {
        match self {
            // The keys of a nested inner table are not checked, so their attributes are
            // rejected like in `delegate_components!`.
            DelegateMapping::Normal(mapping) => {
                mapping.value.validate_attributes(diagnostics);
                mapping.key.to_keys_with_check_params(diagnostics)
            }
            DelegateMapping::Direct(mapping) => {
                mapping.value.validate_attributes(diagnostics);
                mapping.key.to_keys_with_check_params(diagnostics)
            }
            DelegateMapping::Redirect(mapping) => {
                // Redirect mappings do not support check params yet, so reject any
                // attribute on the key rather than silently ignoring it.
                mapping.key.validate_attributes(diagnostics);
                Vec::new()
            }
            DelegateMapping::TypeShorthand(entry) => {
                vec![shorthand_check_key(entry.component_key())]
            }
            DelegateMapping::GetterShorthand(entry) => {
                vec![shorthand_check_key(entry.component_key())]
            }
            // A preset's keys are produced by `with_components!`, not by this macro.
            DelegateMapping::Preset(_) => Vec::new(),
        }
    }
}

fn shorthand_check_key(key_type: syn::Type) -> KeyWithCheckParams {
    KeyWithCheckParams {
        check_params: CheckParamsAttribute::Default,
        generics: ImplGenerics::default(),
        key_type,
    }
}

impl ToKeysWithCheckParams for DelegateEntries {
    fn to_keys_with_check_params(&self, diagnostics: &mut Diagnostics) -> Vec<KeyWithCheckParams> {
        let mut out = Vec::new();

        // Statement forms (`for`/`namespace`/`open`) are intentionally not checked
        // for now. They still produce delegate impls via `eval`, but no check
        // entries are generated for them. Since they cannot carry check params,
        // reject any attribute on their keys rather than silently ignoring it.
        for statement in &self.statements {
            statement.validate_attributes(diagnostics);
        }

        for entry in &self.entries {
            out.extend(entry.to_keys_with_check_params(diagnostics));
        }

        out
    }
}
