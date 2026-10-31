use proc_macro2::TokenStream;
use syn::{ItemImpl, parse2};

use crate::ir::{Diagnostics, FamilyIr, IrFamily, Pipeline, ProviderImpl, run};
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::macro_core::types::keyword::Keyword;

/// `#[cgp_new_provider]`: `#[cgp_provider]` that also declares the provider struct.
pub struct CgpNewProviderMacro;

impl Pipeline for CgpNewProviderMacro {
    const NAME: &'static str = "cgp_new_provider";
    const FAMILY: IrFamily = ProviderImpl::FAMILY;
    type Ir = ProviderImpl;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_impl: ItemImpl = parse2(body)?;
        let mut args: ProviderArgs = parse2(attr)?;
        args.new = Some(Keyword::default());

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

/// `cgp_new_provider` entry point.
pub fn cgp_new_provider(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpNewProviderMacro>(attr, body)
}
