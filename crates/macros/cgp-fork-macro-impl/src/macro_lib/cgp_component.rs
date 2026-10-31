use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, parse2};

use crate::ir::{Diagnostics, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentArgs, ItemCgpComponent};

/// `#[cgp_component]`: a trait becomes a component (consumer trait, provider trait,
/// marker struct, and the standard providers).
pub struct CgpComponentMacro;

impl Pipeline for CgpComponentMacro {
    const NAME: &'static str = "cgp_component";
    const FAMILY: IrFamily = IrFamily::ComponentDef;
    type Ir = ItemCgpComponent;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let args: CgpComponentArgs = parse2(attr)?;
        let item_trait: ItemTrait = parse2(body)?;

        Ok(ItemCgpComponent { args, item_trait })
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    /// Runs the `preprocess → eval → to_items` stages of the component.
    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let derived = ir.preprocess()?.eval()?.to_items()?;

        Ok(quote! {
            #( #derived )*
        })
    }
}

/// `#[cgp_component]` entry point.
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpComponentMacro>(attr, item)
}
