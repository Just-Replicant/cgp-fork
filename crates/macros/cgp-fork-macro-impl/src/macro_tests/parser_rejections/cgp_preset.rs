//! `cgp_preset!` rejects a body that is not a name, optional parents, and a
//! table. The table reuses the `delegate_components!` entry grammar, so a
//! shorthand written without its value fails here too.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_preset_without_body() {
    assert_macro_rejects("cgp_preset without a table body", || {
        crate::macro_lib::cgp_preset(quote!(PersonPreset))
    });
}

#[test]
fn rejects_override_without_a_parent() {
    assert_macro_rejects("override in a preset that has no parent", || {
        crate::macro_lib::cgp_preset(quote!(
            Base {
                override GreeterComponent: Loud,
            }
        ))
    });
}

#[test]
fn rejects_bare_override_with_two_parents() {
    assert_macro_rejects("bare override when a preset has two parents", || {
        crate::macro_lib::cgp_preset(quote!(
            Child: Left + Right {
                override GreeterComponent: Loud,
            }
        ))
    });
}

#[test]
fn rejects_version_that_is_not_a_u64() {
    assert_macro_rejects("version attribute that is not a u64", || {
        crate::macro_lib::cgp_preset(quote!(
            #[version(-1)]
            Base {
                GreeterComponent: Hello,
            }
        ))
    });
}

#[test]
fn rejects_preset_type_shorthand_without_value() {
    assert_macro_rejects("cgp_preset type shorthand without `=`", || {
        crate::macro_lib::cgp_preset(quote!(
            PersonPreset {
                type Name,
            }
        ))
    });
}
