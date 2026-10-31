//! The `ProviderImpl` IR family: implementations of a component by a provider.

use proc_macro2::TokenStream;
use quote::quote;

use crate::extra_macro_core::types::cgp_computer::ItemCgpComputer;
use crate::extra_macro_core::types::cgp_producer::ItemCgpProducer;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;
use crate::ir::{Diagnostics, FamilyIr, IrFamily};
use crate::macro_core::types::cgp_auto_error::ItemCgpAutoError;
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
    /// `#[cgp_auto_error]`: error-raising methods as error providers.
    AutoError(Box<ItemCgpAutoError>),
}

impl FamilyIr for ProviderImpl {
    const FAMILY: IrFamily = IrFamily::ProviderImpl;

    /// No family-wide checks yet; each variant still checks while lowering.
    fn validate(&self, diagnostics: &mut Diagnostics) {
        let _ = diagnostics;
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
            ProviderImpl::Computer(item) => lower_handler_fn(&item.preprocess()?.eval()?),
            ProviderImpl::Producer(item) => lower_handler_fn(&item.preprocess()?.eval()?),
            ProviderImpl::AutoError(item) => {
                let items = item.to_items()?;

                Ok(quote! {
                    #( #items )*
                })
            }
        }
    }
}
