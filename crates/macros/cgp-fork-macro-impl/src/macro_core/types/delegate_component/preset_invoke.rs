use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident, Path, Type};

use crate::macro_core::exports::{DelegateComponent, IsProviderFor};

/// Emit `Preset::with_components!` so one preset entry becomes a
/// `DelegateComponent` / `IsProviderFor` pair per component. `exclude` drops
/// keys the surrounding table already implements, which is how a later entry
/// overrides a preset.
pub fn invoke_with_components(
    preset_module: &Path,
    table_type: &Type,
    generics: &Generics,
    exclude: &[Type],
) -> TokenStream {
    let (impl_generics, _, _) = generics.split_for_impl();

    let mut check_generics = generics.clone();
    check_generics.params.push(syn::parse_quote!(__Context__));
    check_generics
        .params
        .push(syn::parse_quote!(__Params__: ?Sized));
    let (check_impl_generics, _, _) = check_generics.split_for_impl();

    let existing_predicates = generics
        .where_clause
        .as_ref()
        .map(|clause| &clause.predicates);

    // Resolved in the crate that is expanding this macro (the preset's user). The full
    // path is passed in: a `macro_rules` `$path::item` paste does not produce a macro call.
    let root = crate::crate_path::crate_root();

    quote! {
        #preset_module::with_components! {
            @ #root::prelude::cgp_for_each,
            exclude [ #(#exclude),* ],
            __Component__ => {
                impl #impl_generics #DelegateComponent<__Component__> for #table_type
                where
                    #existing_predicates
                    #preset_module::Provider: #DelegateComponent<__Component__>,
                {
                    type Delegate = <#preset_module::Provider as #DelegateComponent<__Component__>>::Delegate;
                }

                impl #check_impl_generics #IsProviderFor<__Component__, __Context__, __Params__>
                    for #table_type
                where
                    #existing_predicates
                    #preset_module::Provider: #DelegateComponent<__Component__>,
                    <#preset_module::Provider as #DelegateComponent<__Component__>>::Delegate:
                        #IsProviderFor<__Component__, __Context__, __Params__>,
                {}
            }
        }
    }
}

/// The `with_components!` macro a preset module exports. It repeats `body` for
/// each of the preset's own keys, then forwards to each parent preset.
///
/// `#[macro_export]` places `#macro_name` at the crate root so `pub use` can
/// re-export it. A plain `macro_rules` cannot be `pub`.
pub fn define_with_components_macro(
    macro_name: &Ident,
    keys: &[Type],
    parents: &[Path],
) -> TokenStream {
    // `@ $for_each` is the `cgp_for_each` path of the crate that expands the macro, passed by
    // [`invoke_with_components`]. Baking `::cgp_fork` in here would name the facade as the
    // preset's own crate sees it.
    quote! {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! #macro_name {
            ( @ $for_each:path, $ph:ident => { $($body:tt)* } ) => {
                #macro_name! { @ $for_each, exclude [], $ph => { $($body)* } }
            };
            ( @ $for_each:path, exclude [ $($exclude:ty),* $(,)? ], $ph:ident => { $($body:tt)* } ) => {
                $for_each! {
                    [ #(#keys),* ],
                    exclude [ $($exclude),* ],
                    $ph => { $($body)* }
                }
                #(
                    #parents::with_components! {
                        @ $for_each,
                        exclude [ $($exclude),* ],
                        $ph => { $($body)* }
                    }
                )*
            };
        }
    }
}
