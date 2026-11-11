use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Colon, Plus};
use syn::{Attribute, Error, Ident, LitInt, Path, Type, braced};

use crate::ir::Diagnostics;
use crate::macro_core::exports::{DelegateComponent, IsProviderFor, PresetContains};
use crate::macro_core::functions::to_snake_case_str;
use crate::macro_core::types::delegate_component::{
    ComponentKey, DelegateEntries, DelegateMapping, EvalDelegateEntries, OverrideDelegateEntry,
    define_with_components_macro, invoke_with_components, negate_cfgs,
};
use crate::macro_core::types::empty_struct::EmptyStruct;

/// `cgp_preset! { Name: Parent + Other { entries } }`.
///
/// The name is a module whose `Provider` implements `DelegateComponent` for
/// every entry. `with_components!` replays those keys, and each parent is
/// forwarded so two presets combine into one.
pub struct ItemCgpPreset {
    pub name: Ident,
    /// `#[version(N)]` on the preset, when written.
    pub version: Option<LitInt>,
    pub parents: Vec<Path>,
    pub entries: DelegateEntries,
}

impl Parse for ItemCgpPreset {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let version = preset_version(&attributes)?;
        let name = input.parse()?;

        let parents = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            Punctuated::<Path, Plus>::parse_separated_nonempty(input)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };

        let entries = {
            let content;
            braced!(content in input);
            content.parse()?
        };

        Ok(Self {
            name,
            version,
            parents,
            entries,
        })
    }
}

fn preset_version(attributes: &[Attribute]) -> syn::Result<Option<LitInt>> {
    let mut version = None;

    for attribute in attributes {
        if !attribute.path().is_ident("version") {
            return Err(Error::new(
                attribute.span(),
                "the only attribute on `cgp_preset!` is `#[version(N)]`",
            ));
        }

        if version.is_some() {
            return Err(Error::new(attribute.span(), "duplicate `#[version]`"));
        }

        let literal: LitInt = attribute.parse_args()?;
        literal.base10_parse::<u64>()?;
        version = Some(literal);
    }

    Ok(version)
}

impl ItemCgpPreset {
    /// An `override` entry must name one of this preset's parents. Whether that parent
    /// actually wires the component is checked by the [`PresetContains`] bound the
    /// expansion emits, because the parent's keys are not known to this macro.
    pub fn validate_overrides(&self, diagnostics: &mut Diagnostics) {
        for entry in &self.entries.entries {
            let DelegateMapping::Override(override_entry) = entry else {
                continue;
            };

            if let Err(error) = self.resolve_parent(override_entry) {
                diagnostics.push(error);
            }
        }
    }

    /// Generates the preset. Its attributes and overrides are checked by the
    /// `WiringTable` validate stage, which runs first.
    pub fn expand(&self) -> syn::Result<TokenStream> {
        let name = &self.name;
        let provider_ty: Type = syn::parse_quote!(#name::Provider);
        let own_impls = self
            .entries
            .build_impls(&provider_ty, &Default::default())?;

        let own_keys = self.component_keys(&provider_ty)?;
        let own_key_types = own_keys
            .iter()
            .map(|key| key.ty.clone())
            .collect::<Vec<_>>();

        let parent_impls = self.parents.iter().map(|parent| {
            invoke_with_components(parent, &provider_ty, &Default::default(), &own_key_types)
        });
        let fallbacks = self.fallback_impls(&provider_ty, &own_keys);

        let macro_name = Ident::new(
            &format!(
                "__cgp_with_{}",
                to_snake_case_str(&self.name.unraw().to_string())
            ),
            Span::call_site(),
        );
        let with_components = define_with_components_macro(&macro_name, &own_keys, &self.parents);

        let provider = EmptyStruct::new(
            Ident::new("Provider", Span::call_site()),
            Default::default(),
        )
        .with_doc(" Provider bundle for this preset.");

        let contains = PresetContains;
        let root = crate::crate_path::crate_root();
        let own_contains = own_keys.iter().map(|key| {
            let ty = &key.ty;
            let cfgs = &key.cfgs;
            let present = quote! {
                #(#cfgs)*
                impl #contains<#ty> for #name::Provider {}
            };
            let missing = key
                .fallback
                .as_ref()
                .filter(|_| !cfgs.is_empty())
                .map(|parent| {
                    let negated = negate_cfgs(cfgs);
                    quote! {
                        #negated
                        impl #contains<#ty> for #name::Provider
                        where
                            #parent::Provider: #contains<#ty>,
                        {}
                    }
                });
            quote! {
                #present
                #missing
            }
        });
        let inherited_contains = self.parents.iter().map(|parent| {
            quote! {
                #parent::with_components! {
                    @ #root::prelude::cgp_for_each,
                    exclude [ #(#own_key_types),* ],
                    __Component__ => {
                        impl #contains<__Component__> for #name::Provider
                        where
                            #parent::Provider: #contains<__Component__>,
                        {}
                    }
                }
            }
        });
        let override_checks = self.override_checks()?;
        let version = self.version.as_ref().map(|literal| {
            quote! {
                /// The preset version written as `#[version(N)]`.
                pub const VERSION: u64 = #literal;
            }
        });
        let inspect = crate::wiring::preset_inspect_item(self)?;
        let name = &self.name;

        Ok(quote! {
            #(#own_impls)*

            #(#parent_impls)*

            #fallbacks

            #(#own_contains)*

            #(#inherited_contains)*

            #override_checks

            #inspect

            #[allow(non_snake_case)]
            pub mod #name {
                #provider

                #version

                #with_components

                pub use #macro_name as with_components;
            }
        })
    }

    fn resolve_parent(&self, entry: &OverrideDelegateEntry) -> syn::Result<Path> {
        if let Some(parent) = &entry.parent {
            let named = parent.to_token_stream().to_string();
            let known = self
                .parents
                .iter()
                .any(|candidate| candidate.to_token_stream().to_string() == named);

            if !known {
                return Err(Error::new(
                    entry.span,
                    format!("`{named}` is not a parent of this preset"),
                ));
            }

            return Ok(parent.clone());
        }

        match self.parents.as_slice() {
            [only] => Ok(only.clone()),
            [] => Err(Error::new(
                entry.span,
                "`override` needs a parent preset (`Name: Parent { override ... }`)",
            )),
            _ => Err(Error::new(
                entry.span,
                "`override` must name the parent when the preset has more than one: `override(Parent) Key: Provider`",
            )),
        }
    }

    /// Keys the preset exposes, in source order. An `override` records its parent so a
    /// cfg-gated replacement falls back to that parent when the predicate is false.
    fn component_keys(&self, provider: &Type) -> syn::Result<Vec<ComponentKey>> {
        let mut keys = Vec::new();

        for statement in &self.entries.statements {
            for entry in statement.eval_entries(provider)? {
                keys.push(ComponentKey {
                    ty: entry.key,
                    cfgs: entry.cfgs,
                    fallback: None,
                });
            }
        }

        for mapping in &self.entries.entries {
            if let DelegateMapping::Override(override_entry) = mapping {
                let parent = self.resolve_parent(override_entry)?;

                for entry in override_entry.entry.eval_entries(provider)? {
                    keys.push(ComponentKey {
                        ty: entry.key,
                        cfgs: entry.cfgs,
                        fallback: Some(parent.clone()),
                    });
                }
            } else {
                for entry in mapping.eval_entries(provider)? {
                    keys.push(ComponentKey {
                        ty: entry.key,
                        cfgs: entry.cfgs,
                        fallback: None,
                    });
                }
            }
        }

        Ok(keys)
    }

    /// `not(cfg)` forwarding impls for cfg-gated overrides.
    ///
    /// The child's own provider impl is already gated by `build_impls`. When that
    /// predicate is false the parent still provides the component, so the preset
    /// keeps `DelegateComponent` and `IsProviderFor` by forwarding.
    fn fallback_impls(&self, provider: &Type, keys: &[ComponentKey]) -> TokenStream {
        let delegate = DelegateComponent;
        let is_provider = IsProviderFor;
        let mut impls = TokenStream::new();

        for key in keys {
            let Some(parent) = key.fallback.as_ref().filter(|_| !key.cfgs.is_empty()) else {
                continue;
            };

            let ty = &key.ty;
            let negated = negate_cfgs(&key.cfgs);
            impls.extend(quote! {
                #negated
                impl #delegate<#ty> for #provider
                where
                    #parent::Provider: #delegate<#ty>,
                {
                    type Delegate = <#parent::Provider as #delegate<#ty>>::Delegate;
                }

                #negated
                impl<__Context__, __Params__: ?Sized> #is_provider<#ty, __Context__, __Params__>
                    for #provider
                where
                    #parent::Provider: #delegate<#ty>,
                    <#parent::Provider as #delegate<#ty>>::Delegate:
                        #is_provider<#ty, __Context__, __Params__>,
                {}
            });
        }

        impls
    }

    /// A function per overridden key. It is type-checked even though nothing calls it,
    /// which is what rejects an override of a component the parent does not wire.
    ///
    /// A cfg-gated override is checked only when that predicate is true. The fallback
    /// impl is what keeps the component available in the other configuration.
    fn override_checks(&self) -> syn::Result<TokenStream> {
        let contains = PresetContains;
        let snake = to_snake_case_str(&self.name.unraw().to_string());
        let mut checks = TokenStream::new();
        let mut index = 0_u32;

        for entry in &self.entries.entries {
            let DelegateMapping::Override(override_entry) = entry else {
                continue;
            };

            let parent = self.resolve_parent(override_entry)?;
            let provider: Type = syn::parse_quote!(());
            let evaluated = override_entry.entry.eval_entries(&provider)?;

            for evaluated in evaluated {
                let fn_name = Ident::new(
                    &format!("__cgp_override_{snake}_{index}"),
                    override_entry.span,
                );
                index += 1;
                let key = evaluated.key;
                let cfgs = &evaluated.cfgs;

                checks.extend(quote! {
                    #(#cfgs)*
                    #[allow(dead_code, non_snake_case)]
                    fn #fn_name() {
                        fn assert_in_parent<__Component__>()
                        where
                            #parent::Provider: #contains<__Component__>,
                        {}

                        assert_in_parent::<#key>();
                    }
                });
            }
        }

        Ok(checks)
    }
}
