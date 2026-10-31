use proc_macro2::TokenStream;
use syn::{ItemTrait, parse2};

use crate::ir::{ComponentDef, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentArgs, ItemCgpComponent};

/// `#[cgp_component]`: a trait becomes a component (consumer trait, provider trait,
/// marker struct, and the standard providers).
pub struct CgpComponentMacro;

impl Pipeline for CgpComponentMacro {
    const NAME: &'static str = "cgp_component";
    type Ir = ComponentDef;

    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let args: CgpComponentArgs = parse2(attr)?;
        let item_trait: ItemTrait = parse2(body)?;

        Ok(ComponentDef::Component(ItemCgpComponent {
            args,
            item_trait,
        }))
    }
}

/// `cgp_component` entry point.
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpComponentMacro>(attr, item)
}
