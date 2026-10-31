use proc_macro2::TokenStream;
use syn::{Error, ItemTrait, parse2};

use crate::extra_macro_core::types::cgp_auto_dispatch::ItemCgpAutoDispatch;
use crate::ir::{ComponentDef, Diagnostics, FamilyIr, IrFamily, Pipeline, run};

/// `#[cgp_auto_dispatch]`: a trait implemented for enums by dispatching to each variant.
pub struct CgpAutoDispatchMacro;

impl Pipeline for CgpAutoDispatchMacro {
    const NAME: &'static str = "cgp_auto_dispatch";
    const FAMILY: IrFamily = ComponentDef::FAMILY;
    type Ir = ComponentDef;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        if !attr.is_empty() {
            return Err(Error::new_spanned(
                attr,
                "`#[cgp_auto_dispatch]` takes no arguments",
            ));
        }

        let item_trait: ItemTrait = parse2(body)?;

        Ok(ComponentDef::AutoDispatch(ItemCgpAutoDispatch {
            item_trait,
        }))
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// `cgp_auto_dispatch` entry point.
pub fn cgp_auto_dispatch(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpAutoDispatchMacro>(attr, body)
}
