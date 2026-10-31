use proc_macro2::TokenStream;
use syn::{Ident, ItemFn, parse2};

use crate::extra_macro_core::types::cgp_producer::ItemCgpProducer;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;
use crate::ir::{IrFamily, Pipeline, run};

/// `#[cgp_producer]`: a function becomes a `Producer` provider with its promotions.
pub struct CgpProducerMacro;

impl Pipeline for CgpProducerMacro {
    const NAME: &'static str = "cgp_producer";
    const FAMILY: IrFamily = IrFamily::ProviderImpl;
    type Ir = ItemCgpProducer;

    /// The attribute optionally names the provider.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let item_fn: ItemFn = parse2(body)?;

        let ident: Option<Ident> = parse2(attr)?;

        Ok(ItemCgpProducer { ident, item_fn })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        lower_handler_fn(&ir.preprocess()?.eval()?)
    }
}

/// `cgp_producer` entry point.
pub fn cgp_producer(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpProducerMacro>(attr, body)
}
