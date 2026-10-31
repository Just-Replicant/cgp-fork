use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, parse2};

use crate::ir::{Diagnostics, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use crate::macro_core::types::cgp_type::{ItemCgpType, extract_item_type_from_trait};

/// `#[cgp_type]`: a trait with one associated type becomes an abstract-type component.
pub struct CgpTypeMacro;

impl Pipeline for CgpTypeMacro {
    const NAME: &'static str = "cgp_type";
    const FAMILY: IrFamily = IrFamily::ComponentDef;
    type Ir = ItemCgpComponent;

    /// Parses the component, naming the provider `{Type}TypeProvider` by default.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let mut raw_args: CgpComponentRawArgs = parse2(attr)?;

        let item_trait: ItemTrait = parse2(body)?;

        let item_type = extract_item_type_from_trait(&item_trait)?;

        if raw_args.provider_ident.is_none() {
            raw_args.provider_ident = Some(Ident::new(
                &format!("{}TypeProvider", item_type.ident),
                item_type.ident.span(),
            ));
        }

        Ok(ItemCgpComponent {
            args: raw_args.try_into()?,
            item_trait,
        })
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let item_cgp_type = ItemCgpType {
            item_component: ir.preprocess()?.eval()?,
        };

        let items = item_cgp_type.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `#[cgp_type]` entry point.
pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpTypeMacro>(attrs, body)
}
