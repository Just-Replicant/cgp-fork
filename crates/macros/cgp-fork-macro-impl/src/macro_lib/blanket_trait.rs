use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemTrait, parse2};

use crate::ir::{IrFamily, Pipeline, run};
use crate::macro_core::types::blanket_trait::ItemBlanketTrait;

/// `#[blanket_trait]`: a trait whose supertraits imply it gets a blanket impl.
pub struct BlanketTraitMacro;

impl Pipeline for BlanketTraitMacro {
    const NAME: &'static str = "blanket_trait";
    const FAMILY: IrFamily = IrFamily::TraitRewrite;
    type Ir = ItemBlanketTrait;

    /// The attribute optionally names the context type of the blanket impl
    /// (`__Context__` by default).
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        let context_ident: Ident = if attr.is_empty() {
            Ident::new("__Context__", Span::call_site())
        } else {
            parse2(attr)?
        };

        let item_trait: ItemTrait = parse2(body)?;

        Ok(ItemBlanketTrait {
            context_ident,
            item_trait,
        })
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        let items = ir.to_items()?;

        Ok(quote! {
            #( #items )*
        })
    }
}

/// `blanket_trait` entry point.
pub fn blanket_trait(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<BlanketTraitMacro>(attr, body)
}
