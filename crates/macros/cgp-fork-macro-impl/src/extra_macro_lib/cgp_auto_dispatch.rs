use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ItemTrait, parse2};

use crate::extra_macro_core::types::cgp_auto_dispatch::ItemCgpAutoDispatch;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;
use crate::ir::{IrFamily, Pipeline, run};

/// `#[cgp_auto_dispatch]`: a trait implemented for enums by dispatching to each variant.
pub struct CgpAutoDispatchMacro;

impl Pipeline for CgpAutoDispatchMacro {
    const NAME: &'static str = "cgp_auto_dispatch";
    const FAMILY: IrFamily = IrFamily::ComponentDef;
    type Ir = ItemCgpAutoDispatch;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        if !attr.is_empty() {
            return Err(Error::new_spanned(
                attr,
                "`#[cgp_auto_dispatch]` takes no arguments",
            ));
        }

        let item_trait: ItemTrait = parse2(body)?;

        Ok(ItemCgpAutoDispatch { item_trait })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let evaluated = ir.preprocess()?.eval()?;

        let item_trait = &evaluated.item_trait;
        let blanket_impl = &evaluated.blanket_impl;

        let mut output = quote! {
            #item_trait
            #blanket_impl
        };

        for computer in &evaluated.computers {
            output.extend(lower_handler_fn(&computer.preprocess()?.eval()?)?);
        }

        Ok(output)
    }
}

/// `cgp_auto_dispatch` entry point.
pub fn cgp_auto_dispatch(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoDispatchMacro>(attr, body)
}
