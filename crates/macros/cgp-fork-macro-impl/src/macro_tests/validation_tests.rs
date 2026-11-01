//! Checks that belong to the validate stage: they run before emit, and every error is
//! reported at once.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::extra_macro_lib::CgpAutoLogMacro;
use crate::ir::{Diagnostics, FamilyIr, MAX_IDENT_LEN, Pipeline};
use crate::macro_lib::{
    CgpAutoErrorMacro, CgpAutoGetterMacro, CgpAutoImplMacro, CgpNamespaceMacro, CgpPresetMacro,
    DelegateAndCheckComponentsMacro, DelegateComponentsMacro, DeriveProviderMacro, SymbolMacro,
};

fn messages(error: Error) -> Vec<String> {
    error.into_iter().map(|e| e.to_string()).collect()
}

/// Parses with `P` and runs only the validate stage, so a check that still lives in
/// emit is not counted.
fn validate<P: Pipeline>(attr: TokenStream, body: TokenStream) -> Vec<String> {
    let ir = P::parse(attr, body).unwrap_or_else(|error| panic!("parses: {error}"));
    let mut diagnostics = Diagnostics::new();
    ir.validate(&mut diagnostics);
    diagnostics.finish().err().map(messages).unwrap_or_default()
}

#[test]
fn delegate_components_reports_every_illegal_attribute() {
    let found = validate::<DelegateComponentsMacro>(
        quote!(),
        quote! {
            #[on_table]
            App {
                #[first] #[second]
                FooComponent: UseFoo,
                #[on_multi]
                [ #[in_multi] BarComponent, BazComponent ]: UseBar,
                QuxComponent: UseDelegate<new Inner {
                    #[in_inner] Key: Value,
                }>,
            }
        },
    );
    assert_eq!(
        found,
        [
            "unsupported attribute: on_table",
            "unsupported attribute: first",
            "unsupported attribute: second",
            "unsupported attribute: on_multi",
            "unsupported attribute: in_multi",
            "unsupported attribute: in_inner",
        ]
    );
}

#[test]
fn namespace_reports_every_illegal_attribute() {
    let found = validate::<CgpNamespaceMacro>(
        quote!(),
        quote! {
            App {
                #[first] #[second]
                FooComponent: UseFoo,
            }
        },
    );
    assert_eq!(
        found,
        [
            "unsupported attribute: first",
            "unsupported attribute: second"
        ]
    );
}

#[test]
fn preset_attributes_are_rejected_in_validate() {
    let found = validate::<CgpPresetMacro>(
        quote!(),
        quote! {
            AppPreset {
                #[first] #[second]
                FooComponent: UseFoo,
                #[third]
                BarComponent: UseBar,
            }
        },
    );
    assert_eq!(
        found,
        [
            "unsupported attribute: first",
            "unsupported attribute: second",
            "unsupported attribute: third",
        ]
    );
}

#[test]
fn preset_without_attributes_expands() {
    crate::macro_lib::cgp_preset(quote! {
        AppPreset {
            FooComponent: UseFoo,
        }
    })
    .expect("a preset without attributes expands");
}

#[test]
fn delegate_and_check_accepts_its_legal_attributes() {
    let body = quote! {
        #[check_trait(CanUseApp)]
        App {
            #[skip_check]
            FooComponent: UseFoo,
            #[check_params(u8, u16)]
            BarComponent: UseBar,
        }
    };
    assert!(validate::<DelegateAndCheckComponentsMacro>(quote!(), body.clone()).is_empty());
    let out = crate::macro_lib::delegate_and_check_components(body).expect("expands");
    assert!(out.to_string().contains("CanUseApp"), "{out}");
}

#[test]
fn delegate_and_check_reports_every_illegal_attribute_in_validate() {
    let found = validate::<DelegateAndCheckComponentsMacro>(
        quote!(),
        quote! {
            #[check_trait(CanUseApp)]
            #[on_table]
            #[check_trait(Again)]
            App {
                #[skip_check] #[extra]
                FooComponent: UseFoo,
                #[skip_check] #[check_params(u8)]
                BarComponent: UseBar,
                #[unknown]
                BazComponent: UseBaz,
                #[skip_check]
                [ #[check_params(u8)] QuxComponent ]: UseQux,
                #[on_path]
                @app.name: UseName,
                #[on_redirect]
                RedirectComponent => @app.other,
                InnerComponent: UseDelegate<new Inner {
                    #[skip_check] Key: Value,
                }>,
            }
        },
    );
    assert_eq!(
        found,
        [
            "Expected `#[check_trait]` attribute for specifying the check trait name",
            "Expected exactly one attribute for the check trait name",
            "Expected either `#[skip_check]` or `#[check_params]` attribute for specifying the check generics",
            "Expected at most one `#[check_params]` or `#[skip_check]` attribute",
            "Expected either `#[skip_check]` or `#[check_params]` attribute for specifying the check generics",
            "cannot combine #[skip_check] with #[check_params]",
            "unsupported attribute: on_path",
            "unsupported attribute: on_redirect",
            "unsupported attribute: skip_check",
        ]
    );
}

#[test]
fn with_provider_multi_method_getter_is_rejected_in_validate() {
    let found = validate::<DeriveProviderMacro>(
        quote!(WithProvider),
        quote! {
            #[cgp_component(NameGetter)]
            pub trait HasNames {
                fn first_name(&self) -> &str;
                fn last_name(&self) -> &str;
            }
        },
    );
    assert_eq!(
        found,
        ["`#[derive_provider(WithProvider)]` supports a getter with a single method"]
    );
}

#[test]
fn with_provider_reports_the_name_parameter_and_the_method_count_together() {
    let found = validate::<DeriveProviderMacro>(
        quote!(WithProvider),
        quote! {
            #[cgp_component { provider: NameGetter, name: NameGetterComponent<Undeclared> }]
            pub trait HasNames {
                fn first_name(&self) -> &str;
                fn last_name(&self) -> &str;
            }
        },
    );
    assert_eq!(found.len(), 2, "{found:?}");
    assert_eq!(
        found[1],
        "`#[derive_provider(WithProvider)]` supports a getter with a single method"
    );
}

#[test]
fn with_provider_accepts_a_single_method_getter_and_an_abstract_type() {
    for body in [
        quote! {
            #[cgp_component(NameGetter)]
            pub trait HasName {
                fn name(&self) -> &str;
            }
        },
        quote! {
            #[cgp_component(NameTypeProvider)]
            pub trait HasNameType {
                type Name;
            }
        },
    ] {
        assert!(validate::<DeriveProviderMacro>(quote!(WithProvider), body).is_empty());
    }
}

/// Every attribute macro without arguments rejects one with the shared message. Outside a
/// compiler `proc_macro2` spans carry no location, so the span on the argument tokens is
/// pinned by the `tests/ui/no_argument_attributes.rs` case of the `cgp-fork` crate.
#[test]
fn no_argument_attributes_reject_an_argument() {
    fn check<P: Pipeline>(body: TokenStream) {
        let error = match P::parse(quote!(Unexpected), body) {
            Ok(_) => panic!("`{}` accepted an argument", P::NAME),
            Err(error) => error,
        };
        assert_eq!(
            messages(error),
            [format!(
                "#[{}] does not accept any attribute argument",
                P::NAME
            )]
        );
    }

    let trait_body = quote!(
        pub trait HasName {
            fn name(&self) -> &str;
        }
    );
    check::<CgpAutoGetterMacro>(trait_body.clone());
    check::<CgpAutoImplMacro>(trait_body.clone());
    check::<CgpAutoLogMacro>(trait_body);
    check::<CgpAutoErrorMacro>(quote!(
        impl ProvideAppError {
            type Error = String;
        }
    ));
}

#[test]
fn symbol_string_gets_the_identifier_cap() {
    let at_cap = "x".repeat(MAX_IDENT_LEN);
    assert!(validate::<SymbolMacro>(quote!(), quote!(#at_cap)).is_empty());

    let over_cap = "x".repeat(MAX_IDENT_LEN + 1);
    let found = validate::<SymbolMacro>(quote!(), quote!(#over_cap));
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("this string is longer than 255 characters"));

    // The cap counts characters, like the identifier cap, not bytes.
    let multibyte = "ü".repeat(MAX_IDENT_LEN);
    assert!(validate::<SymbolMacro>(quote!(), quote!(#multibyte)).is_empty());

    let error = crate::macro_lib::Symbol(quote!(#over_cap)).expect_err("rejected");
    assert!(error.to_string().starts_with("this string is longer"));
}
