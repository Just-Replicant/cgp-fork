use proc_macro2::TokenStream;
use syn::{ItemImpl, parse2};

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Pipeline, ProviderImpl, run};
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};

/// `#[cgp_provider]`: a provider-trait impl gets its `IsProviderFor` impl.
pub struct CgpProviderMacro;

impl Pipeline for CgpProviderMacro {
    const NAME: &'static str = "cgp_provider";
    const FAMILY: IrFamily = ProviderImpl::FAMILY;
    type Ir = ProviderImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse2(body)?;
        let args: ProviderArgs = parse2(attr)?;

        Ok(ProviderImpl::Provider(Box::new(ItemCgpProvider {
            args,
            item_impl,
        })))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `cgp_provider` entry point.
pub fn cgp_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpProviderMacro>(attr, body)
}
