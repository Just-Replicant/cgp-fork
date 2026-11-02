//! The `ProviderImpl` IR family: implementations of a component by a provider.

use proc_macro2::TokenStream;
use quote::quote;

use crate::extra_macro_core::types::cgp_computer::ItemCgpComputer;
use crate::extra_macro_core::types::cgp_handler::{ItemCgpHandler, lower_handler};
use crate::extra_macro_core::types::cgp_producer::ItemCgpProducer;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;
use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::functions::allow_deprecated_in;
use crate::macro_core::types::cgp_auto_error::{ItemCgpAutoError, ItemCgpErrorEnum};
use crate::macro_core::types::cgp_impl::ItemCgpImpl;
use crate::macro_core::types::cgp_provider::ItemCgpProvider;

/// A provider implementation.
pub enum ProviderImpl {
    /// `#[cgp_impl]`: an impl written against the consumer trait.
    Impl(Box<ItemCgpImpl>),
    /// `#[cgp_provider]` and `#[cgp_new_provider]` (`args.new` set): a provider-trait impl
    /// and its `IsProviderFor` impl.
    Provider(Box<ItemCgpProvider>),
    /// `#[cgp_computer]`: a function as a `Computer` provider with its promotions.
    Computer(Box<ItemCgpComputer>),
    /// `#[cgp_producer]`: a function as a `Producer` provider with its promotions.
    Producer(Box<ItemCgpProducer>),
    /// `#[cgp_handler]`: a function as a unified `Handler` or `AsyncHandler` provider.
    Handler(Box<ItemCgpHandler>),
    /// `#[cgp_auto_error]`: error-raising methods as error providers.
    AutoError(Box<ItemCgpAutoError>),
    /// `#[cgp_auto_error]` on an enum: `Display`, `Error`, and `From`.
    ErrorEnum(Box<ItemCgpErrorEnum>),
}

impl FamilyIr for ProviderImpl {
    const FAMILY: IrFamily = IrFamily::ProviderImpl;

    fn validate(&self, diagnostics: &mut Diagnostics) {
        if let ProviderImpl::ErrorEnum(item) = self {
            item.validate(diagnostics);
        }
    }

    /// Generates the provider's items.
    fn emit(self) -> syn::Result<TokenStream> {
        match self {
            ProviderImpl::Impl(item) => {
                let mut lowered = item.lower()?;

                let bare_impls = lowered.lower()?;

                let helper_items = &lowered.helper_items;
                let default_impls = &lowered.default_impls;

                Ok(quote! {
                    #(#helper_items)*
                    #bare_impls
                    #(#default_impls)*
                })
            }
            ProviderImpl::Provider(item) => {
                let lowered = item.lower()?;

                Ok(quote! {
                    #lowered
                })
            }
            // The 0.10 expansions name deprecated items; the user is warned at the macro
            // itself (`#[cgp_computer]` and `#[cgp_producer]` are deprecated), not by
            // the code it generates.
            ProviderImpl::Computer(item) => {
                allow_deprecated_in_tokens(lower_handler_fn(&item.preprocess()?.eval()?)?)
            }
            ProviderImpl::Producer(item) => {
                allow_deprecated_in_tokens(lower_handler_fn(&item.preprocess()?.eval()?)?)
            }
            ProviderImpl::Handler(item) => lower_handler(&item.eval()?),
            ProviderImpl::AutoError(item) => {
                let items = item.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
            ProviderImpl::ErrorEnum(item) => item.emit(),
        }
    }
}

/// `#[allow(deprecated)]` on every impl in `tokens`.
fn allow_deprecated_in_tokens(tokens: TokenStream) -> syn::Result<TokenStream> {
    let mut file: syn::File = syn::parse2(tokens)?;
    allow_deprecated_in(&mut file.items);
    let items = &file.items;
    Ok(quote! { #( #items )* })
}
