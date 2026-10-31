use proc_macro2::TokenStream;
use quote::quote;

use crate::extra_macro_core::exports::CgpCompat010;
use crate::extra_macro_core::types::cgp_handler::EvaluatedCgpHandler;
use crate::macro_core::functions::allow_deprecated_in;

/// Lower `#[cgp_handler]`: the function, the provider impl with its `IsProviderFor`
/// impl and struct, the async promotion, and the 0.10 wiring. The 0.10 wiring names
/// deprecated components, so its impls allow `deprecated`, and it goes through the
/// facade's `__cgp_compat_0_10!`, which drops it when `compat-0.10` is off.
pub fn lower_handler(evaluated: &EvaluatedCgpHandler) -> syn::Result<TokenStream> {
    let item_fn = &evaluated.item_fn;
    let provider = evaluated.provider.lower()?;

    let delegate_table = match &evaluated.delegate_table {
        Some(table) => {
            let table = table.eval()?;
            quote! { #table }
        }
        None => TokenStream::new(),
    };

    let legacy_table = match &evaluated.legacy_table {
        Some(table) => {
            let table = table.eval()?;
            let mut file: syn::File = syn::parse2(quote! { #table })?;
            allow_deprecated_in(&mut file.items);
            let items = &file.items;
            quote! {
                #CgpCompat010! {
                    #( #items )*
                }
            }
        }
        None => TokenStream::new(),
    };

    Ok(quote! {
        #item_fn
        #provider
        #delegate_table
        #legacy_table
    })
}
